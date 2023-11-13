from src.alphadb.utils.concatenate.column import concatenate_column

def test_full_column_concatenate():
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
        {
            "_id": "0.0.2",
            "altertable": {
                "table": {
                    "modifycolumn": {
                        "col": {
                            "recreate": False,
                            "unique": True
                        }
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
                            "length": 100
                        }
                    }
                }
            }
        },
        {
            "_id": "0.0.4",
            "altertable": {
                "table": {
                    "dropcolumn": ["col"]
                }
            }
        },
        {
            "_id": "0.0.5",
            "altertable": {
                "table": {
                    "addcolumn": {
                        "col": {
                            "type": "TEXT",
                            "length": 7000
                        }
                    }
                }
            }
        },
        {
            "_id": "0.0.6",
            "altertable": {
                "table": {
                    "renamecolumn": {
                        "col": "renamedcol"
                    }
                }
            }
        },
        {
            "_id": "0.0.7",
            "altertable": {
                "table": {
                    "modifycolumn": {
                        "col": {
                            "length": 300
                        }
                    }
                }
            }
        }
    ]

    concatenated = {
        "type": "TEXT",
        "length": 300
    }

    assert concatenate_column(versions, table_name="table", column_name="col") == concatenated
