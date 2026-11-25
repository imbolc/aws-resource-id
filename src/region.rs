//! # AWS Region ID
use std::{convert::TryFrom, fmt, str::FromStr};

/// Error encountered when parsing an AWS region
#[derive(Debug, thiserror::Error)]
#[error("Unknown region: {0}")]
pub struct RegionError(String);

macro_rules! region_name {
    ($area:literal, $location:literal) => {
        concat!($area, " (", $location, ")")
    };
}

macro_rules! define_regions {
    ($($variant:ident => ($name:literal, $abbreviation:literal, $area:literal, $location:literal),)+) => {
        /// AWS Region ID
        #[repr(u8)]
        #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum AwsRegionId {
            $(
                #[doc = region_name!($area, $location)]
                $variant,
            )+
        }

        impl AwsRegionId {
            pub(crate) fn from_name(s: &str) -> Result<Self, RegionError> {
                match s {
                    $($name => Ok(Self::$variant),)+
                    _ => Err(RegionError(s.into())),
                }
            }

            pub(crate) fn from_abbreviation(s: &str) -> Result<Self, RegionError> {
                match s {
                    $($abbreviation => Ok(Self::$variant),)+
                    _ => Err(RegionError(s.into())),
                }
            }

            pub(crate) const fn abbreviation(self) -> &'static str {
                match self {
                    $(Self::$variant => $abbreviation,)+
                }
            }

            const fn labels(self) -> (&'static str, &'static str, &'static str) {
                match self {
                    $(Self::$variant => ($area, $location, region_name!($area, $location)),)+
                }
            }
        }

        impl From<AwsRegionId> for &'static str {
            fn from(region: AwsRegionId) -> Self {
                match region {
                    $(AwsRegionId::$variant => $name,)+
                }
            }
        }
    };
}

define_regions! {
    AfSouth1 => ("af-south-1", "afs1", "Africa", "Cape Town"),
    ApEast1 => ("ap-east-1", "ape1", "Asia Pacific", "Hong Kong"),
    ApNortheast1 => ("ap-northeast-1", "apne1", "Asia Pacific", "Tokyo"),
    ApNortheast2 => ("ap-northeast-2", "apne2", "Asia Pacific", "Seoul"),
    ApNortheast3 => ("ap-northeast-3", "apne3", "Asia Pacific", "Osaka"),
    ApSouth1 => ("ap-south-1", "aps1", "Asia Pacific", "Mumbai"),
    ApSouth2 => ("ap-south-2", "aps2", "Asia Pacific", "Hyderabad"),
    ApSoutheast1 => ("ap-southeast-1", "apse1", "Asia Pacific", "Singapore"),
    ApSoutheast2 => ("ap-southeast-2", "apse2", "Asia Pacific", "Sydney"),
    ApSoutheast3 => ("ap-southeast-3", "apse3", "Asia Pacific", "Jakarta"),
    ApSoutheast4 => ("ap-southeast-4", "apse4", "Asia Pacific", "Melbourne"),
    CaCentral1 => ("ca-central-1", "cac1", "Canada", "Central"),
    CaWest1 => ("ca-west-1", "caw1", "Canada West", "Calgary"),
    EuCentral1 => ("eu-central-1", "euc1", "Europe", "Frankfurt"),
    EuCentral2 => ("eu-central-2", "euc2", "Europe", "Zurich"),
    EuNorth1 => ("eu-north-1", "eun1", "Europe", "Stockholm"),
    EuSouth1 => ("eu-south-1", "eus1", "Europe", "Milan"),
    EuSouth2 => ("eu-south-2", "eus2", "Europe", "Spain"),
    EuWest1 => ("eu-west-1", "euw1", "Europe", "Ireland"),
    EuWest2 => ("eu-west-2", "euw2", "Europe", "London"),
    EuWest3 => ("eu-west-3", "euw3", "Europe", "Paris"),
    IlCentral1 => ("il-central-1", "ilc1", "Israel", "Tel Aviv"),
    MeCentral1 => ("me-central-1", "mec1", "Middle East", "UAE"),
    MeSouth1 => ("me-south-1", "mes1", "Middle East", "Bahrain"),
    SaEast1 => ("sa-east-1", "sae1", "South America", "São Paulo"),
    UsEast1 => ("us-east-1", "use1", "US East", "N. Virginia"),
    UsEast2 => ("us-east-2", "use2", "US East", "Ohio"),
    UsWest1 => ("us-west-1", "usw1", "US West", "N. California"),
    UsWest2 => ("us-west-2", "usw2", "US West", "Oregon"),
}

impl AwsRegionId {
    /// Broad geographic area
    ///
    /// ```
    /// # use aws_resource_id::AwsRegionId;
    /// assert_eq!(AwsRegionId::EuCentral1.area(), "Europe");
    /// ```
    pub const fn area(self) -> &'static str {
        self.labels().0
    }

    /// Short location label
    ///
    /// ```
    /// # use aws_resource_id::AwsRegionId;
    /// assert_eq!(AwsRegionId::EuCentral1.location(), "Frankfurt");
    /// ```
    pub const fn location(self) -> &'static str {
        self.labels().1
    }

    /// Full human-readable region name
    ///
    /// ```
    /// # use aws_resource_id::AwsRegionId;
    /// assert_eq!(AwsRegionId::EuCentral1.name(), "Europe (Frankfurt)");
    /// ```
    pub const fn name(self) -> &'static str {
        self.labels().2
    }
}

impl TryFrom<&str> for AwsRegionId {
    type Error = crate::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Self::from_name(s).map_err(Into::into)
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
        Postgres, Type,
        postgres::{PgTypeInfo, PgValueRef},
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
            let abbreviation = region.abbreviation();
            assert_eq!(
                AwsRegionId::from_abbreviation(abbreviation).unwrap(),
                region
            );
            assert!(AwsRegionId::try_from(abbreviation).is_err());
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
            sqlx::query_scalar!(r#"SELECT 'eu-central-1'::varchar AS "val: AwsRegionId""#)
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
            sqlx::query_scalar!(r#"SELECT 'eu-central-1'::text AS "val: AwsRegionId""#)
                .fetch_one(&pool)
                .await?
                .unwrap();
        assert_eq!(deserialized, region);
        Ok(())
    }
}
