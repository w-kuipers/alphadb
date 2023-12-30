import pytest

from alphadb.utils.exceptions import IncompatibleColumnAttributes, IncompleteVersionObject
from alphadb.utils.query.table.createtable import createtable
from tests.test_utils.version_source import wrap_partial_in_createtable


def test_missing_column_type():
    with pytest.raises(IncompleteVersionObject):
        test_data = wrap_partial_in_createtable(
            {
                "col": {"length": 10},
            }
        )
        createtable(version_source=test_data, table_name="table", version="0.0.1")


def test_incompatible_column_attributes():
    with pytest.raises(IncompatibleColumnAttributes):
        test_data = wrap_partial_in_createtable(
            {
                "col": {
                    "type": "VARCHAR",
                    "null": True,
                    "a_i": True,
                },
            }
        )
        createtable(version_source=test_data, table_name="table", version="0.0.1")


def test_incomplete_foreign_key_object():
    #### Missing key
    with pytest.raises(IncompleteVersionObject):
        test_data = wrap_partial_in_createtable({"col": {"foreign_key": {"references": "test"}}})
        createtable(version_source=test_data, table_name="table", version="0.0.1")

    #### Missing references
    with pytest.raises(IncompleteVersionObject):
        test_data = wrap_partial_in_createtable({"col": {"foreign_key": {"key": "test"}}})
        createtable(version_source=test_data, table_name="table", version="0.0.1")


def test_query():
    test_data = wrap_partial_in_createtable(
        {
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
    )

    assert (
        createtable(table_name="table", version_source=test_data, version="0.0.1")
        == " CREATE TABLE table ( id INT NOT NULL AUTO_INCREMENT, col1 VARCHAR(30) NOT NULL UNIQUE, PRIMARY KEY (id), FOREIGN KEY (key) REFERENCES other_table (key) ON DELETE CASCADE ) ENGINE = InnoDB;"
    )
