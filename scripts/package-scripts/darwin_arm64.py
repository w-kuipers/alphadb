import os
import shutil
import sys

if len(sys.argv) == 1 or not sys.argv[1][0] == "v":
    raise Exception("No valid version supplied.")

if not sys.argv[1][0] == "v":
    raise Exception("No valid version supplied.")

version = sys.argv[1]

#### Tarball
os.system(f"cd dist/darwin_arm64/alphadb; tar -czvf ../cli/alphadb-cli_{version}_Darwin_arm64.tar.gz .")
