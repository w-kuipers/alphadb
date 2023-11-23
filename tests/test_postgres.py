import json

import pytest

from alphadb import AlphaDBPostgreSQL
from alphadb.utils.exceptions import NeedsConfirmation

db = AlphaDBPostgreSQL()
db.connect(host="localhost", user="root", password="test", database="test")

#### Init method
def test_postgres_init():
    init = db.init()
    assert init == True
    init = db.init()
    assert init == "already-initialized"

#### Status method
def test_postgres_status():
    status = db.status()
    assert status == {'init': True, 'version': '0.0.0', 'name': 'test', 'template': None}
#
# #### Update method
# def test_postgres_update():
#     with open("../tests/assets/test-db-structure.json") as f:
#         structure = json.loads(f.read())
#
#     assert db.update(version_source=structure) == True
#
#
#### Vacate method
def test_postgres_vacate():
    #### Confirm not specified
    with pytest.raises(NeedsConfirmation):
        db.vacate()

    assert db.vacate(confirm=True) == True
