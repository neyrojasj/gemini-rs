//! JSON mode integration test
//!
//! Run with: cargo test --test json_mode_test -- --ignored

use gemini_rs::{Client, Model};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
struct Transaction {
    merchant_name: String,
    amount: f64,
    currency: String,
    transaction_date: String,
    transaction_type: String,
    card_last_digits: String,
}

#[tokio::test]
#[ignore]
async fn test_json_extraction() {
    let api_key =
        std::env::var("GOOGLE_API_KEY").expect("GOOGLE_API_KEY environment variable not set");

    let client = Client::new(api_key);
    let model = client.model(Model::Gemini25Flash);

    let email_html = r#"
        <html>
        <body>
            <h1>Transaction Alert</h1>
            <p>Merchant: UBER RIDES</p>
            <p>Amount: $15.50 USD</p>
            <p>Date: 2025-11-23</p>
            <p>Card: ****1234</p>
        </body>
        </html>
    "#;

    let prompt = format!(
        r#"Extract transaction information from this email.
        Return ONLY valid JSON with these fields:
        - merchant_name
        - amount (number)
        - currency
        - transaction_date (YYYY-MM-DD)
        - transaction_type
        - card_last_digits

        Email:
        {}
        "#,
        email_html
    );

    let transaction: Transaction = model
        .generate_json(&prompt)
        .await
        .expect("Failed to extract transaction");
    println!("Transaction: {:#?}", transaction);
    assert_eq!(transaction.merchant_name, "UBER RIDES");
    assert!((transaction.amount - 15.50).abs() < 0.01);
    assert_eq!(transaction.currency, "USD");
    assert_eq!(transaction.card_last_digits, "1234");
}
