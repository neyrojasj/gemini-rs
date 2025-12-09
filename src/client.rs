//! HTTP client and model-specific clients for the Gemini API.
//!
//! This module provides the core client types:
//! - [`Client`] - Main API client that holds credentials
//! - [`ModelClient`] - Model-specific client with configuration
//! - [`ChatSession`] - Stateful chat with message history

use crate::error::{Error, Result};
use crate::models::Model;
use crate::types::{
    Content, GenerateContentRequest, GenerateContentResponse, GenerationConfig, SafetySetting,
};
use reqwest::Client as HttpClient;

const BASE_URL: &str = "https://generativelanguage.googleapis.com/v1beta";

/// Main Gemini API client.
///
/// The `Client` holds your API key and creates model-specific clients.
/// It can be cloned efficiently as it shares the underlying HTTP client.
///
/// # Example
///
/// ```rust,no_run
/// use gemini_rs::{Client, Model};
///
/// let client = Client::new("YOUR_API_KEY");
/// let model = client.model(Model::Gemini25Flash);
/// ```
#[derive(Clone)]
pub struct Client {
    http_client: HttpClient,
    api_key: String,
    base_url: String,
}

impl Client {
    /// Create a new Gemini client with the provided API key.
    ///
    /// Get your API key from: <https://aistudio.google.com/app/apikey>
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your Google AI API key
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use gemini_rs::Client;
    ///
    /// // From a string
    /// let client = Client::new("YOUR_API_KEY");
    ///
    /// // From an environment variable
    /// let client = Client::new(std::env::var("GOOGLE_API_KEY").unwrap());
    /// ```
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            http_client: HttpClient::new(),
            api_key: api_key.into(),
            base_url: BASE_URL.to_string(),
        }
    }

    /// Get a model-specific client for the specified model.
    ///
    /// The returned [`ModelClient`] can be configured with generation settings,
    /// safety settings, and system instructions before making API calls.
    ///
    /// # Arguments
    ///
    /// * `model` - The Gemini model to use
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use gemini_rs::{Client, Model};
    ///
    /// let client = Client::new("YOUR_API_KEY");
    ///
    /// // Use the latest model
    /// let model = client.model(Model::Gemini25Flash);
    ///
    /// // Or a specific model for your use case
    /// let fast_model = client.model(Model::Gemini15Flash8B);
    /// ```
    pub fn model(&self, model: Model) -> ModelClient {
        ModelClient {
            client: self.clone(),
            model,
            generation_config: None,
            safety_settings: None,
            system_instruction: None,
        }
    }
}

/// Model-specific client with configuration.
///
/// `ModelClient` wraps a specific Gemini model and allows you to:
/// - Configure generation parameters (temperature, max tokens, etc.)
/// - Set safety settings
/// - Add system instructions
/// - Generate content, JSON, or start chat sessions
///
/// # Example
///
/// ```rust,no_run
/// use gemini_rs::{Client, Model, GenerationConfig};
///
/// # async fn example() -> Result<(), gemini_rs::Error> {
/// let client = Client::new("YOUR_API_KEY");
///
/// let model = client
///     .model(Model::Gemini25Flash)
///     .with_config(GenerationConfig::new().temperature(0.7))
///     .with_system_instruction("You are a helpful assistant");
///
/// let response = model.generate_content("Hello!").await?;
/// # Ok(())
/// # }
/// ```
pub struct ModelClient {
    client: Client,
    model: Model,
    generation_config: Option<GenerationConfig>,
    safety_settings: Option<Vec<SafetySetting>>,
    system_instruction: Option<Content>,
}

impl ModelClient {
    /// Set generation configuration.
    ///
    /// Use [`GenerationConfig`] to control temperature, top_p, max_tokens, etc.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use gemini_rs::{Client, Model, GenerationConfig};
    ///
    /// let client = Client::new("YOUR_API_KEY");
    /// let model = client
    ///     .model(Model::Gemini25Flash)
    ///     .with_config(GenerationConfig::new().temperature(0.5));
    /// ```
    pub fn with_config(mut self, config: GenerationConfig) -> Self {
        self.generation_config = Some(config);
        self
    }

