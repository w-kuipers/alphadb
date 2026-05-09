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

## Installation

### Install using `Cargo`

    cargo install alphadb-cli

## Usage

Connect to a database

```bash
alphadb connect
```

You will be asked to provide the database credentials. After connecting the connection will be saved for later use.

Make sure the database is empty, back it up if necessary. If the database is not empty, you can use the `vacate` method.
Note that this function will erase ALL data in the database and this action is irriversible.

```bash
alphadb vacate
```

The database is now ready to be initialized. The `init` command will create the `adb_conf` table. This holds configuration data for the database.

```bash
alphadb init
```

Now we update the database. For this we need to give it a structure. Read more about [version sources](https://alphadb.w-kuipers.com/version-source).

```json
{
  "name": "mydb",
  "engine": "postgres",
  "version": [
    {
      "_id": "0.1.0",
      "createtable": {
        "customers": {
          "primary_key": "id",
          "name": {
            "type": "VARCHAR",
            "length": 100
          },
          "id": {
            "type": "INTEGER",
            "generated": "BY DEFAULT"
          }
        }
      }
    },
    {
      "_id": "1.0.0",
      "createtable": {
        "orders": {
          "primary_key": "id",
          "id": {
            "type": "INTEGER",
            "generated": "BY DEFAULT"
          },
          "date": {
            "type": "TIMESTAMP"
          },
          "note": {
            "type": "TEXT",
            "null": true
          }
        }
      }
    }
  ]
}
```

Then run the update command.

```bash
alphadb update
```

You will be asked to select a version source. This can be a path to a JSON file or a URL returning JSON data.

## License

[GPL-3.0 LICENSE](https://github.com/w-kuipers/alphadb/blob/main/LICENSE)
