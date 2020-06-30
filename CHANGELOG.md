# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [2.2.0] - 2020-06-30

### Changed

- Make gz compression optional too, matching behavior of other compression formats.

## [2.1.1] - 2020-06-30

### Changed

- Replace `GzDecoder` with `MultiGzDecoder` for gzip decompression.

## [2.1.0] - 2020-06-06

### Added

- `sniff` function to detect compression format, and add benchmarks for most methods (#25)
- More test on compression level conversion (#26)

## [2.0.1] - 2020-05-20

### Added

- Clone and Copy traits to pub enums [@schmidmt](https://github.com/schmidmt). 

## [2.0.0] - 2020-04-27

### Added

- New functions: `get_reader`, `get_writer`, `from_path`, `to_path`,
- Compression format and compression level enums
- Documentation with examples
- CI using GitHub Actions

### Changed

- Replace failure with thiserror for error handling

### Fixed

- Reorganize crate internal organization

### Removed

- All previous functions and enums were renamed and reorganized.

## [1.0.0] - 2019-12-07

### Added

- Rename crate from `ocf` to `niffler`
- Import codebase from sourmash repo (which copied it from the yacrd repo)

[unreleased]: https://github.com/luizirber/niffler/compare/v2.0.1...HEAD
[2.0.1]: https://github.com/luizirber/niffler/compare/v2.0.0..v2.0.1
[2.0.0]: https://github.com/luizirber/niffler/compare/v1.0.0..v2.0.0
[1.0.0]: https://github.com/luizirber/niffler/releases/tag/v1.0.0
