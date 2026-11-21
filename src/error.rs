//! Error types for the rebuilderd-website component.

/// Errors fetching or parsing rebuilderd API data.
#[derive(Debug, thiserror::Error)]
pub enum WebsiteError {
    /// HTTP error.
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),

    /// JSON parse error.
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
}
