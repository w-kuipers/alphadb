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

import os
from configparser import ConfigParser
from typing import Callable

from alphadb import AlphaDB
from cryptography.fernet import Fernet
from simpleUID import urlsafe

from cli.utils import globals
from cli.utils.common import console
from cli.utils.config import config_get


def config_check(func: Callable) -> Callable:
    "Initialize config when it's not yet initialized"

    def _(*args, **kwargs):
        config = ConfigParser()
        if os.path.isfile(globals.CONFIG_PATH):
            config.read(globals.CONFIG_PATH)

        if not "CONFIG" in config:
            config["CONFIG"] = {}
        if not "secret" in config["CONFIG"]:
            config["CONFIG"]["secret"] = urlsafe("padding")

        with open(globals.CONFIG_PATH, "w+") as configfile:
            config.write(configfile)

        return func(*args, **kwargs)

    return _


def connection_check() -> Callable:
    "Check if a database connection exists"

    def _(func: Callable) -> Callable:
        def _wrapper(*args, **kwargs):
            if globals.db.connection == None:
                conn = config_get("DB_SESSION", fallback=True)
                main_config = config_get("CONFIG")
                secret = main_config["secret"] if "secret" in main_config else None

                if conn and secret:
                    globals.db = AlphaDB()

                    try:
                        globals.db.connect(
                            host=conn["host"],
                            user=conn["user"],
                            password=Fernet(secret)
                            .decrypt(conn["password"][1:-1])
                            .decode(),  ## Decrypt
                            database=conn["database"],
                            port=conn["port"],
                        )

                        globals.db.connection.autocommit = True
                    except:
                        console.print(
                            "[red]Unable to authorize using saved credentials. Please reconnect to the database.[/red]"
                        )
                        return

                    return func(*args, **kwargs)

                console.print("\n[yellow]No database connection found.[/yellow]\n")

        return _wrapper

    return _
