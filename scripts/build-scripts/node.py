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

if not os.path.isdir("temp/node"):
    os.makedirs("temp/node")

os.system("pyinstaller src/node/wrapper.py --name pywrapper --onefile")

shutil.move("dist/pywrapper", "temp/node/pywrapper")

#### Change version number in package.json
with open("src/node/package.json", "r") as fr:
    version_updated = fr.read().replace('"indev"', f'"{version[1:]}"')
    with open("temp/node/package.json", "w") as fw:
        fw.write(version_updated)

shutil.copy("src/node/index.ts", "temp/node/index.ts")
shutil.copy("src/node/tsconfig.json", "temp/node/tsconfig.json")
shutil.copy("LICENSE", "temp/node/LICENSE")
shutil.copy("src/node/.npmignore", "temp/node/.npmignore")
shutil.copy("readme/npm.md", "temp/node/README.md")
# shutil.copy("src/node/.npmrc", "temp/node/.npmrc")

os.system("cd temp/node; yarn; yarn build")
