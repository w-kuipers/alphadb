import os
import shutil
import sys

if len(sys.argv) == 1 or not sys.argv[1][0] == "v":
    raise Exception("No valid version supplied.")

if not sys.argv[1][0] == "v":
    raise Exception("No valid version supplied.")

version = sys.argv[1]

#### Create zip file
os.system(f"cd {os.path.join(os.getcwd(), "dist/win32")} && tar -czf alphadb-cli_{version}_Windows_x86_64.zip --exclude=alphadb-cli_{version}_Windows_x86_64.zip .")
shutil.move(f"dist/win32/alphadb-cli_{version}_Windows_x86_64.zip", f"dist/alphadb-cli_{version}_Windows_x86_64.zip")
