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

from typing import Callable

from alphadb.utils.decorators import conn_test, init_test
from alphadb.utils.exceptions import DBConfigIncomplete, DBTemplateNoMatch, IncompleteVersionData, MissingVersionData, NeedsConfirmation
from alphadb.utils.globals import CONFIG_TABLE_NAME
from alphadb.utils.query.default_data import create_default_data
from alphadb.utils.query.table.altertable import altertable
from alphadb.utils.query.table.createtable import createtable

from mysql.connector import MySQLConnection
from mysql.connector.cursor import MySQLCursor


class AlphaDB:
    db_name: str
    connection: MySQLConnection | None = None
    cursor: Callable[..., MySQLCursor]

    def __init__(self):
        self.connection = None

    @conn_test
    def check(self):
        with self.cursor() as cursor:
            current_version = None

            #### Check if the config table (adb_conf) exists
            cursor.execute(
                f"SELECT table_name FROM information_schema.tables WHERE table_schema = %s AND table_name = %s",
                (self.db_name, CONFIG_TABLE_NAME),
            )

            table_check = cursor.fetchall()

            fetched = None
            if table_check:
                cursor.execute(
                    f"SELECT version FROM {CONFIG_TABLE_NAME} WHERE db = %s",
                    (self.db_name,),
                )
                fetched = cursor.fetchone()
                if fetched:
                    current_version = fetched[0]

            #### Check True means the database is ready for use
            check = True if table_check and fetched else False

        return {"check": check, "current_version": current_version}

    def connect(self, host: str, password: str, user: str, database: str, port: int = 3306) -> bool:
        self.connection = MySQLConnection(
            host=host,
            user=user,
            password=password,
            port=port,
            database=database,
            buffered=True,
        )
        self.connection.autocommit = True
        self.cursor = self.connection.cursor
        self.db_name = database

        return self.check()["check"] == True

    @conn_test
    def init(self):
        #### Check if the database is already initialized
        if self.check()["check"] == True:
            return "already-initialized"

        try:
            with self.cursor() as cursor:
                #### Create configuration table
                cursor.execute(f"CREATE TABLE IF NOT EXISTS {CONFIG_TABLE_NAME} (db VARCHAR(100) NOT NULL, version VARCHAR(50) NOT NULL, template VARCHAR(50) NULL, PRIMARY KEY (db)) ENGINE = InnoDB;")

                cursor.execute(
                    f"INSERT INTO {CONFIG_TABLE_NAME} (db, version) values (%s, %s)",
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
            #### Check if the config table (adb_conf) exists
            cursor.execute(
                f"SELECT table_name FROM information_schema.tables WHERE table_schema = %s AND table_name = %s",
                (self.db_name, CONFIG_TABLE_NAME),
            )
            table_check = cursor.fetchall()

            #### If it exists, get current version
            fetched = None
            if table_check:
                cursor.execute(
                    f"SELECT version, template FROM {CONFIG_TABLE_NAME} WHERE db = %s",
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
    def update_queries(self, version_source, update_to_version=None, no_data=False):

        #### Some error handling
        if version_source == None:
            raise MissingVersionData()
        else:
            if not "version" in version_source or not "name" in version_source:
                raise IncompleteVersionData()

        #### Start update process
        database_version = None
        with self.cursor() as cursor:
            try:
                cursor.execute(
                    f"SELECT version, template FROM {CONFIG_TABLE_NAME} WHERE db = %s",
                    (self.db_name,),
                )
                db_data = cursor.fetchone()

                if not db_data == None:
                    #### Check if the database template matches
                    if not version_source["name"] == db_data[1]:
                        #### If no template is defined, use the current one
                        if db_data[1] == None:
                            cursor.execute(
                                f"UPDATE {CONFIG_TABLE_NAME} SET template = '{version_source['name']}' WHERE db = %s",
                                (self.db_name,),
                            )
                        else:
                            raise DBTemplateNoMatch()
                    database_version = db_data[0]  ## Get database version
            except Exception as e:
                raise e

            #### If no database version is returned from the database, config might be broken
            if database_version == None or database_version == "":
                raise DBConfigIncomplete(missing="version")

            #### Get the latest database version
            latest = max(version_source["version"], key=lambda x: int(x["_id"].replace(".", "")))["_id"]
            database_version_latest = latest if update_to_version == None else update_to_version

            ### Check if database needs to be updated
            try:
                if not int(database_version_latest.replace(".", "")) > int(database_version.replace(".", "")):
                    return "up-to-date"

            #### If db version number can not be formatted to int, it's invalid
            except ValueError:
                raise DBConfigIncomplete(missing="version")

            try:
                #### List to be populated with queries
                query_list = []

                #### Loop through update data
                for version in version_source["version"]:
                    #### Check if version number is larger than current version
                    if int(version["_id"].replace(".", "")) <= int(database_version.replace(".", "")):
                        continue

                    #### Continue if latest version is current
                    if int(database_version_latest.replace(".", "")) < int(version["_id"].replace(".", "")):
                        continue

                    #### Create tables
                    if "createtable" in version:
                        for table in version["createtable"]:
                            query = createtable(version_source=version_source, table_name=table, version=version["_id"])
                            query_list.append(query)

                    #### Alter tables
                    if "altertable" in version:
                        for table in version["altertable"]:
                            query = altertable(version_source=version_source, table_name=table, version=version["_id"])
                            query_list.append(query)

                    #### Insert default data
                    if no_data == False:
                        if "default_data" in version:
                            for table in version["default_data"]:
                                for item in version["default_data"][table]:
                                    query = create_default_data(table_name=table, item=item)
                                    query_list.append(query)
            except KeyError as e:
                raise e
            except Exception as e:
                raise e

            query_list.append(
                (
                    f"UPDATE `{CONFIG_TABLE_NAME}` SET version=%s WHERE `db` = %s",
                    (database_version_latest, self.db_name),
                )
            )

            return query_list

    @conn_test
    @init_test
    def update(self, version_source, update_to_version=None, no_data=False):
        queries = self.update_queries(version_source=version_source, update_to_version=update_to_version, no_data=no_data)

        if queries == "up-to-date":
            return "up-to-date"

        with self.cursor() as cursor:
            for query in queries:
                #### If type is typle, data is passed with the query
                if type(query) == tuple:
                    cursor.execute(*query)
                else:
                    cursor.execute(query)

        return True

    @conn_test
    def vacate(self, confirm=False):
        if not confirm == True:
            raise NeedsConfirmation()

        with self.cursor() as cursor:
            cursor.execute("SET foreign_key_checks = 0;")
            cursor.execute("show tables;")
            tables = cursor.fetchall()

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