    /// Set safety settings.
    ///
    /// Control content filtering for harmful categories.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use gemini_rs::{Client, Model, SafetySettings};
    ///
    /// let client = Client::new("YOUR_API_KEY");
    /// let model = client
    ///     .model(Model::Gemini25Flash)
    ///     .with_safety(SafetySettings::block_none());
    /// ```
    pub fn with_safety(mut self, settings: Vec<SafetySetting>) -> Self {
        self.safety_settings = Some(settings);
        self
    }

    /// Set a system instruction for the model.
    ///
    /// System instructions guide the model's behavior and persona.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use gemini_rs::{Client, Model};
    ///
    /// let client = Client::new("YOUR_API_KEY");
    /// let model = client
    ///     .model(Model::Gemini25Flash)
    ///     .with_system_instruction("You are a friendly coding assistant");
    /// ```
    pub fn with_system_instruction(mut self, instruction: impl Into<String>) -> Self {
        self.system_instruction = Some(Content::text(instruction));
        self
    }

    /// Generate content from a text prompt.
    ///
    /// This is the primary method for simple text generation.
    ///
    /// # Arguments
    ///
    /// * `prompt` - The text prompt to send to the model
    ///
    /// # Returns
    ///
    /// A [`GenerateContentResponse`] containing the model's output.
    /// Use `.text()` to get the response text.
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
    /// let response = model.generate_content("Write a haiku about Rust").await?;
    /// println!("{}", response.text());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn generate_content(
        &self,
        prompt: impl Into<String>,
    ) -> Result<GenerateContentResponse> {
        let content = Content::text(prompt);
        self.generate_content_from_parts(vec![content]).await
    }

    /// Generate content from multiple content parts.
    ///
    /// Use this for multi-turn conversations or multimodal content.
    ///
    /// # Arguments
    ///
    /// * `contents` - Vector of [`Content`] parts to send
    ///
    /// # Returns
    ///
    /// A [`GenerateContentResponse`] containing the model's output.
    pub async fn generate_content_from_parts(
        &self,
        contents: Vec<Content>,
    ) -> Result<GenerateContentResponse> {
        let url = format!(
            "{}/models/{}:generateContent?key={}",
            self.client.base_url,
            self.model.as_str(),
            self.client.api_key
        );

        let request = GenerateContentRequest {
            contents,
            generation_config: self.generation_config.clone(),
            safety_settings: self.safety_settings.clone(),
            system_instruction: self.system_instruction.clone(),
        };

        let response = self
            .client
            .http_client
            .post(&url)
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(Error::ApiError {
                message: format!("HTTP {}: {}", status, error_text),
                code: Some(status.as_u16() as i32),
            });
        }

        let gemini_response: GenerateContentResponse = response.json().await?;

        if gemini_response.candidates.is_none() {
            return Err(Error::NoResponse);
        }

        Ok(gemini_response)
    }

    /// Generate structured JSON output and deserialize into a type.
    ///
    /// This method enables JSON mode and automatically parses the response.
    /// The model is instructed to return valid JSON that matches your type.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The type to deserialize into (must implement `serde::Deserialize`)
    ///
    /// # Arguments
    ///
    /// * `prompt` - A prompt describing the JSON structure you want
    ///
    /// # Returns
    ///
    /// The parsed value of type `T`.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use gemini_rs::{Client, Model};
    /// use serde::Deserialize;
    ///
    /// #[derive(Deserialize)]
    /// struct Person {
    ///     name: String,
    ///     age: u32,
    /// }
    ///
    /// # async fn example() -> Result<(), gemini_rs::Error> {
    /// let client = Client::new("YOUR_API_KEY");
    /// let model = client.model(Model::Gemini25Flash);
    ///
    /// let person: Person = model
    ///     .generate_json("Generate a random person with name and age")
    ///     .await?;
    ///
    /// println!("{} is {} years old", person.name, person.age);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn generate_json<T: serde::de::DeserializeOwned>(
        &self,
        prompt: impl Into<String>,
    ) -> Result<T> {
        let config = self
            .generation_config
            .clone()
            .unwrap_or_default()
            .json_mode();

        let model_with_json = ModelClient {
            generation_config: Some(config),
            ..self.clone()
        };

        let response = model_with_json.generate_content(prompt).await?;
        let text = response.text();

        // Clean up markdown code blocks if present
        let json_text = text
            .trim()
            .trim_start_matches("```json")
            .trim_start_matches("```")
            .trim_end_matches("```")
            .trim();

        serde_json::from_str(json_text).map_err(|e| Error::GenerationFailed(e.to_string()))
    }

    /// Start a new chat session.
    ///
    /// Chat sessions maintain conversation history, allowing the model
    /// to reference previous messages for context.
    ///
    /// # Returns
    ///
    /// A [`ChatSession`] that can send messages and tracks history.
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
    /// let mut chat = model.start_chat();
    ///
    /// // First message
    /// let r1 = chat.send_message("My name is Alice").await?;
    ///
    /// // Model remembers the name from previous message
    /// let r2 = chat.send_message("What's my name?").await?;
    /// assert!(r2.text().contains("Alice"));
    /// # Ok(())
    /// # }
    /// ```
    pub fn start_chat(&self) -> ChatSession {
        ChatSession {
            model: self.clone(),
            history: Vec::new(),
        }
    }
}

