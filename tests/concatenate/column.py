from src.alphadb.utils.concatenate.column import concatenate_column

def test_concatenate_remove_recreate():
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
    ]

    result = {
        "type": "VARCHAR",
        "length": 200,
        "unique": True
    }

    assert concatenate_column(versions, table_name="table", column_name="col") == result

# def test_rename_single_column():
#     versions = [
#         {
#             "_id": "0.0.1",
#             "createtable": {
#                 "table": {
#                     "col": {
#                         "type": "VARCHAR",
#                         "length": 200
#                     }
#                 }
#             }
#         },
#         {
#             "_id": "0.0.2",
#             "altertable": {
#                 "table": {
#                     "renamecolumn": {
#                         "col": "renamed"
#                     }
#                 }
#             }
#         },
#         {
#             "_id": "0.0.3",
#             "altertable": {
#                 "table": {
#                     "modifycolumn": {
#                         "col": {
#                             "recreate": False,
#                             "unique": True
#                         }
#                     }
#                 }
#             }
#         },
#         {
#             "_id": "0.0.4",
#             "altertable": {
#                 "table": {
#                     "modifycolumn": {
#                         "renamed": {
#                             "recreate": False,
#                             "null": True
#                         }
#                     }
#                 }
#             }
#         }
#     ]
#
#     result = {
#         "type": "VARCHAR",
#         "length": 200,
#         "null": True,
#         "unique": True
#     }
#
#     assert concatenate_column(versions, table_name="table", column_name="renamed") == result

def test_rename_multiply_columns():
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
                            "unique": True
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
        }
    ]

    result = {
        "type": "VARCHAR",
        "length": 200,
        "null": True,
        "unique": False
    }

    assert concatenate_column(versions, table_name="table", column_name="rerenamed") == result
