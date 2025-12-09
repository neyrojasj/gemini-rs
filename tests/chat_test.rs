//! Chat session integration test
//!
//! Run with: cargo test --test chat_test -- --ignored

use gemini_rs::{Client, Model};

#[tokio::test]
#[ignore]
async fn test_chat_session() {
    let api_key =
        std::env::var("GOOGLE_API_KEY").expect("GOOGLE_API_KEY environment variable not set");

    let client = Client::new(api_key);
    let model = client.model(Model::Gemini25Flash);

    let mut chat = model.start_chat();

    // First message
    let response1 = chat
        .send_message("My name is Ney. Remember this.")
        .await
        .expect("Failed to send first message");
    assert!(!response1.text().is_empty());

    // Second message - should remember context
    let response2 = chat
        .send_message("What is my name?")
        .await
        .expect("Failed to send second message");

    let text = response2.text().to_lowercase();
    assert!(
        text.contains("ney"),
        "Model should remember the name from previous message"
    );

    // Verify history
    assert_eq!(chat.history().len(), 4); // 2 user + 2 model messages
}

#[tokio::test]
#[ignore]
async fn test_chat_clear_history() {
    let api_key =
        std::env::var("GOOGLE_API_KEY").expect("GOOGLE_API_KEY environment variable not set");

    let client = Client::new(api_key);
    let model = client.model(Model::Gemini20Flash);

    let mut chat = model.start_chat();

    chat.send_message("Hello")
        .await
        .expect("Failed to send message");
    assert!(!chat.history().is_empty());

    chat.clear_history();
    assert!(chat.history().is_empty());
}
