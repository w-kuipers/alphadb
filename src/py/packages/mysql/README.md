![AlphaDB](https://github.com/w-kuipers/alphadb/blob/main/assets/alphadb-banner.png?raw=true)

[![GitHub releases](https://img.shields.io/github/v/release/w-kuipers/alphadb?include_prereleases)](https://github.com/w-kuipers/alphadb/releases)
[![PyPI release](https://img.shields.io/pypi/v/alphadb-mysql.svg)](https://pypi.org/project/alphadb-mysql/)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![](https://img.shields.io/github/last-commit/w-kuipers/alphadb?label=last%20modified)](https://github.com/w-kuipers/alphadb)
[![Tests](https://github.com/w-kuipers/alphadb/actions/workflows/run-tests.yml/badge.svg)](https://github.com/w-kuipers/alphadb/actions/workflows/run-tests.yml)

# AlphaDB MySQL

AlphaDB is a SQL schema versioning tool that lets you define your database structure in a JSON format and handles applying migrations across environments. This package contains the Python bindings for AlphaDB built specifically with the MySQL engine.

---

## Beta

AlphaDB is currently in `beta` stage. Breaking changes should be expected.

---

## Key Features

- **MySQL Engine Package**: Installs AlphaDB with MySQL support only.
- **JSON-Based Database Schema**: Define your database structure in a clear, human-readable JSON format.
- **Easy Migration Management**: Apply, track, and roll back migrations seamlessly across MySQL databases.
- **Version Control for Your Database**: Keep your database schema in sync with your application code.
- **Lightweight and Developer-Friendly**: Designed to integrate smoothly into your Python workflow.

## Installation

```bash
pip install alphadb-mysql
```

## Usage

```python
import alphadb

db = alphadb.AlphaDB()
db.connect("localhost", "root", "password", "database")
```

The default MySQL port is `3306`.

## Documentation

Visit the [official documentation](https://alphadb.w-kuipers.com/).
