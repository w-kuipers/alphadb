import os
import sys

if len(sys.argv) < 3:
    raise Exception("Both a version and architecture must be specified")

else:
    if not sys.argv[1][0] == "v":
        raise Exception("No valid version supplied.")

    version = sys.argv[1]
    architecture = sys.argv[2]

os.system(f"pyinstaller src/node/wrapper.py --name pywrapper_linux_{architecture}")
