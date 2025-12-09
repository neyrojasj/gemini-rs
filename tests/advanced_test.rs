//! Advanced configuration integration test
//!
//! Run with: cargo test --test advanced_test -- --ignored

use gemini_rs::{Client, GenerationConfig, Model, SafetySettings};

#[tokio::test]
#[ignore]
async fn test_with_configuration() {
    let api_key =
        std::env::var("GOOGLE_API_KEY").expect("GOOGLE_API_KEY environment variable not set");

    let client = Client::new(api_key);

    let config = GenerationConfig::new()
        .temperature(0.7)
        .top_p(0.9)
        .max_tokens(100);

    let model = client
        .model(Model::Gemini25Flash)
        .with_config(config)
        .with_safety(SafetySettings::block_none())
        .with_system_instruction("You are a helpful assistant. Be concise.");

    let response = model
        .generate_content("Explain AI in one sentence")
        .await
        .expect("Failed to generate content");

    let text = response.text();
    assert!(!text.is_empty());
    // With max_tokens=100, response should be relatively short
    assert!(text.len() < 500, "Response should be concise");
}

#[tokio::test]
#[ignore]
async fn test_json_mode_config() {
    let api_key =
        std::env::var("GOOGLE_API_KEY").expect("GOOGLE_API_KEY environment variable not set");

    let client = Client::new(api_key);

    let config = GenerationConfig::new().json_mode().temperature(0.1); // Low temperature for consistent JSON

    let model = client.model(Model::Gemini20Flash).with_config(config);

    use serde::Deserialize;

    #[derive(Deserialize)]
    struct SimpleResponse {
        message: String,
    }

    let result: SimpleResponse = model
        .generate_json(r#"Return JSON: {"message": "Hello from Gemini"}"#)
        .await
        .expect("Failed to generate JSON");

    assert_eq!(result.message, "Hello from Gemini");
}
