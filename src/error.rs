use thiserror::Error;

#[derive(Error, Debug)]
pub enum KitaabeError {
    #[error("No search query provided")]
    MissingQuery,

    #[error("Request failed: {0}")]
    Request(#[from] reqwest::Error),

    #[error("Failed to parse response: {0}")]
    Parse(#[from] serde_json::Error),
}
