from typing import List, Literal, Optional, Tuple, TypedDict, Union

class Check(TypedDict):
    check: bool
    version: str

class Status(TypedDict):
    init: bool
    version: Union[str, None]
    name: str
    template: Union[str, None]

VerificationIssueLevel = Literal["LOW"] | Literal["HIGH"] | Literal["CRITICAL"]
VerificationIssueLevel.__doc__ = """
LOW: Will work, but will not have any effect on the database.
HIGH: Will still work, but might produce a different result than desired.
CRITICAL: Will not execute.
"""

class AlphaDB:
    def connect(self, host: str, user: str, password: str, database: str, port: Optional[int] = 3306): ...
    """Connect to a database"""

    def init(self): ...
    """Initialize the database"""

    def status(self) -> Status: ...
    """Get the databases status"""

    def update_queries(self, version_source: str, update_to_version: Optional[str] = None) -> List[Tuple[str, List[str]]]: ...
    """Generate queries to update the database"""

    def update(self, version_source: str, update_to_version: Optional[str] = None, no_date=False, verify=False, allowed_error_priority: Optional[VerificationIssueLevel] = "LOW"): ...
    """Update the databae"""

    def vacate(self): ...
    """
    Empty the database
    
    WARNING: This will erase all data in the database. This is not reversable.
    """
