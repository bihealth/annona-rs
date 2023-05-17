#![allow(clippy::must_use_candidate)]
#![allow(clippy::module_name_repetitions)]
#![warn(missing_docs)]
//! Genome annotation stored in `RocksDB`.

pub mod common;
pub mod cons;
pub mod dbsnp;
mod error;
pub mod gnomad_mtdna;
pub mod gnomad_nuclear;
pub mod helixmtdb;
pub mod tsv;

pub use crate::error::*;

/// The version of the `annona-rs` package.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
