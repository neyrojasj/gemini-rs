# gemini-rs API Reference

This document provides a quick reference for the gemini-rs public API.

## Core Types

### `Client`

Main entry point for the Gemini API.

```rust
impl Client {
    /// Create a new client with an API key
    pub fn new(api_key: impl Into<String>) -> Self;
    
    /// Get a model-specific client
    pub fn model(&self, model: Model) -> ModelClient;
}
```

### `ModelClient`

Client for a specific Gemini model.

```rust
impl ModelClient {
    /// Set generation configuration
    pub fn with_config(self, config: GenerationConfig) -> Self;
    
    /// Set safety settings
    pub fn with_safety(self, settings: Vec<SafetySetting>) -> Self;
    
    /// Set system instruction
    pub fn with_system_instruction(self, instruction: impl Into<String>) -> Self;
    
    /// Generate content from a text prompt
    pub async fn generate_content(&self, prompt: impl Into<String>) 
        -> Result<GenerateContentResponse>;
    
    /// Generate content from multiple parts
    pub async fn generate_content_from_parts(&self, contents: Vec<Content>) 
        -> Result<GenerateContentResponse>;
    
    /// Generate and parse JSON response
    pub async fn generate_json<T: DeserializeOwned>(&self, prompt: impl Into<String>) 
        -> Result<T>;
    
    /// Start a chat session
    pub fn start_chat(&self) -> ChatSession;
}
```

### `ChatSession`

Stateful chat session with message history.

```rust
impl ChatSession {
    /// Send a message and get a response
    pub async fn send_message(&mut self, message: impl Into<String>) 
        -> Result<GenerateContentResponse>;
    
    /// Get the chat history
    pub fn history(&self) -> &[Content];
    
    /// Clear the chat history
    pub fn clear_history(&mut self);
}
```

### `Model`

Available Gemini models.

```rust
pub enum Model {
    Gemini25Flash,   // gemini-2.5-flash (latest)
    Gemini20Flash,   // gemini-2.0-flash
    Gemini15Pro,     // gemini-1.5-pro
    Gemini15Flash,   // gemini-1.5-flash
    Gemini15Flash8B, // gemini-1.5-flash-8b
    Gemini10Pro,     // gemini-1.0-pro (legacy)
}

impl Model {
    /// Get the API model identifier
    pub fn as_str(&self) -> &'static str;
    
    /// Get the full model name for API calls
    pub fn full_name(&self) -> String;
}
```

### `GenerationConfig`

Configuration for content generation.

```rust
impl GenerationConfig {
    pub fn new() -> Self;
    
    /// Set temperature (0.0 - 2.0, default: 1.0)
    pub fn temperature(self, temp: f32) -> Self;
    
    /// Set top_p for nucleus sampling (0.0 - 1.0)
    pub fn top_p(self, top_p: f32) -> Self;
    
    /// Set top_k for sampling
    pub fn top_k(self, top_k: i32) -> Self;
    
    /// Set maximum output tokens
    pub fn max_tokens(self, max: i32) -> Self;
    
    /// Enable JSON mode
    pub fn json_mode(self) -> Self;
}
```

### `SafetySettings`

Builder for safety settings.

```rust
impl SafetySettings {
    /// Create settings that block no content
    pub fn block_none() -> Vec<SafetySetting>;
}
```

### `Content`

Content structure for requests/responses.

```rust
impl Content {
    /// Create content from text
    pub fn text(text: impl Into<String>) -> Self;
    
    /// Create user-role content
    pub fn user(text: impl Into<String>) -> Self;
    
    /// Create model-role content
    pub fn model(text: impl Into<String>) -> Self;
}
```

### `GenerateContentResponse`

Response from content generation.

```rust
impl GenerateContentResponse {
    /// Get the text from the first candidate
    pub fn text(&self) -> String;
    
    /// Parse JSON response
    pub fn json<T: DeserializeOwned>(&self) -> Result<T, serde_json::Error>;
}
```

### `Error`

Error types.

```rust
pub enum Error {
    HttpError(reqwest::Error),
    JsonError(serde_json::Error),
    ApiError { message: String, code: Option<i32> },
    NoResponse,
    InvalidApiKey,
    RateLimitExceeded,
    InvalidModel(String),
    GenerationFailed(String),
    InvalidInput(String),
}
```

## Result Type

```rust
pub type Result<T> = std::result::Result<T, Error>;
```

## Feature Flags

| Feature | Description | Default |
|---------|-------------|---------|
| `multimodal` | Image support via base64 | ✓ |
