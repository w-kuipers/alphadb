import os
import sys
import shutil

if len(sys.argv) < 3:
    raise Exception("Both a version and architecture must be specified")

else:
    if not sys.argv[1][0] == "v":
        raise Exception("No valid version supplied.")

    version = sys.argv[1]
    architecture = sys.argv[2]

    with open("src/cli/__init__.py", "r") as i:
        c = i.read()
        with open("src/cli/__init__.py", "w") as i2:
            i2.write(c.replace('"indev"', f'"{version}"'))

    os.system(f"pyinstaller src/cli/__main__.py --name alphadb-{architecture} --onefile")
    os.mkdir(f"dist/linux_{architecture}")
    shutil.move(f"dist/alphadb-{architecture}", f"dist/linux_{architecture}/alphadb")
