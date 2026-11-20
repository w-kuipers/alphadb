# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

Only releases with user-facing changes are listed. A release that is not listed
contains packaging or CI changes only; the occasional fix it carries is included
in the next listed release. `dev` builds are not listed.

## [Unreleased]

## [1.0.0-beta.58] - 2026-11-20

### Added

- `droptable` version source method. A version can drop a table and create it again.
- Verification of `droptable`.

### Removed

- **BREAKING**: Default data. The `default_data` version source method, the `no_data` parameter of `update` and `update_queries`, and the `--no-data` CLI flag are gone. Seeding the database is up to the user.

## [1.0.0-beta.57] - 2026-11-19

### Added

- CLI: `connect` asks which database engine to use.

### Changed

- The Python bindings require Python 3.10 or newer.

### Fixed

- Length of decimal columns was built incorrectly.
- `modifycolumn` stripped the column length by always redefining the type.

## [1.0.0-beta.55] - 2026-10-25

### Added

- `update` runs its queries in a transaction. MySQL still commits implicitly on every DDL statement.
- Release workflow for Homebrew and the AUR.

### Changed

- The internal config table is defined by its own version source and migrates itself during `update`.

## [1.0.0-beta.54] - 2026-10-23

### Fixed

- An `altertable` block that only changes indexes no longer emits an empty `ALTER TABLE` query.
- The `version-source` feature was not enabled in the Cargo release.

## [1.0.0-beta.53] - 2026-10-09

### Added

- Python and Node bindings accept a path to a version source file or directory, as well as JSON contents.

### Changed

- Bumped dependencies.

## [1.0.0-beta.51] - 2026-10-08

### Added

- Check constraints in `altertable` (`add_check`, `drop_check`).

### Fixed

- Column renames were converted to uppercase.

## [1.0.0-beta.50] - 2026-06-29

### Added

- `version-source` feature: a version source can be split over multiple files in a directory.
- CLI: file based version sources can be selected.
- Foreign keys can be dropped and modified in `altertable`.
- Primary keys can be changed in `altertable`.

### Changed

- **BREAKING**: Removed the `verify` flag from the `update` function signature.
- Versions are sorted by version number before they are processed, so their order in the version source no longer matters.
- Errors carry a version trace more consistently.

### Fixed

- Version number validation replaced dots with commas.

## [1.0.0-beta.49] - 2026-05-10

### Added

- PostgreSQL extensions through the `createextension`, `alterextension` and `dropextension` version source methods.
- The Python bindings work with both engines and are published as `alphadb-mysql` and `alphadb-postgres`.
- The Node bindings are built per engine.

## [1.0.0-beta.43] - 2026-05-07

### Added

- Check constraints in `createtable`, for MySQL and PostgreSQL.
- Verification of boolean default values.

### Fixed

- A boolean column default could not be converted to a string.

## [1.0.0-beta.42] - 2026-04-14

### Added

- Primary keys spanning multiple columns.

### Changed

- Database engines are plugged in through a runtime configuration.

## [1.0.0-beta.41] - 2026-04-08

### Added

- Verification checks that a column type is supported by the engine.

## [1.0.0-beta.40] - 2026-04-06

### Changed

- **BREAKING**: The core and the engines moved into a single crate. Engines are selected with the `mysql` and `postgres` features.
- Verification is driven by a per-engine configuration.

## [1.0.0-beta.39] - 2026-03-25

### Added

- PostgreSQL engine, including `UUID` and `BIGSERIAL` columns.
- Indexes, with an optional `unique` flag.
- CLI: connects using the engine of the saved connection.

### Changed

- **BREAKING**: Foreign keys are defined as an array instead of a single object.
- The order of keys in the version source is preserved.

## [1.0.0-beta.38] - 2025-10-19

### Added

- Verification of default data: unknown columns, missing values for `NOT NULL` columns, value types and duplicates in unique columns.
- `AlphaDB` is exported in the prelude.
- `VersionTrace`, which points to the place in the version source an issue or error refers to.

### Changed

- **BREAKING**: `update` parameter `update_to_version` renamed to `target_version`.
- The engine independent core is separated from the MySQL engine.

### Fixed

