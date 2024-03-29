import os
import shutil
import sys

if len(sys.argv) == 1 or not sys.argv[1][0] == "v":
    raise Exception("No valid version supplied.")

if not sys.argv[1][0] == "v":
    raise Exception("No valid version supplied.")

version = sys.argv[1]

os.makedirs("dist/cli")

#### Tarball
shutil.copy("README.md", "dist/linux_x86_64/README.md")
shutil.copy("LICENSE", "dist/linux_x86_64/LICENSE")
os.system(f"cd dist/linux_x86_64; tar -czvf ../cli/alphadb-cli_{version}_Linux_x86_64.tar.gz alphadb README.md LICENSE")

#### DEB package
os.makedirs("temp/deb/DEBIAN")
os.makedirs("temp/deb/usr/local/bin")
os.system("cp dist/linux_x86_64/alphadb temp/deb/usr/local/bin/alphadb")
with open("temp/deb/DEBIAN/control", "a") as cf:
    cf.write(f"Package: alphadb\nVersion: {version[1:]}\nMaintainer: Wibo Kuipers\nArchitecture: amd64\nDescription: Command line interface for Alphadb MySQL version manager\n")
os.system("dpkg-deb --build temp/deb")
shutil.move("temp/deb.deb", f"dist/cli/alphadb-cli_{version}_Linux_x86_64.deb")
