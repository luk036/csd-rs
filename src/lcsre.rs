// Rust program to find the longest repeated
// non-overlapping substring

// Returns the longest repeating non-overlapping
// substring in cstr
pub fn longest_repeated_substring(cstr: &[char]) -> String {
    let n = cstr.len();
    let mut lcsre = vec![vec![0; n + 1]; n + 1];

    let mut res = String::from("");  // To store result
    let mut res_length = 0;  // To store length of result

    // building table in bottom-up manner
    let mut index = 0;
    for i in 1..(n + 1) {
        for j in (i + 1)..(n + 1) {

            // (j-i) > lcsre[i-1][j-1] to remove
            // overlapping
            if cstr[i - 1] == cstr[j - 1] &&
                    lcsre[i - 1][j - 1] < (j - i) {
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

// Driver Code
fn main() {
    let cstr = String::from("+-00+-00+-00+-0");
    println!("{}", longest_repeated_substring(&cstr.chars().collect::<Vec<char>>()));
}

// This code is contributed by ita_c
