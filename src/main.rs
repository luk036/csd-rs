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
// use crate::lib::{to_csd, to_csdfixed, to_decimal};
use crate::lib::{to_csd, to_decimal};

// extern crate structopt;

use structopt::StructOpt;

#[derive(StructOpt)]
struct Options {
    #[structopt(short = "c", long = "to_csd")]
    /// Input a number in decimal, e.g. 56.5
    decimal: Option<f64>,

    #[structopt(short = "d", long = "to_decimal")]
    /// Input a number in CSD format, e.g. "+0-0.0-+"
    csd: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = Options::from_args();
    // let message = std::env::args().nth(1)
    //     .expect("Missing the message. Usage: catsay < message>");

    match &options.csd {
        Some (number) => {
            println!("The answer is {}", to_decimal(&number));
        },
        None => {
            // ... print the cat as before
            // println!("Please input a number");
        }
    }

    match &options.decimal {
        Some (number) => {
            println!("The answer is {}", to_csd(*number, 4));
        },
        None => {
            // ... print the cat as before
            // println!("Please input a number");
        }
    }
    Ok(())
}
