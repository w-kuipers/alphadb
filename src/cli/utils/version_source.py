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

import json
import os

from cli.utils.config import config_get, config_write, config_get_items
from requests import get
from requests.exceptions import JSONDecodeError, ConnectionError
from inquirer import Text, prompt, List, Path
from inquirer.errors import ValidationError
from cli.utils.common import console, clear

def select_version_source() -> str | None:
    "Get version sources from config and promt user to select one"
    version_sources = config_get_items("VERSION_SOURCES")
    version_source_path = None

    #### If no versions sources exist, ask to create one
    if len(version_sources) == 0:
        console.print("[cyan]You have no saved version sources, so let's find one.[/cyan]\n")
        version_source_path = add_version_source()
        print("\n")
     
    #### Ask user to select version source
    if version_source_path == None:

        choices = [f"{i[0]} ({'web' if i[1][0:4] == 'http' else 'file'})" for i in version_sources]
        choices.append("+ New version source")
        questions = [List("version_source", message=f"Found {len(version_sources)} version sources, which one do you wish to use?", choices=choices)]

        answers = prompt(questions)

        #### If answers is None, user probably aborted
        if answers == None: return

        version_source = answers["version_source"]

        clear()

        if version_source == "+ New version source":
            version_source_path = add_version_source()
        else:
            version_source_path = config_get("VERSION_SOURCES")[version_source.split(" ")[0]]

        return version_source_path


def add_version_source() -> str | None:
    "Adding a new version source to config"


    print(
        "Version sources can either be local JSON files or URL's returning JSON data.\n"
    )

    try:  ## To catch ConnectionError


        def pathval(path: str):
            if path[0:4] == "http":
                r = get(path)
                if not r.status_code == 200:
                    raise ValidationError(
                        "", reason=f"URL responded with {r.status_code}"
                    )
                return True

            if not os.path.isfile(path):
                raise ValidationError(
                    "", reason="This path does not point towards a JSON file."
                )
            return True

        questions = [
            Path(
                "path",
                message="Please provide either an absolute local file path or a local/remote URL",
                validate=lambda _, x: pathval(x),
            )
        ]
        answers = prompt(questions)

        #### If answers is None, user probably aborted
        if answers == None:
            return

        path = answers["path"]

        if path[0:4] == "http":
            template_name = get(path).json()["name"]
        else:
            with open(path) as version_json:
                template_name = json.load(version_json)["name"]

    #### TODO This connection error catch does not seem to work
    except ConnectionError:
        console.print("\n[red]Unable to establish connection[/red]\n")
        return

    except JSONDecodeError:
        console.print(
            "\n[red]The supplied URL did not respond with compatible data[/red]\n"
        )
        return

    ##### TODO maybe add a check if the data has no errors

    questions = [Text("name", message="Name the template", default=template_name)]
    answers = prompt(questions)

    #### If answers is None, user probably aborted
    if answers == None:
        return

    name = answers["name"]

    vs = config_get("VERSION_SOURCES", fallback=True)
    if vs == None:
        vs = {}
    vs[name] = path
    config_write({"VERSION_SOURCES": vs})

    return path

def vs_path_to_json(path) -> dict | None:
    
    #### Get version invormation from path
    if path[0:4] == "http":
        try:
            r = get(path)
            if not r.status_code == 200:
                console.print(f"[red]No version information was returned from the server, it instead responded with status {r.status_code}[/red]\n")
                return


            return r.json()

        #### TODO This connection error catch does not seem to work  
        except ConnectionError:
            console.print("[red]Unable to establish connection[/red]\n")
            return

        except JSONDecodeError:
            console.print("[red]The supplied URL did not respond with compatible data[/red]\n")
            return
    else:
        with open(path) as version_json:
            return json.load(version_json)
