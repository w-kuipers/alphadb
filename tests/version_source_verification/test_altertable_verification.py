from alphadb import VersionSourceVerification

def test_empty_altertable():
    
    version_empty_createtable_altertable = {}

    verification_empty_createtable = VersionSourceVerification({})
    verification_empty_createtable.altertable(version_empty_createtable_altertable)

    assert verification_empty_createtable.issues == [("LOW", "Unknown version -> altertable: Does not contain any data")]
