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

from configparser import ConfigParser, SectionProxy
from cli.utils.exceptions import ConfigIncoplete
from cli.utils.globals import CONFIG_PATH
from typing import overload, Literal

config = ConfigParser()

@overload
def config_get(section: str, key: str) -> str: ...

@overload
def config_get(section: str, key: str, fallback: Literal[True]) -> None: ...

@overload
def config_get(section: str) -> SectionProxy: ...

@overload
def config_get(section:str, *, fallback: Literal[True]) -> None: ...

"Get data from config file"
def config_get(section: str, key: str | None = None, fallback: bool = False) -> SectionProxy | str | None:
    
    config.read(CONFIG_PATH)

    try:
        if key == None:
            return config[section]
        
        return config[section][key]
    except KeyError:
        if fallback: return None
        raise ConfigIncoplete()


"Write to config file"
def config_write(data: dict | list) -> None:

    config.read(CONFIG_PATH)

    #### Loop through root level dict keys
    for key in data:
        config[key] = data[key]

    #### Write changes to config file
    with open(CONFIG_PATH, "w") as configfile:
        config.write(configfile)

    return

"Remove from config file"
def config_remove(section: str, key: str | None = None):

    if key == None:
        config.remove_section(section)
    else:
        config.remove_option(section, key)

    #### Write changes to config file
    with open(CONFIG_PATH, 'w') as configfile:
        config.write(configfile)

    return

"Get all keys of available items in config section"
def config_get_items(key: str) -> list:

    config.read(CONFIG_PATH)

    try:
        return config.items(key)
    except:
        return []
