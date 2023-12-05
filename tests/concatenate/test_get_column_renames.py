from alphadb.utils.concatenate.column import get_column_renames

def test_get_column_renames_desc():

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
                    "renamecolumn": {
                        "col": "renamed"
                    }
                }
            }
        },
        {
            "_id": "0.0.3", ## Should be ignored because uses old column name
            "altertable": {
                "table": {
                    "modifycolumn": {
                        "col": {
                            "recreate": False,
                            "unique": True,
                            "length": 7000
                        }
                    }
                }
            }
        },
        {
            "_id": "0.0.4",
            "altertable": {
                "table": {
                    "modifycolumn": {
                        "renamed": {
                            "recreate": False,
                            "null": True
                        }
                    }
                }
            }
        },
        {
            "_id": "0.0.5",
            "altertable": {
                "table": {
                    "renamecolumn": {
                        "renamed": "rerenamed"
                    }
                }
            }
        },
        {
            "_id": "0.0.6",
            "altertable": {
                "table": {
                    "modifycolumn": {
                        "rerenamed": {
                            "recreate": False,
                            "unique": False
                        }
                    }
                }
            }
        },
        {
            "_id": "0.0.7",
            "altertable": {
                "table": {
                    "renamecolumn": {
                        "rerenamed": "multiplerenamed"
                    }
                }
            }
        },
        {
            "_id": "0.0.8",
            "altertable": {
                "table": {
                    "modifycolumn": {
                        "multiplerenamed": {
                            "recreate": False,
                            "length": 2300
                        }
                    }
                }
            }
        }
    ]

    assert get_column_renames(version_list=versions, table_name="table", column_name="multiplerenamed") == [{'old_name': 'rerenamed', 'rename_version': 7}, {'old_name': 'renamed', 'rename_version': 5}, {'old_name': 'col', 'rename_version': 2}]

def test_get_column_renames_asc():

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
                    "renamecolumn": {
                        "col": "renamed"
                    }
                }
            }
        },
        {
            "_id": "0.0.3", ## Should be ignored because uses old column name
            "altertable": {
                "table": {
                    "modifycolumn": {
                        "col": {
                            "recreate": False,
                            "unique": True,
                            "length": 7000
                        }
                    }
                }
            }
        },
        {
            "_id": "0.0.4",
            "altertable": {
                "table": {
                    "modifycolumn": {
                        "renamed": {
                            "recreate": False,
                            "null": True
                        }
                    }
                }
            }
        },
        {
            "_id": "0.0.5",
            "altertable": {
                "table": {
                    "renamecolumn": {
                        "renamed": "rerenamed"
                    }
                }
            }
        },
        {
            "_id": "0.0.6",
            "altertable": {
                "table": {
                    "modifycolumn": {
                        "rerenamed": {
                            "recreate": False,
                            "unique": False
                        }
                    }
                }
            }
        },
        {
            "_id": "0.0.7",
            "altertable": {
                "table": {
                    "renamecolumn": {
                        "rerenamed": "multiplerenamed"
                    }
                }
            }
        },
        {
            "_id": "0.0.8",
            "altertable": {
                "table": {
                    "modifycolumn": {
                        "multiplerenamed": {
                            "recreate": False,
                            "length": 2300
                        }
                    }
                }
            }
        }
    ]

    assert get_column_renames(version_list=versions, table_name="table", column_name="col", order="ASC") == [{'new_name': 'renamed', 'rename_version': 2}, {'new_name': 'rerenamed', 'rename_version': 5}, {'new_name': 'multiplerenamed', 'rename_version': 7}]
