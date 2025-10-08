/// Find the highest power of two less than or equal to a given number
///
/// The `highest_power_of_two_in` function calculates the highest power of two that is less than or
/// equal to a given number. This is done through a bit manipulation technique that fills all bits
/// below the most significant bit (MSB) with 1s, then shifts and XORs to isolate just the MSB.
///
/// Reference:
///
/// * <https://thecodingbot.com/find-the-greatest-power-of-2-less-than-or-equal-to-a-given-number/>
///
/// Arguments:
///
/// * `x`: The parameter `x` is an unsigned 32-bit integer. It represents the number for which we want
///        to find the highest power of two that is less than or equal to it.
///
/// Returns:
///
/// The function `highest_power_of_two_in` returns the highest power of two that is less than or equal
/// to the given number.
///
/// # Examples
///
/// ```
/// use csd::csd::highest_power_of_two_in;
///
/// assert_eq!(highest_power_of_two_in(14), 8);
/// assert_eq!(highest_power_of_two_in(8), 8);
/// assert_eq!(highest_power_of_two_in(1), 1);
/// assert_eq!(highest_power_of_two_in(0), 0);
/// assert_eq!(highest_power_of_two_in(3), 2);
/// assert_eq!(highest_power_of_two_in(2), 2);
/// ```
#[must_use]
#[inline]
pub const fn highest_power_of_two_in(mut x: u32) -> u32 {
    // Propagate the highest set bit to all lower bits
    x |= x >> 1;
    x |= x >> 2;
    x |= x >> 4;
    x |= x >> 8;
    x |= x >> 16;
    // Isolate the highest bit by XORing with the value shifted right by 1
    x ^ (x >> 1)
}

/// Convert to CSD (Canonical Signed Digit) String representation
///
/// The `to_csd` function converts a given number to its Canonical Signed Digit (CSD) representation
/// with a specified number of decimal places. CSD is a number system where each digit can be -1, 0, or +1
/// (represented by '-', '0', '+'), and no two adjacent digits are non-zero.
///
/// - Original author: Harnesser
/// - <https://sourceforge.net/projects/pycsd/>
/// - License: GPL2
///
/// Arguments:
///
/// * `decimal_value`: The `decimal_value` parameter is a double precision floating-point number that represents the value
///         to be converted to CSD (Canonical Signed Digit) representation.
/// * `places`: The `places` parameter represents the number of decimal places to include in the CSD
///         (Canonical Signed Digit) representation of the given `decimal_value`.
///
/// Returns:
///
/// The function `to_csd` returns a string representation of the given `decimal_value` in Canonical Signed Digit
/// (CSD) format.
///
/// # Examples
///
/// ```
/// use csd::csd::to_csd;
///
/// assert_eq!(to_csd(28.5, 2), "+00-00.+0".to_string());
/// assert_eq!(to_csd(-0.5, 2), "0.-0".to_string());
/// assert_eq!(to_csd(0.0, 2), "0.00".to_string());
/// assert_eq!(to_csd(0.0, 0), "0.".to_string());
/// ```
/// # Panics
///
/// Panics if the resulting CSD string is not valid UTF-8.
#[must_use]
pub fn to_csd(decimal_value: f64, places: i32) -> String {
    if decimal_value == 0.0 {
        let mut csd = "0.".to_string();
        for _ in 0..places {
            csd.push('0');
        }
        return csd;
    }
    let absnum = decimal_value.abs();
    // Handle numbers less than 1.0 specially
    let (mut rem, mut csd) = if absnum < 1.0 {
        (0, vec![b'0'])
    } else {
        // Calculate the highest power of two needed
        #[allow(clippy::cast_possible_truncation)]
        let rem = (absnum * 1.5).log2().ceil() as i32;
        #[allow(clippy::cast_sign_loss)]
        (
            rem,
            Vec::with_capacity((rem.abs() + places.abs() + 2) as usize),
        ) // +2 for '.' and potential sign
    };
    let mut p2n = 2.0_f64.powi(rem);
    let mut decimal_value = decimal_value;
    // Closure to handle both integer and fractional parts
    let mut loop_fn = |value: i32, csd: &mut Vec<u8>| {
        while rem > value {
            rem -= 1;
            p2n /= 2.0;
            let det = 1.5 * decimal_value;
            if det > p2n {
                csd.push(b'+');
                decimal_value -= p2n;
            } else if det < -p2n {
                csd.push(b'-');
                decimal_value += p2n;
            } else {
                csd.push(b'0');
            }
        }
    };
    // Process integer part
    loop_fn(0, &mut csd);
    csd.push(b'.');
    // Process fractional part
    loop_fn(-places, &mut csd);

    String::from_utf8(csd).unwrap()
}

