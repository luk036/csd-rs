/// Find the longest repeating non-overlapping substring in cstr
///
/// The `longest_repeated_substring` function takes a null-terminated string and its length as input and
/// returns the longest repeated non-overlapping substring in the string.
///
/// Arguments:
///
/// * `sv`: A reference to a character array representing the input string. It is assumed that the
/// string is null-terminated.
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
    let ndim = sv.len() + 1;
    let mut lcsre = vec![vec![0usize; ndim]; ndim];

    let mut res_length = 0; // To store length of result

    // Building table in bottom-up manner
    let mut index = 0; // To store the starting index of the result substring
    for i in 1..ndim {
        for j in i + 1..ndim {
            // (j-i) > lcsre[i-1][j-1] to avoid overlapping
            if sv.chars().nth(i - 1) == sv.chars().nth(j - 1) && lcsre[i - 1][j - 1] < (j - i) {
                lcsre[i][j] = lcsre[i - 1][j - 1] + 1;

                // Updating maximum length of the substring and the starting index
                if lcsre[i][j] > res_length {
                    res_length = lcsre[i][j];
                    index = i;
                }
            } else {
                lcsre[i][j] = 0;
            }
        }
    }

    // Constructing the result substring if there's a non-empty result

    if res_length > 0 {
        sv[index - res_length..index].to_string()
    } else {
        "".to_string()
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
