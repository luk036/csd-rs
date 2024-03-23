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
// mod lib;
// use crate::lib::{to_csd, to_csdfixed, to_decimal};
mod csd;
mod lcsre;

use crate::csd::{to_csd, to_csdfixed, to_decimal};
use argparse::{ArgumentParser, Print, Store, StoreTrue};

fn main() {
    let mut verbose = false;
    let mut decimal = f64::INFINITY;
    let mut decimal2 = f64::INFINITY;
    let mut csdstr = String::new();
    let mut nnz = 3;
    let mut places = 4;
    {
        // this block limits scope of borrows by ap.refer() method
        let mut ap = ArgumentParser::new();
        ap.set_description("Canonical Signed Digit (CSD) Conversion.");
        ap.refer(&mut verbose)
            .add_option(&["-v", "--verbose"], StoreTrue, "Be verbose");
        ap.refer(&mut csdstr)
            .add_option(&["-d", "--to_decimal"], Store, "Convert to decimal");
        ap.refer(&mut decimal).add_option(
            &["-c", "--to_csd"],
            Store,
            "Convert to CSD with places (default is 4)",
        );
        ap.refer(&mut places)
            .add_option(&["-p", "--places"], Store, "Specify the places");
        ap.refer(&mut nnz)
            .add_option(&["-z", "--nnz"], Store, "Specify the number of non-zeros");
        ap.refer(&mut decimal2).add_option(
            &["-f", "--to_csdfixed"],
            Store,
            "Convert to CSD with fixed number of non-zeros (default is 3)",
        );
        ap.add_option(
            &["-V", "--version"],
            Print(env!("CARGO_PKG_VERSION").to_string()),
            "Show version",
        );
        ap.parse_args_or_exit();
    }

    if verbose {
        println!("Starting crazy calculations...");
    }

    if decimal != f64::INFINITY {
        let ans = to_csd(decimal, places);
        println!("{}", ans);
    }
    if decimal2 != f64::INFINITY {
        let ans = to_csdfixed(decimal2, nnz);
        println!("{}", ans);
    }
    if !csdstr.is_empty() {
        let ans = to_decimal(&csdstr);
        println!("{}", ans);
    }

    if verbose {
        println!("Script ends here");
    }
}
