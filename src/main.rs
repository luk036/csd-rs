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

/// CLI result type
pub type CliResult = Result<String, String>;

/// Handle the to_csd command
pub fn handle_to_csd(args: &[String]) -> CliResult {
    if args.len() < 3 {
        return Err("Error: to_csd requires a value".to_string());
    }
    let value: f64 = args[2].parse().map_err(|e| format!("Error parsing value: {}", e))?;
    let places: i32 = if args.len() > 3 {
        args[3].parse().unwrap_or(4)
    } else {
        4
    };
    Ok(to_csd(value, places))
}

/// Handle the to_csdnnz command
pub fn handle_to_csdnnz(args: &[String]) -> CliResult {
    if args.len() < 3 {
        return Err("Error: to_csdnnz requires a value".to_string());
    }
    let value: f64 = args[2].parse().map_err(|e| format!("Error parsing value: {}", e))?;
    let nnz: u32 = if args.len() > 3 {
        args[3].parse().unwrap_or(3)
    } else {
        3
    };
    Ok(to_csdnnz(value, nnz))
}

/// Handle the to_decimal command
pub fn handle_to_decimal(args: &[String]) -> CliResult {
    if args.len() < 3 {
        return Err("Error: to_decimal requires a CSD string".to_string());
    }
    Ok(to_decimal(&args[2]).to_string())
}

