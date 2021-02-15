# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.1.0] - 2021-02-15

### Added

- Support for "{}" imports, courtesy of @corollari via [pull request #7](https://github.com/paulrberg/multisol/pull/7)

## [1.0.2] - 2021-01-10

### Added

- Dataset that shows when name conflicts occur

### Fixed

- Case when two or more contracts share the same name (reported by @corollari via [issue #8](https://github.com/paulrberg/multisol/issues/8))

## [1.0.1] - 2020-12-28

### Added

- Console log for successful execution

### Fixed

- Clippy warnings in the "multisol" and "multisol-writer" crates
- The name of the CLI application when running `--help`

## [1.0.0] - 2020-12-27

### Added

- First release of the tool

[1.1.0]: https://github.com/paulrberg/multisol/compare/v1.0.2...v1.1.0
[1.0.2]: https://github.com/paulrberg/multisol/compare/v1.0.1...v1.0.2
[1.0.1]: https://github.com/paulrberg/multisol/compare/v1.0.0...v1.0.1
[1.0.0]: https://github.com/paulrberg/multisol/releases/tag/v1.0.0
