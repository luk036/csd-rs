pub mod csd;
pub mod lcsre;

pub use crate::csd::{to_csd, to_csd_i, to_csdfixed, to_decimal, to_decimal_i};
pub use crate::lcsre::longest_repeated_substring;

#[cfg(test)]
mod tests {
    use super::csd::*;
    use super::lcsre::*;
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
        assert_eq!(to_csd(0.0, 0), "0".to_string());
        assert_eq!(to_csd(28.5, -1), "+00-0".to_string());
    }

    #[test]
    fn test_to_decimal() {
        assert_eq!(to_decimal("+00-00.+"), 28.5);
        assert_eq!(to_decimal("0.-"), -0.5);
        assert_eq!(to_decimal("0"), 0.0);
        assert_eq!(to_decimal("0.0"), 0.0);
        assert_eq!(to_decimal("0.+"), 0.5);
        assert_eq!(to_decimal("0.-"), -0.5);
        assert_eq!(to_decimal("0.++"), 0.75);
        assert_eq!(to_decimal("0.-+"), -0.25);
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
        let chars: Vec<_> = "+00-00".to_string().chars().collect();
        assert_eq!(to_decimal_i(&chars), 28);
        let chars: Vec<_> = "0".to_string().chars().collect();
        assert_eq!(to_decimal_i(&chars), 0);
    }

    #[test]
    #[should_panic]
    fn test_to_decimal_i_invalid() {
        let chars: Vec<_> = "+00-00.00+".to_string().chars().collect();
        let _res = to_decimal_i(&chars);
    }

    #[test]
    fn test_to_csdfixed() {
        assert_eq!(to_csdfixed(28.5, 4), "+00-00.+".to_string());
        assert_eq!(to_csdfixed(-0.5, 4), "0.-".to_string());
        assert_eq!(to_csdfixed(0.0, 4), "0".to_string());
        assert_eq!(to_csdfixed(0.0, 0), "0".to_string());
        assert_eq!(to_csdfixed(0.5, 4), "0.+".to_string());
        assert_eq!(to_csdfixed(-0.5, 4), "0.-".to_string());
        assert_eq!(to_csdfixed(28.5, 2), "+00-00".to_string());
        assert_eq!(to_csdfixed(28.5, 1), "+00000".to_string());
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
        let chars: Vec<_> = csd.chars().collect();
        d == to_decimal_i(&chars)
    }

    #[test]
    fn test_lcsre() {
        let cstr = "+-00+-00+-00+-0".to_string();
        let res = longest_repeated_substring(&cstr.chars().collect::<Vec<char>>());
        assert_eq!(res, "+-00+-0".to_string());
        let cstr = "abcdefgh".to_string();
        let res = longest_repeated_substring(&cstr.chars().collect::<Vec<char>>());
        assert_eq!(res, "".to_string());
    }
}
