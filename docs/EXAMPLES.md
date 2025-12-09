# Examples

This document provides practical examples for common use cases.

## Basic Usage

### Simple Text Generation

```rust
use gemini_rs::{Client, Model};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("GOOGLE_API_KEY")?;
    let client = Client::new(api_key);
    let model = client.model(Model::Gemini25Flash);

    let response = model
        .generate_content("Write a short poem about coding")
        .await?;
    
    println!("{}", response.text());
    Ok(())
}
```

## JSON Mode

### Extract Structured Data

```rust
use gemini_rs::{Client, Model};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Movie {
    title: String,
    year: u32,
    director: String,
    genres: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new(std::env::var("GOOGLE_API_KEY")?);
    let model = client.model(Model::Gemini25Flash);

    let movie: Movie = model
        .generate_json("Generate info about the movie Inception as JSON with title, year, director, and genres")
        .await?;
    
    println!("{:#?}", movie);
    Ok(())
}
```

### Parse Email Data

```rust
use gemini_rs::{Client, Model};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Transaction {
    merchant_name: String,
    amount: f64,
    currency: String,
    transaction_date: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new(std::env::var("GOOGLE_API_KEY")?);
    let model = client.model(Model::Gemini25Flash);

    let email_html = r#"
        <p>Your purchase at AMAZON for $29.99 USD on 2025-01-15 was successful.</p>
    "#;

    let prompt = format!(
        "Extract transaction info from this email as JSON: {}",
        email_html
    );

    let transaction: Transaction = model.generate_json(&prompt).await?;
    println!("{:#?}", transaction);
    Ok(())
}
```

## Chat Sessions

### Conversational AI

```rust
use gemini_rs::{Client, Model};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new(std::env::var("GOOGLE_API_KEY")?);
    let model = client.model(Model::Gemini25Flash);

    let mut chat = model.start_chat();

    // Start conversation
    let r1 = chat.send_message("Hi! I'm learning Rust. What should I know?").await?;
    println!("Assistant: {}", r1.text());

    // Follow-up (model remembers context)
    let r2 = chat.send_message("What about error handling?").await?;
    println!("Assistant: {}", r2.text());

    // Another follow-up
    let r3 = chat.send_message("Can you give me a code example?").await?;
    println!("Assistant: {}", r3.text());

    Ok(())
}
```

## Advanced Configuration

### Custom Settings

```rust
use gemini_rs::{Client, Model, GenerationConfig, SafetySettings};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new(std::env::var("GOOGLE_API_KEY")?);

    let config = GenerationConfig::new()
        .temperature(0.3)  // More focused responses
        .top_p(0.8)
        .max_tokens(500);

    let model = client
        .model(Model::Gemini25Flash)
        .with_config(config)
        .with_system_instruction("You are a helpful coding assistant. Be concise and provide code examples.");

    let response = model
        .generate_content("How do I read a file in Rust?")
        .await?;
    
    println!("{}", response.text());
    Ok(())
}
```

### Creative Writing Mode

```rust
use gemini_rs::{Client, Model, GenerationConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new(std::env::var("GOOGLE_API_KEY")?);

    let config = GenerationConfig::new()
        .temperature(1.5)  // More creative
        .top_p(0.95);

    let model = client
        .model(Model::Gemini25Flash)
        .with_config(config);

    let response = model
        .generate_content("Write a creative short story about a robot discovering art")
        .await?;
    
    println!("{}", response.text());
    Ok(())
}
```

## Error Handling

### Graceful Error Recovery

```rust
use gemini_rs::{Client, Model, Error};

#[tokio::main]
async fn main() {
    let client = Client::new(std::env::var("GOOGLE_API_KEY").unwrap_or_default());
    let model = client.model(Model::Gemini25Flash);

    match model.generate_content("Hello").await {
        Ok(response) => {
            println!("Success: {}", response.text());
        }
        Err(Error::InvalidApiKey) => {
            eprintln!("Please set a valid GOOGLE_API_KEY");
        }
        Err(Error::RateLimitExceeded) => {
            eprintln!("Rate limited. Please wait before retrying.");
        }
        Err(Error::ApiError { message, code }) => {
            eprintln!("API Error ({}): {}", code.unwrap_or(0), message);
        }
        Err(e) => {
            eprintln!("Unexpected error: {}", e);
        }
    }
}
```

## Real-World Workflow

### Multi-Step Processing

```rust
use gemini_rs::{Client, Model};
use serde::Deserialize;

#[derive(Deserialize)]
struct ClassificationResult {
    category: String,
    confidence: u8,
}

async fn classify_text(
    model: &gemini_rs::client::ModelClient,
    text: &str,
    categories: &[&str],
) -> Result<ClassificationResult, gemini_rs::Error> {
    let prompt = format!(
        r#"Classify this text into one category.
Return JSON: {{"category": "name", "confidence": 0-100}}

Text: {}

Categories: {}

Return only JSON."#,
        text,
        categories.join(", ")
    );

    model.generate_json(&prompt).await
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new(std::env::var("GOOGLE_API_KEY")?);
    let model = client.model(Model::Gemini25Flash);

    let result = classify_text(
        &model,
        "I love the new features in the latest iPhone!",
        &["Technology", "Sports", "Politics", "Entertainment"],
    ).await?;

    println!("Category: {} ({}% confident)", result.category, result.confidence);
    Ok(())
}
```
