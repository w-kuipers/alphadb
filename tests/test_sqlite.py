from alphadb import AlphaDBSQLite

db = AlphaDBSQLite()
db.connect("test.db")


#### Status method
def test_sqlite_status():
    status = db.status()
    assert "name" in status and "version" in status and "init" in status and "template" in status
