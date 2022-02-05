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
// mod super::lib;
// use crate::lib::to_csd;
// use crate::lib::to_decimal;
// use crate::lib::to_csdfixed;
// use crate::lib::{to_csd, to_decimal, to_csdfixed};

#[cfg(test)]
mod tests {
    use super::crate::lib::{to_csd, to_decimal, to_csdfixed};
    
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

