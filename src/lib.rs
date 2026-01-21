pub mod csd;

#[cfg(feature = "multiplier")]
pub mod csd_multiplier;

#[cfg(feature = "lcsre")]
pub mod lcsre;

pub use crate::csd::{
    highest_power_of_two_in, to_csd, to_csd_i, to_csd_i128, to_csd_i64, to_csdnnz, to_csdnnz_i,
    to_csdnnz_i128, to_csdnnz_i64, to_decimal, to_decimal_fractional, to_decimal_i,
    to_decimal_i128, to_decimal_i128_result, to_decimal_i64, to_decimal_i64_result,
    to_decimal_i_result, to_decimal_integral, to_decimal_result, CsdError, CsdResult,
};

#[cfg(feature = "multiplier")]
pub use crate::csd_multiplier::{CsdMultiplier, CsdMultiplierError};

#[cfg(feature = "lcsre")]
pub use crate::lcsre::longest_repeated_substring;
