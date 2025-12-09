//! Request and response types for the Gemini API.
//!
//! This module contains all the data structures used to communicate
//! with the Gemini API, including content types, configuration, and responses.

use serde::{Deserialize, Serialize};

/// Content for generation requests and responses.
///
/// `Content` represents a message in a conversation, containing one or more
/// [`Part`]s (text, images, etc.) and an optional role.
///
/// # Example
///
/// ```rust
/// use gemini_rs::Content;
///
/// // Simple text content
/// let content = Content::text("Hello, world!");
///
/// // User message (for chat)
/// let user_msg = Content::user("What's the weather?");
///
/// // Model message (for chat history)
/// let model_msg = Content::model("It's sunny today!");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Content {
    /// The parts that make up this content (text, images, etc.)
    pub parts: Vec<Part>,
    /// The role of this content (user, model, or none for simple prompts)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}

impl Content {
    /// Create content from text.
    ///
    /// Use this for simple text prompts without a specific role.
    ///
    /// # Example
    ///
    /// ```rust
    /// use gemini_rs::Content;
    ///
    /// let content = Content::text("Explain quantum computing");
    /// ```
    pub fn text(text: impl Into<String>) -> Self {
        Self {
            parts: vec![Part::Text { text: text.into() }],
            role: None,
        }
    }

    /// Create user-role content.
    ///
    /// Use this for user messages in chat conversations.
    ///
    /// # Example
    ///
    /// ```rust
    /// use gemini_rs::Content;
    ///
    /// let user_message = Content::user("Hello!");
    /// assert_eq!(user_message.role, Some("user".to_string()));
    /// ```
    pub fn user(text: impl Into<String>) -> Self {
        Self {
            parts: vec![Part::Text { text: text.into() }],
            role: Some("user".to_string()),
        }
    }

    /// Create model-role content.
    ///
    /// Use this for model messages in chat history.
    ///
    /// # Example
    ///
    /// ```rust
    /// use gemini_rs::Content;
    ///
    /// let model_message = Content::model("Hello! How can I help?");
    /// assert_eq!(model_message.role, Some("model".to_string()));
    /// ```
    pub fn model(text: impl Into<String>) -> Self {
        Self {
            parts: vec![Part::Text { text: text.into() }],
            role: Some("model".to_string()),
        }
    }
}

/// A part of content (text, image, etc.)
///
/// Currently supports text and (with the `multimodal` feature) inline data
/// for images.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Part {
    /// Text content.
    Text {
        /// The text string.
        text: String,
    },
    /// Inline data (images, etc.) - requires `multimodal` feature.
    #[cfg(feature = "multimodal")]
    InlineData {
        /// The inline data with MIME type and base64-encoded content.
        inline_data: InlineData,
    },
}

/// Inline data for multimodal content.
///
/// Used to include images or other binary data in requests.
/// Requires the `multimodal` feature.
#[cfg(feature = "multimodal")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineData {
    /// The MIME type (e.g., "image/jpeg", "image/png").
    pub mime_type: String,
    /// Base64-encoded data.
    pub data: String,
}

/// Internal request structure for the generateContent API.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateContentRequest {
    /// The content to send to the model.
    pub contents: Vec<Content>,
    /// Optional generation configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation_config: Option<GenerationConfig>,
    /// Optional safety settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_settings: Option<Vec<SafetySetting>>,
    /// Optional system instruction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_instruction: Option<Content>,
}

/// Configuration for content generation.
///
/// Use `GenerationConfig` to control how the model generates responses.
/// All fields are optional; unset fields use model defaults.
///
/// # Example
///
/// ```rust
/// use gemini_rs::GenerationConfig;
///
/// let config = GenerationConfig::new()
///     .temperature(0.7)    // More creative (0.0-2.0)
///     .top_p(0.9)          // Nucleus sampling
///     .max_tokens(1000);   // Limit response length
/// ```
///
/// # JSON Mode
///
/// Enable JSON mode to get structured output:
///
/// ```rust
/// use gemini_rs::GenerationConfig;
///
/// let config = GenerationConfig::new().json_mode();
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GenerationConfig {
    /// Controls randomness. Range: 0.0 (deterministic) to 2.0 (very random).
    /// Default: 1.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,

    /// Nucleus sampling threshold. Range: 0.0 to 1.0.
    /// Only tokens with cumulative probability <= top_p are considered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,

    /// Top-k sampling. Only the top k tokens are considered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<i32>,

    /// Maximum number of tokens to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_output_tokens: Option<i32>,

    /// Sequences that stop generation when encountered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_sequences: Option<Vec<String>>,

    /// Response MIME type. Set to "application/json" for JSON mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_mime_type: Option<String>,
}

