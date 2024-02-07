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

from typing import Literal, Optional, Union, get_args

import typer

from alphadb.utils.types import ValidationIssueLevel
from cli import __app_name__, __version__, commands
from cli.utils.common import console

app = typer.Typer()


@app.command(help="Initialize the currently active database")
def init() -> None:
    commands.init()


@app.command(help="Show to status of the currently active database")
def status() -> None:
    commands.status()


@app.command(help="Update the database (requires a version source)")
def update(
    nodata: Optional[bool] = typer.Option(
        False,
        "--no-data",
        help="Update only to the database structure, but do not include default data",
    ),
    verify: Optional[bool] = typer.Option(
        True,
        "--verify",
        help="Wether the version source should be verified before updating",
    ),
    allowed_error_priority: Optional[str] = typer.Option("LOW", "--allowed-error-priority", help="Set the priority level for the verification to cancel the update process"),
) -> None:
    prio: Union[ValidationIssueLevel, Literal["ALL"]] = "LOW"
    if not allowed_error_priority == None:
        levels = [get_args(x)[0] if not x == "ALL" else "ALL" for x in ["ALL", *get_args(ValidationIssueLevel)]]
        prio = allowed_error_priority.upper()  ## LSP will error, but this is checked below

        if not prio in levels:
            console.print(f"[red]{prio} is not a valid priority for --allowed-error-priority[/red]\n")
            console.print("Supported priorities are:")
            for l in levels:
                console.print(f" - {l}")

            print("  ")
            return
    commands.update(nodata=nodata if not nodata == None else False, verify=verify, allowed_error_priority=prio)


@app.command(help="Irriversibally deletes ALL data in the database")
def vacate(
    confirm: Optional[bool] = typer.Option(
        None,
        "--confirm",
        help="Needs to be specified. This is a safety feature. Only with this option specified the vacate function will be called",
    )
) -> None:
    commands.vacate(confirm=confirm if not confirm == None else False)


@app.command(help="Connect to a new database")
def connect() -> None:
    commands.connect()


@app.command("verify", help="Verify version source")
def verify_version_source() -> None:
    commands.verify_version_source()


def version_callback(value: bool) -> None:
    if value:
        typer.echo(f"{__app_name__} {__version__}")
        raise typer.Exit()
    return


@app.callback()
def main(
    version: Optional[bool] = typer.Option(
        None,
        "--version",
        "-V",
        help="Show the application's version and exit.",
        callback=version_callback,
        is_eager=True,
    )
) -> None:
    return
