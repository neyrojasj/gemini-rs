//! # gemini-rs
//!
//! Ergonomic Rust SDK for Google Gemini AI with a Python-like API.
//!
//! ## Features
//!
//! - **Simple API** - Python-like ergonomics in Rust
//! - **Type-safe** - Full type safety with Rust's type system
//! - **Async/await** - Built on tokio for async operations
//! - **JSON mode** - Generate structured JSON outputs with automatic parsing
//! - **Chat sessions** - Maintain conversation history
//! - **Multiple models** - Support for all Gemini models
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use gemini_rs::{Client, Model};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create client with your API key
//!     let client = Client::new("YOUR_API_KEY");
//!     
//!     // Get a model instance
//!     let model = client.model(Model::Gemini25Flash);
//!     
//!     // Generate content
//!     let response = model.generate_content("Explain quantum computing").await?;
//!     println!("{}", response.text());
//!     
//!     Ok(())
//! }
//! ```
//!
//! ## JSON Mode
//!
//! Generate structured JSON that deserializes directly into your types:
//!
//! ```rust,no_run
//! use gemini_rs::{Client, Model};
//! use serde::Deserialize;
//!
//! #[derive(Deserialize)]
//! struct Person {
//!     name: String,
//!     age: u32,
//! }
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let client = Client::new("YOUR_API_KEY");
//! let model = client.model(Model::Gemini25Flash);
//!
//! let person: Person = model
//!     .generate_json("Generate a random person with name and age")
//!     .await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Chat Sessions
//!
//! Maintain conversation context across multiple messages:
//!
//! ```rust,no_run
//! use gemini_rs::{Client, Model};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let client = Client::new("YOUR_API_KEY");
//! let model = client.model(Model::Gemini25Flash);
//!
//! let mut chat = model.start_chat();
//! let r1 = chat.send_message("My name is Alice").await?;
//! let r2 = chat.send_message("What's my name?").await?; // Remembers "Alice"
//! # Ok(())
//! # }
//! ```
//!
//! ## Configuration
//!
//! Fine-tune generation with [`GenerationConfig`]:
//!
//! ```rust,no_run
//! use gemini_rs::{Client, Model, GenerationConfig};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let client = Client::new("YOUR_API_KEY");
//!
//! let config = GenerationConfig::new()
//!     .temperature(0.7)
//!     .max_tokens(1000);
//!
//! let model = client
//!     .model(Model::Gemini25Flash)
//!     .with_config(config);
//! # Ok(())
//! # }
//! ```
//!
//! ## Error Handling
//!
//! All operations return [`Result<T, Error>`](Error):
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
//!     Err(Error::RateLimitExceeded) => eprintln!("Rate limited"),
//!     Err(e) => eprintln!("Error: {}", e),
//! }
//! # }
//! ```

pub mod client;
pub mod error;
pub mod models;
pub mod types;

pub use client::{ChatSession, Client, ModelClient};
pub use error::{Error, Result};
pub use models::Model;
pub use types::{Content, GenerateContentResponse, GenerationConfig, Part, SafetySettings};