/// Run the CLI application with the given arguments
pub fn run_cli(args: &[String]) -> Result<String, String> {
    if args.len() < 2 {
        let help = "Usage: csd-rs <command> [args]\n\nCommands:\n  to_csd <value> [places]    - Convert decimal to CSD\n  to_csdnnz <value> [nnz]    - Convert decimal to CSD with limited non-zeros\n  to_decimal <csd_string>    - Convert CSD to decimal\n\nExamples:\n  csd-rs to_csd 28.5 2\n  csd-rs to_csdnnz 28.5 4\n  csd-rs to_decimal '+00-00.+'";
        return Ok(help.to_string());
    }

    match args[1].as_str() {
        "to_csd" => handle_to_csd(args),
        "to_csdnnz" => handle_to_csdnnz(args),
        "to_decimal" => handle_to_decimal(args),
        _ => Err(format!("Unknown command: {}", args[1])),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match run_cli(&args) {
        Ok(output) => println!("{}", output),
        Err(error) => eprintln!("{}", error),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_to_csd_basic() {
        let args = vec!["csd-rs".to_string(), "to_csd".to_string(), "28.5".to_string(), "2".to_string()];
        let result = handle_to_csd(&args);
        assert!(result.is_ok());
        assert!(result.unwrap().contains("+00-00"));
    }

    #[test]
    fn test_handle_to_csd_default_places() {
        let args = vec!["csd-rs".to_string(), "to_csd".to_string(), "28.5".to_string()];
        let result = handle_to_csd(&args);
        assert!(result.is_ok());
        assert!(result.unwrap().contains("+00-00"));
    }

    #[test]
    fn test_handle_to_csd_zero() {
        let args = vec!["csd-rs".to_string(), "to_csd".to_string(), "0".to_string(), "2".to_string()];
        let result = handle_to_csd(&args);
        assert!(result.is_ok());
        assert!(result.unwrap().contains("0."));
    }

    #[test]
    fn test_handle_to_csd_negative() {
        let args = vec!["csd-rs".to_string(), "to_csd".to_string(), "-28.5".to_string(), "2".to_string()];
        let result = handle_to_csd(&args);
        assert!(result.is_ok());
        assert!(result.unwrap().starts_with('-'));
    }

    #[test]
    fn test_handle_to_csd_invalid_value() {
        let args = vec!["csd-rs".to_string(), "to_csd".to_string(), "invalid".to_string()];
        let result = handle_to_csd(&args);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Error parsing value"));
    }

    #[test]
    fn test_handle_to_csd_missing_value() {
        let args = vec!["csd-rs".to_string(), "to_csd".to_string()];
        let result = handle_to_csd(&args);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("to_csd requires a value"));
    }

    #[test]
    fn test_handle_to_csdnnz_basic() {
        let args = vec!["csd-rs".to_string(), "to_csdnnz".to_string(), "28.5".to_string(), "4".to_string()];
        let result = handle_to_csdnnz(&args);
        assert!(result.is_ok());
        assert!(result.unwrap().contains("+00-00"));
    }

    #[test]
    fn test_handle_to_csdnnz_default_nnz() {
        let args = vec!["csd-rs".to_string(), "to_csdnnz".to_string(), "28.5".to_string()];
        let result = handle_to_csdnnz(&args);
        assert!(result.is_ok());
        assert!(result.unwrap().contains("+00-00"));
    }

    #[test]
    fn test_handle_to_csdnnz_zero() {
        let args = vec!["csd-rs".to_string(), "to_csdnnz".to_string(), "0".to_string(), "4".to_string()];
        let result = handle_to_csdnnz(&args);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "0");
    }

    #[test]
    fn test_handle_to_csdnnz_invalid_value() {
        let args = vec!["csd-rs".to_string(), "to_csdnnz".to_string(), "invalid".to_string()];
        let result = handle_to_csdnnz(&args);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Error parsing value"));
    }

    #[test]
    fn test_handle_to_csdnnz_missing_value() {
        let args = vec!["csd-rs".to_string(), "to_csdnnz".to_string()];
        let result = handle_to_csdnnz(&args);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("to_csdnnz requires a value"));
    }

    #[test]
    fn test_handle_to_decimal_basic() {
        let args = vec!["csd-rs".to_string(), "to_decimal".to_string(), "+00-00.+".to_string()];
        let result = handle_to_decimal(&args);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "28.5");
    }

    #[test]
    fn test_handle_to_decimal_integer() {
        let args = vec!["csd-rs".to_string(), "to_decimal".to_string(), "+00-00".to_string()];
        let result = handle_to_decimal(&args);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "28");
    }

    #[test]
    fn test_handle_to_decimal_zero() {
        let args = vec!["csd-rs".to_string(), "to_decimal".to_string(), "0".to_string()];
        let result = handle_to_decimal(&args);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "0");
    }

    #[test]
    fn test_handle_to_decimal_negative() {
        let args = vec!["csd-rs".to_string(), "to_decimal".to_string(), "-00+00".to_string()];
        let result = handle_to_decimal(&args);
        assert!(result.is_ok());
        assert!(result.unwrap().starts_with('-'));
    }

    #[test]
    fn test_handle_to_decimal_missing_csd() {
        let args = vec!["csd-rs".to_string(), "to_decimal".to_string()];
        let result = handle_to_decimal(&args);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("to_decimal requires a CSD string"));
    }

    #[test]
    fn test_run_cli_help() {
        let args = vec!["csd-rs".to_string()];
        let result = run_cli(&args);
        assert!(result.is_ok());
        assert!(result.unwrap().contains("Usage: csd-rs"));
    }

    #[test]
    fn test_run_cli_unknown_command() {
        let args = vec!["csd-rs".to_string(), "unknown".to_string()];
        let result = run_cli(&args);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unknown command"));
    }

    #[test]
    fn test_run_cli_to_csd() {
        let args = vec!["csd-rs".to_string(), "to_csd".to_string(), "28.5".to_string(), "2".to_string()];
        let result = run_cli(&args);
        assert!(result.is_ok());
        assert!(result.unwrap().contains("+00-00"));
    }

    #[test]
    fn test_run_cli_to_csdnnz() {
        let args = vec!["csd-rs".to_string(), "to_csdnnz".to_string(), "28.5".to_string(), "4".to_string()];
        let result = run_cli(&args);
        assert!(result.is_ok());
        assert!(result.unwrap().contains("+00-00"));
    }

    #[test]
    fn test_run_cli_to_decimal() {
        let args = vec!["csd-rs".to_string(), "to_decimal".to_string(), "+00-00.+".to_string()];
        let result = run_cli(&args);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "28.5");
    }

    #[test]
    fn test_run_cli_roundtrip() {
        let args1 = vec!["csd-rs".to_string(), "to_csd".to_string(), "42.5".to_string(), "4".to_string()];
        let csd = run_cli(&args1).unwrap();
        let args2 = vec!["csd-rs".to_string(), "to_decimal".to_string(), csd.trim().to_string()];
        let decimal: f64 = run_cli(&args2).unwrap().parse().unwrap();
        assert!((decimal - 42.5).abs() < 0.1);
    }
}
