import pytest
from alphadb import AlphaDB

db = AlphaDB()
db.connect(host="localhost", user="root", password="test", database="test")


def test_init():
    db.init()

    with pytest.raises(RuntimeError, match="The database is already initialized"):
        db.init()


def test_status():
    status = db.status()
    assert status == {
        "init": True,
        "version": "0.0.0",
        "name": "test",
        "template": None,
    }


def test_update():
    with open("../../assets/test-db-structure.json") as f:
        structure = f.read()

    db.update(version_source=structure)

    status = db.status()
    assert status == {
        "init": True,
        "version": "0.2.6",
        "name": "test",
        "template": "test",
    }


def test_vacate():
    db.vacate()

    status = db.status()
    assert status == {
        "init": False,
        "version": None,
        "name": "test",
        "template": None,
    }
