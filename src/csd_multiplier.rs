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
//! # Single Multiplier (with LCSRe optimization)
//!
//! When the CSD string contains a repeated non-overlapping pattern with ≥2 non-zero digits,
//! the generated Verilog shares hardware via a sub-expression wire `_pat`, reducing adder count.
//!
//! # Multi-Coefficient Cross-CSE
//!
//! `generate_csd_multipliers()` finds repeated substrings across **different** coefficients and
//! creates a shared common sub-expression (CSE) wire, reducing total hardware across the filter.

use std::collections::{BTreeSet, HashMap, HashSet};
use std::fmt::Write;

use crate::lcsre::longest_repeated_substring;

/// Error type for CSD multiplier operations.
#[derive(Debug, Clone, PartialEq)]
pub enum CsdMultiplierError {
    /// Invalid character found in CSD string (only '+', '-', '0' allowed)
    InvalidCharacter,
    /// Length of CSD string doesn't match expected length (max_power + 1)
    LengthMismatch,
    /// At least one coefficient is required
    EmptyCoefficients,
    /// All coefficients must share the same input_width and max_power
    WidthMismatch,
}

/// A CSD-based constant multiplier that generates Verilog code
///
/// # Example
///
/// ```rust
/// use csd::csd_multiplier::{CsdMultiplier, CsdMultiplierError};
///
/// // Create a multiplier for the CSD pattern "+00-00+" (value: 57)
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

/// Specification for a single CSD multiplier coefficient
///
/// Used with [`generate_csd_multipliers()`] for multi-coefficient
/// cross-common-subexpression elimination.
#[derive(Debug, Clone)]
pub struct MultiplierSpec {
    /// Output port name (e.g. "y0", "y1")
    pub name: String,
    /// CSD string ('+', '-', '0')
    pub csd: String,
    /// Bit width of input x
    pub input_width: usize,
    /// Highest power (len(csd) - 1)
    pub max_power: usize,
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq)]
enum TermOp {
    Add,
    Sub,
}

/// Parse a CSD string into (power, operation) pairs.
fn parse_terms(
    csd_str: &str,
    max_power: usize,
) -> Result<Vec<(usize, TermOp)>, CsdMultiplierError> {
    let mut terms = Vec::new();
    let bytes = csd_str.as_bytes();
    for (i, &c) in bytes.iter().enumerate() {
        let power = max_power - i;
        match c {
            b'+' => terms.push((power, TermOp::Add)),
            b'-' => terms.push((power, TermOp::Sub)),
            b'0' => {}
            _ => return Err(CsdMultiplierError::InvalidCharacter),
        }
    }
    Ok(terms)
}

/// Build a flat Verilog expression for a range [start, start+length) of the CSD string.
fn build_range_expr(csd_str: &str, start: usize, length: usize, max_power: usize) -> String {
    let mut expr = String::new();
    let mut first = true;
    let bytes = csd_str.as_bytes();
    let end = start.saturating_add(length).min(bytes.len());
    for (i, &c) in bytes.iter().enumerate().skip(start).take(end - start) {
        let power = max_power - i;
        match c {
            b'+' => {
                if first {
                    write!(expr, "x_shift{}", power).unwrap();
                    first = false;
                } else {
                    write!(expr, " + x_shift{}", power).unwrap();
                }
            }
            b'-' => {
                if first {
                    write!(expr, "-x_shift{}", power).unwrap();
                    first = false;
                } else {
                    write!(expr, " - x_shift{}", power).unwrap();
                }
            }
            _ => {}
        }
    }
    expr
}

/// Compute output width from input_width and max_power.
///
/// $$ W_{\text{out}} = W_{\text{in}} + m $$
///
/// where $W_{\text{in}}$ is the input bit width and $m$ is the maximum power of two.
fn output_width(input_width: usize, max_power: usize) -> usize {
    input_width + max_power
}

// ---------------------------------------------------------------------------
// CsdMultiplier (struct-based, backward compatible)
// ---------------------------------------------------------------------------

