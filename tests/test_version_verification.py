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
    
    version_empty_createtable = {
        "_id": "1.0.0",
        "createtable": {

        }
    }

    verification_empty_createtable = SourceVeficication({})
    verification_empty_createtable.createtable(version_empty_createtable)

    assert verification_empty_createtable.issues == [("LOW", "Createtable method on version at index 0 does not contain any data")]
