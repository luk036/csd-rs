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
mod lib;
use crate::lib::{to_csd, to_csdfixed, to_decimal};

fn main() {
    println!("CHECK: +00-00.+0 == {}", to_csd(28.5, 2));
    println!("CHECK: 0.-0 == {}", to_csd(-0.5, 2));

    println!("CHECK: 28.5 == {}", to_decimal(&String::from("+00-00.+")));
    println!("CHECK: -0.5 == {}", to_decimal(&String::from("0.-")));

    println!("CHECK: +00-00.+ == {}", to_csdfixed(28.5, 4));
    println!("CHECK: 0.- == {}", to_csdfixed(-0.5, 4));
}
