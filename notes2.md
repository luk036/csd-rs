```rust
use std::env;
use std::str::FromStr;

use csdigit::csd::{to_csd, to_decimal};

fn parse_args(args: &[String]) -> (f64, String, usize) {
    let mut decimal = f64::INFINITY;
    let mut csdstr = String::new();
    let mut places = 4;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "-c" | "--to_csd" => {
                decimal = f64::from_str(&args[i + 1]).unwrap();
                i += 2;
            }
            "-d" | "--to_decimal" => {
                csdstr = args[i + 1].clone();
                i += 2;
            }
            "-p" | "--places" => {
                places = usize::from_str(&args[i + 1]).unwrap();
                i += 2;
            }
            _ => i += 1,
        }
    }

    (decimal, csdstr, places)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let (decimal, csdstr, places) = parse_args(&args[1..]);

    if decimal != f64::INFINITY {
        let ans = to_csd(decimal, places);
        println!("{}", ans);
    }

    if !csdstr.is_empty() {
        let ans = to_decimal(&csdstr);
        println!("{}", ans);
    }
}
```
