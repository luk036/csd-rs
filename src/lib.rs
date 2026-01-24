pub mod csd;

#[cfg(feature = "multiplier")]
pub mod csd_multiplier;

#[cfg(feature = "lcsre")]
pub mod lcsre;

pub use crate::csd::{
    highest_power_of_two_in, to_csd, to_csd_i, to_csdnnz, to_csdnnz_safe,
    to_csdnnz_i128, to_csdnnz_i64, to_decimal, to_decimal_fractional, to_decimal_fractional_safe,
    to_decimal_i, to_decimal_i128_result, to_decimal_i64_result,
    to_decimal_i_result, to_decimal_integral_safe, to_decimal_result,
    to_decimal_safe, validate_csd_format, CsdBuilder, CsdError, CsdResult, RoundingStrategy,
};

#[cfg(feature = "multiplier")]
pub use crate::csd_multiplier::{CsdMultiplier, CsdMultiplierError};

#[cfg(feature = "lcsre")]
pub use crate::lcsre::longest_repeated_substring;
