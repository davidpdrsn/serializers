# Change Log

All user visible changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](http://semver.org/), as described
for Rust libraries in [RFC #1105](https://github.com/rust-lang/rfcs/blob/master/text/1105-api-evolution.md)

## Unreleased

### Added

N/A

### Changed

N/A

### Removed

N/A

### Fixed

N/A

## [0.2.0] - 2018-11-11

### Changed

- Move the generic `'a` lifetime on `ToJson` from the trait itself to the method that actually needs it. This might be a breaking change for users who directly use the `ToJson` trait, but it should be quick fix.
- Create a struct from `serializer!` macro rather than a function. While generating a function did work I think this approach is simpler and more inline with what users would expect.

### Removed

- Since the generic lifetime on `ToJson` has been removed, the whole trait can actually be removed. All its methods has been moved to `Serializer`.
- Auto implementation for functions with special signature. It is not longer needed since the macro now generates structs.

## [0.1.2] - 2018-10-30

### Added

- Support `pub` and `pub(crate)` serializers from macro.

### Changed

- Change syntax for `serializer!` macro to better match normal generics.

## [0.1.1] - 2018-10-29

Had to bump the version to make the readme show up on [crates.io](https://crates.io/crates/serializers).

## 0.1.0 - 2018-10-29

Initial release.

[0.2.0]: https://github.com/davidpdrsn/serializers/compare/v0.1.2...v0.2.0
[0.1.2]: https://github.com/davidpdrsn/serializers/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/davidpdrsn/serializers/compare/v0.1.0...v0.1.1
