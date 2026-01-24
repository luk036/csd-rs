# AGENTS.md

This file provides guidance for agentic coding agents working in the csd-rs repository.

## Project Overview

**csd-rs** is a Rust library for Canonical Signed Digit (CSD) conversion. CSD is a signed-digit representation where each digit can be -1, 0, or +1, with no two consecutive non-zero digits. This is commonly used in digital signal processing and hardware design.

### Key Features
- Convert decimal numbers to CSD representation
- Convert CSD strings back to decimal
- Support for limited non-zero digits (approximation)
- Support for i32, i64, and i128 integer types
- Optional multiplier module for Verilog code generation
- Optional longest common substring with repeated elements

## Build Commands

### Basic Commands
```bash
# Build the project
cargo build

# Build with optimizations
cargo build --release

# Run tests
cargo test

# Run tests with all features
cargo test --all-features

# Run a single test
cargo test test_name

# Run tests for specific module
cargo test csd::tests

# Run benchmarks
cargo bench

# Check formatting
cargo fmt --all --check

# Format code
cargo fmt --all

# Run clippy lints
cargo clippy --all-targets --all-features --workspace -- -D warnings

# Generate documentation
cargo doc --no-deps --document-private-items --all-features --workspace --examples

# Run the CLI binary
cargo run --bin csd-rs -- [args]

# Install the binary
cargo install --path .
```

### Coverage
```bash
# Generate coverage report (requires cargo-llvm-cov)
cargo llvm-cov --all-features --workspace --lcov --output-path lcov.info
```

### Security Audit
```bash
# Run security audit
cargo audit
```

## Code Style Guidelines

### Formatting
- Use `rustfmt` for all code formatting
- Standard Rust formatting rules apply
- No custom rustfmt.toml configuration - use defaults

### Linting
- Clippy is enforced with `-D warnings` (treat warnings as errors)
- Key clippy rules to follow:
  - No `unwrap()` on potentially fallible operations in production code
  - Use proper error handling with `Result` types
  - Avoid `panic!()` in library code (except in test functions)
  - Prefer `const fn` where possible for pure functions
  - Use `#[must_use]` for functions that return important values

### Naming Conventions
- Functions: `snake_case` (e.g., `to_csd`, `to_decimal_i`)
- Types: `PascalCase` (e.g., `CsdError`, `CsdResult`)
- Constants: `SCREAMING_SNAKE_CASE`
- Variables: `snake_case`
- Modules: `snake_case`

### Documentation
- Use `///` for public API documentation
- Include `# Examples` sections with runnable code examples
- Use `# Panics` sections for functions that may panic
- Use `# Errors` sections for functions returning `Result`
- Include mathematical algorithms and references where applicable

### Error Handling
- Use custom `CsdError` enum for library-specific errors
- Return `CsdResult<T>` (alias for `Result<T, CsdError>`) for fallible operations
- Provide both panicking and non-panicking versions of functions where appropriate
- Example: `to_decimal()` (panics on invalid input) vs `to_decimal_result()` (returns Result)

### Performance Guidelines
- Use `Vec<u8>` for string building when performance matters
- Pre-allocate capacity for known-size collections
- Use bit manipulation for power-of-two calculations
- Prefer `const fn` for compile-time evaluation where possible

### Testing
- Unit tests go in `#[cfg(test)]` modules within each file
- Integration tests go in `tests/` directory (none currently)
- Use `quickcheck` for property-based testing where appropriate
- Include edge case testing (zero, negative numbers, very large/small numbers)
- Test round-trip conversions (decimal → CSD → decimal)

### Module Structure
```
src/
├── lib.rs          # Library root, public exports
├── main.rs         # CLI binary entry point
├── csd.rs          # Core CSD conversion functions
├── csd_multiplier.rs  # Optional multiplier module (feature-gated)
└── lcsre.rs        # Optional longest common substring (feature-gated)
```

### Feature Flags
- `multiplier` (default): CSD multiplier module for Verilog code generation
- `lcsre` (default): Longest common substring with repeated elements
- `docsrs`: For documentation generation on docs.rs

### Dependencies
- Minimal external dependencies
- `svgbobdoc` for documentation diagrams (optional, docs.rs only)
- `quickcheck` and `criterion` for testing and benchmarking (dev-dependencies only)

## CLI Usage

The binary provides these commands:
```bash
csd-rs to_csd <value> [places]      # Convert decimal to CSD
csd-rs to_csdnnz <value> [nnz]      # Convert with limited non-zeros
csd-rs to_decimal <csd_string>      # Convert CSD to decimal
```

## Development Workflow

1. Always run `cargo test` before committing
2. Run `cargo clippy` and fix all warnings
3. Check formatting with `cargo fmt --all --check`
4. For performance changes, run `cargo bench`
5. Update documentation for public API changes

## Common Patterns

### Function Design
- Provide both simple and advanced versions (e.g., `to_csd` vs `to_csdnnz`)
- Use type suffixes for different integer sizes (`_i32`, `_i64`, `_i128`)
- Include `#[must_use]` for functions whose return value should be used

### String Handling
- Use `Vec<u8>` for building strings efficiently, then convert to `String`
- Use `String::from_utf8().unwrap()` only when UTF-8 validity is guaranteed

### Mathematical Operations
- Use bit manipulation for power-of-two calculations
- Handle floating-point precision carefully in conversions
- Provide both exact and approximate conversion methods

## Testing Single Functions

To test a specific function:
```bash
# Test a specific function
cargo test test_to_csd

# Test all functions in a module
cargo test csd::tests

# Test with specific features
cargo test --features "multiplier lcsre"

# Test with verbose output
cargo test -- --nocapture test_name
```

## Notes for Agents

- This is a mathematical library - precision matters
- CSD representation has specific constraints (no consecutive non-zeros)
- The library includes both simple and advanced conversion algorithms
- Performance is important for large numbers and batch operations
- Error handling should be user-friendly but mathematically correct
- Documentation includes mathematical algorithms and references