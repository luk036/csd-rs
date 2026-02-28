//! Canonical Signed Digit (CSD) conversion library.
//!
//! This library provides functions to convert between decimal numbers and
//! Canonical Signed Digit (CSD) representation.
//!
//! # Features
//!
//! - `multiplier` (default): CSD multiplier module for Verilog code generation
//! - `lcsre` (default): Longest common substring with repeated elements
//! - `std` (optional): Logging support via env_logger
//!
//! # Quick Start
//!
//! ```rust
//! use csd::{to_csd, to_decimal};
//!
//! // Convert decimal to CSD
//! let csd = to_csd(28.5, 2);
//! assert_eq!(csd, "+00-00.+0");
//!
//! // Convert CSD back to decimal
//! let value = to_decimal("+00-00.+0");
//! assert!((value - 28.5).abs() < 1e-10);
//! ```

pub mod csd;

#[cfg(feature = "multiplier")]
pub mod csd_multiplier;

#[cfg(feature = "lcsre")]
pub mod lcsre;

pub use crate::csd::{
    highest_power_of_two_in, to_csd, to_csd_i, to_csdnnz, to_csdnnz_i128, to_csdnnz_i64,
    to_csdnnz_safe, to_decimal, to_decimal_fractional, to_decimal_fractional_safe, to_decimal_i,
    to_decimal_i128_result, to_decimal_i64_result, to_decimal_i_result, to_decimal_integral_safe,
    to_decimal_result, to_decimal_safe, validate_csd_format, CsdBuilder, CsdError, CsdResult,
    RoundingStrategy,
};

#[cfg(feature = "multiplier")]
pub use crate::csd_multiplier::{CsdMultiplier, CsdMultiplierError};

#[cfg(feature = "lcsre")]
pub use crate::lcsre::longest_repeated_substring;

#[cfg(feature = "std")]
pub mod logging;
