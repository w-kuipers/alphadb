import os

import pytest
from alphadb import AlphaDB

engine = os.environ.get("ALPHADB_ENGINE", "mysql")

engines = {
    "mysql": {
        "connect": {
            "host": "localhost",
            "user": "root",
            "password": "test",
            "database": "adb_test1",
            "port": 333,
        },
        "structure": "test-mysql-db-structure.json",
    },
    "postgres": {
        "connect": {
            "host": "localhost",
            "user": "postgres",
            "password": "test",
            "database": "adb_test1",
            "port": 544,
        },
        "structure": "test-postgres-db-structure.json",
    },
}

if engine not in engines:
    raise ValueError(f"Unsupported ALPHADB_ENGINE '{engine}'")

config = engines[engine]
db = AlphaDB()


def test_connect():
    assert not db.is_connected
    db.connect(**config["connect"])
    assert db.is_connected


def test_init():
    db.init()

    with pytest.raises(RuntimeError, match="The database is already initialized"):
        db.init()


def test_status():
    status = db.status()
    assert status == {
        "init": True,
        "version": "0.0.0",
        "name": config["connect"]["database"],
        "template": None,
    }


def test_update():
    with open(f"../../assets/{config['structure']}") as f:
        structure = f.read()

    db.update(version_source=structure)

    status = db.status()
    assert status == {
        "init": True,
        "version": "0.2.6",
        "name": config["connect"]["database"],
        "template": "test",
    }


def test_vacate():
    db.vacate()

    status = db.status()
    assert status == {
        "init": False,
        "version": None,
        "name": config["connect"]["database"],
        "template": None,
    }
