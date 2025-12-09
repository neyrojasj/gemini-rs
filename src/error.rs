//! Error types for the Gemini API client.
//!
//! This module defines all possible errors that can occur when using
//! the gemini-rs crate.
//!
//! # Error Handling Example
//!
//! ```rust,no_run
//! use gemini_rs::{Client, Model, Error};
//!
//! # async fn example() {
//! let client = Client::new("YOUR_API_KEY");
//! let model = client.model(Model::Gemini25Flash);
//!
//! match model.generate_content("Hello").await {
//!     Ok(response) => println!("{}", response.text()),
//!     Err(Error::RateLimitExceeded) => {
//!         eprintln!("Rate limited! Wait before retrying.");
//!     }
//!     Err(Error::ApiError { message, code }) => {
//!         eprintln!("API error (code {:?}): {}", code, message);
//!     }
//!     Err(e) => eprintln!("Error: {}", e),
//! }
//! # }
//! ```

use thiserror::Error;

/// A `Result` type alias using the [`Error`](enum@Error) enum as the error type.
pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur when using the Gemini API.
///
/// This enum covers all possible error conditions, from network issues
/// to API errors to parsing failures.
#[derive(Debug, Error)]
pub enum Error {
    /// HTTP request failed (network error, timeout, etc.)
    ///
    /// This typically indicates connectivity issues. Consider retrying
    /// with exponential backoff.
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),

    /// Failed to parse JSON response.
    ///
    /// This can occur when the API returns unexpected data or when
    /// parsing JSON mode output into your custom type.
    #[error("Failed to parse JSON: {0}")]
    JsonError(#[from] serde_json::Error),

    /// API returned an error response.
    ///
    /// Check the message and code for details. Common codes:
    /// - 400: Bad request (invalid parameters)
    /// - 401: Unauthorized (invalid API key)
    /// - 403: Forbidden (quota exceeded or region restricted)
    /// - 429: Too many requests (rate limited)
    /// - 500: Server error (retry later)
    #[error("API error: {message}")]
    ApiError {
        /// Error message from the API.
        message: String,
        /// HTTP status code, if available.
        code: Option<i32>,
    },

    /// No response candidates from API.
    ///
    /// The API returned successfully but with no content. This can happen
    /// if the prompt was blocked by safety filters.
    #[error("No response from API")]
    NoResponse,

    /// Invalid API key provided.
    ///
    /// Verify your API key is correct and has not been revoked.
    /// Get a new key from: <https://aistudio.google.com/app/apikey>
    #[error("Invalid API key")]
    InvalidApiKey,

    /// Rate limit exceeded.
    ///
    /// You've made too many requests. Wait before retrying.
    /// Consider implementing exponential backoff.
    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    /// Invalid model name.
    ///
    /// The specified model doesn't exist or isn't available.
    #[error("Invalid model: {0}")]
    InvalidModel(String),

    /// Content generation failed.
    ///
    /// This typically occurs when JSON parsing fails after receiving
    /// a response. Check that your prompt clearly specifies the expected
    /// JSON structure.
    #[error("Content generation failed: {0}")]
    GenerationFailed(String),

    /// Invalid input provided.
    ///
    /// The input parameters were invalid. Check the error message
    /// for details.
    #[error("Invalid input: {0}")]
    InvalidInput(String),
}
