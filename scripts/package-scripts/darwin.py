import os
import sys

if len(sys.argv) < 3:
    raise Exception("Both a version and architecture must be specified")

else:
    if not sys.argv[1][0] == "v":
        raise Exception("No valid version supplied.")

    version = sys.argv[1]
    architecture = sys.argv[2]

version = sys.argv[1]

os.makedirs("dist/cli")

#### Tarball
os.system(f"cd dist/darwin_arm64/alphadb; tar -czvf ../../cli/alphadb-cli_{version}_Darwin_{architecture}.tar.gz .")
