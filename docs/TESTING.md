# Testing Guide

This document describes the testing strategy and how to run tests for gemini-rs.

## Test Organization

```
tests/
├── unit_tests.rs       # No API key required
├── basic_test.rs       # Basic generation tests
├── json_mode_test.rs   # Structured output tests
├── chat_test.rs        # Chat session tests
├── advanced_test.rs    # Configuration tests
└── costacontrol_test.rs # Real-world workflow test
```

## Test Categories

### Unit Tests (No API Key)

Located in `tests/unit_tests.rs`. These tests verify:

- Model enum conversions
- GenerationConfig builder
- JSON mode configuration
- Client creation
- Type correctness

**Run:**
```bash
cargo test --test unit_tests
```

### Integration Tests (Require API Key)

All other test files require a valid `GOOGLE_API_KEY` environment variable. They are marked with `#[ignore]` so they don't run by default.

#### Basic Test (`basic_test.rs`)
- Simple text generation
- Multiple model testing

#### JSON Mode Test (`json_mode_test.rs`)
- Structured output extraction
- Type deserialization

#### Chat Test (`chat_test.rs`)
- Conversation context
- History management
- Clear history functionality

#### Advanced Test (`advanced_test.rs`)
- Custom GenerationConfig
- Safety settings
- System instructions

#### CostaControl Test (`costacontrol_test.rs`)
- Transaction extraction from HTML
- Multi-step AI workflow
- Category classification

## Running Tests

### Prerequisites

1. Rust toolchain installed
2. For integration tests: Valid Google API key

### Run All Unit Tests

```bash
cargo test --test unit_tests
```

### Run All Integration Tests

```bash
export GOOGLE_API_KEY="your-api-key"
cargo test -- --ignored
```

### Run Specific Integration Test

```bash
export GOOGLE_API_KEY="your-api-key"
cargo test --test basic_test -- --ignored
cargo test --test json_mode_test -- --ignored
cargo test --test chat_test -- --ignored
cargo test --test advanced_test -- --ignored
cargo test --test costacontrol_test -- --ignored
```

### Run With Verbose Output

```bash
cargo test -- --nocapture
```

## Writing New Tests

### Unit Test Example

```rust
#[test]
fn test_new_feature() {
    // Test without API calls
    let config = GenerationConfig::new().temperature(0.5);
    assert_eq!(config.temperature, Some(0.5));
}
```

### Integration Test Example

```rust
#[tokio::test]
#[ignore] // Mark as ignored (requires API key)
async fn test_api_feature() {
    let api_key = std::env::var("GOOGLE_API_KEY")
        .expect("GOOGLE_API_KEY environment variable not set");

    let client = Client::new(api_key);
    let model = client.model(Model::Gemini25Flash);

    let response = model
        .generate_content("Test prompt")
        .await
        .expect("Failed to generate");

    assert!(!response.text().is_empty());
}
```

## Test Best Practices

1. **Unit tests should not require API keys** - Test logic, not the API
2. **Integration tests should be marked `#[ignore]`** - Don't run in CI without secrets
3. **Use descriptive assertions** - Make failures easy to diagnose
4. **Test edge cases** - Empty responses, malformed JSON, etc.
5. **Keep tests focused** - One concept per test function

## CI/CD Considerations

For GitHub Actions or other CI systems:

```yaml
# Run unit tests (always)
- run: cargo test --test unit_tests

# Run integration tests (only if secret available)
- run: cargo test -- --ignored
  env:
    GOOGLE_API_KEY: ${{ secrets.GOOGLE_API_KEY }}
  if: ${{ secrets.GOOGLE_API_KEY != '' }}
```

## Troubleshooting

### "GOOGLE_API_KEY environment variable not set"

Set the environment variable:
```bash
export GOOGLE_API_KEY="your-key"
```

### API Rate Limiting

If tests fail with rate limit errors:
- Wait a few minutes before retrying
- Reduce test parallelism: `cargo test -- --test-threads=1`

### JSON Parsing Failures

If JSON mode tests fail:
- Check the prompt is clear about expected format
- Verify the model supports JSON mode
- Check for markdown code blocks in response
