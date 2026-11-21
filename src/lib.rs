//! SigmaOS rebuilderd-website component.
//!
//! Rust implementation of `archlinux/rebuilderd-website`: the status page
//! for Arch's rebuilderd reproducible-builds fleet.
//!
//! Upstream is a small React app that queries the rebuilderd HTTP API and
//! renders builder/package status sections. This port implements the
//! aggregation layer: parsing rebuilderd API responses and computing the
//! statistics the site displays (builder health, per-package build status,
//! reproducibility percentages).

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod api;
pub mod error;
pub mod model;
pub mod stats;

pub use api::RebuilderdClient;
pub use model::{BUILDER_ENDPOINT, PACKAGE_ENDPOINT};
pub use error::WebsiteError;
pub use model::{Builder, BuilderStatus, BuildStatus, PackageBuild, RebuilderdData};
pub use stats::{FleetStats, fleet_stats, package_stats};
