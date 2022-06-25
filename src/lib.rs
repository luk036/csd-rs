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
    use super::*;

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
}
