import json

import pytest

from alphadb import AlphaDBPostgreSQL
from alphadb.utils.exceptions import NeedsConfirmation

db = AlphaDBPostgreSQL()
db.connect(host="localhost", user="root", password="test", database="test")


# #### Status method
# def test_mysql_status():
#     status = db.status()
#     assert "name" in status and "version" in status and "init" in status and "template" in status
#
#
# #### Init method
# def test_mysql_init():
#     init = db.init()
#     assert init == True
#     init = db.init()
#     assert init == "already-initialized"
#
#
# #### Update method
# def test_mysql_update():
#     with open("../tests/assets/test-db-structure.json") as f:
#         structure = json.loads(f.read())
#
#     assert db.update(version_source=structure) == True
#
#
# #### Vacate method
# def test_mysql_vacate():
#     #### Confirm not specified
#     with pytest.raises(NeedsConfirmation):
#         db.vacate()
#
#     assert db.vacate(confirm=True) == True
