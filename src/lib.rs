#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]
#![warn(clippy::all, missing_docs, nonstandard_style, future_incompatible)]

pub mod availability_zone;
pub mod general;
pub mod region;

pub use availability_zone::*;
pub use general::*;
pub use region::*;

/// AWS resource ID parsing or validating error
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// Parsing an AWS availability zone name
    #[error(transparent)]
    AvailabilityZone(#[from] AvailabilityZoneError),
    /// Parsing AWS resource ID in the general format
    #[error(transparent)]
    General(#[from] GeneralResourceError),
    /// Parsing AWS region ID
    #[error(transparent)]
    Region(#[from] RegionError),
}
