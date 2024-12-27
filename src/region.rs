//! # AWS Region ID
use std::{convert::TryFrom, fmt, str::FromStr};

/// Error encountered when parsing an AWS region
#[derive(Debug, thiserror::Error)]
#[error("Unknown region: {0}")]
pub struct RegionError(String);

/// AWS Region ID
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AwsRegionId {
    /// Africa (Cape Town)
    AfSouth1,
    /// Asia Pacific (Hong Kong)
    ApEast1,
    /// Asia Pacific (Tokyo)
    ApNortheast1,
    /// Asia Pacific (Seoul)
    ApNortheast2,
    /// Asia Pacific (Osaka)
    ApNortheast3,
    /// Asia Pacific (Mumbai)
    ApSouth1,
    /// Asia Pacific (Hyderabad)
    ApSouth2,
    /// Asia Pacific (Singapore)
    ApSoutheast1,
    /// Asia Pacific (Sydney)
    ApSoutheast2,
    /// Asia Pacific (Jakarta)
    ApSoutheast3,
    /// Asia Pacific (Melbourne)
    ApSoutheast4,
    /// Canada (Central)
    CaCentral1,
    /// Canada West (Calgary)
    CaWest1,
    /// Europe (Frankfurt)
    EuCentral1,
    /// Europe (Zurich)
    EuCentral2,
    /// Europe (Stockholm)
    EuNorth1,
    /// Europe (Milan)
    EuSouth1,
    /// Europe (Spain)
    EuSouth2,
    /// Europe (Ireland)
    EuWest1,
    /// Europe (London)
    EuWest2,
    /// Europe (Paris)
    EuWest3,
    /// Israel (Tel Aviv)
    IlCentral1,
    /// Middle East (UAE)
    MeCentral1,
    /// Middle East (Bahrain)
    MeSouth1,
    /// South America (São Paulo)
    SaEast1,
    /// US East (N. Virginia)
    UsEast1,
    /// US East (Ohio)
    UsEast2,
    /// US West (N. California)
    UsWest1,
    /// US West (Oregon)
    UsWest2,
}

impl TryFrom<&str> for AwsRegionId {
    type Error = crate::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "af-south-1" => Ok(AwsRegionId::AfSouth1),
            "ap-east-1" => Ok(AwsRegionId::ApEast1),
            "ap-northeast-1" => Ok(AwsRegionId::ApNortheast1),
            "ap-northeast-2" => Ok(AwsRegionId::ApNortheast2),
            "ap-northeast-3" => Ok(AwsRegionId::ApNortheast3),
            "ap-south-1" => Ok(AwsRegionId::ApSouth1),
            "ap-south-2" => Ok(AwsRegionId::ApSouth2),
            "ap-southeast-1" => Ok(AwsRegionId::ApSoutheast1),
            "ap-southeast-2" => Ok(AwsRegionId::ApSoutheast2),
            "ap-southeast-3" => Ok(AwsRegionId::ApSoutheast3),
            "ap-southeast-4" => Ok(AwsRegionId::ApSoutheast4),
            "ca-central-1" => Ok(AwsRegionId::CaCentral1),
            "ca-west-1" => Ok(AwsRegionId::CaWest1),
            "eu-central-1" => Ok(AwsRegionId::EuCentral1),
            "eu-central-2" => Ok(AwsRegionId::EuCentral2),
            "eu-north-1" => Ok(AwsRegionId::EuNorth1),
            "eu-south-1" => Ok(AwsRegionId::EuSouth1),
            "eu-south-2" => Ok(AwsRegionId::EuSouth2),
            "eu-west-1" => Ok(AwsRegionId::EuWest1),
            "eu-west-2" => Ok(AwsRegionId::EuWest2),
            "eu-west-3" => Ok(AwsRegionId::EuWest3),
            "il-central-1" => Ok(AwsRegionId::IlCentral1),
            "me-central-1" => Ok(AwsRegionId::MeCentral1),
            "me-south-1" => Ok(AwsRegionId::MeSouth1),
            "sa-east-1" => Ok(AwsRegionId::SaEast1),
            "us-east-1" => Ok(AwsRegionId::UsEast1),
            "us-east-2" => Ok(AwsRegionId::UsEast2),
            "us-west-1" => Ok(AwsRegionId::UsWest1),
            "us-west-2" => Ok(AwsRegionId::UsWest2),
            _ => Err(RegionError(s.into()).into()),
        }
    }
}

