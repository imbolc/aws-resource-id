#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]
#![warn(clippy::all, missing_docs, nonstandard_style, future_incompatible)]

pub mod general;
pub mod region;

pub use general::*;
pub use region::*;

/// AWS resource ID parsing or validating error
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Parsing AWS resource ID in the general format
    #[error(transparent)]
    General(#[from] GeneralResourceError),
    /// Parsing AWS region ID
    #[error(transparent)]
    Region(#[from] RegionError),
}
