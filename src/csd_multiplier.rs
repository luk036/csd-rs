//! CSD Multiplier Module
//!
//! This module provides functionality to generate Verilog code for efficient constant multiplication
//! using Canonical Signed Digit (CSD) representation. CSD representation minimizes the number of
//! non-zero digits, which reduces the number of adders/subtractors needed in hardware implementation.
//!
//! # Overview
//!
//! In digital signal processing and hardware design, multiplying a variable by a constant is a common
//! operation. Using CSD representation, we can implement these multiplications efficiently using only
//! shifts, additions, and subtractions instead of full multipliers.
//!
//! # Example
//!
//! ```rust
//! use csd::csd_multiplier::{CsdMultiplier, CsdMultiplierError};
//!
//! // Create a multiplier for CSD pattern "+0-" (which represents 3)
//! // n=8 means input is 8 bits, m=2 means the highest power is 2^2
//! let multiplier = CsdMultiplier::new("+0-", 8, 2).unwrap();
//!
//! // Generate Verilog code
//! let verilog = multiplier.generate_verilog();
//! println!("{}", verilog);
//! ```
//!
//! # CSD Representation
//!
//! In CSD, each digit can be:
//! - `+`: +1
//! - `-`: -1
//! - `0`: 0
//!
//! And no two consecutive digits can be non-zero. This property ensures minimal non-zero digits.
//!
//! # Hardware Benefits
//!
//! - Reduced gate count compared to traditional multipliers
//! - Lower power consumption
//! - Faster critical path
//! - No need for full multiplier hardware
//!
//! # Error Handling
//!
//! The module returns `CsdMultiplierError` for:
//! - Invalid characters in CSD string (only '+', '-', '0' allowed)
//! - Length mismatch between CSD string and expected length (m+1)

use std::fmt::Write;

#[derive(Debug)]
pub enum CsdMultiplierError {
    InvalidCharacter,
    LengthMismatch,
}

/// A CSD-based constant multiplier that generates Verilog code
///
/// # Example
///
/// ```rust
/// use csd::csd_multiplier::{CsdMultiplier, CsdMultiplierError};
///
/// // Create a multiplier for the CSD pattern "+00-00+" (value: 57)
/// // n=8: input bit width (8 bits)
/// // m=6: highest power index (2^6 = 64 is the highest power used)
/// let multiplier = CsdMultiplier::new("+00-00+", 8, 6).unwrap();
///
/// // Generate Verilog code
/// let verilog = multiplier.generate_verilog();
/// assert!(verilog.contains("module csd_multiplier"));
/// ```
pub struct CsdMultiplier {
    csd: String,
    n: usize,
    m: usize,
}

impl CsdMultiplier {
    pub fn new(csd: &str, n: usize, m: usize) -> Result<Self, CsdMultiplierError> {
        // Validate CSD string
        if !csd.chars().all(|c| matches!(c, '+' | '-' | '0')) {
            return Err(CsdMultiplierError::InvalidCharacter);
        }

        // Validate length matches M
        if csd.len() != m + 1 {
            return Err(CsdMultiplierError::LengthMismatch);
        }

        Ok(Self {
            csd: csd.to_string(),
            n,
            m,
        })
    }

    /// Calculate the decimal value represented by the CSD string
    fn decimal_value(&self) -> i32 {
        self.csd.chars().fold(0, |acc, c| {
            let acc = acc << 1;
            match c {
                '+' => acc + 1,
                '-' => acc - 1,
                '0' => acc,
                _ => unreachable!(),
            }
        })
    }

    /// Generate the Verilog module code
    pub fn generate_verilog(&self) -> String {
        let mut output = String::new();
        self.generate_header(&mut output);
        self.generate_wires(&mut output);
        self.generate_result(&mut output);
        writeln!(output, "endmodule").unwrap();
        output
    }