impl From<AwsRegionId> for &'static str {
    fn from(region: AwsRegionId) -> Self {
        match region {
            AwsRegionId::AfSouth1 => "af-south-1",
            AwsRegionId::ApEast1 => "ap-east-1",
            AwsRegionId::ApNortheast1 => "ap-northeast-1",
            AwsRegionId::ApNortheast2 => "ap-northeast-2",
            AwsRegionId::ApNortheast3 => "ap-northeast-3",
            AwsRegionId::ApSouth1 => "ap-south-1",
            AwsRegionId::ApSouth2 => "ap-south-2",
            AwsRegionId::ApSoutheast1 => "ap-southeast-1",
            AwsRegionId::ApSoutheast2 => "ap-southeast-2",
            AwsRegionId::ApSoutheast3 => "ap-southeast-3",
            AwsRegionId::ApSoutheast4 => "ap-southeast-4",
            AwsRegionId::CaCentral1 => "ca-central-1",
            AwsRegionId::CaWest1 => "ca-west-1",
            AwsRegionId::EuCentral1 => "eu-central-1",
            AwsRegionId::EuCentral2 => "eu-central-2",
            AwsRegionId::EuNorth1 => "eu-north-1",
            AwsRegionId::EuSouth1 => "eu-south-1",
            AwsRegionId::EuSouth2 => "eu-south-2",
            AwsRegionId::EuWest1 => "eu-west-1",
            AwsRegionId::EuWest2 => "eu-west-2",
            AwsRegionId::EuWest3 => "eu-west-3",
            AwsRegionId::IlCentral1 => "il-central-1",
            AwsRegionId::MeCentral1 => "me-central-1",
            AwsRegionId::MeSouth1 => "me-south-1",
            AwsRegionId::SaEast1 => "sa-east-1",
            AwsRegionId::UsEast1 => "us-east-1",
            AwsRegionId::UsEast2 => "us-east-2",
            AwsRegionId::UsWest1 => "us-west-1",
            AwsRegionId::UsWest2 => "us-west-2",
        }
    }
}

impl AsRef<str> for AwsRegionId {
    fn as_ref(&self) -> &str {
        (*self).into()
    }
}

impl TryFrom<String> for AwsRegionId {
    type Error = crate::Error;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::try_from(s.as_str())
    }
}

impl TryFrom<&String> for AwsRegionId {
    type Error = crate::Error;

    fn try_from(s: &String) -> Result<Self, Self::Error> {
        Self::try_from(s.as_str())
    }
}

impl FromStr for AwsRegionId {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}

impl fmt::Display for AwsRegionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_ref())
    }
}

impl From<AwsRegionId> for String {
    fn from(value: AwsRegionId) -> Self {
        value.to_string()
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for AwsRegionId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        AwsRegionId::try_from(s.as_str()).map_err(serde::de::Error::custom)
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for AwsRegionId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_ref())
    }
}

#[cfg(feature = "sqlx-postgres")]
mod sqlx_impl {
    use super::AwsRegionId;
    use sqlx::{
        postgres::{PgTypeInfo, PgValueRef},
        Postgres, Type,
    };

    impl Type<Postgres> for AwsRegionId {
        fn type_info() -> PgTypeInfo {
            <String as Type<Postgres>>::type_info()
        }

        fn compatible(ty: &PgTypeInfo) -> bool {
            <String as Type<Postgres>>::compatible(ty)
        }
    }

