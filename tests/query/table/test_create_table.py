import pytest

from alphadb.utils.exceptions import IncompatibleColumnAttributes, IncompleteVersionObject
from alphadb.utils.query.table import create_table
from tests.test_utils.version_source import wrap_partial_in_createtable

def test_missing_column_type():
    with pytest.raises(IncompleteVersionObject):
        test_data = wrap_partial_in_createtable({
            "col": {"length": 10},
        })
        create_table(version_source=test_data, table_name="table", version="0.0.1")


def test_incompatible_column_attributes():
    with pytest.raises(IncompatibleColumnAttributes):
        test_data = wrap_partial_in_createtable({
            "col": {
                "type": "VARCHAR",
                "null": True,
                "a_i": True,
            },
        })
        create_table(version_source=test_data, table_name="table", version="0.0.1")


def test_incomplete_foreign_key_object():
    #### Missing key
    with pytest.raises(IncompleteVersionObject):
        test_data = wrap_partial_in_createtable({"col": {"foreign_key": {"references": "test"}}})
        create_table(version_source=test_data, table_name="table", version="0.0.1")

    #### Missing references
    with pytest.raises(IncompleteVersionObject):
        test_data = wrap_partial_in_createtable({"col": {"foreign_key": {"key": "test"}}})
        create_table(version_source=test_data, table_name="table", version="0.0.1")


def test_query():
    #### Test all column types (MYSQL)
    test_data = wrap_partial_in_createtable({
        "primary_key": "id",
        "id": {
            "type": "INT",
            "a_i": True,
        },
        "col1": {"type": "VARCHAR", "length": 30, "unique": True},
        "foreign_key": {
            "references": "other_table",
            "key": "key",
            "on_delete": "cascade",
        },
    })

    assert (
        create_table(table_name="table", version_source=test_data, version="0.0.1")
        == " CREATE TABLE `table` ( `id` INT NOT NULL AUTO_INCREMENT, `col1` VARCHAR(30) NOT NULL UNIQUE, PRIMARY KEY (`id`), FOREIGN KEY (key) REFERENCES other_table (key) ON DELETE CASCADE ) ENGINE = InnoDB;"
    )

    #### Test all column types (SQLite)

    assert (
        create_table(table_name="table", version_source=test_data, version="0.0.1", engine="sqlite")
        == " CREATE TABLE `table` ( `id` INT NOT NULL, `col1` VARCHAR(30) NOT NULL UNIQUE, PRIMARY KEY (`id`), FOREIGN KEY (key) REFERENCES other_table (key) ON DELETE CASCADE );"
    )
