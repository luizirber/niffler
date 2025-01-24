<meta charset="utf-8"/>

# niffler

Simple and transparent support for compressed files.

This library provides two main features:
- sniffs out compression formats from input files and returns a
Read trait object ready for consumption.
- Create a Writer initialized with compression ready for writing.

The goal is to lower the barrier to open and use a file, especially in
bioinformatics workflows.

[![build-status]][github-actions]
[![Crates.io](https://img.shields.io/crates/v/niffler.svg)](https://crates.io/crates/niffler)
[![Documentation](https://docs.rs/niffler/badge.svg)](https://docs.rs/niffler/)

[build-status]: https://github.com/luizirber/niffler/workflows/CI/badge.svg
[github-actions]: https://github.com/luizirber/niffler/actions?query=workflow%3ACI

## Selecting compression formats

By default all supported compression formats are enabled.
If you're working on systems that don't support them you can disable default
features and select the ones you want.
For example,
currently only `gz` is supported in Webassembly environments
(because `niffler` depends on crates that have system dependencies for `bz2` and `lzma` compression),
so you can use this in your `Cargo.toml` to select only the `gz` support:
```
niffler = { version = "2.2.0", default-features = false, features = ["gz"] }
```

You can still use `niffler::sniff()` to find what is the compression format,
even if any feature is disabled.
But if you try to use `niffler::get_reader` for a disabled feature,
it will throw a runtime error.

## Minimum supported Rust version

Currently the minimum supported Rust version is 1.74.0.

## Similar project

Many similar projects exist in other languages:

- C: [rbread](https://github.com/ocxtal/rbread)
- Python: [xphyle](https://github.com/jdidion/xphyle), [xopen](https://github.com/marcelm/xopen)
- Perl: [AnyUncompress](https://perldoc.perl.org/IO/Uncompress/AnyUncompress.html)
- go: [Archiver](https://github.com/mholt/archiver)

## Development

niffler development is open, and [pull requests](https://github.com/luizirber/niffler/pulls) are welcome!

Before creating your pull request, please try to write a test and benchmark (if possible).
Some commands we suggest running to help with these tasks:
```
cargo fmt
cargo test
cargo clippy
```

To run tests use:
```
cargo test --all-features
```

To test benchmark run:
```
cargo test --benches --all-features
```

To run all benchmark use:
```
cargo bench --all-features
```

As a shortcut, you can also run `make`,
which will execute all these commands.

## License

Licensed under either of these:

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
 * MIT License ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

### Contributing

Unless you explicitly state otherwise, any contribution you intentionally submit for inclusion in the work, as defined
in the Apache-2.0 license, shall be dual-licensed as above, without any additional terms or conditions.
