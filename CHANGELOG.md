# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [unreleased]

### Added

- Better error handling.
- `Altertable` option to update method.
- Add version source validation
- NodeJS API
- Version source will be verified when the `update` method is called. Can be disabled.

### Changed

- **BREAKING**: `update` CLI method flag `nodata` changed to `no-data`
- **BREAKING**: `update` method parameter `version_information` changed to `version_source`.
- Latest version does not have to be specified in the version information anymore. Will be looked up in the version list.

### Removed

- `confirm` parameter from `vacate` method. Remains in CLI.

## [0.1.0-alpha.0] - 2023-10-30

### Initialized

- Initialized the project.

[unreleased]: https://github.com/w-kuipers/alphadb/compare/v0.1.0...HEAD
[0.1.0-alpha.0]: https://github.com/w-kuipers/alphadb/releases/tag/v0.1.0-alpha0
