import pytest

from src.alphadb.mysql import AlphaDBMysql
from src.alphadb.utils.decorators import conn_test
from src.alphadb.utils.exceptions import NoConnection


#### Test if a database is connected
class TestDatabaseConnectionDecorator:
    db = AlphaDBMysql()

    #### The function inside the decorator should not execute when no database connection exists. NoConnection exeption should be raised.
    def test_not_connected(self):
        @conn_test
        def conn_test_test(db):
            return True

        with pytest.raises(NoConnection):
            conn_test_test(self.db)

    #### The function inside the decorator should be executed when a database connection exists
    def test_connected(self):
        #### Connect to a database
        self.db.connect(host="localhost", user="root", password="fmm", database="fmm")

        @conn_test
        def conn_test_test(db):
            return True

        assert conn_test_test(self.db) == True