/// Convert to CSD (Canonical Signed Digit) String representation
///
/// The `to_csd_i` function converts an integer into a Canonical Signed Digit (CSD) representation.
/// This version works with integers only and produces a CSD string without a decimal point.
///
/// Arguments:
///
/// * `decimal_value`: The `decimal_value` parameter is an integer that represents the number for which we want to generate
///         the CSD (Canonical Signed Digit) representation.
///
/// Returns:
///
/// The function `to_csd_i` returns a string representation of the given integer in Canonical Signed
/// Digit (CSD) format.
///
/// # Examples
///
/// ```
/// use csd::csd::to_csd_i;
///
/// assert_eq!(to_csd_i(28), "+00-00".to_string());
/// assert_eq!(to_csd_i(-0), "0".to_string());
/// assert_eq!(to_csd_i(0), "0".to_string());
/// ```
/// # Panics
///
/// Panics if the resulting CSD string is not valid UTF-8.
#[allow(dead_code)]
#[must_use]
pub fn to_csd_i(decimal_value: i32) -> String {
    if decimal_value == 0 {
        return "0".to_string();
    }

    // Calculate the highest power of two needed
            #[allow(clippy::cast_sign_loss)]
            let temp = (decimal_value.abs() * 3 / 2) as u32;
            #[allow(clippy::cast_possible_wrap)]
            let mut p2n = highest_power_of_two_in(temp) as i32 * 2;    let mut csd = Vec::with_capacity(32); // Max 32 chars for i32
    let mut decimal_value = decimal_value;

    while p2n > 1 {
        let p2n_half = p2n >> 1;
        let det = 3 * decimal_value;
        if det > p2n {
            csd.push(b'+');
            decimal_value -= p2n_half;
        } else if det < -p2n {
            csd.push(b'-');
            decimal_value += p2n_half;
        } else {
            csd.push(b'0');
        }
        p2n = p2n_half;
    }

    String::from_utf8(csd).unwrap()
}

/// Convert the CSD (Canonical Signed Digit) to a decimal
///
/// The `to_decimal_i` function converts a CSD (Canonical Signed Digit) string to a decimal integer.
/// This function processes the CSD string character by character, building up the decimal value
/// through bit shifting and addition/subtraction operations.
///
/// Arguments:
///
/// * `csd`: The `csd` parameter is a slice of characters representing a CSD (Canonical Signed Digit)
///          string.
///
/// Returns:
///
/// The function `to_decimal_i` returns an `i32` value, which is the decimal representation of the input
/// CSD (Canonical Signed Digit) string.
///
/// # Panics
///
/// Panics if unexpected character is encountered
///
/// # Examples
///
/// ```
/// use csd::csd::to_decimal_i;
///
/// assert_eq!(to_decimal_i("+00-00"), 28);
/// assert_eq!(to_decimal_i("0"), 0);
/// ```
#[allow(dead_code)]
#[must_use]
pub fn to_decimal_i(csd: &str) -> i32 {
    csd.chars().fold(0, |acc, digit| match digit {
        '0' => acc << 1,
        '+' => (acc << 1) + 1,
        '-' => (acc << 1) - 1,
        _ => panic!("Work with 0, +, and - only"),
    })
}

