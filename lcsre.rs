pub fn longest_repeated_substring(cs: &str) -> String {
    let n = cs.len();
    let mut lcs = vec![vec![0; n]; n];
    let mut result = String::new();
    let mut max_length = 0;
    let mut start = 0;

    for i in 1..n {
        for j in i+1..n {
            if cs[i-1] == cs[j-1] && lcs[i][j-1] < (j-i) {
                lcs[i][j] = lcs[i-1][j-1] + 1;
                if lcs[i][j] > max_length {
                    max_length = lcs[i][j];
                    start = std::cmp::max(i, start);
                }
            } else {
                lcs[i][j] = 0;
            }
        }
    }

    if max_length > 0 {
        let slice = &cs[start..start+max_length];
        result.push_str(slice);
    }

    result
}

fn main() {
    let cs = "+-00+-00+-00+-0";
    println!("{}", longest_repeated_substring(cs));
}