impl CsdMultiplier {
    /// Create a new CSD multiplier.
    ///
    /// # Arguments
    ///
    /// * `csd` - The CSD pattern string (e.g., "+0-")
    /// * `n` - Input bit width
    /// * `m` - Highest power index (length of CSD minus 1)
    ///
    /// # Errors
    ///
    /// Returns `CsdMultiplierError::InvalidCharacter` if the CSD string contains
    /// characters other than '+', '-', or '0'.
    ///
    /// Returns `CsdMultiplierError::LengthMismatch` if the CSD string length
    /// doesn't equal `m + 1`.
    pub fn new(csd: &str, n: usize, m: usize) -> Result<Self, CsdMultiplierError> {
        let bytes = csd.as_bytes();
        if !bytes.iter().all(|&c| matches!(c, b'+' | b'-' | b'0')) {
            return Err(CsdMultiplierError::InvalidCharacter);
        }
        if csd.len() != m + 1 {
            return Err(CsdMultiplierError::LengthMismatch);
        }
        Ok(Self {
            csd: csd.to_string(),
            n,
            m,
        })
    }

    /// Calculate the decimal value represented by the CSD string.
    ///
    /// $$ v = \sum_{i=0}^{m} d_i \cdot 2^{m-i}, \quad d_i \in \{-1,0,+1\} $$
    ///
    fn decimal_value(&self) -> i32 {
        self.csd.as_bytes().iter().fold(0, |acc, &c| {
            let acc = acc << 1;
            match c {
                b'+' => acc + 1,
                b'-' => acc - 1,
                b'0' => acc,
                _ => unreachable!(),
            }
        })
    }

