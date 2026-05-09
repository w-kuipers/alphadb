![AlphaDB](https://github.com/w-kuipers/alphadb/blob/main/assets/alphadb-banner.png?raw=true)

[![GitHub releases](https://img.shields.io/github/v/release/w-kuipers/alphadb?include_prereleases)](https://github.com/w-kuipers/alphadb/releases)
[![Crates.io Version](https://img.shields.io/crates/v/alphadb)](https://crates.io/crates/alphadb)
[![PyPI release](https://img.shields.io/pypi/v/alphadb-mysql.svg)](https://pypi.org/project/alphadb-mysql/)
[![NPM release](https://img.shields.io/npm/v/%40w-kuipers%2Falphadb-postgres)](https://www.npmjs.com/package/@w-kuipers/alphadb-postgres)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![](https://img.shields.io/github/last-commit/w-kuipers/alphadb?label=last%20modified)](https://github.com/w-kuipers/alphadb)
[![Tests](https://github.com/w-kuipers/alphadb/actions/workflows/run-tests.yml/badge.svg)](https://github.com/w-kuipers/alphadb/actions/workflows/run-tests.yml)

# AlphaDB

AlphaDB is a SQL schema versioning tool that lets you define your database structure in a JSON format and handles applying migrations across multiple databases. It gives you consistency and control over schema evolution, whether you're working in development, staging, or production environments.

---

## Beta

AlphaDB is currently in `beta` stage. Breaking changes should be expected.

---

## Key Features

- **JSON-Based Database Schema**: Define your database structure in a clear, human-readable JSON format.
- **Easy Migration Management**: Apply, track, and roll back migrations seamlessly across multiple databases.
- **Version Control for Your Database**: Keep your database schema in sync with your application code.
- **Lightweight and Developer-Friendly**: Designed to integrate smoothly into your development workflow.

## Documentation

Visit the [official documentation](https://alphadb.w-kuipers.com/)

## Building Packages

Build the MySQL package from `packages/mysql`:

```bash
cd packages/mysql
maturin build
```

Build the PostgreSQL package from `packages/postgres`:

```bash
cd packages/postgres
maturin build
```

Do not use `maturin build --config pyproject.postgres.toml`; maturin's `--config` option is a key/value override and does not select an alternate pyproject file.
