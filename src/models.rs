//! Gemini model definitions.
//!
//! This module defines the available Gemini models and their API identifiers.

use std::fmt;

/// Available Google Gemini models.
///
/// Each variant corresponds to a specific Gemini model with different
/// capabilities, speed, and cost characteristics.
///
/// # Choosing a Model
///
/// | Model | Best For |
/// |-------|----------|
/// | [`Gemini25Flash`](Model::Gemini25Flash) | Latest capabilities, recommended for most use cases |
/// | [`Gemini20Flash`](Model::Gemini20Flash) | Stable, well-tested alternative |
/// | [`Gemini15Pro`](Model::Gemini15Pro) | Complex reasoning, longer context |
/// | [`Gemini15Flash`](Model::Gemini15Flash) | Balance of speed and quality |
/// | [`Gemini15Flash8B`](Model::Gemini15Flash8B) | High-volume, simple tasks |
/// | [`Gemini10Pro`](Model::Gemini10Pro) | Legacy compatibility |
///
/// # Example
///
/// ```rust
/// use gemini_rs::Model;
///
/// let model = Model::Gemini25Flash;
/// assert_eq!(model.as_str(), "gemini-2.5-flash");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Model {
    /// Gemini 2.5 Flash - Latest and most advanced model.
    ///
    /// Recommended for most use cases. Offers the best balance of
    /// capabilities, speed, and quality.
    Gemini25Flash,

    /// Gemini 2.0 Flash - Previous generation flash model.
    ///
    /// Stable and well-tested. Good alternative if you need
    /// consistent behavior.
    Gemini20Flash,

    /// Gemini 1.5 Pro - Most capable 1.5 generation model.
    ///
    /// Best for complex reasoning tasks, code generation,
    /// and scenarios requiring longer context windows.
    Gemini15Pro,

    /// Gemini 1.5 Flash - Fast and efficient.
    ///
    /// Good balance between speed and quality. Suitable for
    /// production workloads with moderate complexity.
    Gemini15Flash,

    /// Gemini 1.5 Flash-8B - Smallest and fastest model.
    ///
    /// Optimized for high-throughput, low-latency scenarios.
    /// Best for simple tasks like classification or extraction.
    Gemini15Flash8B,

    /// Gemini 1.0 Pro - Legacy model.
    ///
    /// Provided for backward compatibility. Consider upgrading
    /// to newer models for better performance.
    Gemini10Pro,
}

impl Model {
    /// Get the API model identifier.
    ///
    /// Returns the string used in API requests to identify this model.
    ///
    /// # Example
    ///
    /// ```rust
    /// use gemini_rs::Model;
    ///
    /// assert_eq!(Model::Gemini25Flash.as_str(), "gemini-2.5-flash");
    /// assert_eq!(Model::Gemini15Pro.as_str(), "gemini-1.5-pro");
    /// ```
    pub fn as_str(&self) -> &'static str {
        match self {
            Model::Gemini25Flash => "gemini-2.5-flash",
            Model::Gemini20Flash => "gemini-2.0-flash",
            Model::Gemini15Pro => "gemini-1.5-pro",
            Model::Gemini15Flash => "gemini-1.5-flash",
            Model::Gemini15Flash8B => "gemini-1.5-flash-8b",
            Model::Gemini10Pro => "gemini-1.0-pro",
        }
    }

    /// Get the full model name for API calls.
    ///
    /// Returns the complete model path including the "models/" prefix.
    ///
    /// # Example
    ///
    /// ```rust
    /// use gemini_rs::Model;
    ///
    /// assert_eq!(Model::Gemini25Flash.full_name(), "models/gemini-2.5-flash");
    /// ```
    pub fn full_name(&self) -> String {
        format!("models/{}", self.as_str())
    }
}

impl fmt::Display for Model {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Default for Model {
    /// Returns the default model ([`Gemini25Flash`](Model::Gemini25Flash)).
    fn default() -> Self {
        Model::Gemini25Flash
    }
}

impl From<&str> for Model {
    /// Parse a model from its API identifier.
    ///
    /// Unknown strings default to [`Gemini25Flash`](Model::Gemini25Flash).
    ///
    /// # Example
    ///
    /// ```rust
    /// use gemini_rs::Model;
    ///
    /// assert_eq!(Model::from("gemini-2.5-flash"), Model::Gemini25Flash);
    /// assert_eq!(Model::from("gemini-1.5-pro"), Model::Gemini15Pro);
    /// assert_eq!(Model::from("unknown"), Model::Gemini25Flash); // defaults
    /// ```
    fn from(s: &str) -> Self {
        match s {
            "gemini-2.5-flash" => Model::Gemini25Flash,
            "gemini-2.0-flash" => Model::Gemini20Flash,
            "gemini-1.5-pro" => Model::Gemini15Pro,
            "gemini-1.5-flash" => Model::Gemini15Flash,
            "gemini-1.5-flash-8b" => Model::Gemini15Flash8B,
            "gemini-1.0-pro" => Model::Gemini10Pro,
            _ => Model::Gemini25Flash, // Default to latest
        }
    }
}
