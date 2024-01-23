from alphadb import VersionSourceVerification


def test_empty_altertable():
    version_removed_column = {
        "name": "removed_col",
        "version": [
            {"_id": "0.0.1", "createtable": {"table": {"primary_key": "col1", "col1": {"type": "INT"}, "col2": {"type": "INT"}}}},
            {"_id": "0.0.2", "altertable": {"table": {"dropcolumn": ["col1"]}}},
        ],
    }

    verification_empty_createtable = VersionSourceVerification(version_removed_column)
    verification_empty_createtable.verify()

    assert verification_empty_createtable.issues == [("LOW", "Version 0.0.2 -> altertable -> table:table -> dropcolumn: Column col1 is the tables current primary key")]