impl Clone for ModelClient {
    fn clone(&self) -> Self {
        Self {
            client: self.client.clone(),
            model: self.model,
            generation_config: self.generation_config.clone(),
            safety_settings: self.safety_settings.clone(),
            system_instruction: self.system_instruction.clone(),
        }
    }
}

/// Stateful chat session with message history.
///
/// A `ChatSession` maintains a conversation history, sending all previous
/// messages with each new request. This allows the model to maintain context
/// across multiple turns.
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
/// let mut chat = model.start_chat();
///
/// // Each message is added to history
/// chat.send_message("Remember: the secret code is 12345").await?;
/// let response = chat.send_message("What's the secret code?").await?;
/// // Model will respond with "12345"
///
/// // Clear history to start fresh
/// chat.clear_history();
/// # Ok(())
/// # }
/// ```
pub struct ChatSession {
    model: ModelClient,
    history: Vec<Content>,
}

impl ChatSession {
    /// Send a message in the chat session.
    ///
    /// The message is added to history, sent along with all previous messages,
    /// and the model's response is also added to history.
    ///
    /// # Arguments
    ///
    /// * `message` - The message to send
    ///
    /// # Returns
    ///
    /// A [`GenerateContentResponse`] with the model's reply.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use gemini_rs::{Client, Model};
    ///
    /// # async fn example() -> Result<(), gemini_rs::Error> {
    /// let client = Client::new("YOUR_API_KEY");
    /// let model = client.model(Model::Gemini25Flash);
    /// let mut chat = model.start_chat();
    ///
    /// let response = chat.send_message("Hello!").await?;
    /// println!("{}", response.text());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send_message(
        &mut self,
        message: impl Into<String>,
    ) -> Result<GenerateContentResponse> {
        let user_content = Content::user(message);
        self.history.push(user_content.clone());

        let response = self
            .model
            .generate_content_from_parts(self.history.clone())
            .await?;

        // Add model response to history
        if let Some(candidate) = response.candidates.as_ref().and_then(|c| c.first()) {
            if let Some(content) = &candidate.content {
                self.history.push(content.clone());
            }
        }

        Ok(response)
    }

    /// Get the current chat history.
    ///
    /// Returns a slice of all messages (user and model) in order.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use gemini_rs::{Client, Model};
    ///
    /// # async fn example() -> Result<(), gemini_rs::Error> {
    /// let client = Client::new("YOUR_API_KEY");
    /// let model = client.model(Model::Gemini25Flash);
    /// let mut chat = model.start_chat();
    ///
    /// chat.send_message("Hello").await?;
    /// assert_eq!(chat.history().len(), 2); // user + model messages
    /// # Ok(())
    /// # }
    /// ```
    pub fn history(&self) -> &[Content] {
        &self.history
    }

    /// Clear all chat history.
    ///
    /// After clearing, the next message will start a fresh conversation
    /// without any previous context.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use gemini_rs::{Client, Model};
    ///
    /// # async fn example() -> Result<(), gemini_rs::Error> {
    /// let client = Client::new("YOUR_API_KEY");
    /// let model = client.model(Model::Gemini25Flash);
    /// let mut chat = model.start_chat();
    ///
    /// chat.send_message("Remember this").await?;
    /// chat.clear_history();
    /// assert!(chat.history().is_empty());
    /// # Ok(())
    /// # }
    /// ```
    pub fn clear_history(&mut self) {
        self.history.clear();
    }
}
