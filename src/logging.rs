//! Logging utilities for csd-rs applications.
//!
//! This module provides optional logging capabilities that are only available
//! when the `std` feature is enabled. This maintains `#![no_std]` compatibility
//! while allowing applications to use logging for debugging and diagnostics.
//!
//! # Usage
//!
//! ```rust,ignore
//! use csd::logging::init_logger;
//!
//! init_logger();
//! log::info!("Application started");
//!
//! // Use csd functions
//! let csd = csd::to_csd(28.5, 2);
//! log::debug!("CSD representation: {}", csd);
//! ```
//!
//! Set the `RUST_LOG` environment variable to control logging levels:
//!
//! ```bash
//! RUST_LOG=debug cargo run --features std
//! RUST_LOG=warn cargo run --features std
//! ```

use log::LevelFilter;

/// Initialize the logger with the default filter.
///
/// Reads the log level from the `RUST_LOG` environment variable.
/// If not set, defaults to `info` level.
///
/// # Panics
///
/// Panics if the logger has already been initialized.
#[cfg(feature = "std")]
pub fn init_logger() {
    env_logger::Builder::from_default_env()
        .filter_level(LevelFilter::Info)
        .init();
}

/// Initialize the logger with a custom filter string.
///
/// The filter string follows the `env_logger` format:
/// - `error`, `warn`, `info`, `debug`, `trace` for specific levels
/// - `crate_name=level` for module-specific filtering
///
/// # Panics
///
/// Panics if the logger has already been initialized.
///
/// # Example
///
/// ```rust,ignore
/// use csd::logging::init_logger_with_filter;
///
/// init_logger_with_filter("debug");
/// log::debug!("Debug message"); // This will be logged
/// ```
#[cfg(feature = "std")]
pub fn init_logger_with_filter(filter: &str) {
    env_logger::Builder::from_default_env()
        .filter_level(filter.parse().unwrap_or(LevelFilter::Info))
        .init();
}

/// Try to initialize the logger (non-panicking version).
///
/// Returns `Ok(())` if initialization succeeded, or `Err` if the logger
/// has already been initialized.
///
/// # Example
///
/// ```rust,ignore
/// use csd::logging::try_init_logger;
///
/// if try_init_logger().is_ok() {
///     log::info!("Logger initialized");
/// }
/// ```
#[cfg(feature = "std")]
pub fn try_init_logger() -> Result<(), log::SetLoggerError> {
    env_logger::Builder::from_default_env()
        .filter_level(LevelFilter::Info)
        .try_init()
}

/// Try to initialize the logger with a custom filter (non-panicking version).
///
/// Returns `Ok(())` if initialization succeeded, or `Err` if the logger
/// has already been initialized.
///
/// # Example
///
/// ```rust,ignore
/// use csd::logging::try_init_logger_with_filter;
///
/// if try_init_logger_with_filter("csd=debug").is_ok() {
///     log::debug!("Debug messages enabled");
/// }
/// ```
#[cfg(feature = "std")]
pub fn try_init_logger_with_filter(filter: &str) -> Result<(), log::SetLoggerError> {
    env_logger::Builder::from_default_env()
        .filter_level(filter.parse().unwrap_or(LevelFilter::Info))
        .try_init()
}

/// Check if the logger has been initialized.
///
/// Returns `true` if logging is active, `false` otherwise.
///
/// # Example
///
/// ```rust,ignore
/// use csd::logging::{try_init_logger, is_logger_initialized};
///
/// try_init_logger();
/// if is_logger_initialized() {
///     log::info!("Logging is active");
/// }
/// ```
#[cfg(feature = "std")]
pub fn is_logger_initialized() -> bool {
    log::max_level() != LevelFilter::Off
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_init_logger() {
        // Try to initialize - may fail if already initialized in test runner
        let _ = try_init_logger();
    }

    #[test]
    fn test_try_init_logger_with_filter() {
        // Try to initialize with a custom filter
        let _ = try_init_logger_with_filter("info");
    }

    #[test]
    fn test_is_logger_initialized() {
        // Just verify the function can be called
        let _ = is_logger_initialized();
    }
}
