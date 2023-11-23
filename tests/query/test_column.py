import pytest

from src.alphadb.utils.query.column.definecolumn import definecolumn


#### ValueError should be raised when column type is not supported
def test_column_type():
    with pytest.raises(ValueError):
        definecolumn(column_type="test", column_name="test")


#### Test if the function returns the right query
def test_query():
    assert definecolumn(column_name="test", column_type="VARCHAR") == " `test` VARCHAR NOT NULL"
    assert definecolumn(column_name="test", column_type="VARCHAR", null=True) == " `test` VARCHAR NULL"
    assert definecolumn(column_name="test", column_type="VARCHAR", length=100) == " `test` VARCHAR(100) NOT NULL"
    assert definecolumn(column_name="test", column_type="VARCHAR", unique=True) == " `test` VARCHAR NOT NULL UNIQUE"
    assert definecolumn(column_name="test", column_type="VARCHAR", default="fiets") == " `test` VARCHAR NOT NULL DEFAULT 'fiets'"
    assert definecolumn(column_name="test", column_type="VARCHAR", auto_increment=True) == " `test` VARCHAR NOT NULL AUTO_INCREMENT"
    assert (
        definecolumn(
            column_name="test",
            column_type="VARCHAR",
            engine="sqlite",
            auto_increment=True,
        )
        == " `test` VARCHAR NOT NULL"
    )
