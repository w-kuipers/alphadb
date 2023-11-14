from alphadb.utils.query.table import alter_table
from tests.test_utils.version_source import wrap_partial_in_altertable, wrap_version_list_in_base

def test_drop_column():
    test_data = wrap_partial_in_altertable({"dropcolumn": ["col1", "col2", "col3"]})

    assert alter_table(version_source=test_data, table_name="test", version="0.0.1") == " ALTER TABLE `test` DROP COLUMN `col1`, DROP COLUMN `col2`, DROP COLUMN `col3`;"

def test_drop_primary_key():
    test_data = wrap_version_list_in_base([
        {
            "_id": "0.0.1",
            "createtable": {
                "table": {
                    "primary_key": "col",
                    "col": {
                        "type": "INT",
                        "a_i": True
                    }
                }
            }
        },
        {
            "_id": "0.0.2",
            "altertable": {
                "table": {
                    "primary_key": None
                }
            }
        }
    ])

    assert alter_table(version_source=test_data, table_name="table", version="0.0.2") == ""