impl GenerationConfig {
    /// Create a new empty configuration.
    ///
    /// All fields default to `None`, meaning the model will use its defaults.
    ///
    /// # Example
    ///
    /// ```rust
    /// use gemini_rs::GenerationConfig;
    ///
    /// let config = GenerationConfig::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the temperature.
    ///
    /// Controls randomness in generation:
    /// - 0.0: Deterministic, always picks the most likely token
    /// - 1.0: Default, balanced randomness
    /// - 2.0: Very random, more creative but less coherent
    ///
    /// # Example
    ///
    /// ```rust
    /// use gemini_rs::GenerationConfig;
    ///
    /// // More focused/deterministic
    /// let focused = GenerationConfig::new().temperature(0.3);
    ///
    /// // More creative
    /// let creative = GenerationConfig::new().temperature(1.5);
    /// ```
    pub fn temperature(mut self, temp: f32) -> Self {
        self.temperature = Some(temp);
        self
    }

    /// Set the top_p (nucleus sampling) threshold.
    ///
    /// Only tokens with cumulative probability <= top_p are considered.
    /// Lower values make output more focused; higher values more diverse.
    ///
    /// # Example
    ///
    /// ```rust
    /// use gemini_rs::GenerationConfig;
    ///
    /// let config = GenerationConfig::new().top_p(0.9);
    /// ```
    pub fn top_p(mut self, top_p: f32) -> Self {
        self.top_p = Some(top_p);
        self
    }

    /// Set the top_k value.
    ///
    /// Limits sampling to the top k most likely tokens.
    ///
    /// # Example
    ///
    /// ```rust
    /// use gemini_rs::GenerationConfig;
    ///
    /// let config = GenerationConfig::new().top_k(40);
    /// ```
    pub fn top_k(mut self, top_k: i32) -> Self {
        self.top_k = Some(top_k);
        self
    }

    /// Set the maximum number of tokens to generate.
    ///
    /// Use this to limit response length and control costs.
    ///
    /// # Example
    ///
    /// ```rust
    /// use gemini_rs::GenerationConfig;
    ///
    /// let config = GenerationConfig::new().max_tokens(500);
    /// ```
    pub fn max_tokens(mut self, max: i32) -> Self {
        self.max_output_tokens = Some(max);
        self
    }

    /// Enable JSON mode.
    ///
    /// When enabled, the model will return valid JSON that can be parsed
    /// directly. Use with [`ModelClient::generate_json`](crate::client::ModelClient::generate_json).
    ///
    /// # Example
    ///
    /// ```rust
    /// use gemini_rs::GenerationConfig;
    ///
    /// let config = GenerationConfig::new().json_mode();
    /// assert_eq!(config.response_mime_type, Some("application/json".to_string()));
    /// ```
    pub fn json_mode(mut self) -> Self {
        self.response_mime_type = Some("application/json".to_string());
        self
    }
}

/// A single safety setting.
///
/// Configures content filtering for a specific harm category.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetySetting {
    /// The harm category (e.g., "HARM_CATEGORY_HARASSMENT").
    pub category: String,
    /// The blocking threshold (e.g., "BLOCK_NONE", "BLOCK_MEDIUM_AND_ABOVE").
    pub threshold: String,
}

/// Builder for safety settings.
///
/// Provides convenient methods to create common safety configurations.
///
/// # Example
///
/// ```rust
/// use gemini_rs::SafetySettings;
///
/// // Disable all content filtering
/// let settings = SafetySettings::block_none();
/// ```
pub struct SafetySettings;