    /// Generate the Verilog module code (with LCSRe optimization).
    pub fn generate_verilog(&self) -> String {
        let mut output = String::new();
        self.generate_header(&mut output);
        self.generate_wires(&mut output);
        self.generate_result_lcsre(&mut output);
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

    /// Return sorted unique powers of non-zero digits, descending.
    fn get_unique_powers(&self) -> Vec<usize> {
        let mut powers: Vec<usize> = self
            .csd
            .char_indices()
            .filter(|(_, c)| *c != '0')
            .map(|(i, _)| self.m - i)
            .collect();
        powers.sort_unstable_by(|a, b| b.cmp(a));
        powers.dedup();
        powers
    }

    fn generate_wires(&self, output: &mut String) {
        let shift_powers = self.get_unique_powers();
        if shift_powers.is_empty() {
            return;
        }
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

    /// Generate assign statement with LCSRe optimization.
    fn generate_result_lcsre(&self, output: &mut String) {
        let terms = parse_terms(&self.csd, self.m).unwrap_or_default();
        if terms.is_empty() {
            writeln!(output, "\n    // CSD implementation").unwrap();
            writeln!(output, "    assign result = 0;").unwrap();
            return;
        }

        // Detect LCSRe optimization opportunity
        let repeated = longest_repeated_substring(&self.csd);
        let pat_positions = if repeated.len() > 1 {
            let pat_nnz = repeated.chars().filter(|c| *c == '+' || *c == '-').count();
            if pat_nnz >= 2 {
                let pos = find_pattern_occurrences(&self.csd, &repeated);
                if pos.len() >= 2 {
                    Some((repeated, pos))
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };

        if let Some((ref pat, ref positions)) = pat_positions {
            // LCSRe-optimized path
            let base_pos = positions[0];
            let ow = output_width(self.n, self.m);

            let pat_expr = build_range_expr(&self.csd, base_pos, pat.len(), self.m);
            writeln!(output, "\n    // LCSRe: repeated pattern \"{}\"", pat).unwrap();
            writeln!(
                output,
                "    wire signed [{}:0] _pat = {};",
                ow - 1,
                pat_expr
            )
            .unwrap();

            let mut expr = String::new();
            let mut cur = 0;
            for &pos in positions {
                // gap before this occurrence
                if pos > cur {
                    let gap = build_range_expr(&self.csd, cur, pos - cur, self.m);
                    if !gap.is_empty() {
                        if expr.is_empty() {
                            expr = gap;
                        } else {
                            write!(expr, " + {}", gap).unwrap();
                        }
                    }
                }
                // pattern occurrence
                let shift = pos as isize - base_pos as isize;
                let pat_ref = if shift == 0 {
                    "_pat".to_string()
                } else {
                    format!("(_pat >>> {})", shift)
                };
                if expr.is_empty() {
                    expr = pat_ref;
                } else {
                    write!(expr, " + {}", pat_ref).unwrap();
                }
                cur = pos + pat.len();
            }
            // suffix
            if cur < self.csd.len() {
                let suffix = build_range_expr(&self.csd, cur, self.csd.len() - cur, self.m);
                if !suffix.is_empty() {
                    write!(expr, " + {}", suffix).unwrap();
                }
            }

            writeln!(output, "\n    // CSD implementation (LCSRe optimized)").unwrap();
            writeln!(output, "    assign result = {};", expr).unwrap();
        } else {
            // flat path (no repeated pattern)
            writeln!(output, "\n    // CSD implementation with signed arithmetic").unwrap();
            let (first_power, first_op) = terms[0];
            let mut expr = format!(
                "{}x_shift{}",
                if first_op == TermOp::Sub { "-" } else { "" },
                first_power
            );
            for (power, op) in &terms[1..] {
                match op {
                    TermOp::Add => write!(expr, " + x_shift{}", power).unwrap(),
                    TermOp::Sub => write!(expr, " - x_shift{}", power).unwrap(),
                }
            }
            writeln!(output, "    assign result = {};", expr).unwrap();
        }
    }
}

// ---------------------------------------------------------------------------
// Free-function API (matching C++ style)
// ---------------------------------------------------------------------------

/// Find all non-overlapping occurrences of `pattern` in `csd_str`.
fn find_pattern_occurrences(csd_str: &str, pattern: &str) -> Vec<usize> {
    let mut positions = Vec::new();
    let mut pos = 0;
    while let Some(found) = csd_str[pos..].find(pattern) {
        let absolute = pos + found;
        positions.push(absolute);
        pos = absolute + pattern.len();
    }
    positions
}

/// Count non-zero digits ('+' or '-') in a CSD substring.
fn count_nnz(s: &str) -> usize {
    s.as_bytes()
        .iter()
        .filter(|&&c| c == b'+' || c == b'-')
        .count()
}

/// Build a coefficient expression using CSE wire + flat gap terms.
fn build_coeff_expr(
    csd: &str,
    max_power: usize,
    pattern: &str,
    cse_base_pos: usize,
    cse_name: &str,
) -> String {
    if pattern.is_empty() {
        return build_range_expr(csd, 0, csd.len(), max_power);
    }

    let positions = find_pattern_occurrences(csd, pattern);
    let mut parts: Vec<String> = Vec::new();
    let mut cur = 0;

    for pos in positions {
        // gap before this occurrence
        if pos > cur {
            let gap = build_range_expr(csd, cur, pos - cur, max_power);
            if !gap.is_empty() {
                parts.push(gap);
            }
        }
        // CSE reference
        let shift = pos as isize - cse_base_pos as isize;
        if shift == 0 {
            parts.push(cse_name.to_string());
        } else {
            parts.push(format!("({} >>> {})", cse_name, shift));
        }
        cur = pos + pattern.len();
    }
    // suffix
    if cur < csd.len() {
        let gap = build_range_expr(csd, cur, csd.len() - cur, max_power);
        if !gap.is_empty() {
            parts.push(gap);
        }
    }

    if parts.is_empty() {
        return String::new();
    }
    let mut result = parts[0].clone();
    for p in &parts[1..] {
        write!(result, " + {}", p).unwrap();
    }
    result
}

/// Find substrings (NNZ >= 2) that appear in >= 2 different CSD strings.
/// Returns a map: pattern -> [(coeff_index, position), ...].
fn find_cross_patterns(csd_list: &[String]) -> HashMap<String, Vec<(usize, usize)>> {
    let mut patterns: HashMap<String, Vec<(usize, usize)>> = HashMap::new();
    for (ci, csd) in csd_list.iter().enumerate() {
        let bytes = csd.as_bytes();
        let n = bytes.len();
        for i in 0..n {
            // Reusable buffer per start position, extended incrementally
            let mut sub = String::with_capacity(n - i);
            let mut nnz = 0u32;
            for (j, &c) in bytes.iter().enumerate().skip(i) {
                sub.push(c as char);
                if c == b'+' || c == b'-' {
                    nnz += 1;
                }
                if j - i + 1 >= 2 && nnz >= 2 {
                    patterns.entry(sub.clone()).or_default().push((ci, i));
                }
            }
        }
    }
    // Keep only patterns crossing >= 2 different CSD strings
    patterns.retain(|_, occ| {
        let unique: HashSet<usize> = occ.iter().map(|(ci, _)| *ci).collect();
        unique.len() >= 2
    });
    patterns
}

/// Generate Verilog code for a single CSD multiplier module (no cross-CSE).
///
/// Converts a Canonical Signed Digit (CSD) string into a synthesizable
/// Verilog module that performs constant multiplication using shifts and
/// additions/subtractions:
///
/// $$ y = \sum_{i=0}^{m} d_i \cdot (x \ll i), \quad d_i \in \{-1,0,+1\} $$
///
/// where $d_i$ is the CSD digit at position $i$, $x$ is the input, and
/// $m$ is the highest power. When the CSD string contains a repeated
/// non-overlapping pattern, LCSRe optimization shares hardware via a
/// `_pat` wire.
///
/// # Arguments
///
/// * `csd_str` - CSD string using '+', '-', '0' (e.g. "+00-00+0+")
/// * `input_width` - Bit width of the input signal x
/// * `max_power` - Highest power of two in the CSD (must be csd_str.len() - 1)
///
/// # Errors
///
/// Returns `CsdMultiplierError` if csd_str length doesn't match max_power+1
/// or if the string contains characters other than '+', '-', '0'.
///
/// # Examples
///
/// ```
/// use csd::csd_multiplier::generate_csd_multiplier;
///
/// let v = generate_csd_multiplier("+0-", 8, 2).unwrap();
/// assert!(v.contains("module csd_multiplier"));
/// assert!(v.contains("assign result = x_shift2 - x_shift0"));
/// ```
pub fn generate_csd_multiplier(
    csd_str: &str,
    input_width: usize,
    max_power: usize,
) -> Result<String, CsdMultiplierError> {
    // --- validation ---
    let len = csd_str.len();
    if len != max_power + 1 {
        return Err(CsdMultiplierError::LengthMismatch);
    }
    for &c in csd_str.as_bytes() {
        if c != b'+' && c != b'-' && c != b'0' {
            return Err(CsdMultiplierError::InvalidCharacter);
        }
    }

    let terms = parse_terms(csd_str, max_power)?;
    let ow = output_width(input_width, max_power);

    let mut verilog = String::new();

    // --- module header ---
    writeln!(verilog).unwrap();
    writeln!(verilog, "module csd_multiplier (").unwrap();
    writeln!(
        verilog,
        "    input signed [{}:0] x,      // Input value",
        input_width - 1
    )
    .unwrap();
    writeln!(
        verilog,
        "    output signed [{}:0] result // Result of multiplication",
        ow - 1
    )
    .unwrap();
    writeln!(verilog, ");").unwrap();

    // --- wire declarations (deduplicated powers) ---
    if !terms.is_empty() {
        writeln!(verilog).unwrap();
        writeln!(verilog, "    // Create shifted versions of input").unwrap();
        let mut powers_needed: BTreeSet<usize> = BTreeSet::new();
        // Reverse order: highest power first
        for (p, _) in &terms {
            powers_needed.insert(*p);
        }
        for p in powers_needed.into_iter().rev() {
            writeln!(
                verilog,
                "    wire signed [{}:0] x_shift{} = x <<< {};",
                ow - 1,
                p,
                p
            )
            .unwrap();
        }
    }

    // --- detect LCSRe optimization opportunity ---
    let repeated = longest_repeated_substring(csd_str);

    let pat_positions: Vec<usize> = if repeated.len() > 1 {
        let pat_nnz = count_nnz(&repeated);
        if pat_nnz >= 2 {
            let pos = find_pattern_occurrences(csd_str, &repeated);
            if pos.len() >= 2 {
                pos
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        }
    } else {
        Vec::new()
    };

    let use_opt = !pat_positions.is_empty();

    // --- combinational logic ---
    if terms.is_empty() {
        writeln!(verilog).unwrap();
        writeln!(verilog, "    // CSD implementation").unwrap();
        writeln!(verilog, "    assign result = 0;").unwrap();
    } else if use_opt {
        // LCSRe-optimized path
        let base_pos = pat_positions[0];
        let pat_expr = build_range_expr(csd_str, base_pos, repeated.len(), max_power);
        writeln!(verilog).unwrap();
        writeln!(verilog, "    // LCSRe: repeated pattern \"{}\"", repeated).unwrap();
        writeln!(
            verilog,
            "    wire signed [{}:0] _pat = {};",
            ow - 1,
            pat_expr
        )
        .unwrap();

        let mut expr = String::new();
        let mut cur = 0;
        for &pos in &pat_positions {
            // prefix/gap before this occurrence
            if pos > cur {
                let gap = build_range_expr(csd_str, cur, pos - cur, max_power);
                if !gap.is_empty() {
                    if expr.is_empty() {
                        expr = gap;
                    } else {
                        write!(expr, " + {}", gap).unwrap();
                    }
                }
            }
            // pattern occurrence
            let shift = pos as isize - base_pos as isize;
            let pat_ref = if shift == 0 {
                "_pat".to_string()
            } else {
                format!("(_pat >>> {})", shift)
            };
            if expr.is_empty() {
                expr = pat_ref;
            } else {
                write!(expr, " + {}", pat_ref).unwrap();
            }
            cur = pos + repeated.len();
        }
        // suffix
        if cur < csd_str.len() {
            let suffix = build_range_expr(csd_str, cur, csd_str.len() - cur, max_power);
            if !suffix.is_empty() {
                write!(expr, " + {}", suffix).unwrap();
            }
        }

        writeln!(verilog).unwrap();
        writeln!(verilog, "    // CSD implementation (LCSRe optimized)").unwrap();
        writeln!(verilog, "    assign result = {};", expr).unwrap();
    } else {
        // flat path (no repeated pattern)
        writeln!(verilog).unwrap();
        writeln!(verilog, "    // CSD implementation").unwrap();
        let mut expr = String::new();
        for (i, (power, op)) in terms.iter().enumerate() {
            if i == 0 {
                if *op == TermOp::Sub {
                    write!(expr, "-").unwrap();
                }
                write!(expr, "x_shift{}", power).unwrap();
            } else {
                match op {
                    TermOp::Add => write!(expr, " + x_shift{}", power).unwrap(),
                    TermOp::Sub => write!(expr, " - x_shift{}", power).unwrap(),
                }
            }
        }
        writeln!(verilog, "    assign result = {};", expr).unwrap();
    }

    writeln!(verilog, "endmodule").unwrap();
    Ok(verilog)
}

/// Generate Verilog for multiple CSD multipliers with cross-CSE.
///
/// When the same CSD substring appears in multiple coefficients, a shared
/// sub-expression wire is created — reducing total adder count across the
/// entire filter.
///
/// For each coefficient $k$:
///
/// $$ y_k = \sum_{i=0}^{m} d_{k,i} \cdot (x \ll i), \quad d_{k,i} \in \{-1,0,+1\} $$
///
/// All coefficients **must** share the same `input_width` and `max_power`
/// so that the same bit position encodes the same power of two.
///
/// # Arguments
///
/// * `coeffs` - List of coefficient specifications
/// * `module_name` - Name for the generated Verilog module
///
/// # Errors
///
/// Returns `CsdMultiplierError::EmptyCoefficients` if the list is empty.
/// Returns `CsdMultiplierError::WidthMismatch` if coefficient widths differ.
///
/// # Examples
///
/// ```
/// use csd::csd_multiplier::{generate_csd_multipliers, MultiplierSpec};
///
/// let coeffs = vec![
///     MultiplierSpec {
///         name: "y0".to_string(),
///         csd: "+00-00+0+".to_string(),
///         input_width: 8,
///         max_power: 8,
///     },
///     MultiplierSpec {
///         name: "y1".to_string(),
///         csd: "+00-00+0+".to_string(),
///         input_width: 8,
///         max_power: 8,
///     },
/// ];
/// let v = generate_csd_multipliers(&coeffs, "csd_filter").unwrap();
/// assert!(v.contains("module csd_filter"));
/// ```
pub fn generate_csd_multipliers(
    coeffs: &[MultiplierSpec],
    module_name: &str,
) -> Result<String, CsdMultiplierError> {
    if coeffs.is_empty() {
        return Err(CsdMultiplierError::EmptyCoefficients);
    }

    // Validation and uniform-width enforcement
    let input_width = coeffs[0].input_width;
    let max_power = coeffs[0].max_power;

    for spec in coeffs {
        if spec.input_width != input_width || spec.max_power != max_power {
            return Err(CsdMultiplierError::WidthMismatch);
        }
        let len = spec.csd.len();
        if len != max_power + 1 {
            return Err(CsdMultiplierError::LengthMismatch);
        }
        for c in spec.csd.chars() {
            if c != '+' && c != '-' && c != '0' {
                return Err(CsdMultiplierError::InvalidCharacter);
            }
        }
    }

    let ow = output_width(input_width, max_power);

    // Collect all x_shift powers
    let mut all_powers: BTreeSet<usize> = BTreeSet::new();
    for spec in coeffs {
        for (i, c) in spec.csd.char_indices() {
            if c != '0' {
                all_powers.insert(max_power - i);
            }
        }
    }

    // Find best cross-CSD pattern
    let csd_strings: Vec<String> = coeffs.iter().map(|s| s.csd.clone()).collect();
    let cross = find_cross_patterns(&csd_strings);

    let mut best_pattern = String::new();
    let mut best_occurrences: Vec<(usize, usize)> = Vec::new();
    let mut best_score = 0;

    for (pat, occ) in &cross {
        let nnz = count_nnz(pat);
        let score = (nnz.saturating_sub(1)) * (occ.len().saturating_sub(1));
        if score > best_score {
            best_score = score;
            best_pattern.clone_from(pat);
            best_occurrences.clone_from(occ);
        }
    }

    // Base position for the CSE wire
    let cse_base_pos = if best_pattern.is_empty() {
        0
    } else {
        best_occurrences
            .iter()
            .map(|(_, pos)| *pos)
            .min()
            .unwrap_or(0)
    };

    // Build the Verilog module
    let mut verilog = String::new();
    writeln!(verilog).unwrap();
    writeln!(verilog, "module {} (", module_name).unwrap();
    writeln!(
        verilog,
        "    input signed [{}:0] x,      // Input value",
        input_width - 1
    )
    .unwrap();
    for spec in coeffs {
        let ow_spec = output_width(spec.input_width, spec.max_power);
        writeln!(
            verilog,
            "    output signed [{}:0] {}",
            ow_spec - 1,
            spec.name
        )
        .unwrap();
    }
    writeln!(verilog, ");").unwrap();

    // x_shift wires
    if !all_powers.is_empty() {
        writeln!(verilog).unwrap();
        writeln!(verilog, "    // Create shifted versions of input").unwrap();
        for p in all_powers.iter().rev() {
            writeln!(
                verilog,
                "    wire signed [{}:0] x_shift{} = x <<< {};",
                ow - 1,
                p,
                p
            )
            .unwrap();
        }
    }

    // Shared CSE wire
    let cse_name = "_cse_0";
    if !best_pattern.is_empty() {
        let cse_expr = build_range_expr(
            &best_pattern,
            0,
            best_pattern.len(),
            max_power.saturating_sub(cse_base_pos),
        );
        writeln!(verilog).unwrap();
        writeln!(
            verilog,
            "    // Cross-CSE: shared pattern \"{}\"",
            best_pattern
        )
        .unwrap();
        writeln!(
            verilog,
            "    wire signed [{}:0] {} = {};",
            ow - 1,
            cse_name,
            cse_expr
        )
        .unwrap();
    }

    // Set of coeff indices that have the pattern
    let cse_coeffs: HashSet<usize> = best_occurrences.iter().map(|(ci, _)| *ci).collect();

    // Per-coefficient assignments
    for (idx, spec) in coeffs.iter().enumerate() {
        writeln!(verilog).unwrap();
        writeln!(verilog, "    // {}: {}", spec.name, spec.csd).unwrap();

        let has_cse = !best_pattern.is_empty() && cse_coeffs.contains(&idx);
        let expr = if has_cse {
            build_coeff_expr(&spec.csd, max_power, &best_pattern, cse_base_pos, cse_name)
        } else {
            build_coeff_expr(&spec.csd, max_power, "", 0, "")
        };

        if expr.is_empty() {
            writeln!(verilog, "    assign {} = 0;", spec.name).unwrap();
        } else {
            writeln!(verilog, "    assign {} = {};", spec.name, expr).unwrap();
        }
    }

    writeln!(verilog, "endmodule").unwrap();
    Ok(verilog)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // ---- Existing struct-based tests ----

    #[test]
    fn test_valid_csd() {
        let csd = "+00-00+0+";
        let multiplier = CsdMultiplier::new(csd, 8, 8).unwrap();
        assert_eq!(multiplier.decimal_value(), 229);
    }

    #[test]
    fn test_decimal_value() {
        let multiplier = CsdMultiplier::new("+", 8, 0).unwrap();
        assert_eq!(multiplier.decimal_value(), 1);

        let multiplier = CsdMultiplier::new("-", 8, 0).unwrap();
        assert_eq!(multiplier.decimal_value(), -1);

        let multiplier = CsdMultiplier::new("+0-", 8, 2).unwrap();
        assert_eq!(multiplier.decimal_value(), 3);

        let multiplier = CsdMultiplier::new("-0+", 8, 2).unwrap();
        assert_eq!(multiplier.decimal_value(), -3);
    }

    #[test]
    fn test_all_zeros_csd() {
        let csd = "0000";
        let multiplier = CsdMultiplier::new(csd, 8, 3).unwrap();
        let verilog = multiplier.generate_verilog();
        assert!(verilog.contains("assign result = 0;"));
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

    // ---- Free-function tests (matching C++ test_csd_multiplier.cpp) ----

    // Basic structural tests
    #[test]
    fn test_fn_basic_valid() {
        let v = generate_csd_multiplier("+0-", 8, 2).unwrap();
        assert!(v.contains("module csd_multiplier"));
        assert!(v.contains("endmodule"));
        assert!(v.contains("input signed [7:0] x"));
        assert!(v.contains("output signed [9:0] result"));
        assert!(v.contains("assign result = x_shift2 - x_shift0"));
    }

    #[test]
    fn test_fn_positive_only() {
        let v = generate_csd_multiplier("+0+", 4, 2).unwrap();
        assert!(v.contains("assign result = x_shift2 + x_shift0"));
    }

    #[test]
    fn test_fn_negative_only() {
        let v = generate_csd_multiplier("-0-", 8, 2).unwrap();
        assert!(v.contains("assign result = -x_shift2 - x_shift0"));
    }

    #[test]
    fn test_fn_all_zeros() {
        let v = generate_csd_multiplier("000", 8, 2).unwrap();
        assert!(v.contains("assign result = 0;"));
        assert!(!v.contains("x_shift"));
    }

    #[test]
    fn test_fn_single_nonzero() {
        let v = generate_csd_multiplier("+00", 8, 2).unwrap();
        assert!(v.contains("assign result"));
        assert!(v.contains("x_shift2"));
    }

    #[test]
    fn test_fn_invalid_chars() {
        let r = generate_csd_multiplier("123", 8, 2);
        assert_eq!(r, Err(CsdMultiplierError::InvalidCharacter));
    }

    #[test]
    fn test_fn_invalid_length() {
        let r = generate_csd_multiplier("+0-", 8, 3);
        assert_eq!(r, Err(CsdMultiplierError::LengthMismatch));
    }

    // LCSRe optimization tests
    #[test]
    fn test_fn_flat_when_pattern_nnz_is_1() {
        // "+00-00+0" has no repeated pattern with ≥2 nnz
        let v = generate_csd_multiplier("+00-00+0", 8, 7).unwrap();
        assert!(!v.contains("_pat"));
        assert!(v.contains("x_shift7 - x_shift4 + x_shift1"));
    }

    #[test]
    fn test_fn_double_repeat_optimization() {
        // +0-0+0-0: repeated "+0-0" (2 nnz) at positions 0 and 4
        let v = generate_csd_multiplier("+0-0+0-0", 8, 7).unwrap();
        assert!(v.contains("_pat"));
        assert!(v.contains("_pat = x_shift7 - x_shift5"));
        assert!(v.contains("(_pat >>> 4)"));
        assert!(v.contains("LCSRe"));
    }

    #[test]
    fn test_fn_triple_repeat_optimization() {
        // +0-0+0-0+0-0: repeated "+0-0" at positions 0, 4, 8
        let v = generate_csd_multiplier("+0-0+0-0+0-0", 8, 11).unwrap();
        assert!(v.contains("_pat"));
        assert!(v.contains("(_pat >>> 4)"));
        assert!(v.contains("(_pat >>> 8)"));
    }

    #[test]
    fn test_fn_longer_pattern_repeat() {
        // +00-00+00-00: repeated "+00-00" (2 nnz, 5 chars) at positions 0 and 6
        let v = generate_csd_multiplier("+00-00+00-00", 8, 11).unwrap();
        assert!(v.contains("_pat"));
        assert!(v.contains("_pat = x_shift11 - x_shift8"));
        assert!(v.contains("(_pat >>> 6)"));
    }

    #[test]
    fn test_fn_leading_minus_no_optimization() {
        // CSD starting with '-' and no repeated pattern
        let v = generate_csd_multiplier("-0-", 8, 2).unwrap();
        assert!(!v.contains("_pat"));
        assert!(v.contains("-x_shift2 - x_shift0"));
    }

    #[test]
    fn test_fn_pattern_with_leading_minus() {
        // Repeated pattern starting with '-': -0+0-0+0
        let v = generate_csd_multiplier("-0+0-0+0", 8, 7).unwrap();
        assert!(v.contains("_pat"));
        assert!(v.contains("_pat = -x_shift7 + x_shift5"));
        assert!(v.contains("(_pat >>> 4)"));
    }

    #[test]
    fn test_fn_no_optimization_for_single_occurrence() {
        // CSD with unique pattern throughout — no repeat = flat
        let v = generate_csd_multiplier("+0-+00-0", 8, 7).unwrap();
        assert!(!v.contains("_pat"));
    }

    #[test]
    fn test_fn_pat_wire_width_matches_output() {
        // output_width = 8 + 7 = 15, so wire signed [14:0]
        let v = generate_csd_multiplier("+0-0+0-0", 8, 7).unwrap();
        assert!(v.contains("[14:0] _pat"));
    }

    #[test]
    fn test_fn_repeat_with_trailing_gap() {
        // Repeated pattern followed by non-repeating suffix
        let v = generate_csd_multiplier("+0-0+0-0+0", 8, 9).unwrap();
        assert!(v.contains("_pat"));
        assert!(v.contains("(_pat >>> 4)"));
    }

    // Edge cases
    #[test]
    fn test_fn_very_short_csd() {
        // Length-1 CSD
        let v = generate_csd_multiplier("+", 8, 0).unwrap();
        assert!(v.contains("assign result = x_shift0"));
    }

    #[test]
    fn test_fn_all_minus_signs() {
        let v = generate_csd_multiplier("---", 8, 2).unwrap();
        assert!(!v.contains("_pat"));
    }

    #[test]
    fn test_fn_always_has_proper_module_boundaries() {
        let v = generate_csd_multiplier("+0-0+0-0", 8, 7).unwrap();
        assert!(v.contains("\nmodule csd_multiplier"));
        assert!(v.contains("endmodule\n"));
    }

    #[test]
    fn test_fn_lcsre_comment_present_when_optimized() {
        let v = generate_csd_multiplier("+0-0+0-0", 8, 7).unwrap();
        assert!(v.contains("LCSRe"));
    }

    #[test]
    fn test_fn_no_lcsre_comment_when_flat() {
        let v = generate_csd_multiplier("+00-00+0", 8, 7).unwrap();
        assert!(!v.contains("LCSRe"));
    }

    // ---- Multi-coefficient tests ----

    #[test]
    fn test_multi_empty_coeffs() {
        let r = generate_csd_multipliers(&[], "test");
        assert_eq!(r, Err(CsdMultiplierError::EmptyCoefficients));
    }

    #[test]
    fn test_multi_single_coeff() {
        let coeffs = vec![MultiplierSpec {
            name: "y0".to_string(),
            csd: "+0-".to_string(),
            input_width: 8,
            max_power: 2,
        }];
        let v = generate_csd_multipliers(&coeffs, "test_mod").unwrap();
        assert!(v.contains("module test_mod"));
        assert!(v.contains("output signed [9:0] y0"));
    }

    #[test]
    fn test_multi_duplicate_coeffs() {
        let coeffs = vec![
            MultiplierSpec {
                name: "y0".to_string(),
                csd: "+00-00+0+".to_string(),
                input_width: 8,
                max_power: 8,
            },
            MultiplierSpec {
                name: "y1".to_string(),
                csd: "+00-00+0+".to_string(),
                input_width: 8,
                max_power: 8,
            },
        ];
        let v = generate_csd_multipliers(&coeffs, "csd_filter").unwrap();
        assert!(v.contains("Cross-CSE"));
        assert!(v.contains("_cse_0"));
    }

    #[test]
    fn test_multi_width_mismatch() {
        let coeffs = vec![
            MultiplierSpec {
                name: "y0".to_string(),
                csd: "+0-".to_string(),
                input_width: 8,
                max_power: 2,
            },
            MultiplierSpec {
                name: "y1".to_string(),
                csd: "+0-".to_string(),
                input_width: 16,
                max_power: 2,
            },
        ];
        let r = generate_csd_multipliers(&coeffs, "test");
        assert_eq!(r, Err(CsdMultiplierError::WidthMismatch));
    }

    #[test]
    fn test_multi_invalid_chars() {
        let coeffs = vec![MultiplierSpec {
            name: "y0".to_string(),
            csd: "123".to_string(),
            input_width: 8,
            max_power: 2,
        }];
        let r = generate_csd_multipliers(&coeffs, "test");
        assert_eq!(r, Err(CsdMultiplierError::InvalidCharacter));
    }
}
