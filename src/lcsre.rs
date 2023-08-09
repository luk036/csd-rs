/// Find the longest repeating non-overlapping substring in cstr
///
/// The `longest_repeated_substring` function takes a null-terminated string and its length as input and
/// returns the longest repeated non-overlapping substring in the string.
///
/// Arguments:
///
/// * `cstr`: A reference to a character array representing the input string. It is assumed that the
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
/// let chars: Vec<_> = "+-00+-00+-00+-0".to_string().chars().collect();
/// assert_eq!(longest_repeated_substring(&chars), "+-00+-0".to_string());
/// let chars: Vec<_> = "abcdefgh".to_string().chars().collect();
/// assert_eq!(longest_repeated_substring(&chars), "".to_string());
/// ```
#[allow(dead_code)]
pub fn longest_repeated_substring(cstr: &[char]) -> String {
    let n = cstr.len();
    let mut lcsre = vec![vec![0; n + 1]; n + 1];

    let mut res = "".to_string(); // To store result
    let mut res_length = 0; // To store length of result

    // building table in bottom-up manner
    let mut index = 0;
    for i in 1..(n + 1) {
        for j in (i + 1)..(n + 1) {
            // (j-i) > lcsre[i-1][j-1] to remove
            // overlapping
            if cstr[i - 1] == cstr[j - 1] && lcsre[i - 1][j - 1] < (j - i) {
                lcsre[i][j] = lcsre[i - 1][j - 1] + 1;

                // updating maximum length of the
                // substring and updating the finishing
                // index of the suffix
                if lcsre[i][j] > res_length {
                    res_length = lcsre[i][j];
                    index = std::cmp::max(i, index);
                }
            } else {
                lcsre[i][j] = 0;
            }
        }
    }

    // If we have non-empty result, then insert
    // all characters from first character to
    // last character of string
    if res_length > 0 {
        for i in (index - res_length + 1)..(index + 1) {
            res.push(cstr[i - 1]);
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lcsre() {
        let cstr = "+-00+-00+-00+-0".to_string();
        let res = longest_repeated_substring(&cstr.chars().collect::<Vec<char>>());
        assert_eq!(res, "+-00+-0".to_string());
        let cstr = "abcdefgh".to_string();
        let res = longest_repeated_substring(&cstr.chars().collect::<Vec<char>>());
        assert_eq!(res, "".to_string());
    }
}