/// Helper function to convert the integral part of a CSD string to decimal
///
/// This function processes the integral part (before the decimal point) of a CSD string,
/// returning both the converted value and the position of the decimal point if found.
/// # Panics
///
/// Panics if an unexpected character is encountered.
#[must_use]
pub fn to_decimal_integral(csd: &str) -> (i32, usize) {
    let mut decimal_value: i32 = 0;

    for (pos, digit) in csd.chars().enumerate() {
        match digit {
            '0' => decimal_value <<= 1,
            '+' => decimal_value = (decimal_value << 1) + 1,
            '-' => decimal_value = (decimal_value << 1) - 1,
            '.' => {
                return (decimal_value, pos + 1);
            }
            _ => panic!("Work with 0, +, -, and . only"),
        }
    }

    (decimal_value, 0)
}

/// Helper function to convert the fractional part of a CSD string to decimal
///
/// This function processes the fractional part (after the decimal point) of a CSD string,
/// building up the decimal value by progressively halving the scale factor for each digit.
/// # Panics
///
/// Panics if an unexpected character is encountered.
#[must_use]
pub fn to_decimal_fractional(csd: &str) -> f64 {
    let mut decimal_value = 0.0;
    let mut scale = 0.5; // Start with 2^-1

    for digit in csd.chars() {
        match digit {
            '0' => {} // No change to value
            '+' => decimal_value += scale,
            '-' => decimal_value -= scale,
            _ => panic!("Fractional part works with 0, +, and - only"),
        }
        scale /= 2.0; // Move to next fractional bit
    }

    decimal_value
}

/// Convert the CSD (Canonical Signed Digit) to a decimal
///
/// The `to_decimal` function converts a CSD (Canonical Signed Digit) string to a decimal number.
/// This function handles both integral and fractional parts of the CSD representation.
///
/// Arguments:
///
/// * `csd`: The `csd` parameter is a string representing a Canonical Signed Digit (CSD) number.
///
/// Returns:
///
/// The function `to_decimal` returns a decimal number (f64) that is converted from the input CSD
/// (Canonical Signed Digit) string.
///
/// # Panics
///
/// Panics if unexpected character is encountered
///
/// # Examples
///
/// ```
/// use csd::csd::to_decimal;
///
/// assert_eq!(to_decimal("+00-00.+"), 28.5);
/// assert_eq!(to_decimal("0.-"), -0.5);
/// assert_eq!(to_decimal("0"), 0.0);
/// assert_eq!(to_decimal("0.0"), 0.0);
/// assert_eq!(to_decimal("0.+"), 0.5);
/// assert_eq!(to_decimal("0.-"), -0.5);
/// assert_eq!(to_decimal("0.++"), 0.75);
/// assert_eq!(to_decimal("0.-+"), -0.25);
/// ```
#[must_use]
pub fn to_decimal(csd: &str) -> f64 {
    // First convert the integral part
    let (integral, loc) = to_decimal_integral(csd);

    if loc == 0 {
        return f64::from(integral);
    }

    // Then convert the fractional part if present
    let fractional = to_decimal_fractional(&csd[loc..]);
    f64::from(integral) + fractional
}

