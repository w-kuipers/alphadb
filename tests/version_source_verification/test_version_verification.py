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

from alphadb.version_verification import VersionSourceVerification


#### SourceVeficication will first check for _id, then for name
def test_no_version_number_and_name():
    version_source_no_version = {
        "name": "name",
    }

    version_source_no_name = {"version": [{"createtable": {"table1": {"col1": {"type": "VARCHAR", "length": 100}}}}]}

    verification_no_version = VersionSourceVerification(version_source_no_version)
    assert verification_no_version.verify() == [("LOW", "This version source does not contain any versions")]

    verification_no_name = VersionSourceVerification(version_source_no_name)
    assert verification_no_name.verify() == [("CRITICAL", "No rootlevel name was specified"), ("CRITICAL", "Version index 0: Missing a version number")]


def test_bad_primarykey():
    version_bad_primarykey = {"test_table": {"primary_key": "none", "test_column": {"type": "VARCHAR", "length": 10}}}

    verification_bad_primarykey = VersionSourceVerification({})
    verification_bad_primarykey.createtable(version_bad_primarykey)

    assert verification_bad_primarykey.issues == [("CRITICAL", "Unknown version -> createtable -> table:test_table: Primary key does not match any column name")]


def test_column_incompatibility():
    version_null_and_ai = {"type": "INT", "null": True, "a_i": True}

    verification_null_and_ai = VersionSourceVerification({})
    verification_null_and_ai.column_compatibility("test_table", "test_column", version_null_and_ai, "createtable")

    assert verification_null_and_ai.issues == [("CRITICAL", "Unknown version -> createtable -> table:test_table -> column:test_column: Column attributes NULL and AUTO_INCREMENT are incompatible")]

    #### JSON is one of the column types incompatible with AUTO_INCREMENT
    type_json_and_ai = {"type": "JSON", "a_i": True}

    verification_json_and_ai = VersionSourceVerification({})
    verification_json_and_ai.column_compatibility("test_table", "test_column", type_json_and_ai, "createtable")

    assert verification_json_and_ai.issues == [("CRITICAL", "Unknown version -> createtable -> table:test_table -> column:test_column: Column type JSON is incompatible with attribute AUTO_INCREMENT")]
    #### JSON is one of the column types incompatible with UNIQUE
    type_json_and_unique = {"type": "JSON", "unique": True}

    verification_json_and_unique = VersionSourceVerification({})
    verification_json_and_unique.column_compatibility("test_table", "test_column", type_json_and_unique, "createtable")

    assert verification_json_and_unique.issues == [("CRITICAL", "Unknown version -> createtable -> table:test_table -> column:test_column: Column type JSON is incompatible with attribute UNIQUE")]
