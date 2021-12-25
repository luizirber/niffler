# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [2.4.0] - 2021-12-25

### Added

- Zstd basic support (#44)
- GitHub issues templates (#47)

### Changed

- Bump MSRV to 1.51 (#48)

## [2.3.2] - 2021-05-27

### Changed

- Bump bgzip version to 0.2.0, removing dependency on failure

## [2.3.1] - 2020-12-08

### Fixed

- Maintain backward compatibility by re-exporting compression::Level

## [2.3.0] - 2020-12-08

### Added

- Support Seek and Send and prepare for future extension (#37)

## [2.2.0] - 2020-07-01

### Changed

- Make gz compression optional too, matching behavior of other compression formats (#29)

## [2.1.1] - 2020-06-30

### Changed

- Replace `GzDecoder` with `MultiGzDecoder` for gzip decompression (#28)

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

[unreleased]: https://github.com/luizirber/niffler/compare/v2.4.0...HEAD
[2.4.0]: https://github.com/luizirber/niffler/compare/v2.3.2..v2.4.0
[2.3.2]: https://github.com/luizirber/niffler/compare/v2.3.1..v2.3.2
[2.3.1]: https://github.com/luizirber/niffler/compare/v2.3.0..v2.3.1
[2.3.0]: https://github.com/luizirber/niffler/compare/v2.2.0..v2.3.0
[2.2.0]: https://github.com/luizirber/niffler/compare/v2.0.1..v2.2.0
[2.0.1]: https://github.com/luizirber/niffler/compare/v2.0.0..v2.0.1
[2.0.0]: https://github.com/luizirber/niffler/compare/v1.0.0..v2.0.0
[1.0.0]: https://github.com/luizirber/niffler/releases/tag/v1.0.0