/// Convert to CSD representation approximately with fixed number of non-zero
///
/// The `to_csdnnz` function converts a given number into a CSD (Canonic Signed Digit) representation
/// approximately with a specified number of non-zero digits. This version limits the number of
/// non-zero digits in the output representation.
///
/// Arguments:
///
/// * `decimal_value`: The `decimal_value` parameter is a double precision floating-point number that represents the input
///          value for conversion to CSD (Canonic Signed Digit) fixed-point representation.
/// * `nnz`: The parameter `nnz` stands for "number of non-zero bits". It represents the maximum number
///          of non-zero bits allowed in the output CSD (Canonical Signed Digit) representation of the given
///          `decimal_value`.
///
/// Returns:
///
/// The function `to_csdnnz` returns a string representation of the given `decimal_value` in Canonical Signed
/// Digit (CSD) format.
///
/// # Examples
///
/// ```
/// use csd::csd::to_csdnnz;
///
/// let s1 = to_csdnnz(28.5, 4);
/// let s2 = to_csdnnz(-0.5, 4);
///
/// assert_eq!(to_csdnnz(28.5, 4), "+00-00.+".to_string());
/// assert_eq!(to_csdnnz(-0.5, 4), "0.-".to_string());
/// assert_eq!(to_csdnnz(0.0, 4), "0".to_string());
/// assert_eq!(to_csdnnz(0.0, 0), "0".to_string());
/// assert_eq!(to_csdnnz(0.5, 4), "0.+".to_string());
/// assert_eq!(to_csdnnz(-0.5, 4), "0.-".to_string());
/// assert_eq!(to_csdnnz(28.5, 2), "+00-00".to_string());
/// assert_eq!(to_csdnnz(28.5, 1), "+00000".to_string());
/// ```
#[allow(dead_code)]
#[must_use]
pub fn to_csdnnz(decimal_value: f64, nnz: u32) -> String {
    let absnum = decimal_value.abs();
    let (mut rem, mut csd) = if absnum < 1.0 {
        (0, "0".to_string())
    } else {
        #[allow(clippy::cast_possible_truncation)]
        let rem = (absnum * 1.5).log2().ceil() as i32;
        (rem, String::new())
    };
    let mut p2n = 2.0_f64.powi(rem);
    let mut decimal_value = decimal_value;
    let mut nnz = nnz;

    // Process both integer and fractional parts while respecting the nnz limit
    while rem > 0 || (nnz > 0 && decimal_value.abs() > 1e-100) {
        if rem == 0 {
            csd.push('.');
        }
        p2n /= 2.0;
        rem -= 1;
        let det = 1.5 * decimal_value;
        if det > p2n {
            csd.push('+');
            decimal_value -= p2n;
            nnz -= 1;
        } else if det < -p2n {
            csd += "-";
            decimal_value += p2n;
            nnz -= 1;
        } else {
            csd.push('0');
        }
        // Stop processing if we've used all non-zero digits
        if nnz == 0 {
            decimal_value = 0.0;
        }
    }

    csd
}

