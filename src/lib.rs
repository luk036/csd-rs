/// Convert to CSD (Canonical Signed Digit) String representation
///
/// - Original author: Harnesser
/// - <https://sourceforge.net/projects/pycsd/>
/// - License: GPL2
///
/// # Examples
///
/// ```
/// use csd::to_csd;
///
/// let s1 = to_csd(28.5, 2);
/// let s2 = to_csd(-0.5, 2);
///
/// assert_eq!(s1, String::from("+00-00.+0"));
/// assert_eq!(s2, String::from("0.-0"));
/// ```
pub fn to_csd(mut num: f64, places: i32) -> String {
    if num == 0.0 {
        return String::from("0");
    }
    let absnum = num.abs();
    let temp = (absnum * 1.5).log2().ceil() as i32;
    let (mut rem, s) = if absnum < 1.0 { (0, "0") } else { (temp, "") };
    let mut csd = String::from(s);
    let mut pow2n = (2.0_f64).powi(rem);
    while rem > -places {
        if rem == 0 {
            csd.push('.');
        }
        let pow2n_half = pow2n / 2.0;
        let det = 3.0 * num;
        if det > pow2n {
            csd.push('+');
            num -= pow2n_half;
        } else if det < -pow2n {
            csd.push('-');
            num += pow2n_half;
        } else {
            csd.push('0');
        }
        pow2n = pow2n_half;
        rem -= 1;
    }
    csd
}

/// Convert the CSD (Canonical Signed Digit) to a decimal
///
/// - Original author: Harnesser
/// - <https://sourceforge.net/projects/pycsd/>
/// - License: GPL2
///
/// # Examples
///
/// ```
/// use csd::to_decimal_i;
///
/// let chars: Vec<_> = String::from("+00-00").chars().collect();
/// let d1 = to_decimal_i(&chars);
///
/// assert_eq!(d1, 28);
/// ```
#[allow(dead_code)]
pub const fn to_decimal_i(csd: &[char]) -> i32 {
    let mut num: i32 = 0;
    let mut remaining = csd;
    while let [digit, tail @ ..] = remaining {
        if *digit == '0' {
            num *= 2;
        } else if *digit == '+' {
            num = 2 * num + 1;
        } else if *digit == '-' {
            num = 2 * num - 1;
        } // else unknown character
        remaining = tail;
    }
    num
}

/// Convert the CSD (Canonical Signed Digit) to a decimal
///
/// - Original author: Harnesser
/// - <https://sourceforge.net/projects/pycsd/>
/// - License: GPL2
///
/// # Examples
///
/// ```
/// use csd::to_decimal;
///
/// let d1 = to_decimal(&String::from("+00-00.+"));
/// let d2 = to_decimal(&String::from("0.-"));
///
/// assert_eq!(d1, 28.5);
/// assert_eq!(d2, -0.5);
/// ```
pub fn to_decimal(csd: &str) -> f64 {
    let mut num: f64 = 0.0;
    let mut loc: usize = 0;
    for (pos, digit) in csd.chars().enumerate() {
        if digit == '0' {
            num *= 2.0;
        } else if digit == '+' {
            num = num * 2.0 + 1.0;
        } else if digit == '-' {
            num = num * 2.0 - 1.0;
        } else if digit == '.' {
            loc = pos + 1;
        } // else unknown character
    }
    if loc != 0 {
        num /= (2.0_f64).powi((csd.len() - loc) as i32);
    }
    num
}

/// Convert to CSD representation with fixed number of non-zero
///
/// - Original author: Harnesser
/// - <https://sourceforge.net/projects/pycsd/>
/// - License: GPL2
///
/// # Examples
///
/// ```
/// use csd::to_csdfixed;
///
/// let s1 = to_csdfixed(28.5, 4);
/// let s2 = to_csdfixed(-0.5, 4);
///
/// assert_eq!(s1, String::from("+00-00.+"));
/// assert_eq!(s2, String::from("0.-"));
/// ```
#[allow(dead_code)]
pub fn to_csdfixed(mut num: f64, mut nnz: u32) -> String {
    if num == 0.0 {
        return String::from("0");
    }
    let absnum = num.abs();
    let nn = (absnum * 1.5).log2().ceil() as i32;
    let (mut rem, s) = if absnum < 1.0 { (0, "0") } else { (nn, "") };
    let mut csd = String::from(s);
    let mut pow2n = (2.0_f64).powi(rem);
    while rem > 0 || (nnz > 0 && num.abs() > 1e-100) {
        if rem == 0 {
            csd.push('.');
        }
        let pow2n_half = pow2n / 2.0;
        let det = 3.0 * num;
        if det > pow2n {
            csd.push('+');
            num -= pow2n_half;
            nnz -= 1;
        } else if det < -pow2n {
            csd.push('-');
            num += pow2n_half;
            nnz -= 1;
        } else {
            csd.push('0');
        }
        pow2n = pow2n_half;
        rem -= 1;
        if nnz == 0 {
            num = 0.0;
        }
    }
    csd
}

// Rust program to find the longest repeated
// non-overlapping substring

// Returns the longest repeating non-overlapping
// substring in cstr
#[allow(dead_code)]
pub fn longest_repeated_substring(cstr: &[char]) -> String {
    let rem = cstr.len();
    let mut lcsre = vec![vec![0; rem + 1]; rem + 1];

    let mut res_length = 0; // To store length of result

    // building table in bottom-up manner
    let mut index = 0;
    for i in 1..(rem + 1) {
        for j in (i + 1)..(rem + 1) {
            // (j-i) > lcsre[i-1][j-1] to remove
            // overlapping
            if cstr[i - 1] == cstr[j - 1] && lcsre[i - 1][j - 1] < (j - i) {
                lcsre[i][j] = lcsre[i - 1][j - 1] + 1;

                // updating maximum length of the
                // substring and updating the finishing
                // index of the suffix
                if lcsre[i][j] > res_length {
                    res_length = lcsre[i][j];
                    index = std::cmp::max(i, index);
                }
            } else {
                lcsre[i][j] = 0;
            }
        }
    }

    // If we have non-empty result, then insert
    // all characters from first character to
    // last character of string
    let mut res = String::from(""); // To store result
    if res_length > 0 {
        for i in (index - res_length + 1)..(index + 1) {
            res.push(cstr[i - 1]);
        }
    }

    res
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
        let s1 = to_csd(28.5, 2);
        let s2 = to_csd(-0.5, 2);
        assert_eq!(s1, String::from("+00-00.+0"));
        assert_eq!(s2, String::from("0.-0"));
    }

    #[test]
    fn test_to_decimal() {
        let d1 = to_decimal(&String::from("+00-00.+"));
        let d2 = to_decimal(&String::from("0.-"));
        assert_eq!(d1, 28.5);
        assert_eq!(d2, -0.5);
    }

    #[test]
    fn test_to_csdfixed() {
        let s1 = to_csdfixed(28.5, 4);
        let s2 = to_csdfixed(-0.5, 4);
        assert_eq!(s1, String::from("+00-00.+"));
        assert_eq!(s2, String::from("0.-"));
    }

    #[quickcheck]
    fn test_csd(d: i32) -> bool {
        let f = d as f64;
        f == to_decimal(&to_csd(f, 2))
    }

    #[test]
    fn test_lcsre() {
        let cstr = String::from("+-00+-00+-00+-0");
        let res = longest_repeated_substring(&cstr.chars().collect::<Vec<char>>());
        assert_eq!(res, String::from("+-00+-0"));
    }
}
