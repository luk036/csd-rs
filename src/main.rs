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
use csd::csd::{to_csd, to_csdnnz, to_decimal};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage: csd-rs <command> [args]");
        println!("\nCommands:");
        println!("  to_csd <value> [places]    - Convert decimal to CSD");
        println!("  to_csdnnz <value> [nnz]    - Convert decimal to CSD with limited non-zeros");
        println!("  to_decimal <csd_string>    - Convert CSD to decimal");
        println!("\nExamples:");
        println!("  csd-rs to_csd 28.5 2");
        println!("  csd-rs to_csdnnz 28.5 4");
        println!("  csd-rs to_decimal '+00-00.+'");
        return;
    }

    match args[1].as_str() {
        "to_csd" => {
            if args.len() < 3 {
                eprintln!("Error: to_csd requires a value");
                return;
            }
            let value: f64 = match args[2].parse() {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("Error parsing value: {}", e);
                    return;
                }
            };
            let places: i32 = if args.len() > 3 {
                args[3].parse().unwrap_or(4)
            } else {
                4
            };
            let ans = to_csd(value, places);
            println!("{}", ans);
        }
        "to_csdnnz" => {
            if args.len() < 3 {
                eprintln!("Error: to_csdnnz requires a value");
                return;
            }
            let value: f64 = match args[2].parse() {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("Error parsing value: {}", e);
                    return;
                }
            };
            let nnz: u32 = if args.len() > 3 {
                args[3].parse().unwrap_or(3)
            } else {
                3
            };
            let ans = to_csdnnz(value, nnz);
            println!("{}", ans);
        }
        "to_decimal" => {
            if args.len() < 3 {
                eprintln!("Error: to_decimal requires a CSD string");
                return;
            }
            let ans = to_decimal(&args[2]);
            println!("{}", ans);
        }
        _ => {
            eprintln!("Unknown command: {}", args[1]);
        }
    }
}