/// Convert to CSD (Canonical Signed Digit) String representation
///
/// The `to_csdnnz_i` function converts an integer into a Canonical Signed Digit (CSD) representation
/// approximately with a specified number of non-zero digits. This is the integer version of `to_csdnnz`.
///
/// Arguments:
///
/// * `decimal_value`: The `decimal_value` parameter is an integer that represents the number for which we want to generate
///         the CSD (Canonical Signed Digit) representation.
/// * `nnz`: The parameter `nnz` stands for "number of non-zero bits". It represents the maximum number
///          of non-zero bits allowed in the output CSD (Canonical Signed Digit) representation of the given
///          `decimal_value`.
///
/// Returns:
///
/// The function `to_csdnnz_i` returns a string representation of the given integer in Canonical Signed
/// Digit (CSD) format.
///
/// # Examples
///
/// ```
/// use csd::csd::to_csdnnz_i;
///
/// assert_eq!(to_csdnnz_i(28, 4), "+00-00".to_string());
/// assert_eq!(to_csdnnz_i(-0, 4), "0".to_string());
/// assert_eq!(to_csdnnz_i(0, 4), "0".to_string());
/// assert_eq!(to_csdnnz_i(158, 2), "+0+00000".to_string());
/// ```
#[allow(dead_code)]
pub fn to_csdnnz_i(decimal_value: i32, nnz: u32) -> String {
    if decimal_value == 0 {
        return "0".to_string();
    }

    // Calculate the highest power of two needed
    let temp = (decimal_value.abs() * 3 / 2) as u32;
    let mut p2n = highest_power_of_two_in(temp) as i32 * 2;
    let mut csd = String::with_capacity(32); // Max 32 chars for i32
    let mut decimal_value = decimal_value;
    let mut nnz = nnz;

    while p2n > 1 {
        let p2n_half = p2n >> 1;
        let det = 3 * decimal_value;
        if det > p2n {
            csd += "+";
            decimal_value -= p2n_half;
            nnz -= 1;
        } else if det < -p2n {
            csd += "-";
            decimal_value += p2n_half;
            nnz -= 1;
        } else {
            csd += "0";
        }
        p2n = p2n_half;
        // Stop processing if we've used all non-zero digits
        if nnz == 0 {
            decimal_value = 0;
        }
    }

    csd
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck_macros::quickcheck;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_to_csd() {
        assert_eq!(to_csd(28.5, 2), "+00-00.+0".to_string());
        assert_eq!(to_csd(-0.5, 2), "0.-0".to_string());
        assert_eq!(to_csd(0.0, 2), "0.00".to_string());
        assert_eq!(to_csd(0.0, 0), "0.".to_string());
        assert_eq!(to_csd(2.5, 4), "+0.+000".to_string());
    }

    #[test]
    #[should_panic]
    fn test_to_decimal_invalid1() {
        let _res = to_decimal("+00XXX-00.00+");
    }

    #[test]
    #[should_panic]
    fn test_to_decimal_invalid2() {
        let _res = to_decimal("+00-00.0XXX0+");
    }

    #[test]
    fn test_to_decimal_i() {
        assert_eq!(to_decimal_i("+00-00"), 28);
        assert_eq!(to_decimal_i("0"), 0);
    }

    #[test]
    #[should_panic]
    fn test_to_decimal_i_invalid() {
        let _res = to_decimal_i("+00-00.00+");
    }

    #[test]
    fn test_to_csdnnz() {
        assert_eq!(to_csdnnz(28.5, 4), "+00-00.+".to_string());
        assert_eq!(to_csdnnz(-0.5, 4), "0.-".to_string());
        assert_eq!(to_csdnnz(0.0, 4), "0".to_string());
        assert_eq!(to_csdnnz(0.0, 0), "0".to_string());
        assert_eq!(to_csdnnz(0.5, 4), "0.+".to_string());
        assert_eq!(to_csdnnz(-0.5, 4), "0.-".to_string());
        assert_eq!(to_csdnnz(28.5, 2), "+00-00".to_string());
        assert_eq!(to_csdnnz(28.5, 1), "+00000".to_string());
    }

    #[test]
    fn test_to_csdnnz_i() {
        assert_eq!(to_csdnnz_i(28, 4), "+00-00".to_string());
        assert_eq!(to_csdnnz_i(-0, 4), "0".to_string());
        assert_eq!(to_csdnnz_i(0, 4), "0".to_string());
        assert_eq!(to_csdnnz_i(0, 0), "0".to_string());
        assert_eq!(to_csdnnz_i(158, 2), "+0+00000".to_string());
    }

    #[quickcheck]
    fn test_csd(d: i32) -> bool {
        let f = d as f64 / 8.0;
        f == to_decimal(&to_csd(f, 4))
    }

    #[quickcheck]
    fn test_csd_i(d: i32) -> bool {
        let d = d / 3; // prevent overflow
        let csd = to_csd_i(d);
        d == to_decimal_i(&csd)
    }

    // #[quickcheck]
    // fn test_csdnnz(d: i32) -> bool {
    //     let f = d as f64 / 8.0;
    //     let csd = to_csdnnz(f, 4);
    //     let f_hat = to_decimal(&csd);
    //     (f - f_hat).abs() <= 1.5
    // }

    // #[quickcheck]
    // fn test_csdnnz_i(d: i32) -> bool {
    //     let d = d / 3; // prevent overflow
    //     let csd = to_csdnnz_i(d, 4);
    //     let d_hat = to_decimal(&csd);
    //     (d as f64 - d_hat).abs() <= 1.5
    // }

    #[test]
    fn test_highest_power_of_two_in() {
        assert_eq!(highest_power_of_two_in(14), 8);
        assert_eq!(highest_power_of_two_in(8), 8);
        assert_eq!(highest_power_of_two_in(1), 1);
        assert_eq!(highest_power_of_two_in(0), 0);
        assert_eq!(highest_power_of_two_in(3), 2);
        assert_eq!(highest_power_of_two_in(2), 2);
        assert_eq!(highest_power_of_two_in(u32::MAX), 2147483648);
    }
}
