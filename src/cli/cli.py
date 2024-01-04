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

from typing import Optional
import typer
from cli import __app_name__, __version__, commands

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
    )
) -> None:
    commands.update(nodata=nodata if not nodata == None else False)

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
        typer.echo(f"{__app_name__} v{__version__}")
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
