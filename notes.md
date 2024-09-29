use std::env;
use std::f64;
use std::str::FromStr;
use log::{info, debug};
use csdigit::{to_csd, to_csdnnz, to_decimal};

fn parse_args(args: &[String]) -> Result<Args, &'static str> {
let mut decimal = f64::INFINITY;
let mut decimal2 = f64::INFINITY;
let mut csdstr = String::new();
let mut loglevel = log::LevelFilter::Off;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--version" => {
                println!("csdigit {}", env!("CARGO_PKG_VERSION"));
                return Err("");
            }
            "-c" | "--to_csd" => {
                i += 1;
                decimal = f64::from_str(&args[i]).unwrap();
            }
            "-v" | "--verbose" => {
                loglevel = log::LevelFilter::Info;
            }
            _ => {
                csdstr = args[i].clone();
            }
        }
        i += 1;
    }

    Ok(Args {
        decimal,
        decimal2,
        csdstr,
        loglevel,
    })

}

fn setup_logging(loglevel: log::LevelFilter) {
env::set_var("RUST_LOG", "csdigit=debug");
env_logger::builder()
.format_timestamp(Some("%Y-%m-%d %H:%M:%S"))
.format(|buf, record| {
writeln!(
buf,
"[{}] {}: {}",
buf.timestamp(),
record.level(),
record.args()
)
})
.filter(None, loglevel)
.init();
}

fn main() {
let args: Vec<String> = env::args().collect();
let args = parse_args(&args[1..]).unwrap();
setup_logging(args.loglevel);
debug!("Starting crazy calculations...");

    if args.decimal != f64::INFINITY {
        let ans = to_csd(args.decimal, args.places);
        println!("{}", ans);
    }
    if args.decimal2 != f64::INFINITY {
        let ans = to_csdnnz(args.decimal2, args.nnz);
        println!("{}", ans);
    }
    if !args.csdstr.is_empty() {
        let ans = to_decimal(&args.csdstr);
        println!("{}", ans);
    }

    info!("Script ends here");

}

struct Args {
decimal: f64,
decimal2: f64,
csdstr: String,
loglevel: log::LevelFilter,
}

fn run() {
main();
}

#[cfg(test)]
mod tests {
use super::\*;

    #[test]
    fn test_parse_args() {
        let args = vec![
            String::from("-c"),
            String::from("3.14"),
            String::from("-v"),
        ];
        let result = parse_args(&args);
        assert_eq!(result.is_ok(), true);
        let args = result.unwrap();
        assert_eq!(args.decimal, 3.14);
        assert_eq!(args.decimal2, f64::INFINITY);
        assert_eq!(args.csdstr, String::new());
        assert_eq!(args.loglevel, log::LevelFilter::Info);
    }

}
