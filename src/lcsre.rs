/// Find the longest repeating non-overlapping substring in a string.
///
/// $$ \text{LCSRe}\[i\]\[j\] = \begin{cases} \text{LCSRe}\[i-1\]\[j-1\] + 1 & \text{if } s_i = s_j \land j-i > \text{LCSRe}\[i-1\]\[j-1\] \\\\ 0 & \text{otherwise} \end{cases} $$
///
/// Uses dynamic programming with O(n) space (flat 2-row table) instead of O(n²),
/// matching the optimized C++ implementation.
///
/// Arguments:
///
/// * `sv`: A reference to a string slice representing the input string.
///
/// Returns:
///
/// The function `longest_repeated_substring` returns a string, which is the longest repeated substring
/// in the given input string `sv`. Returns an empty string if no repeated substring is found.
///
/// # Examples
///
/// ```
/// use csd::lcsre::longest_repeated_substring;
///
/// let cs = "+-00+-00+-00+-0";
/// assert_eq!(longest_repeated_substring(&cs), "+-00+-0");
/// let cs = "abcdefgh";
/// assert_eq!(longest_repeated_substring(&cs), "");
/// ```
///
/// # Complexity
///
/// Time complexity: O(n²) where n is the length of the input string
/// Space complexity: O(n) — flat vector with 2 rows
pub fn longest_repeated_substring(sv: &str) -> String {
    let chars: Vec<char> = sv.chars().collect();
    let n = chars.len();
    let ndim = n + 1; // dimension = n+1
                      // Flat vector: 2 rows of ndim columns — O(n) space
    let mut lcsre = vec![0usize; 2 * ndim];

    let mut res_length = 0; // length of the longest found substring
    let mut index = 0; // ending index of the result substring (1-based)

    for i in 1..ndim {
        let cur_row = (i % 2) * ndim;
        let prev_row = ((i - 1) % 2) * ndim;
        for j in i + 1..ndim {
            // (j - i) > lcsre[i-1][j-1] ensures non-overlapping
            if chars[i - 1] == chars[j - 1] && lcsre[prev_row + j - 1] < (j - i) {
                lcsre[cur_row + j] = lcsre[prev_row + j - 1] + 1;

                if lcsre[cur_row + j] > res_length {
                    res_length = lcsre[cur_row + j];
                    index = i;
                }
            } else {
                lcsre[cur_row + j] = 0;
            }
        }
    }

    if res_length > 0 {
        sv[index - res_length..index].to_string()
    } else {
        String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lcsre() {
        let cstr = "+-00+-00+-00+-0";
        let res = longest_repeated_substring(cstr);
        assert_eq!(res, "+-00+-0");
        let cstr = "abcdefgh";
        let res = longest_repeated_substring(cstr);
        assert_eq!(res, "");
        let cstr = "banana";
        let res = longest_repeated_substring(cstr);
        assert_eq!(res, "an");
        let cstr = "";
        let res = longest_repeated_substring(cstr);
        assert_eq!(res, "");
        let cstr = "aaaaa";
        let res = longest_repeated_substring(cstr);
        assert_eq!(res, "aa");
        let cstr = "ababa";
        let res = longest_repeated_substring(cstr);
        assert_eq!(res, "ab");
    }
}
