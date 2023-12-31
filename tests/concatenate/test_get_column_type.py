from alphadb.utils.concatenate.column import get_column_type


def test_get_column_type_createtable():
    versions = [
        {"_id": "0.0.1", "createtable": {"table": {"col": {"type": "VARCHAR", "length": 200}}}},
    ]

    assert get_column_type(version_list=versions, table_name="table", column_name="col") == "VARCHAR"


def test_get_column_type_altertable():
    versions = [
        {
            "_id": "0.0.1",
            "createtable": {"table": {"col": {"type": "VARCHAR", "length": 200}}},
        },
        {"_id": "0.0.2", "altertable": {"table": {"modifycolumn": {"col": {"type": "TEXT"}}}}},
        {"_id": "0.0.3", "altertable": {"table": {"modifycolumn": {"col": {"type": "INT"}}}}},
    ]

    assert get_column_type(version_list=versions, table_name="table", column_name="col") == "INT"


def test_get_column_type_renamed():
    versions = [
        {
            "_id": "0.0.1",
            "createtable": {"table": {"col": {"type": "VARCHAR", "length": 200}}},
        },
        {"_id": "0.0.2", "altertable": {"table": {"modifycolumn": {"col": {"type": "TEXT"}}}}},
        {"_id": "0.0.3", "altertable": {"table": {"renamecolumn": {"col": "renamed"}}}},
    ]

    assert get_column_type(version_list=versions, table_name="table", column_name="renamed") == "TEXT"

    versions.append(
        {"_id": "0.0.4", "altertable": {"table": {"modifycolumn": {"renamed": {"type": "DATE"}}}}},
    )

    assert get_column_type(version_list=versions, table_name="table", column_name="renamed") == "DATE"


def test_get_column_type_multiple_renamed():
    versions = [
        {
            "_id": "0.0.1",
            "createtable": {"table": {"col": {"type": "VARCHAR", "length": 200}}},
        },
        {"_id": "0.0.2", "altertable": {"table": {"modifycolumn": {"col": {"type": "TEXT"}}}}},
        {"_id": "0.0.3", "altertable": {"table": {"renamecolumn": {"col": "renamed"}}}},
        {"_id": "0.0.4", "altertable": {"table": {"modifycolumn": {"renamed": {"type": "DATE"}}}}},
        {"_id": "0.0.5", "altertable": {"table": {"renamecolumn": {"renamed": "secondrename"}}}},
    ]

    assert get_column_type(version_list=versions, table_name="table", column_name="secondrename") == "DATE"

    versions.append(
        {"_id": "0.0.6", "altertable": {"table": {"modifycolumn": {"secondrename": {"type": "JSON"}}}}},
    )

    assert get_column_type(version_list=versions, table_name="table", column_name="secondrename") == "JSON"
