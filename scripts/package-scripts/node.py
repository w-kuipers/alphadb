import os
import shutil
import sys

if len(sys.argv) == 1 or not sys.argv[1][0] == "v":
    raise Exception("No valid version supplied.")

if not sys.argv[1][0] == "v":
    raise Exception("No valid version supplied.")

version = sys.argv[1]

if not os.path.isdir("temp/node"):
    os.makedirs("temp/node")

#### Add all binaries
archs = ["linux_x86_64", "win32_x86_64", "darwin_x86_64"]

for arch in archs:
    if os.path.isdir(f"dist/pywrapper_{arch}"):
        shutil.move(f"dist/pywrapper_{arch}", f"temp/node/pywrapper_{arch}")

#### Change version number in package.json
with open("src/node/package.json", "r") as fr:
    version_updated = fr.read().replace('"indev"', f'"{version[1:]}"')
    with open("temp/node/package.json", "w") as fw:
        fw.write(version_updated)

shutil.copy("src/node/index.ts", "temp/node/index.ts")
shutil.copy("src/node/alphadb.ts", "temp/node/alphadb.ts")
shutil.copy("src/node/tsconfig.json", "temp/node/tsconfig.json")
shutil.copy("LICENSE", "temp/node/LICENSE")
shutil.copy("src/node/.npmignore", "temp/node/.npmignore")
shutil.copy("readme/npm.md", "temp/node/README.md")

os.system("cd temp/node; yarn; yarn build")
