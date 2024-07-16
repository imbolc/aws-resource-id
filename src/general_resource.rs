//! # IDs following the general AWS resources format
//!
//! The general resource consists of two parts:
//! - a prefix identifying the resource e.g. `ami-` for AMI
//! - a random alfanumeric 8 or 17 characters long unique string
//!
//! > ## Resource ID length
//! > Prior to January 2016, the IDs assigned to newly created resources of
//! > certain resource types used 8 characters after the hyphen (for example,
//! > i-1a2b3c4d). From January 2016 to June 2018, we changed the IDs of these
//! > resource types to use 17 characters after the hyphen (for example,
//! > i-1234567890abcdef0). Depending on when your account was created, you
//! > might have some existing resources with short IDs, however, any new
//! > resources will receive the longer IDs.
//! > <https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/resource-ids.html>
use sqlx::{
    postgres::{PgTypeInfo, PgValueRef},
    Postgres, Type,
};
use std::{convert::TryFrom, fmt};

/// Gerenal AWS resource error
#[derive(Debug, thiserror::Error)]
#[error("initialization of `{target_type}` by \"{input}\" errored with: {error_detail}")]
pub struct Error {
    /// Target type e.g. [`AwsAmiId`]
    target_type: &'static str,
    /// Initialization input
    input: String,
    /// Error detail
    error_detail: ErrorDetail,
}

/// Details on general AWS resource error
#[derive(Debug, thiserror::Error)]
pub enum ErrorDetail {
    /// Wrong prefix, e.g. not an `ami-` for [`AwsAmiId`]
    #[error("wrong prefix, expected {0}")]
    WrongPrefix(&'static str),
    /// Wrong length of the unique part of the ID
    #[error("the unique part must be 8 or 17 characters, not {0} long")]
    IdLength(usize),
    /// The unique part of the ID contains non-alfanumeric-ASCII characters
    #[error("the unique part contains non ascii alfanumeric characters")]
    NonAsciiAlphanumeric,
}

/// AWS AMI (Amazon Machine Image) ID
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AwsAmiId(AwsResourceId);

/// AWS Snapshot ID
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AwsSnapshotId(AwsResourceId);

/// The alphanumeric part of an AWS resource id
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum AwsResourceId {
    C8([u8; 8]),
    C17([u8; 17]),
}

impl AwsResourceId {
    fn as_slice(&self) -> &[u8] {
        match self {
            Self::C8(x) => x,
            Self::C17(x) => x,
        }
    }
}

macro_rules! impl_resource_id {
    ($type:ident, $prefix:literal) => {
        impl $type {
            const PREFIX: &'static str = $prefix;
        }

        impl TryFrom<&str> for $type {
            type Error = Error;

            fn try_from(s: &str) -> Result<Self, Self::Error> {
                if !s.starts_with(Self::PREFIX) {
                    return Err(Error::new(
                        short_type_name::<$type>(),
                        s,
                        ErrorDetail::WrongPrefix(Self::PREFIX),
                    ));
                }
                if !s[Self::PREFIX.len()..]
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric())
                {
                    return Err(Error::new(
                        short_type_name::<$type>(),
                        s,
                        ErrorDetail::NonAsciiAlphanumeric,
                    ));
                }

                let id = &s[Self::PREFIX.len()..];
                if id.len() == 8 {
                    let mut arr = [0u8; 8];
                    arr.copy_from_slice(id.as_bytes());
                    Ok($type(AwsResourceId::C8(arr)))
                } else if id.len() == 17 {
                    let mut arr = [0u8; 17];
                    arr.copy_from_slice(id.as_bytes());
                    Ok($type(AwsResourceId::C17(arr)))
                } else {
                    return Err(Error::new(
                        short_type_name::<$type>(),
                        s,
                        ErrorDetail::IdLength(id.len()),
                    ));
                }
            }
        }

        impl TryFrom<String> for $type {
            type Error = Error;

            fn try_from(s: String) -> Result<Self, Self::Error> {
                Self::try_from(s.as_str())
            }
        }

        impl fmt::Display for $type {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", Self::PREFIX)?;
                write!(
                    f,
                    "{}",
                    std::str::from_utf8(self.0.as_slice()).unwrap_or_default()
                )
            }
        }