    fn generate_header(&self, output: &mut String) {
        writeln!(
            output,
            "// CSD Multiplier for pattern: {} (value: {})",
            self.csd,
            self.decimal_value()
        )
        .unwrap();

        writeln!(
            output,
            "module csd_multiplier (
    input signed [{}:0] x,      // Input value (signed)
    output signed [{}:0] result // Result (signed)
);",
            self.n - 1,
            self.n + self.m - 1
        )
        .unwrap();
    }

    fn get_terms(&self) -> Vec<(usize, char)> {
        self.csd
            .chars()
            .enumerate()
            .filter_map(|(i, c)| {
                let power = self.m - i;
                match c {
                    '+' => Some((power, '+')),
                    '-' => Some((power, '-')),
                    '0' => None,
                    _ => unreachable!(),
                }
            })
            .collect()
    }

    fn generate_wires(&self, output: &mut String) {
        let terms = self.get_terms();
        if terms.is_empty() {
            return;
        }

        let shift_powers: Vec<_> = {
            let mut powers: Vec<_> = terms.iter().map(|(p, _)| *p).collect();
            powers.sort_by(|a, b| b.cmp(a)); // Sort descending
            powers.dedup();
            powers
        };

        writeln!(
            output,
            "\n    // Signed shifted versions (Verilog handles sign extension)"
        )
        .unwrap();

        for &power in &shift_powers {
            let padding = self.m - power;
            writeln!(
                output,
                "    wire signed [{}:0] x_shift{} = $signed({{ {{{}{{x[{}]}}}}, x}}) << {};",
                self.n + self.m - 1,
                power,
                padding,
                self.n - 1,
                power
            )
            .unwrap();
        }
    }

    fn generate_result(&self, output: &mut String) {
        writeln!(output, "\n    // CSD implementation with signed arithmetic").unwrap();
        let terms = self.get_terms();

        if terms.is_empty() {
            writeln!(output, "    assign result = 0;").unwrap();
        } else {
            let (first_power, first_op) = terms[0];
            let mut expr = format!("{}x_shift{}", first_op, first_power);

            for (power, op) in &terms[1..] {
                expr.push_str(&format!(" {} x_shift{}", op, power));
            }

            writeln!(output, "    assign result = {};", expr.replace("+", "")).unwrap();
        }
    }
}

// fn main() {
//     let csd = "+00-00+"; // Represents 57
//     let n = 8; // Input bit width
//     let m = 6; // Highest power (2^6 for this CSD)

//     let multiplier = CsdMultiplier::new(csd, n, m).expect("Invalid CSD parameters");

//     println!("{}", multiplier.generate_verilog());
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_csd() {
        let csd = "+00-00+0+";
        let multiplier = CsdMultiplier::new(csd, 8, 8).unwrap();
        assert_eq!(multiplier.decimal_value(), 229);
    }

    #[test]
    fn test_invalid_csd_chars() {
        let csd = "+01-00+0+";
        let result = CsdMultiplier::new(csd, 8, 6);
        assert!(matches!(result, Err(CsdMultiplierError::InvalidCharacter)));
    }

    #[test]
    fn test_length_mismatch() {
        let csd = "+00-00+0+";
        let result = CsdMultiplier::new(csd, 8, 5);
        assert!(matches!(result, Err(CsdMultiplierError::LengthMismatch)));
    }

    #[test]
    fn test_verilog_generation() {
        let csd = "+0-";
        let n = 8;
        let m = 2;
        let multiplier = CsdMultiplier::new(csd, n, m).unwrap();
        let expected_verilog = r###"// CSD Multiplier for pattern: +0- (value: 3)
module csd_multiplier (
    input signed [7:0] x,      // Input value (signed)
    output signed [9:0] result // Result (signed)
);

    // Signed shifted versions (Verilog handles sign extension)
    wire signed [9:0] x_shift2 = $signed({ {0{x[7]}}, x}) << 2;
    wire signed [9:0] x_shift0 = $signed({ {2{x[7]}}, x}) << 0;

    // CSD implementation with signed arithmetic
    assign result = x_shift2 - x_shift0;
endmodule
"###;
        assert_eq!(multiplier.generate_verilog(), expected_verilog);
    }
}
