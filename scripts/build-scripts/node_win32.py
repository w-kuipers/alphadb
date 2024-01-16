import os
import sys

if len(sys.argv) == 1 or not sys.argv[1][0] == "v":
    raise Exception("No valid version supplied.")

if not sys.argv[1][0] == "v":
    raise Exception("No valid version supplied.")

version = sys.argv[1]

os.system(f"pyinstaller src/node/wrapper.py --distpath dist/node --name pywrapper_win32_x86_64")
