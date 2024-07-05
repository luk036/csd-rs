# 🔄 csd-rs

[![Crates.io](https://img.shields.io/crates/v/csd-rs.svg)](https://crates.io/crates/csd-rs)
[![Docs.rs](https://docs.rs/csd-rs/badge.svg)](https://docs.rs/csd-rs)
[![CI](https://github.com/luk036/csd-rs/workflows/CI/badge.svg)](https://github.com/luk036/csd-rs/actions)
[![codecov](https://codecov.io/gh/luk036/csd-rs/branch/main/graph/badge.svg?token=tkfuYMvwrl)](https://codecov.io/gh/luk036/csd-rs)

Canonical Signed Digit (CSD) is a type of signed-digit representation of numbers. In CSD, each digit can only be -1, 0, or 1, and no two consecutive digits can be non-zero. This representation has the advantage of being unique and having a minimal number of non-zero digits. CSD is often used in digital signal processing applications, such as filter design, because it allows for efficient implementation of arithmetic operations using simple adders and subtractors. The number of adders/subtracters required to realize a CSD coefficient is one less than the number of nonzero digits in the code

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
