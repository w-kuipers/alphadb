![AlphaDB](https://github.com/w-kuipers/alphadb/blob/main/assets/ALPHADB_Github-Social-Preview.png?raw=true)
[![GitHub releases](https://img.shields.io/github/v/release/w-kuipers/alphadb)](https://github.com/w-kuipers/alphadb/releases)
[![PyPI release](https://img.shields.io/pypi/v/alphadb.svg)](https://pypi.org/project/alphadb/)
[![NPM release](https://img.shields.io/npm/v/%40w-kuipers%2Falphadb)](https://www.npmjs.com/package/@w-kuipers/alphadb)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![](https://img.shields.io/github/last-commit/w-kuipers/alphadb?label=last%20modified)](https://github.com/w-kuipers/alphadb)
[![Tests](https://github.com/w-kuipers/alphadb/actions/workflows/run-tests.yml/badge.svg)](https://github.com/w-kuipers/alphadb/actions/workflows/run-tests.yml)

# AlphaDB

A toolset for MySQL database versioning.

## Still in alpha stage

AlphaDB is currently in `beta` stage. Breaking changes should be expected.

<details>
  <summary>Table of Contents</summary>
  <ol>
    <li><a href="#documentation">Documentation</a></li>
    <li>
      <a href="#installation">Installation</a>
      <ul>
        <li><a href="#install-using-pip">Install using PIP</a></li>
      </ul>
    </li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#exceptions">Exceptions</a></li>
    <li><a href="#license">License</a></li>
  </ol>
</details>

## Documentation

Visit the [official documentation](https://alphadb.w-kuipers.com/)

## Installation

### Install using `NPM`

    npm install @w-kuipers/alphadb


## Usage

Import AlphaDB
``` js
import AlphaDB from "@w-kuipers/alphadb";
```
Connect to a database.
``` js
const db = AlphaDB();
db.connect({
    host: "localhost",
    user: "user",
    password: "password",
    database: "database"
});
```
Make sure the database is empty, back it up if necessary. If the database is not empty, you can use the `vacate` method.
Note that this function will erase ALL data in the database and there is no way to get it back. For extra safety the argument `true` is required for the function to run.
``` js
db.vacate(true);
```
The database is now ready to be initialized. The `init` method will create the `adb_conf` table. This holds configuration data for the database.
``` js
db.init();
```
Now we update the database. For this we need to give it a structure. The database version information is a JSON structure formatted as such:
``` js
const database_version_source = {
    "name": "mydb", // Database name, does not have to, but is advised to match the actual database name
    "version": [ // List containing database versions
        {
            "_id": "0.1.0", // Database version
            "createtable": { // Object containing tables to be created,
                "customers": { // Object key will be used as table name
                    "primary_key": "id",
                    "name": { // Object key will be used as column name
                        "type": "VARCHAR", // Data type
                        "length": 100, // Date max length,
                    },
                    "id": {
                        "type": "INT",
                        "a_i": True
                    }
                },
            }
        },
        {
            "_id": "1.0.0",
            "createtable": {
                "orders": {
                    "primary_key": "id",
                    "id": {
                        "type": "INT",
                        "a_i": True
                    },
                    "date": {
                        "type": "DATETIME",
                    },
                    "note": {
                        "type": "TEXT",
                        "null": True
                    }
                }
            }
        }
    ]
}
```

Then call the `update` method.
``` js
db.update(database_version_source);
```

## License

[GPL-3.0 LICENSE](https://github.com/w-kuipers/alphadb/blob/main/LICENSE)
