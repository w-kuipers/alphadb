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

from alphadb.utils.common import convert_version_number
from alphadb.utils.types import Method, ValidationIssuesList
from alphadb.verification.compatibility import incompatible_types_with_autoincrement, incompatible_types_with_unique


class VersionSourceVerification:
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
                    version_output = f"Version index {i}"
                    self.issues.append(("CRITICAL", f"{version_output}: Missing a version number"))
                else:
                    try:
                        convert_version_number(version["_id"])
                    except:
                        self.issues.append(("CRITICAL", f"{version['_id']}: Version number is not convertable to an integer"))

                    version_output = f"Version {version['_id']}"
                
                for method in version:
                    match method:
                        case "_id": continue
                        case "createtable": 
                            self.createtable(version["createtable"], version_output)
                        case "altertable":
                            self.altertable(version["altertable"], version_output)
                        case _:
                            self.issues.append(("HIGH", f"{version_output}: Method '{method}' does not exist"))
                    

        return self.issues if not len(self.issues) == 0 else True

    def createtable(self, createtable: dict, version_output: str = "Unknown version"):
        "Verify a single versions createtable"

        if len(createtable) == 0:
            self.issues.append(("LOW", f"{version_output} -> createtable: Does not contain any data"))
        else:
            for table in createtable:
                for column in createtable[table]:
                    #### Primary key
                    if column == "primary_key":
                        if not createtable[table]["primary_key"] in createtable[table]:
                            self.issues.append(("CRITICAL", f"{version_output} -> createtable -> table:{table}: Primary key does not match any column name"))
                        continue

                    #### Columns
                    self.column_compatibility(table, column, createtable[table][column], method="createtable", version_output=version_output)

    def altertable(self, altertable: dict, version_output: str = "Unknown version"):
        "Verify a single versions altertable"

        if len(altertable) == 0:
            self.issues.append(("LOW", f"{version_output} -> altertable: Does not contain any data"))
        else:
            for table in altertable:
                #### Modify column
                if "modifycolumn" in altertable[table]:
                    for column in altertable[table]["modifycolumn"]:
                        self.column_compatibility(table, column, altertable[table]["modifycolumn"][column], method="altertable", version_output=version_output)

                #### Primary key
                # if "primary_key" in table:
                #     if

    def column_compatibility(self, table_name: str, column_name: str, data: dict, method: Method, version_output: str = "Unknown version"):
        "Verify column attribute compatibility"

        #### NULL and AUTO_INCREMENT
        if "null" in data and "a_i" in data:
            self.issues.append(("CRITICAL", f"{version_output} -> {method} -> table:{table_name} -> column:{column_name}: Column attributes NULL and AUTO_INCREMENT are incompatible"))

        #### If type is defined
        if not "type" in data:
            if not "recreate" in data or data["recreate"] == True:
                self.issues.append(("CRITICAL", f"{version_output} -> {method} -> table:{table_name} -> column:{column_name}: Does not contain a column type"))
        else:
            #### Types incompatible with AUTO_INCREMENT
            if data["type"].lower() in incompatible_types_with_autoincrement and "a_i" in data:
                self.issues.append(("CRITICAL", f"{version_output} -> {method} -> table:{table_name} -> column:{column_name}: Column type {data['type']} is incompatible with attribute AUTO_INCREMENT"))

            #### Types incompatible with UNIQUE
            if data["type"].lower() in incompatible_types_with_unique and "unique" in data:
                self.issues.append(("CRITICAL", f"{version_output} -> {method} -> table:{table_name} -> column:{column_name}: Column type {data['type']} is incompatible with attribute UNIQUE"))
