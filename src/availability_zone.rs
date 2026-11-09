//! # AWS Availability Zone
use crate::{AwsRegionId, RegionError};
use std::{convert::TryFrom, fmt, str::FromStr};

/// Error encountered when parsing an AWS availability zone name
#[derive(Debug, thiserror::Error)]
pub enum AvailabilityZoneError {
    /// The name does not end in a lowercase ASCII letter
    #[error("Invalid availability zone suffix: {0}")]
    InvalidSuffix(String),
    /// Parsing the region part of the availability zone name
    #[error(transparent)]
    Region(#[from] RegionError),
}

/// AWS standard availability zone name, such as `us-east-1a`
///
/// Accepts an [`AwsRegionId`] followed by one lowercase ASCII letter. This
/// validates the format, not whether the zone exists or is available to an
/// account. AZ IDs, Local Zones, and Wavelength Zones are not supported.
///
/// ```
/// use aws_resource_id::AwsAvailabilityZone;
///
/// let zone: AwsAvailabilityZone = "us-east-1a".parse().unwrap();
/// assert_eq!(zone.to_string(), "us-east-1a");
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AwsAvailabilityZone {
    region: AwsRegionId,
    zone: u8,
}

impl TryFrom<&str> for AwsAvailabilityZone {
    type Error = crate::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let mut chars = s.chars();
        let zone = chars
            .next_back()
            .filter(char::is_ascii_lowercase)
            .ok_or_else(|| AvailabilityZoneError::InvalidSuffix(s.into()))?;
        let region = AwsRegionId::try_from(chars.as_str()).map_err(|error| match error {
            crate::Error::Region(error) => AvailabilityZoneError::Region(error).into(),
            error => error,
        })?;
        Ok(Self {
            region,
            zone: zone as u8,
        })
    }
}

impl TryFrom<String> for AwsAvailabilityZone {
    type Error = crate::Error;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::try_from(s.as_str())
    }
}

impl TryFrom<&String> for AwsAvailabilityZone {
    type Error = crate::Error;

    fn try_from(s: &String) -> Result<Self, Self::Error> {
        Self::try_from(s.as_str())
    }
}

impl FromStr for AwsAvailabilityZone {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}

impl fmt::Display for AwsAvailabilityZone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.region, char::from(self.zone))
    }
}

