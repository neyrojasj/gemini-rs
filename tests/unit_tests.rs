//! Unit tests for gemini-rs crate
//! These tests don't require API keys and test the structure/types

use gemini_rs::{Client, GenerationConfig, Model};

#[test]
fn test_model_enum() {
    assert_eq!(Model::Gemini25Flash.as_str(), "gemini-2.5-flash");
    assert_eq!(Model::Gemini20Flash.as_str(), "gemini-2.0-flash");
    assert_eq!(Model::Gemini15Pro.as_str(), "gemini-1.5-pro");
    assert_eq!(Model::Gemini15Flash.as_str(), "gemini-1.5-flash");
    assert_eq!(Model::Gemini15Flash8B.as_str(), "gemini-1.5-flash-8b");
    assert_eq!(Model::Gemini10Pro.as_str(), "gemini-1.0-pro");
}

#[test]
fn test_model_from_str() {
    assert_eq!(Model::from("gemini-2.5-flash"), Model::Gemini25Flash);
    assert_eq!(Model::from("gemini-2.0-flash"), Model::Gemini20Flash);
    assert_eq!(Model::from("gemini-1.5-pro"), Model::Gemini15Pro);
    assert_eq!(Model::from("gemini-1.5-flash"), Model::Gemini15Flash);
    assert_eq!(Model::from("unknown"), Model::Gemini25Flash); // Default to latest
}

#[test]
fn test_generation_config_builder() {
    let config = GenerationConfig::new()
        .temperature(0.7)
        .top_p(0.9)
        .top_k(40)
        .max_tokens(1000);

    assert_eq!(config.temperature, Some(0.7));
    assert_eq!(config.top_p, Some(0.9));
    assert_eq!(config.top_k, Some(40));
    assert_eq!(config.max_output_tokens, Some(1000));
}

#[test]
fn test_json_mode_config() {
    let config = GenerationConfig::new().json_mode();
    assert_eq!(
        config.response_mime_type,
        Some("application/json".to_string())
    );
}

#[test]
fn test_client_creation() {
    let client = Client::new("test_api_key");
    let model = client.model(Model::Gemini20Flash);
    // Just verify it compiles and creates successfully
    drop(model);
}

#[test]
fn test_model_full_name() {
    assert_eq!(Model::Gemini25Flash.full_name(), "models/gemini-2.5-flash");
    assert_eq!(Model::Gemini20Flash.full_name(), "models/gemini-2.0-flash");
    assert_eq!(Model::Gemini15Flash.full_name(), "models/gemini-1.5-flash");
}
