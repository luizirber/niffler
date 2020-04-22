<meta charset="utf-8"/>

# niffler

Simple and transparent support for compressed files.

This library provides two main features:
- sniffs out compression formats from input files and return a
Read trait object ready for consumption.
- Create a Writer initialized with compression ready for writing.

The goal is to lower the barrier to open and use a file, especially in
bioinformatics workflows.

[![build-status]][github-actions]
[![Crates.io](https://img.shields.io/crates/v/niffler.svg)](https://crates.io/crates/niffler)
[![Documentation](https://docs.rs/niffler/badge.svg)](https://docs.rs/niffler/)

[build-status]: https://github.com/luizirber/niffler/workflows/CI/badge.svg
[github-actions]: https://github.com/luizirber/niffler/actions?query=workflow%3ACI

## License

Licensed under either of these:

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
 * MIT License ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

### Contributing

Unless you explicitly state otherwise, any contribution you intentionally submit for inclusion in the work, as defined
in the Apache-2.0 license, shall be dual-licensed as above, without any additional terms or conditions.
