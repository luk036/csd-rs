/// Find the longest repeating non-overlapping substring in cstr
///
/// The `longest_repeated_substring` function takes a null-terminated string and its length as input and
/// returns the longest repeated non-overlapping substring in the string.
///
/// Arguments:
///
/// * `sv`: A reference to a character array representing the input string. It is assumed that the
///         string is null-terminated.
///
/// Returns:
///
/// The function `longest_repeated_substring` returns a string, which is the longest repeated substring
/// in the given input string `cstr`.
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
#[allow(dead_code)]
pub fn longest_repeated_substring(sv: &str) -> String {
    let ndim = sv.len() + 1;  // Dimension for the DP table (n+1 x n+1)
    let mut lcsre = vec![vec![0usize; ndim]; ndim];  // DP table initialized with zeros

    let mut res_length = 0; // To store length of the longest found substring

    // Building table in bottom-up manner
    let mut index = 0; // To store the starting index of the result substring
    for i in 1..ndim {
        for j in i + 1..ndim {
            // Check if characters match and the substring wouldn't overlap
            // (j-i) > lcsre[i-1][j-1] ensures non-overlapping condition
            if sv.chars().nth(i - 1) == sv.chars().nth(j - 1) && lcsre[i - 1][j - 1] < (j - i) {
                lcsre[i][j] = lcsre[i - 1][j - 1] + 1;  // Extend the length of the common substring

                // Update maximum length and starting index if we found a longer substring
                if lcsre[i][j] > res_length {
                    res_length = lcsre[i][j];
                    index = i;  // Store the ending index of the substring
                }
            } else {
                lcsre[i][j] = 0;  // Reset length if characters don't match
            }
        }
    }

    // Constructing the result substring if there's a non-empty result
    if res_length > 0 {
        // Extract substring from (index - length) to index
        sv[index - res_length..index].to_string()
    } else {
        "".to_string()  // Return empty string if no repeated substring found
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
    }
}
