# AGENTS.md

This file provides guidance for agentic coding agents working in the csd-rs repository.

## Project Overview

**csd-rs** is a Rust library for Canonical Signed Digit (CSD) conversion. CSD is a signed-digit representation where each digit can be -1, 0, or +1, with no two consecutive non-zero digits. Used in digital signal processing and hardware design.

### Key Features
- Convert decimal numbers to CSD representation and back
- Support for limited non-zero digits (approximation)
- Support for i32, i64, and i128 integer types
- Optional multiplier module for Verilog code generation
- Optional longest common substring with repeated elements

### Module Structure
```
src/
├── lib.rs              # Library root, public exports
├── main.rs             # CLI binary entry point
├── csd.rs              # Core CSD conversion functions
├── csd_multiplier.rs   # Optional multiplier (feature-gated)
├── lcsre.rs            # Optional LCSRE (feature-gated)
└── logging.rs          # Optional logging (feature-gated)
```

---

## Build Commands

```bash
# Build
cargo build
cargo build --release

# Test
cargo test                          # Run all tests
cargo test --all-features           # Run with all features
cargo test test_name                # Run single test
cargo test csd::tests               # Run specific module
cargo test -- --nocapture           # Verbose output

# Lint & Format
cargo fmt --all --check              # Check formatting
cargo fmt --all                     # Format code
cargo clippy --all-targets --all-features --workspace -- -D warnings

# Benchmarks
cargo bench

# Documentation
cargo doc --no-deps --document-private-items --all-features --workspace --examples

# CLI
cargo run --bin csd-rs -- [args]
```

### Feature Flags
- `multiplier` (default): CSD multiplier for Verilog code generation
- `lcsre` (default): Longest common substring with repeated elements
- `std` (optional): Logging via env_logger
- `docsrs` (optional): Documentation generation

---

## Code Style Guidelines

### Formatting & Imports
- Use `rustfmt` - no custom config, use defaults
- Import from crate root: `use csd::{to_csd, to_decimal, CsdError};`
- Prefer absolute paths for internal modules: `use crate::csd::CsdBuilder;`

### Naming Conventions
| Element | Convention | Example |
|---------|------------|---------|
| Functions | snake_case | `to_csd`, `to_decimal_i` |
| Types | PascalCase | `CsdError`, `CsdResult` |
| Constants | SCREAMING_SNAKE_CASE | `MAX_DIGITS` |
| Modules | snake_case | `csd`, `lcsre` |

### Linting Rules (enforced with `-D warnings`)
- No `unwrap()` on potentially fallible operations in production code
- Use proper `Result` error handling - never suppress errors
- Avoid `panic!()` in library code (except in tests)
- Prefer `const fn` where possible for pure functions
- Use `#[must_use]` for functions returning important values
- **No `unsafe`** allowed - this library uses only safe Rust

### Documentation
- Use `///` for public API docs
- Include `# Examples` sections with runnable code
- Use `# Panics` for functions that may panic
- Use `# Errors` for functions returning `Result`
- Include mathematical algorithms and references where applicable

---

## Error Handling

```rust
// Custom error enum
pub enum CsdError {
    InvalidFormat(String),
    Overflow,
    // ...
}

// Result alias
pub type CsdResult<T> = Result<T, CsdError>;

// Provide both panicking and non-panicking versions
pub fn to_decimal(csd: &str) -> f64 { ... }        // Panics on invalid
pub fn to_decimal_result(csd: &str) -> CsdResult<f64> { ... }
```

---

## Performance Guidelines

- Use `Vec<u8>` for string building when performance matters
- Pre-allocate capacity for known-size collections
- Use bit manipulation for power-of-two calculations
- Prefer `const fn` for compile-time evaluation

---

## Testing

- Unit tests: `#[cfg(test)]` modules within each source file
- Integration tests: `tests/` directory (e.g., `tests/cli_tests.rs`)
- Use `quickcheck` for property-based testing
- Include edge cases: zero, negative, very large/small numbers
- Test round-trip conversions: decimal → CSD → decimal

### Fuzzing

For additional edge case discovery, use cargo-fuzz:

```bash
cargo install cargo-fuzz
cargo +nightly fuzz run to_decimal  # Fuzz the to_decimal target
```

Run fuzzing targets defined in `fuzz/` directory.

---

## CLI Usage

```bash
csd-rs to_csd <value> [places]      # Convert decimal to CSD
csd-rs to_csdnnz <value> [nnz]      # Convert with limited non-zeros
csd-rs to_decimal <csd_string>      # Convert CSD to decimal
```

---

## Development Workflow

1. Run `cargo test` before committing
2. Run `cargo clippy` - fix all warnings
3. Check formatting: `cargo fmt --all --check`
4. Run `cargo bench` for performance changes
5. Update docs for public API changes

---

## Common Patterns

### Function Design
- Provide both simple and advanced versions: `to_csd` vs `to_csdnnz`
- Use type suffixes: `_i32`, `_i64`, `_i128`
- Include `#[must_use]` on important return values

### String Handling
- Use `Vec<u8>` for building strings efficiently
- Use `String::from_utf8().unwrap()` only when UTF-8 is guaranteed

---

## Notes for Agents

- This is a mathematical library - precision matters
- CSD has specific constraints (no consecutive non-zeros)
- Performance is critical for large numbers and batch operations
- Error handling must be user-friendly but mathematically correct
- Documentation should include mathematical algorithms and references
- `csd.rs` is ~2100 lines - consider splitting if it grows significantly