    impl sqlx::Encode<'_, Postgres> for AwsRegionId {
        fn encode_by_ref(
            &self,
            buf: &mut sqlx::postgres::PgArgumentBuffer,
        ) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
            <&str as sqlx::Encode<Postgres>>::encode(self.as_ref(), buf)
        }
    }

    impl<'r> sqlx::Decode<'r, Postgres> for AwsRegionId {
        fn decode(value: PgValueRef<'r>) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
            let s = <String as sqlx::Decode<Postgres>>::decode(value)?;
            Ok(AwsRegionId::try_from(s)?)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_regions_covered() {
        let all_regions = [
            "af-south-1",
            "ap-east-1",
            "ap-northeast-1",
            "ap-northeast-2",
            "ap-northeast-3",
            "ap-south-1",
            "ap-south-2",
            "ap-southeast-1",
            "ap-southeast-2",
            "ap-southeast-3",
            "ap-southeast-4",
            "ca-central-1",
            "ca-west-1",
            "eu-central-1",
            "eu-central-2",
            "eu-north-1",
            "eu-south-1",
            "eu-south-2",
            "eu-west-1",
            "eu-west-2",
            "eu-west-3",
            "il-central-1",
            "me-central-1",
            "me-south-1",
            "sa-east-1",
            "us-east-1",
            "us-east-2",
            "us-west-1",
            "us-west-2",
        ];
        assert_eq!(all_regions.len(), 29);

        for region_str in all_regions {
            let region = AwsRegionId::try_from(region_str).unwrap();
            assert_eq!(region.as_ref(), region_str);
        }
    }

    #[test]
    fn test_eq() {
        assert_eq!(
            AwsRegionId::try_from("us-east-1").unwrap(),
            AwsRegionId::try_from("us-east-1").unwrap(),
        );
        assert_ne!(
            AwsRegionId::try_from("us-east-1").unwrap(),
            AwsRegionId::try_from("eu-west-2").unwrap(),
        );
    }

    #[test]
    fn test_valid_regions() {
        assert_eq!(
            AwsRegionId::try_from("us-east-1").unwrap(),
            AwsRegionId::UsEast1
        );
        assert_eq!(
            AwsRegionId::try_from("eu-west-2").unwrap(),
            AwsRegionId::EuWest2
        );
    }

    #[test]
    fn test_invalid_region() {
        assert!(AwsRegionId::try_from("invalid-region").is_err());
    }

    #[test]
    fn test_display() {
        assert_eq!(AwsRegionId::UsWest2.to_string(), "us-west-2");
        assert_eq!(AwsRegionId::EuCentral1.to_string(), "eu-central-1");
    }

    #[test]
    fn test_into_str() {
        let s: &str = AwsRegionId::EuCentral1.into();
        assert_eq!(s, "eu-central-1");
    }

    #[test]
    fn test_into_string() {
        let s: String = AwsRegionId::EuCentral1.into();
        assert_eq!(s, "eu-central-1");
    }

    #[test]
    fn test_asref_str() {
        let s: &str = AwsRegionId::EuCentral1.as_ref();
        assert_eq!(s, "eu-central-1");
    }

    #[test]
    fn test_tryfrom_str() {
        assert_eq!(
            AwsRegionId::try_from("eu-central-1").unwrap(),
            AwsRegionId::EuCentral1
        );
    }

    #[test]
    fn test_tryfrom_string() {
        assert_eq!(
            AwsRegionId::try_from("eu-central-1".to_string()).unwrap(),
            AwsRegionId::EuCentral1
        );
    }

    #[test]
    fn test_tryfrom_refstring() {
        assert_eq!(
            AwsRegionId::try_from(&"eu-central-1".to_string()).unwrap(),
            AwsRegionId::EuCentral1
        );
    }

    #[test]
    fn test_fromstr() {
        assert_eq!(
            "eu-central-1".parse::<AwsRegionId>().unwrap(),
            AwsRegionId::EuCentral1
        );
        assert_eq!(
            "eu-central-1".to_string().parse::<AwsRegionId>().unwrap(),
            AwsRegionId::EuCentral1
        );
    }
}

#[cfg(feature = "serde")]
#[cfg(test)]
mod serde_tests {
    use super::*;

    #[test]
    fn test_serialize() {
        let region = AwsRegionId::UsEast1;
        let serialized = serde_json::to_string(&region).unwrap();
        assert_eq!(serialized, "\"us-east-1\"");
    }

    #[test]
    fn test_deserialize() {
        let deserialized: AwsRegionId = serde_json::from_str("\"eu-west-1\"").unwrap();
        assert_eq!(deserialized, AwsRegionId::EuWest1);
    }
}

#[cfg(feature = "sqlx-postgres")]
#[cfg(test)]
mod sqlx_tests {
    use super::*;
    use sqlx::PgPool;

    #[sqlx::test]
    async fn serialize_varchar(pool: PgPool) -> sqlx::Result<()> {
        let region_str = "eu-central-1";
        let region: AwsRegionId = region_str.parse().unwrap();
        let serialized = sqlx::query_scalar!("SELECT $1::varchar", region as _)
            .fetch_one(&pool)
            .await?
            .unwrap();
        assert_eq!(serialized, region_str);
        Ok(())
    }

    #[sqlx::test]
    async fn serialize_text(pool: PgPool) -> sqlx::Result<()> {
        let region_str = "eu-central-1";
        let region: AwsRegionId = region_str.parse().unwrap();
        let serialized = sqlx::query_scalar!("SELECT $1::text", region as _)
            .fetch_one(&pool)
            .await?
            .unwrap();
        assert_eq!(serialized, region_str);
        Ok(())
    }

    #[sqlx::test]
    async fn deserialize_varchar(pool: PgPool) -> sqlx::Result<()> {
        let region: AwsRegionId = "eu-central-1".parse().unwrap();
        let deserialized =
            sqlx::query_scalar!(r#"SELECT 'eu-central-1'::varchar as "val: AwsRegionId""#)
                .fetch_one(&pool)
                .await?
                .unwrap();
        assert_eq!(deserialized, region);
        Ok(())
    }

    #[sqlx::test]
    async fn deserialize_text(pool: PgPool) -> sqlx::Result<()> {
        let region: AwsRegionId = "eu-central-1".parse().unwrap();
        let deserialized =
            sqlx::query_scalar!(r#"SELECT 'eu-central-1'::text as "val: AwsRegionId""#)
                .fetch_one(&pool)
                .await?
                .unwrap();
        assert_eq!(deserialized, region);
        Ok(())
    }
}
