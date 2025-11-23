# iFlow 项目上下文文档 - csd-rs

## 项目概览

**项目名称**: csd-rs  
**类型**: Rust 库  
**描述**: 一个用于在十进制数和规范符号数字（Canonical Signed Digit, CSD）表示法之间进行转换的 Rust 库。CSD 是一种特殊的数字表示方法，每个数字位只能是 -1、0 或 +1（分别用 '-', '0', '+' 表示），且不允许连续的非零位，特别适用于数字信号处理等领域。

## 核心功能

1. `to_csd(decimal_value: f64, places: i32) -> String`: 将十进制浮点数转换为 CSD 字符串表示，支持指定小数位数。
2. `to_csd_i(decimal_value: i32) -> String`: 将整数转换为 CSD 字符串表示（无小数点）。
3. `to_decimal(csd: &str) -> f64`: 将 CSD 字符串转换回十进制浮点数。
4. `to_decimal_i(csd: &str) -> i32`: 将 CSD 字符串转换回十进制整数。
5. `to_csdnnz(decimal_value: f64, nnz: u32) -> String`: 将十进制数转换为 CSD 表示，限制非零位的数量。
6. `to_csdnnz_i(decimal_value: i32, nnz: u32) -> String`: 整数版本的 CSD 转换，限制非零位的数量。
7. `longest_repeated_substring` (来自 lcsre 模块): 寻找最长重复子串（该函数的具体用途可能与 CSD 转换有间接关系）。

## 项目结构

```
src/
├── csd.rs      # CSD 转换的核心算法实现
├── csd_multiplier.rs
├── lcsre.rs    # 最长重复子串算法
└── lib.rs      # 库的公共接口导出
```

## 构建和运行

- **构建项目**: `cargo build`
- **运行测试**: `cargo test`
- **运行基准测试**: `cargo bench`
- **安装到本地**: `cargo install csd-rs`

## 依赖

- `argparse = "0.2.2"`: 命令行参数解析
- `log = "0.4.28"`: 日志记录
- 开发依赖: `quickcheck`, `criterion`, `pprof`

## 开发约定

- 代码遵循 Rust 2021 版本规范
- 使用 `clippy` 进行代码风格检查（通过注释控制特定规则）
- 通过 `#[cfg(test)]` 模块包含单元测试
- 使用 `quickcheck` 进行属性测试
- 提供详细的文档注释和示例代码

## 许可证

双许可证：MIT 或 Apache-2.0