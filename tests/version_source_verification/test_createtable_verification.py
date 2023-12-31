from alphadb import VersionSourceVerification


def test_createtable_no_type():
    version_createtable_no_type = {"test_table": {"test_column": {"length": 10}}}

    verification_createtable_no_type = VersionSourceVerification({})
    verification_createtable_no_type.createtable(version_createtable_no_type)

    assert verification_createtable_no_type.issues == [("CRITICAL", "Unknown version -> createtable -> table:test_table -> column:test_column: Does not contain a column type")]


def test_empty_createtable():
    version_empty_createtable_altertable = {}

    verification_empty_createtable = VersionSourceVerification({})
    verification_empty_createtable.createtable(version_empty_createtable_altertable)

    assert verification_empty_createtable.issues == [("LOW", "Unknown version -> createtable: Does not contain any data")]