        impl fmt::Debug for $type {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_tuple(short_type_name::<Self>())
                    .field(&self.to_string())
                    .finish()
            }
        }

        impl From<$type> for String {
            fn from(value: $type) -> Self {
                value.to_string()
            }
        }

        impl Type<Postgres> for $type {
            fn type_info() -> PgTypeInfo {
                <&str as Type<Postgres>>::type_info()
            }
        }

        impl<'q> sqlx::encode::Encode<'q, Postgres> for $type {
            fn encode_by_ref(
                &self,
                buf: &mut sqlx::postgres::PgArgumentBuffer,
            ) -> sqlx::encode::IsNull {
                <String as sqlx::encode::Encode<Postgres>>::encode_by_ref(&self.to_string(), buf)
            }
        }

        impl<'r> sqlx::decode::Decode<'r, Postgres> for $type {
            fn decode(
                value: PgValueRef<'r>,
            ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
                let s = <&str as sqlx::decode::Decode<Postgres>>::decode(value)?;
                Ok($type::try_from(s).map_err(|e| Box::new(sqlx::Error::Decode(e.into())))?)
            }
        }

        impl serde::Serialize for $type {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_str(&self.to_string())
            }
        }

        impl<'de> serde::Deserialize<'de> for $type {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                $type::try_from(s).map_err(serde::de::Error::custom)
            }
        }
    };
}

impl_resource_id!(AwsAmiId, "ami-");
impl_resource_id!(AwsSnapshotId, "snap-");

fn short_type_name<T>() -> &'static str {
    let name = std::any::type_name::<T>();
    name.split("::").last().unwrap_or(name)
}

impl Error {
    fn new(target_type: &'static str, input: impl Into<String>, error_detail: ErrorDetail) -> Self {
        Self {
            target_type,
            input: input.into(),
            error_detail,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ami(s: &str) -> AwsAmiId {
        AwsAmiId::try_from(s).unwrap()
    }

    #[test]
    fn test_fmt_display() {
        assert_eq!(format!("{}", ami("ami-12345678")), "ami-12345678");
    }

    #[test]
    fn test_fmt_debug() {
        assert_eq!(
            format!("{:?}", ami("ami-12345678")),
            r#"AwsAmiId("ami-12345678")"#
        );
    }

    #[test]
    fn test_serialize() {
        assert_eq!(
            serde_json::to_string(&ami("ami-12345678")).unwrap(),
            "\"ami-12345678\""
        );
    }

    #[test]
    fn test_deserialize() {
        assert_eq!(
            serde_json::from_str::<AwsAmiId>("\"ami-12345678\"").unwrap(),
            ami("ami-12345678"),
        );
    }

    #[test]
    fn test_wrong_prefix() {
        let result = AwsAmiId::try_from("amx-12345678");
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "initialization of `AwsAmiId` by \"amx-12345678\" errored with: wrong prefix, expected ami-"
        );
    }

    #[test]
    fn test_error_wrong_length() {
        let result = AwsAmiId::try_from("ami-1234567");
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "initialization of `AwsAmiId` by \"ami-1234567\" errored with: the unique part must be 8 or 17 characters, not 7 long"
        );

        let result = AwsAmiId::try_from("ami-123456789012345678");
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "initialization of `AwsAmiId` by \"ami-123456789012345678\" errored with: the unique part must be 8 or 17 characters, not 18 long"
        );
    }

    #[test]
    fn test_error_non_alphanumeric() {
        let result = AwsAmiId::try_from("ami-1234567!");
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "initialization of `AwsAmiId` by \"ami-1234567!\" errored with: the unique part contains non ascii alfanumeric characters"
        );
    }

    #[test]
    fn test_valid_ids() {
        assert!(AwsAmiId::try_from("ami-1234abcd").is_ok());
        assert!(AwsAmiId::try_from("ami-1a2b3c4d5e6f7j8h9").is_ok());
        assert!(AwsSnapshotId::try_from("snap-1234abcd").is_ok());
        assert!(AwsSnapshotId::try_from("snap-1a2b3c4d5e6f7j8h9").is_ok());
    }
}
