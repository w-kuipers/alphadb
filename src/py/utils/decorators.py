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


from collections.abc import Callable

from .exceptions import DBNotInitialized, NoConnection


#### Checks for database connection
def conn_test(func: Callable) -> Callable:
    def _(inst, *args, **kwargs):
        if inst.connection == None:
            raise NoConnection()  ## Raise
        else:
            return func(inst, *args, **kwargs)

    return _


#### Wraps around check function, if db is not initialized some functions should not be called
def init_test(func: Callable) -> Callable:
    def _(self, *args, **kwargs):
        if self.check()["check"]:
            return func(self, *args, **kwargs)
        else:
            raise DBNotInitialized()

    return _
