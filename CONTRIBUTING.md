# Contributing to gemini-rs

Thank you for your interest in contributing to gemini-rs! This document provides guidelines and instructions for contributing.

## Table of Contents

- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Running Tests](#running-tests)
- [Code Style](#code-style)
- [Pull Request Process](#pull-request-process)
- [Issue Guidelines](#issue-guidelines)

## Getting Started

### Prerequisites

- **Rust 1.70+** - Install via [rustup](https://rustup.rs/)
- **Git** - For version control
- **Google API Key** - For running integration tests (optional for unit tests)

### Getting Your API Key

1. Go to [Google AI Studio](https://aistudio.google.com/app/apikey)
2. Sign in with your Google account
3. Click "Create API Key"
4. Copy the key for testing

## Development Setup

1. **Clone the repository:**
   ```bash
   git clone https://github.com/neyrojasj/gemini-rs.git
   cd gemini-rs
   ```

2. **Build the project:**
   ```bash
   cargo build
   ```

3. **Set up your API key (for integration tests):**
   ```bash
   export GOOGLE_API_KEY="your-api-key-here"
   ```

## Running Tests

### Unit Tests (No API Key Required)

Unit tests verify the library's internal logic without making API calls:

```bash
cargo test --test unit_tests
```

These tests check:
- Model enum conversions
- GenerationConfig builder
- Type serialization
- Client creation

**All unit tests must pass before submitting a PR.**

### Integration Tests (Requires API Key)

Integration tests make real API calls and require a valid `GOOGLE_API_KEY`:

```bash
# Set your API key
export GOOGLE_API_KEY="your-api-key"

# Run all integration tests
cargo test -- --ignored

# Run specific test file
cargo test --test basic_test -- --ignored
cargo test --test json_mode_test -- --ignored
cargo test --test chat_test -- --ignored
```

**Before submitting a PR, verify integration tests pass with your API key.**

### Running All Tests

```bash
# Unit tests only (always run)
cargo test --test unit_tests

# All tests (requires API key)
cargo test -- --include-ignored
```

### Test with Verbose Output

```bash
cargo test -- --nocapture
```

## Code Style

### Formatting

Format your code with rustfmt:

```bash
cargo fmt
```

### Linting

Check for common issues with clippy:

```bash
cargo clippy -- -D warnings
```

### Documentation

- Document all public items with `///` doc comments
- Include examples in doc comments where helpful
- Keep documentation concise and accurate

Example:
```rust
/// Generate content from a text prompt.
///
/// # Arguments
///
/// * `prompt` - The text prompt to send to the model
///
/// # Example
///
/// ```rust,no_run
/// # use gemini_rs::{Client, Model};
/// # async fn example() -> Result<(), gemini_rs::Error> {
/// let client = Client::new("API_KEY");
/// let model = client.model(Model::Gemini25Flash);
/// let response = model.generate_content("Hello").await?;
/// # Ok(())
/// # }
/// ```
pub async fn generate_content(&self, prompt: impl Into<String>) -> Result<GenerateContentResponse> {
    // ...
}
```

### Code Conventions

- Use `impl Into<String>` for string parameters
- Return `Result<T, Error>` for fallible operations
- Use the builder pattern for configuration structs
- Prefer `async/await` over manual futures

## Pull Request Process

### Before Submitting

1. **Run all checks:**
   ```bash
   cargo fmt --check
   cargo clippy -- -D warnings
   cargo test --test unit_tests
   cargo doc --no-deps
   ```

2. **Update documentation** if you changed public APIs

3. **Add tests** for new functionality

4. **Update CHANGELOG.md** if applicable

### PR Guidelines

1. **Clear description** - Explain what and why
2. **Small, focused changes** - One feature/fix per PR
3. **Tests included** - Unit tests at minimum
4. **Documentation** - Update docs for API changes
5. **Clean commit history** - Squash if needed

### PR Template

```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Documentation update
- [ ] Refactoring

## Testing
- [ ] Unit tests pass
- [ ] Integration tests pass (if applicable)
- [ ] New tests added

## Checklist
- [ ] Code formatted (`cargo fmt`)
- [ ] No clippy warnings (`cargo clippy`)
- [ ] Documentation updated
- [ ] CHANGELOG updated (if applicable)
```

## Issue Guidelines

### Bug Reports

Include:
- **Description** - What went wrong?
- **Steps to reproduce** - How can we see the bug?
- **Expected behavior** - What should happen?
- **Actual behavior** - What actually happens?
- **Environment** - OS, Rust version, crate version
- **Error messages** - Full error output

### Feature Requests

Include:
- **Description** - What feature do you want?
- **Use case** - Why do you need it?
- **Proposed API** - How should it look?
- **Alternatives** - Other approaches considered?

## Development Tips

### Adding a New Model

1. Add variant to `Model` enum in `src/models.rs`
2. Add `as_str()` mapping
3. Add `From<&str>` mapping
4. Add unit test
5. Update documentation

### Adding New Functionality

1. Consider backward compatibility
2. Use feature flags for optional features
3. Add both unit and integration tests
4. Document the new API

### Debugging

Enable request/response logging:
```rust
// Add to your test
std::env::set_var("RUST_LOG", "reqwest=debug");
env_logger::init();
```

## Questions?

- Open an issue for bugs or features
- Start a discussion for questions
- Check existing issues before creating new ones

Thank you for contributing! 🦀
