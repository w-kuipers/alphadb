from alphadb import AlphaDBMySQL

db = AlphaDBMySQL()
db.connect(host="localhost", user="root", password="fmm", database="fmm")


#### Status method
def test_mysql_status():
    status = db.status()
    assert "name" in status and "version" in status and "init" in status and "template" in status
