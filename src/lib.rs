#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]
#![warn(clippy::all, missing_docs, nonstandard_style, future_incompatible)]

pub mod general_resource;

pub use general_resource::{AwsAmiId, AwsSnapshotId};
