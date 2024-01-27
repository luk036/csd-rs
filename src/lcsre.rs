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
/// let cs = "+-00+-00+-00+-0";
/// assert_eq!(longest_repeated_substring(&cs), "+-00+-0");
/// let cs = "abcdefgh";
/// assert_eq!(longest_repeated_substring(&cs), "");
/// ```
#[allow(dead_code)]
pub fn longest_repeated_substring(cs: &str) -> String {
    let ndim = cs.len() + 1;
    let mut lcsr = vec![vec![0; ndim]; ndim];

    let mut res = String::new();
    let mut res_length = 0;
    let mut start = 0;

    // Build the LCSR table in bottom-up manner
    for i in 1..ndim {
        for j in i..ndim {
            if cs.as_bytes()[i - 1] == cs.as_bytes()[j - 1]
                && lcsr[i - 1][j - 1] < (j as isize - i as isize)
            {
                lcsr[i][j] = lcsr[i - 1][j - 1] + 1;

                if lcsr[i][j] > res_length {
                    res_length = lcsr[i][j];
                    start = std::cmp::min(i, start);
                }
            } else {
                lcsr[i][j] = 0;
            }
        }
    }

    // If we have a non-empty result, return the substring
    if res_length > 0 {
        let slice = &cs[start..start + res_length as usize];
        res = String::from(slice);
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lcsre() {
        let cstr = "+-00+-00+-00+-0";
        let res = longest_repeated_substring(&cstr);
        assert_eq!(res, "+-00+-0");
        let cstr = "abcdefgh";
        let res = longest_repeated_substring(&cstr);
        assert_eq!(res, "");
    }
}
