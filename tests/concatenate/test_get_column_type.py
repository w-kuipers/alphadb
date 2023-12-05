from alphadb.utils.concatenate.column import get_column_type
from alphadb.utils.query.table.altertable import altertable

def test_get_column_type_createtable():
    
    versions = [
        {
            "_id": "0.0.1",
            "createtable": {
                "table": {
                    "col": {
                        "type": "VARCHAR",
                        "length": 200
                    }
                }
            }
        },
    ]

    assert get_column_type(version_list=versions, table_name="table", column_name="col") == "VARCHAR"

def test_get_column_type_altertable():
    
    versions = [
        {
            "_id": "0.0.1",
            "createtable": {
                "table": {
                    "col": {
                        "type": "VARCHAR",
                        "length": 200
                    }
                }
            },
        },
        {
            "_id": "0.0.2",
            "altertable": {
                "table": {
                    "col": {
                        "type": "TEXT"
                    }
                }
            }
        },
        {
            "_id": "0.0.3",
            "altertable": {
                "table": {
                    "modifycolumn": {
                        "col": {
                            "type": "INT"
                        }
                    }
                }
            }
        },
    ]

    assert get_column_type(version_list=versions, table_name="table", column_name="col") == "INT"

