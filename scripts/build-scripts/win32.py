import os
import sys
import shutil

if len(sys.argv) == 1 or not sys.argv[1][0] == "v":
    raise Exception("No valid version supplied.")
version = sys.argv[1]

if not sys.argv[1][0] == "v":
    raise Exception("No valid version supplied.")

version = sys.argv[1]

with open("src/cli/__init__.py", "r") as i:
    c = i.read()
    with open("src/cli/__init__.py", "w") as i2:
        i2.write(c.replace('"indev"', f'"{version}"'))

os.system(f"pyinstaller src/cli/__main__.py --name alphadb")
os.mkdir(f"dist/win32")
shutil.move(f"dist/alphadb", f"dist/win32/alphadb")
