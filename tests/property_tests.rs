use csd::{to_csd_i, to_decimal_i};

#[test]
fn roundtrip_small_ints() {
    for n in -100..=100 {
        let csd = to_csd_i(n);
        let result = to_decimal_i(&csd);
        assert_eq!(n, result, "roundtrip failed for {}", n);
    }
}

#[test]
fn zero_int() {
    let csd = to_csd_i(0);
    assert_eq!(csd, "0");
    let result = to_decimal_i(&csd);
    assert_eq!(0, result);
}

#[test]
fn powers_of_two() {
    for i in 0..=10i32 {
        let n = 1 << i;
        let csd = to_csd_i(n);
        let result = to_decimal_i(&csd);
        assert_eq!(n, result, "roundtrip failed for 2^{}", i);
    }
}

#[test]
fn no_consecutive_nonzeros_small() {
    let test_vals = [1, 2, 3, 5, 7, 9, 10, 15, 31];
    for n in test_vals {
        let csd = to_csd_i(n);
        let chars: Vec<char> = csd.chars().collect();
        for window in chars.windows(2) {
            if window[0] != '0' && window[1] != '0' {
                panic!("consecutive non-zeros in {}: {}", n, csd);
            }
        }
    }
}
