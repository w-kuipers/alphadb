from alphadb.utils.query.table.altertable import altertable
from tests.test_utils.version_source import wrap_partial_in_altertable, wrap_version_list_in_base


def test_drop_column():
    test_data = wrap_partial_in_altertable({"dropcolumn": ["col1", "col2", "col3"]})

    assert altertable(version_source=test_data, table_name="table", version="0.0.1") == " ALTER TABLE table DROP COLUMN col1, DROP COLUMN col2, DROP COLUMN col3;"


def test_drop_primary_key():
    test_data = wrap_version_list_in_base(
        [
            {"_id": "0.0.1", "createtable": {"table": {"primary_key": "col", "col": {"type": "INT", "a_i": True}}}},
            {"_id": "0.0.2", "altertable": {"table": {"primary_key": None}}},
        ]
    )

    assert altertable(version_source=test_data, table_name="table", version="0.0.2") == " ALTER TABLE table MODIFY COLUMN col INT NOT NULL, DROP PRIMARY KEY;"
