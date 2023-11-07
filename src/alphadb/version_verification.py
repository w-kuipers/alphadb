# Copyright (C) 2023 Wibo Kuipers
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with this program.  If not, see <https://www.gnu.org/licenses/>.

from typing import Literal
from alphadb.utils.types import ValidationIssuesList, Method
from alphadb.verification.compatibility import incompatible_types_with_autoincrement

class SourceVeficication():

    def __init__(self, version_source):
        self.version_source = version_source
        self.issues: ValidationIssuesList = []
   
    def verify(self) -> ValidationIssuesList | Literal[True]:

        """
        Verify the version source.
        Returns True if no issues are identified,
        else it returns a list with the issues represented as strings
        """

        #### Template name 
        if not "name" in self.version_source:
            self.issues.append(("CRITICAL", "No rootlevel name was specified"))
        
        #### Version list
        if not "version" in self.version_source:
            self.issues.append(("LOW", "This version source does not contain any versions"))
        
        else:
            for i, version in enumerate(self.version_source["version"]):
                if not "_id" in version:
                    self.issues.append(("CRITICAL", f"Version on index {i} is missing a version number"))

                if "createtable" in version: self.createtable(version["createtable"], i)

        return self.issues if not len(self.issues) == 0 else True

    def createtable(self, createtable: dict, index=0):
        "Verify a single versions createtable"

        if len(createtable) == 0: 
            self.issues.append(("LOW", f"Createtable method on version at index {index} does not contain any data"))
        else: 
            self.column_compatibility(createtable, method="createtable", index=index)

    def column_compatibility(self, data: dict, method: Method, index: int = 0):
        "Verify column attribute compatibility"

        #### NULL and AUTO_INCREMENT
        if "null" in data and "a_i" in data:
            self.issues.append(("CRITICAL", "Column attributes NULL and AUTO_INCREMENT are incompatible"))

        #### If type is defined
        if not "type" in data: self.issues.append(("CRITICAL", f"{method.capitalize()} method on version at index {index} does not contain a column type"))
        else:
            #### Types incompatible with AUTO_INCREMENT
            if data["type"].lower() in incompatible_types_with_autoincrement and "a_i" in data:
                self.issues.append(("CRITICAL", f"{method.capitalize()} method on version at index {index} is of type '{data['type']}' which is incompatible with AUTO_INCREMENT"))

