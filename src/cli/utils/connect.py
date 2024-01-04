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

import sys
from typing import TypedDict

import inquirer

from cli.utils.common import console


class MySQLCredentials(TypedDict):
    host: str
    user: str
    password: str
    database: str
    port: int


def check_port(val):
    "Check if port is a number"
    try:
        int(val if val else 3306)
    except ValueError:
        return False
    return True


def get_mysql_creds() -> MySQLCredentials:
    "Prompt user to input credentials"
    console.print(
        "[cyan]Connecting to a MySQL database requires you to provide log-in credentials[/cyan]\n"
    )
    console.print("[cyan]Host:[/cyan]", "URL/IP (Default localhost)")
    console.print("[cyan]User:[/cyan]", "User with permissions to alter the database")
    console.print("[cyan]Password:[/cyan]", "The users password")
    console.print("[cyan]Database:[/cyan]", "The name of the database to connect to")
    console.print("[cyan]Port:[/cyan]", "(Default 3306)\n\n")

    questions = [
        inquirer.Text("host", message="Host (localhost)"),
        inquirer.Text("user", message="User"),
        inquirer.Password("password", message="Password"),
        inquirer.Text("database", message="Database"),
        inquirer.Text(
            "port", message="Port (3306)", validate=lambda _, x: check_port(x)
        ),
    ]

    answers = inquirer.prompt(questions)

    #### If answers is None, user probably aborted
    if answers == None:
        sys.exit()  ## Exit because returning raises error as there is no credentials returned

    return {
        "host": answers["host"] if not answers["host"] == "" else "localhost",
        "user": answers["user"],
        "password": answers["password"],
        "database": answers["database"],
        "port": int(answers["port"]) if not answers["port"] == "" else 3306,
    }
