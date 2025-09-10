# Project Overview

This is a Rust project focused on Canonical Signed Digit (CSD) representation and multiplication. It provides functionalities to convert numbers to and from CSD format, and includes a CSD multiplier.

# Building and Running

This project uses `cargo`, the Rust package manager and build system.

*   **Build the project:**
    ```bash
    cargo build
    ```
    For an optimized release build:
    ```bash
    cargo build --release
    ```

*   **Run the main executable (if applicable):**
    ```bash
    cargo run
    ```

*   **Run tests:**
    ```bash
    cargo test
    ```

*   **Run benchmarks:**
    ```bash
    cargo bench
    ```

# Development Conventions

*   **Language:** Rust
*   **Build System:** Cargo
*   **Testing:** Unit tests are located in `src/` and `tests/`.
*   **Benchmarking:** Benchmarks are located in `benches/` and use the `criterion` crate.
*   **Continuous Integration/Continuous Deployment (CI/CD):** Workflow configurations are located in `.github/workflows/`.
