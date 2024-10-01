# 🔄 csd-rs

[![Crates.io](https://img.shields.io/crates/v/csd-rs.svg)](https://crates.io/crates/csd-rs)
[![Docs.rs](https://docs.rs/csd-rs/badge.svg)](https://docs.rs/csd-rs)
[![CI](https://github.com/luk036/csd-rs/workflows/CI/badge.svg)](https://github.com/luk036/csd-rs/actions)
[![codecov](https://codecov.io/gh/luk036/csd-rs/branch/main/graph/badge.svg?token=tkfuYMvwrl)](https://codecov.io/gh/luk036/csd-rs)

Canonical Signed Digit (CSD) is a type of signed-digit representation of numbers. In CSD, each digit can only be -1, 0, or 1, and no two consecutive digits can be non-zero. This representation has the advantage of being unique and having a minimal number of non-zero digits. CSD is often used in digital signal processing applications, such as filter design, because it allows for efficient implementation of arithmetic operations using simple adders and subtractors. The number of adders/subtracters required to realize a CSD coefficient is one less than the number of nonzero digits in the library.

This library is all about converting numbers between decimal format and a special representation called Canonical Signed Digit (CSD). CSD is a way of writing numbers using only three symbols: 0, +, and -. It's particularly useful in certain areas of computer science and digital signal processing.

The main purpose of this library is to provide functions that can convert decimal numbers to CSD format and vice versa. It takes in regular decimal numbers (like 28.5 or -0.5) and converts them to CSD strings (like "+00-00.+" or "0.-"), and it can also do the reverse, taking CSD strings and converting them back to decimal numbers.

The library contains several functions, each with a specific role:

1. to_csd: This function takes a decimal number and the number of decimal places desired, and outputs a CSD string. For example, it can convert 28.5 to "+00-00.+0" (with 2 decimal places).

2. to_csd_i: Similar to to_csd, but it works specifically with integers. It converts whole numbers to CSD format without a decimal point.

3. to_decimal_i and to_decimal: These functions do the opposite of to_csd. They take a CSD string and convert it back to a decimal number.

4. to_csdnnz: This function is a variation of to_csd that allows you to specify the maximum number of non-zero digits in the result.

The library fulfills its intended function through a sequence of mathematical operations and logical tests. In order to effect a conversion from decimal to CSD, the system employs the use of powers of 2 in order to ascertain which of the three symbols (+, -, or 0) is to be used at each position within the CSD string. The algorithm then performs repeated divisions of the input number by two and compares the result to specific thresholds to determine the appropriate symbol to use.

In order to perform the conversion from CSD to decimal, the algorithm proceeds by multiplying the running total by 2 and then adding, subtracting, or performing no further action based on the value of the symbol in the CSD string. This is done for each symbol in the string, where the symbol values are +, -, or 0. A distinct logic is employed for the integral and fractional parts, respectively.

Furthermore, the library incorporates error-checking mechanisms to guarantee the exclusive utilisation of valid CSD symbols. It also furnishes comprehensive documentation and illustrative examples for each function, thus facilitating user comprehension of the operational procedures.

In conclusion, this library offers a comprehensive set of tools for working with CSD representations, facilitating the conversion between decimal and CSD formats in a variety of ways.

## 🛠️ Installation

### 📦 Cargo

- Install the rust toolchain in order to have cargo installed by following
  [this](https://www.rust-lang.org/tools/install) guide.
- run `cargo install csd-rs`

## 👀 See also

- [csdigit](https://luk036.github.io/csdigit)
- [csd-cpp](https://luk036.github.io/csd-cpp)

## 📜 License

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## 🤝 Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

See [CONTRIBUTING.md](CONTRIBUTING.md).