- Column compatibility checks in `createtable` verification.
- Version number parsing check.

## [1.0.0-beta.37] - 2025-05-09

### Added

- `consolidate` includes default data.

## [1.0.0-beta.36] - 2025-04-30

### Added

- `consolidate`: merge all versions of a version source into a single version. Available in the library and as a CLI command.

## [1.0.0-beta.35] - 2025-04-16

### Added

- CLI: `verify` command.
- Verification of primary keys.
- Verification issues carry a version trace.
- Node: the postinstall script builds from source when the prebuilt binary can not be downloaded.

## [1.0.0-beta.34] - 2025-04-11

### Added

- MySQL keywords and functions such as `CURRENT_TIMESTAMP` and `NOW()` can be used as a column default.

### Changed

- Release binaries are built on Ubuntu 22.04.

## [1.0.0-beta.31] - 2025-01-22

### Added

- `is_connected` is exposed in the Python and Node bindings.

## [1.0.0-beta.30] - 2025-01-04

### Changed

- CLI: the version number is taken from the build.

### Fixed

- CLI: `connect` failed when no saved connections existed.

## [1.0.0-beta.29] - 2025-01-02

### Added

- Default data: rows to insert on `update`, defined in the version source under `default_data`.

### Fixed

- Argument types in the Node bindings.

## [1.0.0-beta.28] - 2024-12-31

### Changed

- Error messages include a version trace.
- Better errors for invalid column definitions.

## [1.0.0-beta.26] - 2024-12-29

### Added

- Node: a postinstall script that downloads a prebuilt binary.

### Changed

- Functions that could panic return an error instead.
- CLI: more descriptive errors.

### Fixed

- Integer values defined as a string in the version source.
- Float values are allowed as the length of column types that support them.
- `connect` argument types in the Python and Node bindings.

## [1.0.0-beta.20] - 2024-12-22

AlphaDB is rewritten in Rust.

### Added

- Rust library, published on crates.io.
- CLI rewritten in Rust, with the `connect`, `init`, `status`, `update` and `vacate` commands.

### Changed

- The Python and Node packages are bindings on top of the Rust library.
- **BREAKING**: CLI `update` flag `nodata` renamed to `no-data`.

### Removed

- `confirm` parameter from the `vacate` method. It remains in the CLI.

## [1.0.0-beta.1] to [1.0.0-beta.19] - 2024-01-08 to 2024-02-14

### Added

- Command line interface.
- NodeJS API, published on npm with TypeScript types.
- macOS builds.
- `dropcolumn` checks whether the column is the primary key.
- The version source is verified when `update` is called. Can be disabled.
- Packages are released on PyPI, npm and GitHub.

### Changed

- **BREAKING**: CLI `update` flag `verify` changed to `no-verify`.

## [1.0.0-beta.0] - 2023-12-31

### Added

- A function that returns the queries `update` would execute.

### Removed

- PostgreSQL and SQLite support. MySQL is the only supported engine.
- `AlphaDBMysql` class.

## [1.0.0-alpha.1] - 2023-11-16

### Added

- `altertable`: `addcolumn`, `modifycolumn`, `renamecolumn` and dropping the primary key.
- Version source verification through the `VersionSourceVerification` class.
- `VersionSourceValueError` exception.

### Changed

- `AUTO_INCREMENT` is removed from a column when its primary key is dropped.

## [1.0.0-alpha.0] - 2023-11-03

### Added

- `altertable`: `dropcolumn`.
- Checks for incompatible column types and attributes.
- `update` checks that the config table exists.

### Changed

- **BREAKING**: `update` method parameter `version_information` changed to `version_source`.
- **BREAKING**: Config table renamed from `fdb_cfg` to `adb_conf`.
- The latest version does not have to be specified in the version source anymore. It is looked up in the version list.

## [0.1.0-alpha.0] - 2023-10-29

### Added

- Initial open-source release, written in Python.

