from alphadb.utils.concatenate.primary_key import get_primary_key

def test_get_primary_key():
    versions = [
        {
            "_id": "0.0.1",
            "createtable": {
                "table": {
                    "primary_key": "col"
                }
            }
        }
    ]

    assert get_primary_key(versions, table_name="table") == "col"

def test_get_primary_key_altered():
    versions = [
        {
            "_id": "0.0.1",
            "createtable": {
                "table": {
                    "primary_key": "col"
                }
            }
        },
        {
            "_id": "0.0.2",
            "altertable": {
                "table": {
                    "primary_key": "other_col"
                }
            }
        }
    ]

    assert get_primary_key(versions, table_name="table") == "other_col"

def test_get_primary_key_deleted():
    
    versions = [
        {
            "_id": "0.0.1",
            "createtable": {
                "table": {
                    "primary_key": "col"
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
    ]

    assert get_primary_key(versions, table_name="table") == None

