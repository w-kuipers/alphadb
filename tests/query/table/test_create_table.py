import pytest

from src.alphadb.utils.exceptions import IncompatibleColumnAttributes, IncompleteVersionObject
from src.alphadb.utils.query.table import create_table


def test_missing_column_type():
    with pytest.raises(IncompleteVersionObject):
        test_data = {
            "col": {"length": 10},
        }
        create_table(table_data=test_data, table_name="test")


def test_incompatible_column_attributes():
    with pytest.raises(IncompatibleColumnAttributes):
        test_data = {
            "col": {
                "type": "VARCHAR",
                "null": True,
                "a_i": True,
            },
        }
        create_table(table_data=test_data, table_name="test")


def test_incomplete_foreign_key_object():
    #### Missing key
    with pytest.raises(IncompleteVersionObject):
        test_data = {"col": {"foreign_key": {"references": "test"}}}
        create_table(table_data=test_data, table_name="test")

    #
    #     #### Missing references
    with pytest.raises(IncompleteVersionObject):
        test_data = {"col": {"foreign_key": {"key": "test"}}}
        create_table(table_data=test_data, table_name="test")


def test_query():
    #### Test all column types (MYSQL)
    test_data = {
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
    }

    assert (
        create_table(table_name="test", table_data=test_data)
        == " CREATE TABLE `test` ( `id` INT NOT NULL AUTO_INCREMENT, `col1` VARCHAR(30) NOT NULL UNIQUE, PRIMARY KEY (`id`), FOREIGN KEY (key) REFERENCES other_table (key) ON DELETE CASCADE ) ENGINE = InnoDB;"
    )

    #### Test all column types (SQLite)

    assert (
        create_table(table_name="test", table_data=test_data, engine="sqlite")
        == " CREATE TABLE `test` ( `id` INT NOT NULL, `col1` VARCHAR(30) NOT NULL UNIQUE, PRIMARY KEY (`id`), FOREIGN KEY (key) REFERENCES other_table (key) ON DELETE CASCADE );"
    )