[Unreleased]: https://github.com/w-kuipers/alphadb/compare/v1.0.0-beta.58...HEAD
[1.0.0-beta.58]: https://github.com/w-kuipers/alphadb/compare/v1.0.0-beta.57...v1.0.0-beta.58
[1.0.0-beta.57]: https://github.com/w-kuipers/alphadb/compare/v1.0.0-beta.55...v1.0.0-beta.57
[1.0.0-beta.55]: https://github.com/w-kuipers/alphadb/compare/v1.0.0-beta.54...v1.0.0-beta.55
[1.0.0-beta.54]: https://github.com/w-kuipers/alphadb/compare/v1.0.0-beta.53...v1.0.0-beta.54
[1.0.0-beta.53]: https://github.com/w-kuipers/alphadb/compare/v1.0.0-beta.51...v1.0.0-beta.53
[1.0.0-beta.51]: https://github.com/w-kuipers/alphadb/compare/v1.0.0-beta.50...v1.0.0-beta.51
[1.0.0-beta.50]: https://github.com/w-kuipers/alphadb/compare/v1.0.0-beta.49...v1.0.0-beta.50
[1.0.0-beta.49]: https://github.com/w-kuipers/alphadb/compare/v1.0.0-beta.43...v1.0.0-beta.49
[1.0.0-beta.43]: https://github.com/w-kuipers/alphadb/compare/v1.0.0-beta.42...v1.0.0-beta.43
[1.0.0-beta.42]: https://github.com/w-kuipers/alphadb/compare/v1.0.0-beta.41...v1.0.0-beta.42
[1.0.0-beta.41]: https://github.com/w-kuipers/alphadb/compare/v1.0.0-beta.40...v1.0.0-beta.41
[1.0.0-beta.40]: https://github.com/w-kuipers/alphadb/compare/v1.0.0-beta.39...v1.0.0-beta.40
[1.0.0-beta.39]: https://github.com/w-kuipers/alphadb/compare/v1.0.0-beta.38...v1.0.0-beta.39
[1.0.0-beta.38]: https://github.com/w-kuipers/alphadb/compare/v1.0.0-beta.37...v1.0.0-beta.38
[1.0.0-beta.37]: https://github.com/w-kuipers/alphadb/compare/v1.0.0-beta.36...v1.0.0-beta.37
[1.0.0-beta.36]: https://github.com/w-kuipers/alphadb/compare/v1.0.0-beta.35...v1.0.0-beta.36
[1.0.0-beta.35]: https://github.com/w-kuipers/alphadb/compare/v1.0.0-beta.34...v1.0.0-beta.35
[1.0.0-beta.34]: https://github.com/w-kuipers/alphadb/compare/v1.0.0-beta.31...v1.0.0-beta.34
[1.0.0-beta.31]: https://github.com/w-kuipers/alphadb/compare/v1.0.0-beta.30...v1.0.0-beta.31
[1.0.0-beta.30]: https://github.com/w-kuipers/alphadb/compare/v1.0.0-beta.29...v1.0.0-beta.30
[1.0.0-beta.29]: https://github.com/w-kuipers/alphadb/compare/v1.0.0-beta.28...v1.0.0-beta.29
[1.0.0-beta.28]: https://github.com/w-kuipers/alphadb/compare/v1.0.0-beta.26...v1.0.0-beta.28
[1.0.0-beta.26]: https://github.com/w-kuipers/alphadb/compare/v1.0.0-beta.20...v1.0.0-beta.26
[1.0.0-beta.20]: https://github.com/w-kuipers/alphadb/compare/v1.0.0b19...v1.0.0-beta.20
[1.0.0-beta.19]: https://github.com/w-kuipers/alphadb/compare/v1.0.0-beta.0...v1.0.0b19
[1.0.0-beta.1]: https://github.com/w-kuipers/alphadb/releases/tag/v1.0.0b1
[1.0.0-beta.0]: https://github.com/w-kuipers/alphadb/compare/v1.0.0-alpha.1...v1.0.0-beta.0
[1.0.0-alpha.1]: https://github.com/w-kuipers/alphadb/compare/v1.0.0-alpha.0...v1.0.0-alpha.1
[1.0.0-alpha.0]: https://github.com/w-kuipers/alphadb/compare/v0.1.0-alpha.0...v1.0.0-alpha.0
[0.1.0-alpha.0]: https://github.com/w-kuipers/alphadb/releases/tag/v0.1.0-alpha.0
