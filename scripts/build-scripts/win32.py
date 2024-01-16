import os
import shutil
import sys

if len(sys.argv) == 1 or not sys.argv[1][0] == "v":
    raise Exception("No valid version supplied.")

if not sys.argv[1][0] == "v":
    raise Exception("No valid version supplied.")

version = sys.argv[1]

if not os.path.isdir("dist"):
    os.mkdir("dist")

with open("src/cli/__init__.py", "r") as i:
    c = i.read()
    with open("src/cli/__init__.py", "w") as i2:
        i2.write(c.replace('"indev"', f'"{version}"'))

os.system(f"pyinstaller src/cli/__main__.py --name alphadb")
shutil.move(f"dist/alphadb", f"dist/win32")
shutil.copy("README.md", "dist/win32/README.md")
shutil.copy("LICENSE", "dist/win32/LICENSE")
