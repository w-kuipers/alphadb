import os
import sys

if len(sys.argv) == 1 or not sys.argv[1][0] == "v":
    raise Exception("No valid version supplied.")

if not sys.argv[1][0] == "v":
    raise Exception("No valid version supplied.")

version = sys.argv[1]

#### Create zip file
os.system(f"cd dist; tar -acf alphadb-cli_{version}_Windows_x86_64.zip win32")
