# Change Log

All user visible changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](http://semver.org/), as described
for Rust libraries in [RFC #1105](https://github.com/rust-lang/rfcs/blob/master/text/1105-api-evolution.md)

## Unreleased

### Added

- Support `pub` and `pub(crate)` serializers from macro.

### Changed

N/A

### Removed

- Removed `ToJson` trait. Its could be merged with `Serializer`.

### Fixed

N/A

## [0.1.1] - 2018-10-29

Had to bump the version to make the readme show up on [crates.io](https://crates.io/crates/serializers).

## 0.1.0 - 2018-10-29

Initial release.

[0.1.1]: https://github.com/davidpdrsn/robin/compare/v0.1.0...v0.1.1