impl SafetySettings {
    /// Create settings that block no content.
    ///
    /// ⚠️ **Warning**: This disables all content filtering. Use with caution
    /// and only when necessary for your use case.
    ///
    /// # Example
    ///
    /// ```rust
    /// use gemini_rs::SafetySettings;
    ///
    /// let settings = SafetySettings::block_none();
    /// assert_eq!(settings.len(), 4);
    /// ```
    pub fn block_none() -> Vec<SafetySetting> {
        vec![
            SafetySetting {
                category: "HARM_CATEGORY_HARASSMENT".to_string(),
                threshold: "BLOCK_NONE".to_string(),
            },
            SafetySetting {
                category: "HARM_CATEGORY_HATE_SPEECH".to_string(),
                threshold: "BLOCK_NONE".to_string(),
            },
            SafetySetting {
                category: "HARM_CATEGORY_SEXUALLY_EXPLICIT".to_string(),
                threshold: "BLOCK_NONE".to_string(),
            },
            SafetySetting {
                category: "HARM_CATEGORY_DANGEROUS_CONTENT".to_string(),
                threshold: "BLOCK_NONE".to_string(),
            },
        ]
    }
}

/// Response from the generateContent API.
///
/// Contains the model's output along with metadata about the generation.
///
/// # Example
///
/// ```rust,no_run
/// use gemini_rs::{Client, Model};
///
/// # async fn example() -> Result<(), gemini_rs::Error> {
/// let client = Client::new("YOUR_API_KEY");
/// let model = client.model(Model::Gemini25Flash);
///
/// let response = model.generate_content("Hello").await?;
///
/// // Get the text response
/// println!("{}", response.text());
///
/// // Or parse as JSON
/// // let data: MyType = response.json()?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateContentResponse {
    /// The generated candidates (usually one).
    pub candidates: Option<Vec<Candidate>>,
    /// Feedback about the prompt (e.g., if it was blocked).
    pub prompt_feedback: Option<PromptFeedback>,
}

impl GenerateContentResponse {
    /// Get the text from the first candidate.
    ///
    /// This is the most common way to get the model's response.
    /// Returns an empty string if no candidates or text is available.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use gemini_rs::{Client, Model};
    ///
    /// # async fn example() -> Result<(), gemini_rs::Error> {
    /// let client = Client::new("YOUR_API_KEY");
    /// let model = client.model(Model::Gemini25Flash);
    ///
    /// let response = model.generate_content("Say hello").await?;
    /// let text = response.text();
    /// println!("{}", text);
    /// # Ok(())
    /// # }
    /// ```
    pub fn text(&self) -> String {
        self.candidates
            .as_ref()
            .and_then(|c| c.first())
            .and_then(|c| c.content.as_ref())
            .and_then(|content| content.parts.first())
            .and_then(|part| match part {
                Part::Text { text } => Some(text.clone()),
                #[cfg(feature = "multimodal")]
                _ => None,
            })
            .unwrap_or_default()
    }

    /// Parse the response text as JSON.
    ///
    /// Use this when the model was configured with JSON mode.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The type to deserialize into
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use gemini_rs::{Client, Model, GenerationConfig};
    /// use serde::Deserialize;
    ///
    /// #[derive(Deserialize)]
    /// struct Data {
    ///     value: i32,
    /// }
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::new("YOUR_API_KEY");
    /// let model = client
    ///     .model(Model::Gemini25Flash)
    ///     .with_config(GenerationConfig::new().json_mode());
    ///
    /// let response = model.generate_content("Return {\"value\": 42}").await?;
    /// let data: Data = response.json()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn json<T: serde::de::DeserializeOwned>(&self) -> Result<T, serde_json::Error> {
        let text = self.text();
        serde_json::from_str(&text)
    }
}

/// A single candidate response from the model.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Candidate {
    /// The generated content.
    pub content: Option<Content>,
    /// Why generation stopped (e.g., "STOP", "MAX_TOKENS").
    pub finish_reason: Option<String>,
    /// Safety ratings for the response.
    pub safety_ratings: Option<Vec<SafetyRating>>,
}

/// Safety rating for a response.
#[derive(Debug, Deserialize)]
pub struct SafetyRating {
    /// The harm category that was rated.
    pub category: String,
    /// The probability of harm (e.g., "NEGLIGIBLE", "LOW", "MEDIUM", "HIGH").
    pub probability: String,
}

/// Feedback about the prompt.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PromptFeedback {
    /// Why the prompt was blocked (if applicable).
    pub block_reason: Option<String>,
    /// Safety ratings for the prompt.
    pub safety_ratings: Option<Vec<SafetyRating>>,
}
