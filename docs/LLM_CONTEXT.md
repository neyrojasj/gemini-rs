# LLM Agent Context

> **For AI Assistants**: This document provides essential context for understanding and modifying the gemini-rs codebase.

## Quick Summary

**gemini-rs** is a Rust SDK for Google's Gemini AI API. It wraps the REST API with an ergonomic, type-safe interface.

## Key Files

| File | Purpose | When to Modify |
|------|---------|----------------|
| `src/lib.rs` | Public exports | Adding new public types |
| `src/client.rs` | API client, model client, chat | Adding features, fixing bugs |
| `src/models.rs` | Model enum | Adding new Gemini models |
| `src/types.rs` | Request/response types | API changes, new fields |
| `src/error.rs` | Error definitions | New error cases |

## Common Tasks

### Adding a New Model

1. **Edit** `src/models.rs`:
   ```rust
   pub enum Model {
       Gemini25Flash,
       Gemini20Flash,
       NewModel,  // Add here
       // ...
   }
   ```

2. **Add** the API identifier:
   ```rust
   impl Model {
       pub fn as_str(&self) -> &'static str {
           match self {
               Model::NewModel => "new-model-name",
               // ...
           }
       }
   }
   ```

3. **Add** From<&str> conversion:
   ```rust
   impl From<&str> for Model {
       fn from(s: &str) -> Self {
           match s {
               "new-model-name" => Model::NewModel,
               // ...
           }
       }
   }
   ```

### Adding a New Configuration Option

1. **Add field** to `GenerationConfig` in `src/types.rs`
2. **Add builder method** for the field
3. **Ensure** serde serialization is correct
4. **Add test** in `tests/unit_tests.rs`

### Adding a New Method to ModelClient

1. **Define method** in `src/client.rs` under `impl ModelClient`
2. **Add async** if it calls the API
3. **Return** `Result<T>` with appropriate error handling
4. **Add tests** - unit if possible, integration if API required

## API Patterns

### Request Flow

```
User calls method → Build request → HTTP POST → Parse response → Return typed result
```

### Error Handling

All fallible operations return `Result<T, Error>`. Use `?` for propagation.

### JSON Mode

Set `response_mime_type = "application/json"` in GenerationConfig, then parse with serde.

## Testing Requirements

- Unit tests: `cargo test --test unit_tests`
- Integration tests: `GOOGLE_API_KEY=... cargo test -- --ignored`

## Code Style

- Use `impl Into<String>` for string parameters
- Use builder pattern for configuration
- Use `#[serde(skip_serializing_if = "Option::is_none")]` for optional fields
- Document public items with `///` doc comments

## Dependencies

Core dependencies (don't remove):
- `reqwest` - HTTP client
- `tokio` - Async runtime
- `serde` / `serde_json` - Serialization
- `thiserror` - Error derive

## Environment Variables

| Variable | Purpose |
|----------|---------|
| `GOOGLE_API_KEY` | API authentication |

## Links

- [Gemini API Docs](https://ai.google.dev/gemini-api/docs)
- [API Reference](https://ai.google.dev/api/rest)
