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

from .globals import CONFIG_TABLE_NAME


class NoConnection(Exception):
    def __init__(self):
        super().__init__("No database connection active.")


class DBConfigIncomplete(Exception):
    def __init__(self, missing: str):
        super().__init__(f'There seems to be an issue with the database config. It is initialized, but does not return a valid {missing}. Please manually check the "{CONFIG_TABLE_NAME}" table in your database.')


class NoDatabaseEngineSpecified(ValueError):
    def __init__(self):
        super().__init__("No database type was specified.")


class DBNotInitialized(Exception):
    def __init__(self):
        super().__init__("The database has not yet been initialized.")


class NoNetworkConnection(Exception):
    def __init__(self):
        super().__init__("No active internet connection was found.")


class MissingVersionData(ValueError):
    def __init__(self) -> None:
        super().__init__("Version information data must be supplied for the update to run")


class IncompleteVersionData(ValueError):
    def __init__(self) -> None:
        super().__init__(
            'Version information data not complete. Must contain "latest", "version" and "name". Latest is the latest version number, version is a JSON object containing the database structure and name is the database template name.'
        )


class NeedsConfirmation(ValueError):
    def __init__(self) -> None:
        super().__init__("Did you forget to set confirm to True? This is a safety feature!")


class IncompleteVersionObject(ValueError):
    def __init__(self, key=None, object=None) -> None:
        if key == None or object == None:
            message = "Database version data is incomplete or broken."
        else:
            message = f'Database version data is incomplete or broken. "{object}" is missing key "{key}".'
        super().__init__(message)


class DBTemplateNoMatch(ValueError):
    def __init__(self) -> None:
        super().__init__("This database uses a different database version source. The template name does not match the one previously used to update this database.")


class MissingDependencies(ModuleNotFoundError):
    def __init__(self, class_name: str, dependency: str) -> None:
        super().__init__(f'"{class_name}" requires "{dependency}" to be installed manually.')


class IncompatibleColumnAttributes(ValueError):
    def __init__(self, *args) -> None:
        attr_list = []
        for attr in args:
            attr_list.append(f'"{attr}"')

        super().__init__(f'Column attributes {", ".join(attr_list)} are not compatible.')
