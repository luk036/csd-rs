use std::fmt::Write;

pub struct CsdMultiplier {
    csd: String,
    n: usize,
    m: usize,
}

impl CsdMultiplier {
    pub fn new(csd: &str, n: usize, m: usize) -> Result<Self, String> {
        // Validate CSD string
        if !csd.chars().all(|c| matches!(c, '+' | '-' | '0')) {
            return Err("CSD string can only contain '+', '-', or '0'".to_string());
        }
        
        // Validate length matches M
        if csd.len() != m + 1 {
            return Err(format!(
                "CSD length {} doesn't match M={} (should be M+1)", 
                csd.len(), m
            ));
        }

        Ok(Self {
            csd: csd.to_string(),
            n,
            m,
        })
    }

    /// Calculate the decimal value represented by the CSD string
    fn decimal_value(&self) -> i32 {
        self.csd.chars().enumerate().map(|(i, c)| {
            let power = self.m - i;
            match c {
                '+' => 1 << power,
                '-' => -(1 << power),
                '0' => 0,
                _ => unreachable!(),
            }
        }).sum()
    }

    /// Generate the Verilog module code
    pub fn generate_verilog(&self) -> String {
        // Parse non-zero terms
        let terms: Vec<_> = self.csd.chars().enumerate()
            .filter_map(|(i, c)| {
                let power = self.m - i;
                match c {
                    '+' => Some((power, '+')),
                    '-' => Some((power, '-')),
                    '0' => None,
                    _ => unreachable!(),
                }
            })
            .collect();

        // Calculate needed shift powers
        let shift_powers: Vec<_> = {
            let mut powers: Vec<_> = terms.iter().map(|(p, _)| *p).collect();
            powers.sort_by(|a, b| b.cmp(a)); // Sort descending
            powers.dedup();
            powers
        };

        let mut output = String::new();

        // Module header with comment showing decimal value
        writeln!(
            output,
            "// CSD Multiplier for pattern: {} (value: {})",
            self.csd,
            self.decimal_value()
        ).unwrap();

        writeln!(
            output,
            "module csd_multiplier (
    input signed [{}:0] x,      // Input value (signed)
    output signed [{}:0] result // Result (signed)
);",
            self.n - 1,
            self.n + self.m - 1
        ).unwrap();

        // Generate shifted versions
        if !terms.is_empty() {
            writeln!(
                output,
                "\n    // Signed shifted versions (Verilog handles sign extension)"
            ).unwrap();

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
                ).unwrap();
            }
        }

        // Generate the computation
        writeln!(output, "\n    // CSD implementation with signed arithmetic").unwrap();

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

        writeln!(output, "endmodule").unwrap();
        output
    }
}

fn main() {
    let csd = "+00-00+";  // Represents 57
    let n = 8;              // Input bit width
    let m = 6;              // Highest power (2^6 for this CSD)

    let multiplier = CsdMultiplier::new(csd, n, m)
        .expect("Invalid CSD parameters");

    println!("{}", multiplier.generate_verilog());
}

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
        assert!(CsdMultiplier::new(csd, 8, 6).is_err());
    }

    #[test]
    fn test_length_mismatch() {
        let csd = "+00-00+0+";
        assert!(CsdMultiplier::new(csd, 8, 5).is_err());
    }
}
