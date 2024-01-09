import os
import shutil
import sys

if len(sys.argv) == 1 or not sys.argv[1][0] == "v":
    print("No valid version supplied.")
    version = "dev"
    if not input("Would you like to use version 'dev'? (y/N): ") == "y":
        exit()
else:
    version = sys.argv[1]

with open("src/cli/__init__.py", "r") as i:
    c = i.read()
    with open("src/cli/__init__.py", "w") as i2:
        i2.write(c.replace('"indev"', f'"{version}"'))

os.system("pyinstaller src/cli/__main__.py --name alphadb --onefile")

os.makedirs("temp/deb/DEBIAN")
os.makedirs("temp/deb/usr/local/bin")

os.system("cp dist/alphadb temp/deb/usr/local/bin/alphadb")
with open("temp/deb/DEBIAN/control", "a") as cf:
    cf.write(f"Package: alphadb\nVersion: {version[1:]}\nMaintainer: Wibo Kuipers\nArchitecture: amd64\nDescription: Command line interface for Alphadb MySQL version manager\n")

os.system("dpkg-deb --build temp/deb")

shutil.move("temp/deb.deb", f"dist/alphadb-cli_{version}_Linux_x86_64.deb")

with open("src/cli/__init__.py", "r") as i:
    c = i.read()
    with open("src/cli/__init__.py", "w") as i2:
        i2.write(c.replace(f'"{version}"', '"indev"'))
