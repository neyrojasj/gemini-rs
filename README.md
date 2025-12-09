# gemini-rs 🦀

[![Crates.io](https://img.shields.io/crates/v/gemini-rs.svg)](https://crates.io/crates/gemini-rs)
[![Documentation](https://docs.rs/gemini-rs/badge.svg)](https://docs.rs/gemini-rs)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Ergonomic Rust SDK for Google Gemini AI with a Python-like API.

## Features

- ✅ **Simple API** - Python-like ergonomics in Rust
- ✅ **Type-safe** - Full type safety with Rust's type system
- ✅ **Async/await** - Built on tokio for async operations
- ✅ **JSON mode** - Generate structured JSON outputs with automatic parsing
- ✅ **Chat sessions** - Maintain conversation history
- ✅ **Multiple models** - Support for all Gemini models (2.5, 2.0, 1.5, 1.0)
- ✅ **Configuration** - Fine-tune generation parameters
- ✅ **Error handling** - Comprehensive error types
- ✅ **Multimodal** - Optional image support via base64 encoding

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
gemini-rs = "0.1"
tokio = { version = "1", features = ["full"] }
```

## Quick Start

```rust
use gemini_rs::{Client, Model};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new("YOUR_API_KEY");
    let model = client.model(Model::Gemini25Flash);
    
    let response = model.generate_content("Explain quantum computing").await?;
    println!("{}", response.text());
    
    Ok(())
}
```

## Examples

### Basic Text Generation

```rust
let client = Client::new(api_key);
let model = client.model(Model::Gemini25Flash);

let response = model
    .generate_content("Write a haiku about Rust")
    .await?;

println!("{}", response.text());
```

### JSON Mode (Structured Output)

```rust
use serde::Deserialize;

#[derive(Deserialize)]
struct Person {
    name: String,
    age: u32,
    occupation: String,
}

let person: Person = model
    .generate_json("Generate a random person with name, age, and occupation")
    .await?;

println!("{} is {} years old", person.name, person.age);
```

### Chat Sessions

```rust
let mut chat = model.start_chat();

let response1 = chat.send_message("Hello! My name is Alice.").await?;
println!("{}", response1.text());

let response2 = chat.send_message("What's my name?").await?;
println!("{}", response2.text()); // Model remembers "Alice"
```

### Advanced Configuration

```rust
use gemini_rs::{GenerationConfig, SafetySettings};

let config = GenerationConfig::new()
    .temperature(0.7)
    .top_p(0.9)
    .max_tokens(1000);

let model = client
    .model(Model::Gemini25Flash)
    .with_config(config)
    .with_safety(SafetySettings::block_none())
    .with_system_instruction("You are a helpful assistant");

let response = model.generate_content("Your prompt").await?;
```

## Available Models

| Model | Description | Use Case |
|-------|-------------|----------|
| `Model::Gemini25Flash` | Latest and most advanced ⭐ | Recommended for most use cases |
| `Model::Gemini20Flash` | Previous generation | Stable, well-tested |
| `Model::Gemini15Pro` | Most capable 1.5 model | Complex reasoning tasks |
| `Model::Gemini15Flash` | Fast and efficient | Balance of speed and quality |
| `Model::Gemini15Flash8B` | Smallest and fastest | High-volume, simple tasks |
| `Model::Gemini10Pro` | Legacy model | Backward compatibility |

## Error Handling

```rust
use gemini_rs::Error;

match model.generate_content("prompt").await {
    Ok(response) => println!("{}", response.text()),
    Err(Error::ApiError { message, code }) => {
        eprintln!("API error {}: {}", code.unwrap_or(0), message);
    }
    Err(Error::RateLimitExceeded) => {
        eprintln!("Rate limit exceeded, please wait");
    }
    Err(e) => eprintln!("Error: {}", e),
}
```

## Environment Setup

Get your API key from: https://aistudio.google.com/app/apikey

```bash
export GOOGLE_API_KEY="your-api-key-here"
```

## Documentation

- [API Reference](https://docs.rs/gemini-rs) - Full API documentation
- [Contributing Guide](CONTRIBUTING.md) - How to contribute
- [Architecture](docs/ARCHITECTURE.md) - Internal architecture for contributors

## Running Tests

```bash
# Unit tests (no API key required)
cargo test --test unit_tests

# Integration tests (requires GOOGLE_API_KEY)
export GOOGLE_API_KEY="your-key"
cargo test --test basic_test -- --ignored
cargo test --test json_mode_test -- --ignored
cargo test --test chat_test -- --ignored
```

## License

MIT License - see [LICENSE](LICENSE) for details.

## Acknowledgments

Inspired by the official Python SDK for Google Gemini AI.
