import pytest

from alphadb import AlphaDB
from alphadb.utils.decorators import conn_test
from alphadb.utils.exceptions import NoConnection


#### Test if a database is connected
class TestDatabaseConnectionDecorator:
    db = AlphaDB()

    #### The function inside the decorator should not execute when no database connection exists. NoConnection exeption should be raised.
    def test_not_connected(self):
        @conn_test
        def conn_test_test(_):
            return True

        with pytest.raises(NoConnection):
            conn_test_test(self.db)

    #### The function inside the decorator should be executed when a database connection exists
    def test_connected(self):
        #### Connect to a database
        self.db.connect(host="localhost", user="root", password="test", database="test")

        @conn_test
        def conn_test_test(_):
            return True

        assert conn_test_test(self.db) == True
