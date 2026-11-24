//! # AWS Availability Zone
use crate::{AwsRegionId, RegionError};
use std::{convert::TryFrom, fmt, str::FromStr};

/// Error encountered when parsing an AWS zone name or ID
#[derive(Debug, thiserror::Error)]
pub enum AvailabilityZoneError {
    /// The zone suffix does not match a supported format
    #[error("Invalid availability zone suffix: {0}")]
    InvalidSuffix(String),
    /// The region or abbreviated region prefix is unknown
    #[error(transparent)]
    Region(#[from] RegionError),
}

/// Six-byte AWS Availability Zone, Local Zone, or Wavelength Zone name or ID
///
/// Accepts the following spellings for regions supported by [`AwsRegionId`].
///
/// | Format | Example |
/// | --- | --- |
/// | Standard AZ name | `us-east-1a` |
/// | Standard AZ ID | `use1-az1` |
/// | Local Zone name | `us-west-2-lax-1a` |
/// | Local Zone ID | `usw2-lax1-az1`, `apse1-han-az1` |
/// | Wavelength Zone name | `us-east-1-wl1-bos-wlz-1`, `eu-west-3-cmn-wlz-1a` |
/// | Wavelength Zone ID | `use1-wl1-bos-wlz1`, `euw3-cmn1-wlz1` |
///
/// Parsing validates syntax only. Location codes consist of three lowercase
/// ASCII letters. Numeric suffixes use decimal values from 1 through 255 with
/// no sign or leading zeros.
///
/// Equality compares the exact identifier spelling. Ordering compares the
/// region, then format, then location and suffix.
///
/// ```
/// use aws_resource_id::AwsAvailabilityZone;
///
/// let zone: AwsAvailabilityZone = "us-east-1a".parse().unwrap();
/// assert_eq!(zone.to_string(), "us-east-1a");
/// ```
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AwsAvailabilityZone {
    region: AwsRegionId,
    format: ZoneFormat,
    location: u16,
    suffix: [u8; 2],
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum ZoneFormat {
    StandardName,
    StandardId,
    LocalName,
    LocalId,
    WavelengthGroupName,
    WavelengthName,
    WavelengthGroupId,
    WavelengthId,
}

impl TryFrom<&str> for AwsAvailabilityZone {
    type Error = crate::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let invalid_suffix = || AvailabilityZoneError::InvalidSuffix(s.into());
        let (region, format, location, suffix) =
            if let Some((region, name)) = split_extended_name(s) {
                let (format, location, suffix) =
                    parse_extended_name(name).ok_or_else(invalid_suffix)?;
                (AwsRegionId::from_name(region), format, location, suffix)
            } else if let Some((region, id)) = split_id(s) {
                let (format, location, suffix) = parse_id(id).ok_or_else(invalid_suffix)?;
                (
                    AwsRegionId::from_abbreviation(region),
                    format,
                    location,
                    suffix,
                )
            } else {
                let (region, letter) = split_letter(s).ok_or_else(invalid_suffix)?;
                (
                    AwsRegionId::from_name(region),
                    ZoneFormat::StandardName,
                    0,
                    [letter, 0],
                )
            };

        Ok(Self {
            region: region.map_err(AvailabilityZoneError::from)?,
            format,
            location,
            suffix,
        })
    }
}

fn split_extended_name(s: &str) -> Option<(&str, &str)> {
    let mut remaining = s;
    while let Some((part, tail)) = remaining.split_once('-') {
        if !part.is_empty() && part.bytes().all(|byte| byte.is_ascii_digit()) {
            // The region ends at its numeric component. Extended names have
            // at least two components after it.
            let region_end = s.len() - tail.len() - 1;
            return tail.contains('-').then_some((&s[..region_end], tail));
        }
        remaining = tail;
    }
    None
}

fn split_id(s: &str) -> Option<(&str, &str)> {
    let (region, suffix) = s.split_once('-')?;
    let last = suffix.rsplit_once('-').map_or(suffix, |(_, last)| last);
    let abbreviated_region = region.as_bytes().last().is_some_and(u8::is_ascii_digit);
    (abbreviated_region || last.starts_with("az") || last.starts_with("wlz"))
        .then_some((region, suffix))
}

fn parse_extended_name(s: &str) -> Option<(ZoneFormat, u16, [u8; 2])> {
    let mut parts = s.splitn(4, '-');
    match (parts.next()?, parts.next()?, parts.next(), parts.next()) {
        (location, suffix, None, None) => Some((
            ZoneFormat::LocalName,
            encode_location(location)?,
            parse_number_and_letter(suffix)?,
        )),
        (location, "wlz", Some(suffix), None) => Some((
            ZoneFormat::WavelengthName,
            encode_location(location)?,
            parse_number_and_letter(suffix)?,
        )),
        (group, location, Some("wlz"), Some(number)) => Some((
            ZoneFormat::WavelengthGroupName,
            encode_location(location)?,
            [
                parse_number(group.strip_prefix("wl")?)?,
                parse_number(number)?,
            ],
        )),
        _ => None,
    }
}

fn parse_id(s: &str) -> Option<(ZoneFormat, u16, [u8; 2])> {
    let mut parts = s.splitn(3, '-');
    match (parts.next()?, parts.next(), parts.next()) {
        (number, None, None) => Some((
            ZoneFormat::StandardId,
            0,
            [parse_number(number.strip_prefix("az")?)?, 0],
        )),
        (location, Some(number), None) => {
            let (location, group) = location.split_at_checked(3)?;
            let location = encode_location(location)?;
            if let Some(number) = number.strip_prefix("az") {
                // Hanoi's Local Zone ID omits the location group.
                let group = if group.is_empty() {
                    0
                } else {
                    parse_number(group)?
                };
                Some((
                    ZoneFormat::LocalId,
                    location,
                    [group, parse_number(number)?],
                ))
            } else {
                Some((
                    ZoneFormat::WavelengthId,
                    location,
                    [
                        parse_number(group)?,
                        parse_number(number.strip_prefix("wlz")?)?,
                    ],
                ))
            }
        }
        (group, Some(location), Some(number)) => Some((
            ZoneFormat::WavelengthGroupId,
            encode_location(location)?,
            [
                parse_number(group.strip_prefix("wl")?)?,
                parse_number(number.strip_prefix("wlz")?)?,
            ],
        )),
        _ => None,
    }
}

fn encode_location(s: &str) -> Option<u16> {
    let [a @ b'a'..=b'z', b @ b'a'..=b'z', c @ b'a'..=b'z'] = s.as_bytes() else {
        return None;
    };
    let digit = |byte| u16::from(byte - b'a');
    Some((digit(*a) * 26 + digit(*b)) * 26 + digit(*c))
}

fn split_letter(s: &str) -> Option<(&str, u8)> {
    let &letter @ b'a'..=b'z' = s.as_bytes().last()? else {
        return None;
    };
    Some((&s[..s.len() - 1], letter))
}

fn parse_number(s: &str) -> Option<u8> {
    // A nonzero first digit rules out both leading zeros and signs.
    if !matches!(s.as_bytes(), [b'1'..=b'9', ..]) {
        return None;
    }
    s.parse().ok()
}

fn parse_number_and_letter(s: &str) -> Option<[u8; 2]> {
    let (number, letter) = split_letter(s)?;
    Some([parse_number(number)?, letter])
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
        let region = self.region;
        let abbreviated = region.abbreviation();
        let [first, second] = self.suffix;
        let location = [
            b'a' + (self.location / (26 * 26)) as u8,
            b'a' + (self.location / 26 % 26) as u8,
            b'a' + (self.location % 26) as u8,
        ];
        let location = std::str::from_utf8(&location).expect("encoded location is ASCII");

        match self.format {
            ZoneFormat::StandardName => write!(f, "{region}{}", char::from(first)),
            ZoneFormat::StandardId => write!(f, "{abbreviated}-az{first}"),
            ZoneFormat::LocalName => write!(f, "{region}-{location}-{first}{}", char::from(second)),
            ZoneFormat::LocalId if first == 0 => write!(f, "{abbreviated}-{location}-az{second}"),
            ZoneFormat::LocalId => write!(f, "{abbreviated}-{location}{first}-az{second}"),
            ZoneFormat::WavelengthGroupName => {
                write!(f, "{region}-wl{first}-{location}-wlz-{second}")
            }
            ZoneFormat::WavelengthName => {
                write!(f, "{region}-{location}-wlz-{first}{}", char::from(second))
            }
            ZoneFormat::WavelengthGroupId => {
                write!(f, "{abbreviated}-wl{first}-{location}-wlz{second}")
            }
            ZoneFormat::WavelengthId => write!(f, "{abbreviated}-{location}{first}-wlz{second}"),
        }
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
const ZONE_EXAMPLES: &[&str] = &[
    "us-east-1a",
    "us-east-1f",
    "us-west-2d",
    "eu-central-1b",
    "use1-az1",
    "us-west-2-lax-1a",
    "us-west-2-den-1a",
    "us-west-2-wlz-1a",
    "usw2-lax1-az1",
    "apse1-han-az1",
    "us-east-1-wl1-bos-wlz-1",
    "eu-west-2-wl2-man-wlz-1",
    "eu-west-3-cmn-wlz-1a",
    "us-west-2-wl1-sfo-wlz-1",
    "use1-wl1-bos-wlz1",
    "euw3-cmn1-wlz1",
    "us-west-2-aaa-255a",
    "us-west-2-zzz-1z",
    "use1-az255",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_zones() {
        for &name in ZONE_EXAMPLES {
            let zone: AwsAvailabilityZone = name.parse().unwrap_or_else(|e| panic!("{name}: {e}"));
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
            "us-west-2-lax-256a",
            "us-west-2-lax-0a",
            "us-west-2-lax-01a",
            "us-west-2-lax-+1a",
            "us-west-2-lax-1ab",
            "us-west-2-lax-1a-extra",
            "us-west-2-LAX-1a",
            "us-west-2-läx-1a",
            "use1-az+1",
            "use1-az",
            "use1-az01",
            "usw2-han0-az1",
            "us-east-1-wl+1-bos-wlz-1",
            "us-east-1-wl1-bos-wlz-+1",
            "us-east-1-wl1-bos-wlz-1a",
            "us-east-1-cmn-wlz-1",
            "us-east-1-wl1-bos-wlz1",
            "use1-wl1-bos-wlz-1",
        ] {
            assert!(
                matches!(
                    name.parse::<AwsAvailabilityZone>(),
                    Err(crate::Error::AvailabilityZone(
                        AvailabilityZoneError::InvalidSuffix(_)
                    ))
                ),
                "expected a suffix error for: {name}"
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
            "unknown-1-lax-1a",
            "unknown-1-wl1-bos-wlz-1",
            "xxe1-az1",
            "xxe1-lax1-az1",
            "xxe1-wl1-bos-wlz1",
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
    fn test_equality_and_order() {
        let [east_name, east_id, lax, den, han, han_group] = [
            "us-east-1a",
            "use1-az1",
            "us-west-2-lax-1a",
            "us-west-2-den-1a",
            "apse1-han-az1",
            "apse1-han1-az1",
        ]
        .map(|s| s.parse::<AwsAvailabilityZone>().unwrap());
        assert_eq!(east_name.region, east_id.region);
        assert_ne!(east_name, east_id);
        assert_ne!(lax, den);
        assert_ne!(han, han_group);

        let [eu_last, east_first, east_last] = ["eu-central-1z", "us-east-1a", "us-east-1z"]
            .map(|s| s.parse::<AwsAvailabilityZone>().unwrap());
        assert!(eu_last < east_first && east_first < east_last);
    }

    #[test]
    fn test_copy_and_size() {
        fn assert_copy<T: Copy>() {}
        assert_copy::<AwsAvailabilityZone>();
        assert_eq!(std::mem::size_of::<AwsAvailabilityZone>(), 6);
    }
}

#[cfg(feature = "serde")]
#[cfg(test)]
mod serde_tests {
    use super::*;

    #[test]
    fn test_round_trip() {
        for &name in ZONE_EXAMPLES {
            let zone: AwsAvailabilityZone = name.parse().unwrap();
            let serialized = serde_json::to_string(&zone).unwrap();
            assert_eq!(serialized, format!("\"{name}\""));
            assert_eq!(
                serde_json::from_str::<AwsAvailabilityZone>(&serialized).unwrap(),
                zone
            );
        }
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
        for &name in ZONE_EXAMPLES {
            let zone: AwsAvailabilityZone = name.parse().unwrap();
            let serialized = sqlx::query_scalar!("SELECT $1::text", zone as _)
                .fetch_one(&pool)
                .await?
                .unwrap();
            assert_eq!(serialized, name);
            let deserialized = sqlx::query_scalar!(
                r#"SELECT $1::text as "val: AwsAvailabilityZone""#,
                serialized
            )
            .fetch_one(&pool)
            .await?
            .unwrap();
            assert_eq!(deserialized, zone);
        }
        Ok(())
    }

    #[sqlx::test]
    async fn test_varchar_round_trip(pool: PgPool) -> sqlx::Result<()> {
        for &name in ZONE_EXAMPLES {
            let zone: AwsAvailabilityZone = name.parse().unwrap();
            let serialized = sqlx::query_scalar!("SELECT $1::varchar", zone as _)
                .fetch_one(&pool)
                .await?
                .unwrap();
            assert_eq!(serialized, name);
            let deserialized = sqlx::query_scalar!(
                r#"SELECT $1::varchar as "val: AwsAvailabilityZone""#,
                serialized
            )
            .fetch_one(&pool)
            .await?
            .unwrap();
            assert_eq!(deserialized, zone);
        }
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
