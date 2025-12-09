# gemini-rs Architecture

This document provides a comprehensive overview of the gemini-rs crate architecture for contributors and LLM-based development assistants.

## Project Overview

**gemini-rs** is a Rust SDK for Google's Gemini AI API. It provides an ergonomic, Python-like interface for generating content, structured JSON outputs, and managing chat sessions.

### Design Goals

1. **Ergonomic API** - Mirror the simplicity of Python's google-generativeai
2. **Type Safety** - Leverage Rust's type system for compile-time guarantees
3. **Async-First** - Built on tokio for non-blocking operations
4. **Zero Boilerplate** - Minimize setup code for common use cases

## Module Structure

```
src/
├── lib.rs       # Public API exports and crate documentation
├── client.rs    # HTTP client, model client, and chat sessions
├── models.rs    # Model enum definitions
├── types.rs     # Request/response types, content structures
└── error.rs     # Error types and Result alias
```

### Module Responsibilities

#### `lib.rs` - Public API
- Re-exports all public types
- Contains crate-level documentation
- Defines the public interface

#### `client.rs` - Core Client Logic
- `Client` - Main API client, holds HTTP client and API key
- `ModelClient` - Model-specific client with configuration
- `ChatSession` - Stateful chat with message history

#### `models.rs` - Model Definitions
- `Model` enum - All supported Gemini models
- Model name conversions (API identifiers)
- Default model selection

#### `types.rs` - Data Structures
- `Content` - Text/multimodal content
- `Part` - Individual content parts (text, images)
- `GenerateContentRequest` - API request structure
- `GenerateContentResponse` - API response structure
- `GenerationConfig` - Temperature, top_p, max_tokens, etc.
- `SafetySetting` - Content safety configuration

#### `error.rs` - Error Handling
- `Error` enum - All possible errors
- `Result<T>` type alias

## Data Flow

### Simple Generation

```
User Code                    gemini-rs                      Gemini API
─────────                    ─────────                      ──────────
                                                            
client.model(...)       →   Create ModelClient
                                                            
model.generate_content  →   Build GenerateContentRequest
                        →   POST /models/{model}:generateContent
                                                       →   Process
                                                       ←   JSON Response
                        ←   Parse GenerateContentResponse
response.text()         ←   Extract text from candidates
```

### JSON Mode

```
model.generate_json<T>  →   Set response_mime_type = "application/json"
                        →   Build request with JSON mode config
                        →   POST to API
                        ←   Parse response
                        ←   serde_json::from_str::<T>()
```

### Chat Session

```
chat.send_message("hi") →   Append user message to history
                        →   Send all history in request
                        ←   Get model response
                        ←   Append model response to history
                        ←   Return response
```

## Key Design Patterns

### Builder Pattern
`GenerationConfig` uses the builder pattern for fluent configuration:

```rust
let config = GenerationConfig::new()
    .temperature(0.7)
    .top_p(0.9)
    .max_tokens(1000);
```

### Method Chaining
`ModelClient` supports chaining for setup:

```rust
let model = client
    .model(Model::Gemini25Flash)
    .with_config(config)
    .with_safety(safety)
    .with_system_instruction("...");
```

### Type-Safe Deserialization
JSON mode leverages Rust's type system:

```rust
// Compile-time type checking
let person: Person = model.generate_json(prompt).await?;
```

## API Endpoints

All requests go to: `https://generativelanguage.googleapis.com/v1beta`

| Operation | Endpoint | Method |
|-----------|----------|--------|
| Generate Content | `/models/{model}:generateContent` | POST |

## Error Handling Strategy

Errors are categorized by source:

| Error Type | Cause | Recovery |
|------------|-------|----------|
| `HttpError` | Network issues | Retry with backoff |
| `ApiError` | API returned error | Check message/code |
| `RateLimitExceeded` | Too many requests | Wait and retry |
| `NoResponse` | Empty response | Retry or check prompt |
| `GenerationFailed` | JSON parsing failed | Check prompt format |

## Feature Flags

```toml
[features]
default = ["multimodal"]
multimodal = ["base64", "mime"]  # Image support
```

## Dependencies

| Crate | Purpose |
|-------|---------|
| `reqwest` | HTTP client |
| `tokio` | Async runtime |
| `serde` / `serde_json` | Serialization |
| `thiserror` | Error derive macro |
| `base64` (optional) | Image encoding |

## Testing Strategy

See [TESTING.md](TESTING.md) for detailed testing documentation.

- **Unit tests** - No API key, test structures and logic
- **Integration tests** - Require API key, test real API calls

## Extension Points

### Adding a New Model

1. Add variant to `Model` enum in `models.rs`
2. Add `as_str()` match arm
3. Add `From<&str>` match arm
4. Update documentation

### Adding New Functionality

1. Add types to `types.rs` if needed
2. Add methods to `ModelClient` in `client.rs`
3. Add error variants to `error.rs` if needed
4. Add tests
5. Update documentation

## Performance Considerations

- HTTP client is reused via `Clone` on `Client`
- Requests are async, non-blocking
- Response parsing is streaming-capable (future enhancement)
- Memory: Chat history grows with conversation

## Security Notes

- API keys should be provided via environment variables
- Never log or expose API keys
- Use `SafetySettings::block_none()` carefully
