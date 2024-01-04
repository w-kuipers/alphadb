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

from cli.utils.common import print_title
from cli.utils.connect import get_mysql_creds
from alphadb import AlphaDB, VersionSourceVerification
from alphadb.utils.exceptions import DBTemplateNoMatch, IncompleteVersionData, MissingVersionData, DBConfigIncomplete
from mysql.connector import DatabaseError, InterfaceError
from cryptography.fernet import Fernet
from cli.utils.config import config_get, config_write 
from cli.utils.common import console
from cli.utils.decorators import connection_check
from cli.utils.version_source import select_version_source, vs_path_to_json
from inquirer import prompt, Confirm
from cli.utils import globals

def connect():
    "Connect to a database"

    print_title("connect")
        
    globals.db = AlphaDB()
    creds = get_mysql_creds()

    try:
        globals.db.connect(
            host=creds["host"],
            user=creds["user"],
            password=creds["password"],
            database=creds["database"],
            port=creds["port"],
        )

    except (DatabaseError, InterfaceError) as e:
        if hasattr(e, "msg"): console.print(f"\n[red]{e.msg}[/red]\n")
        else: console.print(f"\n[red]{e}[/red]\n")
        return

    pass_bytes = creds["password"].encode()
    f = Fernet(config_get("CONFIG", "secret"))
    pass_encrypted = f.encrypt(pass_bytes)

    config_write({
        "DB_SESSION": {
            "host": creds["host"],
            "user": creds["user"],
            "password": pass_encrypted,
            "database": creds["database"],
            "port": creds["port"],
        }
    })

    console.print(f'\n[green]Successfully connected to database:[/green] [cyan]"{creds["database"]}"[/cyan]\n')


    return

@connection_check()
def init():
    "Initialize database"
    
    #### Initialize loader
    with console.status("[cyan]Getting the database ready[/cyan]", spinner="bouncingBall") as _:
        init = globals.db.init()

    if init == "already-initialized":
        console.print("[yellow]The database is already initialized[/yellow]\n")
        return

    if init == True:
        console.print("[green]Database successfully initialized[/green]\n")
        return

@connection_check()
def status():
    "Get database status"
    print_title("database status")

    check = globals.db.status()

    print(f'Database: {check["name"]}')
    print(f'Template: {check["template"]}')
    if check["init"] == True:
        console.print("Status: ", "[cyan]Initialized[/cyan]")
    else:
        console.print("Status: ", "[yellow]Uninitialized[/yellow]")
    print(f'Version: {check["version"]}\n')

    return

@connection_check()
def update(nodata=False):
    "Update database"

    print_title("update")

    #### Initialize loader
    with console.status("[cyan]Checking database[/cyan]", spinner="bouncingBall") as loader:

        #### Check database status
        status = globals.db.status()
        if status["init"] == False:
            console.print(f"[yellow]Database [cyan]{status['name']}[/cyan] has not yet been initialized[/yellow]\n")
            return

    try:

        version_source_path = select_version_source()
    
        #### If source path is None, user probably aborted
        if version_source_path == None: return
        
        with console.status("[cyan]Reading version source[/cyan]", spinner="bouncingBall") as loader:
           
            version_information = vs_path_to_json(version_source_path)

            #### If version information is None, an error has likely occured and we should not proceed
            if version_information == None: return

            loader.update("[cyan]Running updates on the database[/cyan]")

            update = globals.db.update(version_source=version_information, no_data=nodata)

            if update:
                if update == "up-to-date":
                    console.print(f"[blue]Database is already the latest version [cyan]({status['version']})[/cyan][/blue]\n")
                    return

                console.print("[green]Database successfully updated to the latest version[/green]\n")
            else:
                console.print("[red]An error occured[/red]\n")

    except (DBTemplateNoMatch, IncompleteVersionData, MissingVersionData, DBConfigIncomplete) as e:
        console.print(f"[red]{e}[/red]\n")

@connection_check()
def vacate(confirm=False):
    "Empty database"
    print_title("vacate")
    
    #### Only works when confirm==True is specified
    if not confirm == True:
        console.print(f"[yellow]The vacate function requires the[/yellow] [red]--confirm[/red] [yellow]option.[/yellow]")
        print("This is a safety feature that hopefully prevents unintended data loss.\n\nDon't worry, you'll still be prompted for confirmation!\n")
        return

    console.print(f"[yellow]The vacate function[/yellow] [red]deletes all data in the database[/red].")
    console.print("[yellow]This action can [red]NOT[/red] be undone.[/yellow]\n")
    
    #### Prompt user for confirmation
    questions = [Confirm("confirm", message="Are you absolutely sure you want to completely delete all data?")]
    answers = prompt(questions)

    #### If answers is None, user likely aborted
    if answers == None: return

    if answers["confirm"]:
        with console.status("[cyan]Removing all data from database[/cyan]", spinner="bouncingBall") as _:

            #### Empty db
            if globals.db.vacate(confirm=confirm):
                console.print("[green]The database had successfully been emptied.[/green]\n")
    else:
        console.print("[cyan]Not empying[/cyan]\n")
    return

def verify_version_source():
    print_title("verify version source")
    
    version_source_path = select_version_source()

    #### If version source path is None, user likely aborted
    if version_source_path == None: return

    version_source = vs_path_to_json(version_source_path)

    #### If version source is None, an error likely occured and we should not proceed
    if version_source == None: return

    verification = VersionSourceVerification(version_source)

    output = verification.verify()

    if output == True:
        console.print(f"[green]Version source at [blue]{version_source_path}[/blue] verified without errors[/green]\n")
        return

    console.print(f"Version source at [blue]{version_source_path}[/blue] has [red]{len(output)} errors[/red]\n\n")

    for issue in sorted(output):
            

        if ":" in issue[1]:
            issue_path, issue_text = issue[1].rsplit(': ', 1)
        else:
            issue_path = ""
            issue_text = issue[1]

        if issue[0] == "LOW":
            console.print("[black on white]LOW VULNERABILITY: [/black on white]", f"[cyan]{issue_path}[/cyan]", issue_text)

        if issue[0] == "HIGH":
            console.print("[white on yellow]HIGH VULNERABILITY: [/white on yellow]", f"[cyan]{issue_path}[/cyan]", f"[yellow]{issue_text}[/yellow]")

        if issue[0] == "CRITICAL":
            console.print("[white on red]CRITICAL: [/white on red]", f"[cyan]{issue_path}[/cyan]", f"[red]{issue_text}[/red]")
        
    console.line()
