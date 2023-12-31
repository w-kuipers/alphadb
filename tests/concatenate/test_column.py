from src.alphadb.utils.concatenate.column import concatenate_column


#### Make sure recreate key is removed from the result
def test_concatenate_remove_recreate():
    versions = [
        {"_id": "0.0.1", "createtable": {"table": {"col": {"type": "VARCHAR", "length": 200}}}},
        {"_id": "0.0.2", "altertable": {"table": {"modifycolumn": {"col": {"recreate": False, "unique": True}}}}},
    ]

    result = {"type": "VARCHAR", "length": 200, "unique": True}

    assert concatenate_column(versions, table_name="table", column_name="col") == result


def test_concatenate():
    versions = [
        {"_id": "0.0.1", "createtable": {"table": {"col": {"type": "VARCHAR", "length": 200}, "col2": {"type": "TEXT", "length": 9000}}}},
        {"_id": "0.0.2", "altertable": {"table": {"modifycolumn": {"col": {"recreate": False, "unique": True}}}}},
        {"_id": "0.0.5", "altertable": {"table": {"modifycolumn": {"col": {"recreate": False, "null": True, "length": 240}, "col2": {"type": "TEXT", "length": 200}}}}},
    ]

    result = {"type": "VARCHAR", "length": 240, "unique": True, "null": True}

    assert concatenate_column(versions, table_name="table", column_name="col") == result


def test_rename_single_column():
    versions = [
        {"_id": "0.0.1", "createtable": {"table": {"col": {"type": "VARCHAR", "length": 200}, "col2": {"type": "TEXT", "length": 200}}}},
        {"_id": "0.0.2", "altertable": {"table": {"renamecolumn": {"col": "renamed"}}}},
        {"_id": "0.0.3", "altertable": {"table": {"modifycolumn": {"col": {"recreate": False, "unique": True}}}}},  ## Should be ignored because uses old column name
        {"_id": "0.0.4", "altertable": {"table": {"modifycolumn": {"renamed": {"recreate": False, "null": True}, "col2": {"type": "TEXT", "length": 935}}}}},
    ]

    result = {
        "type": "VARCHAR",
        "length": 200,
        "null": True,
    }

    assert concatenate_column(versions, table_name="table", column_name="renamed") == result

    #### Don't break on column that has not been renamed
    result_col2 = {"type": "TEXT", "length": 935}

    assert concatenate_column(versions, table_name="table", column_name="col2") == result_col2


def test_rename_multiply_columns():
    versions = [
        {"_id": "0.0.1", "createtable": {"table": {"col": {"type": "VARCHAR", "length": 200}}}},
        {"_id": "0.0.2", "altertable": {"table": {"renamecolumn": {"col": "renamed"}}}},
        {"_id": "0.0.3", "altertable": {"table": {"modifycolumn": {"col": {"recreate": False, "unique": True, "length": 7000}}}}},  ## Should be ignored because uses old column name
        {"_id": "0.0.4", "altertable": {"table": {"modifycolumn": {"renamed": {"recreate": False, "null": True}}}}},
        {"_id": "0.0.5", "altertable": {"table": {"renamecolumn": {"renamed": "rerenamed"}}}},
        {"_id": "0.0.6", "altertable": {"table": {"modifycolumn": {"rerenamed": {"recreate": False, "unique": False}}}}},
        {"_id": "0.0.7", "altertable": {"table": {"renamecolumn": {"rerenamed": "multiplerenamed"}}}},
        {"_id": "0.0.8", "altertable": {"table": {"modifycolumn": {"multiplerenamed": {"recreate": False, "length": 2300}}}}},
    ]

    result = {"type": "VARCHAR", "length": 2300, "null": True, "unique": False}

    assert concatenate_column(versions, table_name="table", column_name="multiplerenamed") == result


def test_modify_recreate():
    versions_recreate = [
        {"_id": "0.0.1", "createtable": {"table": {"col": {"type": "VARCHAR", "length": 200}}}},
        {"_id": "0.0.2", "altertable": {"table": {"modifycolumn": {"col": {"length": 300}}}}},
    ]

    result_recreate = {"length": 300}

    assert concatenate_column(versions_recreate, table_name="table", column_name="col") == result_recreate

    versions_no_recreate = [
        {"_id": "0.0.1", "createtable": {"table": {"col": {"type": "VARCHAR", "length": 200}}}},
        {
            "_id": "0.0.2",
            "altertable": {
                "table": {
                    "modifycolumn": {
                        "col": {
                            "recreate": False,
                            "length": 300,
                        }
                    }
                }
            },
        },
    ]

    result_no_recreate = {"type": "VARCHAR", "length": 300}

    assert concatenate_column(versions_no_recreate, table_name="table", column_name="col") == result_no_recreate
