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

from sys import platform
import os
from pathlib import Path
from cli.utils.types import AlphaDBMock

global CONFIG_PATH
global db
db = AlphaDBMock()

if platform == "linux" or platform == "linux2" or platform == "darwin":
    config_dir = os.path.join(str(Path.home()), ".config/alphadb/")
    if not os.path.exists(config_dir): os.mkdir(config_dir)
    CONFIG_PATH = os.path.join(config_dir, "cli-config.ini")
else:
    CONFIG_PATH = "config.ini"

