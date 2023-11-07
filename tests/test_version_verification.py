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

from alphadb.utils.query.table import create_table
from alphadb.version_verification import SourceVeficication
import pytest

#### SourceVeficication will first check for _id, then for name
def test_no_version_number_and_name():
        
    version_source_no_version = {
        "name": "name",
    }

    version_source_no_name = {
        "version": {
            "create_table": {
                "table1": {
                    "col1": {
                        "type": "VARCHAR",
                        "length": 100
                    }
                }
            }
        }
    }



    verification_no_version = SourceVeficication(version_source_no_version)
    assert verification_no_version.verify() == [("LOW", "This version source does not contain any versions")]
    
    verification_no_name = SourceVeficication(version_source_no_name)
    assert verification_no_name.verify() == [("CRITICAL", "No rootlevel name was specified"), ("CRITICAL", "Version on index 0 is missing a version number")]

def test_empty_createtable():
    
    version_empty_createtable = {}

    verification_empty_createtable = SourceVeficication({})
    verification_empty_createtable.createtable(version_empty_createtable)

    assert verification_empty_createtable.issues == [("LOW", "Createtable method on version at index 0 does not contain any data")]

def test_createtable_no_type():
    version_createtable_no_type = {
        "length": 10
    }

    verification_createtable_no_type = SourceVeficication({})
    verification_createtable_no_type.createtable(version_createtable_no_type)

    assert verification_createtable_no_type.issues == [("CRITICAL", "Createtable method on version at index 0 does not contain a column type")]


def test_column_incompatibility():
    version_null_and_ai = {
        "type": "INT",
        "null": True,
        "a_i": True
    }

    verification_null_and_ai = SourceVeficication({})
    verification_null_and_ai.column_compatibility(version_null_and_ai, "createtable", index=0)

    assert verification_null_and_ai.issues == [("CRITICAL", "Column attributes NULL and AUTO_INCREMENT are incompatible")]

    type_json_and_ai = {
        "type": "JSON",
        "a_i": True
    }

    verification_json_and_ai = SourceVeficication({})
    verification_json_and_ai.column_compatibility(type_json_and_ai, "createtable", index=0)

    assert verification_json_and_ai.issues == [("CRITICAL", "Createtable method on version at index 0 is of type 'JSON' which is incompatible with AUTO_INCREMENT")]
