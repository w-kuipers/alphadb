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


#### Local imports
from .utils.decorators import conn_test, init_test
from .utils.exceptions import DBTemplateNoMatch, IncompleteVersionData, MissingVersionData, NeedsConfirmation, NoDatabaseType
from .utils.query.default_data import create_default_data
from .utils.query.table import create_table
from .utils.types import Database, SQLEscapeString


class AlphaDB:
    database_type: Database
    sql_escape_string: SQLEscapeString
    db_name: str

    def __init__(self, engine: Database):
        # self.database_type = database_type
        print(engine)

        self.get_sql_escape_string()

        self.connection = None

    def get_sql_escape_string(self):
        if self.database_type == "mysql":
            self.sql_escape_string = "%s"
        if self.database_type == "sqlite":
            self.sql_escape_string = "?"

    @conn_test
    def check(self):
        #### Database type is needed to know how to check for existing tables
        if not hasattr(self, "database_type"):
            raise NoDatabaseType()

        with self.cursor() as cursor:
            current_version = None

            #### Check if the config table (fdb_cfg) exists

            #### SQLite does not have an information_schema, so we check for existing tables differently
            if self.database_type == "sqlite":
                cursor.execute("SELECT name FROM sqlite_master WHERE type='table' AND name='fdb_cfg';")
            else:
                cursor.execute(
                    f'SELECT table_name FROM information_schema.tables WHERE table_schema = {self.sql_escape_string} AND table_name = "fdb_cfg"',
                    (self.db_name,),
                )
            table_check = cursor.fetchall()

            #### If it exists, get current version
            if table_check:
                cursor.execute(
                    f"SELECT version FROM fdb_cfg WHERE db = {self.sql_escape_string}",
                    (self.db_name,),
                )
                fetched = cursor.fetchone()
                if fetched:
                    current_version = fetched[0]

            #### Check True means the database is ready for use
            check = True if table_check and fetched else False

        return {"check": check, "current_version": current_version}

    @conn_test
    def init(self):
        #### Check if the database is already initialized
        if self.check()["check"] == True:
            return "already-initialized"

        try:
            with self.cursor() as cursor:
                #### Create configuration table

                if self.database_type == "sqlite":
                    cursor.execute(f"CREATE TABLE IF NOT EXISTS `fdb_cfg` (`db` VARCHAR(100) NOT NULL, `version` VARCHAR(50) NOT NULL, `template` VARCHAR(50) NULL, PRIMARY KEY (`db`));")
                else:
                    cursor.execute(f"CREATE TABLE IF NOT EXISTS `fdb_cfg` (`db` VARCHAR(100) NOT NULL, `version` VARCHAR(50) NOT NULL, `template` VARCHAR(50) NULL, PRIMARY KEY (`db`)) ENGINE = InnoDB;")

                #### Set the version to 0.0.0
                cursor.execute(
                    f"INSERT INTO fdb_cfg (`db`, `version`) values ({self.sql_escape_string}, {self.sql_escape_string})",
                    (self.db_name, "0.0.0"),
                )
        except Exception as e:
            raise Exception(e)

        return True

    @conn_test
    def status(self):
        current_version = None
        template = None

        with self.cursor() as cursor:
            #### Check if fdb_cfg (fmm config table) exists
            cursor.execute(
                f'SELECT * FROM information_schema.tables WHERE table_schema = {self.sql_escape_string} AND table_name = "fdb_cfg"',
                (self.db_name,),
            )
            table_check = cursor.fetchall()

            #### If it exists, get current version
            if table_check:
                cursor.execute(
                    f"SELECT version, template FROM fdb_cfg WHERE db = {self.sql_escape_string}",
                    (self.db_name,),
                )
                fetched = cursor.fetchone()
                if fetched:
                    current_version = fetched[0]
                    template = fetched[1]

            #### Check True means the database is initialized
            check = True if table_check and fetched else False

        return {
            "init": check,
            "version": current_version,
            "name": self.db_name,
            "template": template,
        }

    @conn_test
    @init_test
    def update(self, version_information=None, update_to_version=None, no_data=False):
        #### Some error handling
        if version_information == None:
            raise MissingVersionData()
        else:
            if not "latest" in version_information or not "version" in version_information or not "name" in version_information:
                raise IncompleteVersionData()

        #### Start update process
        with self.cursor() as cursor:
            try:
                cursor.execute(
                    f"SELECT version, template FROM fdb_cfg WHERE `db` = {self.sql_escape_string}",
                    (self.db_name,),
                )
                db_data = cursor.fetchone()

                #### Check if the database template matches
                if not version_information["name"] == db_data[1]:
                    #### If no template is defined, use the current one
                    if db_data[1] == None:
                        cursor.execute(
                            f'UPDATE fdb_cfg SET template="{version_information["name"]}" WHERE `db` = {self.sql_escape_string}',
                            (self.db_name,),
                        )
                    else:
                        raise DBTemplateNoMatch()
                database_version = db_data[0]  ## Get database version
            except Exception as e:
                raise Exception(e)

            database_version_latest = version_information["latest"] if update_to_version == None else update_to_version

            ### Check if database needs to be updated
            if not int(database_version_latest.replace(".", "")) > int(database_version.replace(".", "")):
                return "up-to-date"

            try:
                #### Loop through update data
                for version in version_information["version"]:
                    #### Check if version number is larger than current version
                    if int(version["_id"].replace(".", "")) <= int(database_version.replace(".", "")):
                        continue

                    #### Continue if latest version is current
                    if int(database_version_latest.replace(".", "")) < int(version["_id"].replace(".", "")):
                        continue

                    #### Create tables
                    for table in version["createtable"]:
                        query = create_table(table_data=version["createtable"][table], table_name=table)
                        cursor.execute(query)

                    #### Insert default data
                    if no_data == False:
                        if "default_data" in version:
                            for table in version["default_data"]:
                                for item in version["default_data"][table]:
                                    query = create_default_data(table_name=table, item=item)
                                    cursor.execute(query)
            except KeyError as e:
                raise e
            except Exception as e:
                raise Exception(e)

            cursor.execute(
                f"UPDATE fdb_cfg SET version={self.sql_escape_string} WHERE `db` = {self.sql_escape_string}",
                (database_version_latest, self.db_name),
            )

            return True

    @conn_test
    def vacate(self, confirm=False):
        if not confirm == True:
            raise NeedsConfirmation()

        with self.cursor() as cursor:
            cursor.execute("SET foreign_key_checks = 0;")

            if self.database_type == "sqlite":
                cursor.execute("SELECT name FROM sqlite_master WHERE type='table';")
            else:
                cursor.execute("show tables;")
            tables = cursor.fetchall()

            print(tables)
            for t in tables:
                cursor.execute(f"DROP TABLE {t[0]}")

            cursor.execute("SET foreign_key_checks = 1;")

        return True

    @conn_test
    def export(self):
        with self.cursor() as cursor:
            cursor.execute("show tables;")
            tables = cursor.fetchall()

            data = {}

            for t in tables:
                cursor.execute(f"SELECT * FROM INFORMATION_SCHEMA.COLUMNS WHERE TABLE_NAME = N'{t[0]}'")
                data[t[0]] = {"columns": [t[3] for t in cursor.fetchall()]}

                cursor.execute(f"SELECT * FROM {t[0]}")
                data[t[0]]["data"] = cursor.fetchall()

        return data
