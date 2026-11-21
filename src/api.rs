//! rebuilderd HTTP API client.

use crate::model::{Builder, PackageBuild, RebuilderdData};
use crate::WebsiteError;

/// Client for a rebuilderd instance.
pub struct RebuilderdClient {
    client: reqwest::blocking::Client,
    base_url: String,
}

impl RebuilderdClient {
    /// New client for `base_url` (e.g. `https://reproducible.archlinux.org`).
    pub fn new(base_url: impl Into<String>) -> Result<Self, WebsiteError> {
        Ok(Self {
            client: reqwest::blocking::Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .build()?,
            base_url: base_url.into().trim_end_matches('/').to_string(),
        })
    }

    /// Fetch builders list.
    pub fn builders(&self) -> Result<Vec<Builder>, WebsiteError> {
        #[derive(serde::Deserialize)]
        struct Resp {
            builders: Vec<Builder>,
        }
        let url = format!("{}{}", self.base_url, crate::BUILDER_ENDPOINT);
        let resp: Resp = self.client.get(&url).send()?.json()?;
        Ok(resp.builders)
    }

    /// Fetch package build results.
    pub fn packages(&self) -> Result<Vec<PackageBuild>, WebsiteError> {
        #[derive(serde::Deserialize)]
        struct Resp {
            packages: Vec<PackageBuild>,
        }
        let url = format!("{}{}", self.base_url, crate::PACKAGE_ENDPOINT);
        let resp: Resp = self.client.get(&url).send()?.json()?;
        Ok(resp.packages)
    }

    /// Fetch everything into [`RebuilderdData`].
    pub fn fetch_all(&self) -> Result<RebuilderdData, WebsiteError> {
        Ok(RebuilderdData {
            builders: self.builders()?,
            packages: self.packages()?,
        })
    }
}