impl From<AwsAvailabilityZone> for String {
    fn from(value: AwsAvailabilityZone) -> Self {
        value.to_string()
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for AwsAvailabilityZone {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::try_from(s).map_err(serde::de::Error::custom)
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for AwsAvailabilityZone {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[cfg(feature = "sqlx-postgres")]
mod sqlx_impl {
    use super::AwsAvailabilityZone;
    use sqlx::{
        Postgres, Type,
        postgres::{PgTypeInfo, PgValueRef},
    };

    impl Type<Postgres> for AwsAvailabilityZone {
        fn type_info() -> PgTypeInfo {
            <String as Type<Postgres>>::type_info()
        }

        fn compatible(ty: &PgTypeInfo) -> bool {
            <String as Type<Postgres>>::compatible(ty)
        }
    }

    impl sqlx::Encode<'_, Postgres> for AwsAvailabilityZone {
        fn encode_by_ref(
            &self,
            buf: &mut sqlx::postgres::PgArgumentBuffer,
        ) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
            <String as sqlx::Encode<Postgres>>::encode_by_ref(&self.to_string(), buf)
        }
    }

    impl<'r> sqlx::Decode<'r, Postgres> for AwsAvailabilityZone {
        fn decode(value: PgValueRef<'r>) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
            let s = <&str as sqlx::Decode<Postgres>>::decode(value)?;
            Ok(Self::try_from(s)?)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_zones() {
        for name in ["us-east-1a", "us-east-1f", "us-west-2d", "eu-central-1b"] {
            let zone: AwsAvailabilityZone = name.parse().unwrap();
            assert_eq!(zone.to_string(), name);
        }

        for letter in 'a'..='z' {
            let name = format!("us-east-1{letter}");
            let zone: AwsAvailabilityZone = name.parse().unwrap();
            assert_eq!(zone.to_string(), name);
        }
    }

    #[test]
    fn test_invalid_suffixes() {
        for name in [
            "",
            "us-east-1",
            "us-east-1A",
            "us-east-10",
            "us-east-1a ",
            "us-east-1é",
            "us-east-1💥",
            "💥",
            "use1-az1",
            "us-east-1-wl1-bos-wlz-1",
        ] {
            assert!(
                matches!(
                    name.parse::<AwsAvailabilityZone>(),
                    Err(crate::Error::AvailabilityZone(
                        AvailabilityZoneError::InvalidSuffix(_)
                    ))
                ),
                "accepted invalid zone: {name}"
            );
        }
    }

    #[test]
    fn test_invalid_regions() {
        for name in [
            "a",
            "us-east-1aa",
            "us-east-1-a",
            " us-east-1a",
            "éa",
            "unknown-1a",
            "us-west-2-lax-1a",
        ] {
            assert!(
                matches!(
                    name.parse::<AwsAvailabilityZone>(),
                    Err(crate::Error::AvailabilityZone(
                        AvailabilityZoneError::Region(_)
                    ))
                ),
                "expected a region error for: {name}"
            );
        }

        let error = "unknown-1a".parse::<AwsAvailabilityZone>().unwrap_err();
        assert_eq!(error.to_string(), "Unknown region: unknown-1");
    }

    #[test]
    fn test_string_conversions() {
        let name = "eu-central-1a".to_string();
        let zone = AwsAvailabilityZone::try_from(name.as_str()).unwrap();
        assert_eq!(AwsAvailabilityZone::try_from(&name).unwrap(), zone);
        assert_eq!(AwsAvailabilityZone::try_from(name.clone()).unwrap(), zone);
        assert_eq!(String::from(zone), name);
    }

    #[test]
    fn test_copy_and_size() {
        fn assert_copy<T: Copy>() {}
        assert_copy::<AwsAvailabilityZone>();
        assert_eq!(std::mem::size_of::<AwsAvailabilityZone>(), 2);
    }
}

#[cfg(feature = "serde")]
#[cfg(test)]
mod serde_tests {
    use super::*;

    #[test]
    fn test_round_trip() {
        let zone: AwsAvailabilityZone = "us-east-1f".parse().unwrap();
        let serialized = serde_json::to_string(&zone).unwrap();
        assert_eq!(serialized, "\"us-east-1f\"");
        assert_eq!(
            serde_json::from_str::<AwsAvailabilityZone>(&serialized).unwrap(),
            zone
        );
    }

    #[test]
    fn test_invalid_zone() {
        for json in ["\"us-east-1A\"", "\"unknown-1a\"", "null", "42"] {
            assert!(serde_json::from_str::<AwsAvailabilityZone>(json).is_err());
        }
    }
}

#[cfg(feature = "sqlx-postgres")]
#[cfg(test)]
mod sqlx_tests {
    use super::*;
    use sqlx::PgPool;

    #[sqlx::test]
    async fn test_text_round_trip(pool: PgPool) -> sqlx::Result<()> {
        let zone: AwsAvailabilityZone = "us-east-1f".parse().unwrap();
        let serialized = sqlx::query_scalar!("SELECT $1::text", zone as _)
            .fetch_one(&pool)
            .await?
            .unwrap();
        assert_eq!(serialized, "us-east-1f");
        let deserialized = sqlx::query_scalar!(
            r#"SELECT $1::text as "val: AwsAvailabilityZone""#,
            serialized
        )
        .fetch_one(&pool)
        .await?
        .unwrap();
        assert_eq!(deserialized, zone);
        Ok(())
    }

    #[sqlx::test]
    async fn test_varchar_round_trip(pool: PgPool) -> sqlx::Result<()> {
        let zone: AwsAvailabilityZone = "eu-central-1b".parse().unwrap();
        let serialized = sqlx::query_scalar!("SELECT $1::varchar", zone as _)
            .fetch_one(&pool)
            .await?
            .unwrap();
        assert_eq!(serialized, "eu-central-1b");
        let deserialized = sqlx::query_scalar!(
            r#"SELECT $1::varchar as "val: AwsAvailabilityZone""#,
            serialized
        )
        .fetch_one(&pool)
        .await?
        .unwrap();
        assert_eq!(deserialized, zone);
        Ok(())
    }

    #[sqlx::test]
    async fn test_invalid_zone(pool: PgPool) -> sqlx::Result<()> {
        let result =
            sqlx::query_scalar!(r#"SELECT 'us-east-1A'::text AS "val: AwsAvailabilityZone""#)
                .fetch_one(&pool)
                .await;
        assert!(matches!(result, Err(sqlx::Error::ColumnDecode { .. })));
        Ok(())
    }
}
