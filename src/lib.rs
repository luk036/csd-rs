/**
 Canonical Signed Digit Functions

 Handles:
  * Decimals
  *
  *

 eg, +00-00+000.0 or 0.+0000-00+
 Where: '+' is +1
        '-' is -1

 Harnesser
 License: GPL2
*/

/**
 * @brief Convert to CSD (Canonical Signed Digit) String representation
 *
 * Original author: Harnesser
 * https://sourceforge.net/projects/pycsd/
 * License: GPL2
 *
 * @param num
 * @param places
 * @return String
 */
pub fn to_csd(mut num: f64, places: i32) -> String {
    if num == 0.0 {
        return String::from("0");
    }
    let absnum = num.abs();
    let nn = (absnum * 1.5).log2().ceil() as i32;
    let mut n = if absnum < 1.0 { 0 } else { nn };
    let str = if absnum < 1.0 { "0" } else { "" };
    let mut csd_str = String::from(str);
    let mut pow2n = (2.0_f64).powi(n - 1);
    while n > -places {
        if n == 0 {
            csd_str.push('.');
        }
        n -= 1;
        let d = 1.5 * num;
        if d > pow2n {
            csd_str.push('+');
            num -= pow2n;
        } else if d < -pow2n {
            csd_str.push('-');
            num += pow2n;
        } else {
            csd_str.push('0');
        }
        pow2n /= 2.0;
    }
    csd_str
}

/**
 * @brief Convert the CSD String to a decimal
 *
 * @param csd_str
 * @return f64
 */
pub fn to_decimal(csd_str: &str) -> f64 {
    let mut num: f64 = 0.0;
    let mut loc: usize = 0;
    for (i, c) in csd_str.chars().enumerate() {
        if c == '0' {
            num *= 2.0;
        } else if c == '+' {
            num = num * 2.0 + 1.0;
        } else if c == '-' {
            num = num * 2.0 - 1.0;
        } else if c == '.' {
            loc = i + 1;
        } // else unknown character
    }
    if loc != 0 {
        num /= (2.0_f64).powi((csd_str.len() - loc) as i32);
    }
    num
}

/**
 * @brief Convert to CSD (Canonical Signed Digit) String representation
 *
 * @param[in] num
 * @param[in] nnz number of non-zero
 * @return String
 */
pub fn to_csdfixed(mut num: f64, mut nnz: u32) -> String {
    if num == 0.0 {
        return String::from("0");
    }
    let absnum = num.abs();
    let nn = (absnum * 1.5).log2().ceil() as i32;
    let mut n = if absnum < 1.0 { 0 } else { nn };
    let s = if absnum < 1.0 { "0" } else { "" };
    let mut csd_str = String::from(s);
    let mut pow2n = (2.0_f64).powi(n - 1);
    while n > 0 || (nnz > 0 && num.abs() > 1e-100) {
        if n == 0 {
            csd_str.push('.');
        }
        n -= 1;
        let d = 1.5 * num;
        if d > pow2n {
            csd_str.push('+');
            num -= pow2n;
            nnz -= 1;
        } else if d < -pow2n {
            csd_str.push('-');
            num += pow2n;
            nnz -= 1;
        } else {
            csd_str.push('0');
        }
        pow2n /= 2.0;
        if nnz == 0 {
            num = 0.0;
        }
    }
    csd_str
}

#[cfg(test)]
mod tests {
    use super::{to_csd, to_csdfixed, to_decimal};

    #[test]
    fn test_to_csd() {
        assert_eq!(String::from("+00-00.+0"), to_csd(28.5, 2));
        assert_eq!(String::from("0.-0"), to_csd(-0.5, 2));
    }

    #[test]
    fn test_to_decimal() {
        assert_eq!(to_decimal(&String::from("+00-00.+")), 28.5);
        assert_eq!(to_decimal(&String::from("0.-")), -0.5);
    }

    #[test]
    fn test_to_csdfixed() {
        assert_eq!(String::from("+00-00.+"), to_csdfixed(28.5, 4));
        assert_eq!(String::from("0.-"), to_csdfixed(-0.5, 4));
    }
}
