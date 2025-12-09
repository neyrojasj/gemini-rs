//! Basic integration test: Generate content from text prompt
//!
//! This test requires GOOGLE_API_KEY environment variable to be set.
//! Run with: cargo test --test basic_test -- --ignored

use gemini_rs::{Client, Model};

#[tokio::test]
#[ignore] // Requires API key
async fn test_basic_generation() {
    let api_key =
        std::env::var("GOOGLE_API_KEY").expect("GOOGLE_API_KEY environment variable not set");

    let client = Client::new(api_key);
    let model = client.model(Model::Gemini25Flash);

    let response = model
        .generate_content("Say 'Hello from Rust!' and nothing else")
        .await
        .expect("Failed to generate content");

    let text = response.text();
    println!("Response: {}", text);
    assert!(!text.is_empty(), "Response should not be empty");
    assert!(
        text.to_lowercase().contains("hello"),
        "Response should contain 'hello'"
    );
}

#[tokio::test]
#[ignore] // Requires API key
async fn test_multiple_models() {
    let api_key =
        std::env::var("GOOGLE_API_KEY").expect("GOOGLE_API_KEY environment variable not set");

    let client = Client::new(api_key);

    // Test Gemini 2.5 Flash
    let model_25 = client.model(Model::Gemini25Flash);
    let response_25 = model_25
        .generate_content("Say 'test'")
        .await
        .expect("Gemini 2.5 Flash failed");
    assert!(!response_25.text().is_empty());

    // Test Gemini 2.0 Flash
    let model_20 = client.model(Model::Gemini20Flash);
    let response_20 = model_20
        .generate_content("Say 'test'")
        .await
        .expect("Gemini 2.0 Flash failed");
    assert!(!response_20.text().is_empty());
}
