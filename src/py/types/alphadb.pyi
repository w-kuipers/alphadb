from typing import TypedDict

class Check(TypedDict):
    check: bool
    version: str

class AlphaDB:
    def check(self) -> Check: ...
    """Check current database status"""
