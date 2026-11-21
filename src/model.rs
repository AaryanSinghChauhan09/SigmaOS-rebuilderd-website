//! rebuilderd API data model — matches the JSON the website consumes.
//!
//! rebuilderd exposes `/api/v1/builders` (builder list) and
//! `/api/v1/packages` (per-package build results) endpoints.

use serde::Deserialize;

/// Endpoint paths on the rebuilderd host.
pub const BUILDER_ENDPOINT: &str = "/api/v1/builders";
pub const PACKAGE_ENDPOINT: &str = "/api/v1/packages";

/// Builder build status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuilderStatus {
    /// Builder online and accepting work.
    Online,
    /// Builder offline.
    Offline,
    /// Unknown/other state.
    Unknown,
}

/// Per-package build status (rebuilderd verdict).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuildStatus {
    /// Package reproduced bit-identically.
    Good,
    /// Build differed or failed.
    Bad,
    /// Not yet built by this builder.
    Unknown,
}

/// A builder entry from `/api/v1/builders`.
#[derive(Debug, Clone, Deserialize)]
pub struct Builder {
    /// Builder hostname/ID.
    pub name: String,
    /// Host architecture.
    #[serde(default)]
    pub host_arch: String,
    /// Synchronized package count.
    #[serde(default)]
    pub packages: u64,
    /// Status string from API ("online"/"offline").
    #[serde(default)]
    pub status: String,
}

impl Builder {
    /// Normalized status.
    pub fn status(&self) -> BuilderStatus {
        match self.status.to_lowercase().as_str() {
            "online" | "ready" | "idle" => BuilderStatus::Online,
            "offline" | "down" => BuilderStatus::Offline,
            _ => BuilderStatus::Unknown,
        }
    }
}

/// A package build result from `/api/v1/packages`.
#[derive(Debug, Clone, Deserialize)]
pub struct PackageBuild {
    /// Package name.
    pub pkgname: String,
    /// Version `epoch:pkgver-pkgrel`.
    pub pkgver: String,
    /// Builder that produced the result.
    pub builder: String,
    /// Status string ("good"/"bad").
    pub status: String,
}

impl PackageBuild {
    /// Normalized status.
    pub fn status(&self) -> BuildStatus {
        match self.status.to_lowercase().as_str() {
            "good" | "ok" | "reproducible" => BuildStatus::Good,
            "bad" | "failed" | "diff" => BuildStatus::Bad,
            _ => BuildStatus::Unknown,
        }
    }
}

/// Combined rebuilderd dataset.
#[derive(Debug, Default)]
pub struct RebuilderdData {
    /// Builders.
    pub builders: Vec<Builder>,
    /// Package build results.
    pub packages: Vec<PackageBuild>,
}
