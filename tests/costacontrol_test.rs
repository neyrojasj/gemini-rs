//! CostaControl workflow integration test
//!
//! Run with: cargo test --test costacontrol_test -- --ignored

use gemini_rs::{Client, Model};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Transaction {
    merchant_name: String,
    amount: f64,
    currency: String,
    transaction_date: String,
    transaction_type: String,
    card_last_digits: String,
}

#[derive(Debug, Deserialize)]
struct CategoryResult {
    category: String,
    confidence: u8,
}

#[tokio::test]
#[ignore]
async fn test_costacontrol_workflow() {
    let api_key =
        std::env::var("GOOGLE_API_KEY").expect("GOOGLE_API_KEY environment variable not set");

    let client = Client::new(api_key);
    let model = client.model(Model::Gemini25Flash);

    let email_html = r#"
        <html>
        <body>
            <h2>Transaction Alert - BAC San José</h2>
            <table>
                <tr><td>Merchant:</td><td>UBER RIDES</td></tr>
                <tr><td>Amount:</td><td>₡1,824.60</td></tr>
                <tr><td>Currency:</td><td>CRC</td></tr>
                <tr><td>Date:</td><td>2025-11-23</td></tr>
                <tr><td>Type:</td><td>COMPRA</td></tr>
                <tr><td>Card:</td><td>************8662</td></tr>
            </table>
        </body>
        </html>
    "#;

    // Step 1: Extract transaction
    let extract_prompt = format!(
        r#"Extract transaction information from this bank email.
Return ONLY valid JSON with these exact fields:
- merchant_name: string
- amount: number
- currency: string (CRC, USD, etc)
- transaction_date: string (YYYY-MM-DD format)
- transaction_type: string
- card_last_digits: string (last 4 digits only)

Email HTML:
{}

Return ONLY the JSON, no markdown or explanations."#,
        email_html
    );

    let transaction: Transaction = model
        .generate_json(&extract_prompt)
        .await
        .expect("Failed to extract transaction");

    assert_eq!(transaction.merchant_name, "UBER RIDES");
    assert!((transaction.amount - 1824.60).abs() < 0.01);
    assert_eq!(transaction.currency, "CRC");
    assert_eq!(transaction.card_last_digits, "8662");

    // Step 2: Categorize transaction
    let categories = vec![
        "Supermercados",
        "Transporte y Combustible",
        "Restaurantes",
        "Entretenimiento",
        "Servicios Públicos",
        "Salud y Medicina",
        "Ropa y Accesorios",
        "Educación",
        "Otros",
    ];

    let categorize_prompt = format!(
        r#"Categorize this transaction into ONE of the following categories.
Return ONLY valid JSON: {{"category": "exact_category_name", "confidence": 85}}

Transaction:
- Merchant: {}
- Amount: {} {}
- Type: {}

Available categories:
{}

Rules:
- Use the EXACT category name from the list
- Confidence: 0-100 (how certain you are)

Return ONLY the JSON, no markdown or explanations."#,
        transaction.merchant_name,
        transaction.amount,
        transaction.currency,
        transaction.transaction_type,
        categories.join("\n")
    );

    let category_result: CategoryResult = model
        .generate_json(&categorize_prompt)
        .await
        .expect("Failed to categorize transaction");

    assert_eq!(category_result.category, "Transporte y Combustible");
    assert!(
        category_result.confidence >= 70,
        "Confidence should be high for UBER"
    );
}
