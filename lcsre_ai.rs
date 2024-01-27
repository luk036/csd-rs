use std::mem;

fn longest_repeated_substring(cs: &str) -> String {
    let len = cs.len();
    let lcsr = vec![vec![0; len + 1]; len + 1];

    let mut res = String::new();
    let mut res_length = 0;
    let mut start = 0;

    // Build the LCSR table in bottom-up manner
    for i in 1..len + 1 {
        for j in i..len + 1 {
            if cs.as_bytes()[i - 1] == cs.as_bytes()[j - 1]
                && lcsr[i - 1][j - 1] < (j as isize - i as isize) {
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
        let slice = &cs[start as usize..start as usize + res_length];
        res = String::from(slice);
    }

    res
}

fn main() {
    let cs = "+-00+-00+-00+-0";
    println!("{}", longest_repeated_substring(&cs));
}

/*
This Rust code creates a 2D vector `lcsr` of size `(len + 1) x (len + 1)`, where `len` is the length of the given string. It then implements the logic described in the Python code using this vector and returns the longest repeated non-overlapping substring as a Rust `String`.

Keep in mind that, for larger inputs, you may want to consider using a more memory-efficient data structure like a vector of vectors or a dynamic array. The current implementation is just to show a possible one-to-one translation from the Python code to Rust while keeping the same logic and input/output format.
*/
