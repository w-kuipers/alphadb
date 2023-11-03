import json
import os

import pytest

from alphadb import AlphaDBSQLite
from alphadb.utils.exceptions import NeedsConfirmation

db = AlphaDBSQLite()
db.connect("test.db")


class TestSQLite:
    #### Status method
    def test_sqlite_status(self):
        status = db.status()
        assert "name" in status and "version" in status and "init" in status and "template" in status

    #### Init method
    def test_sqlite_init(self):
        init = db.init()
        assert init == True
        init = db.init()
        assert init == "already-initialized"

    #### Update method
    def test_sqlite_update(self):
        with open("../tests/assets/test-db-structure.json") as f:
            structure = json.loads(f.read())

        assert db.update(version_source=structure) == True

    #### Vacate method
    def test_mysql_vacate(self):
        with pytest.raises(NeedsConfirmation):
            db.vacate()

        assert db.vacate(confirm=True) == True

    #### After all tests, remove test db
    def __del__(self):
        if os.path.exists(("test.db")):
            os.remove("test.db")
