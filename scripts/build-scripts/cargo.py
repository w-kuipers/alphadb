import os
import subprocess
import sys

if len(sys.argv) == 1 or not sys.argv[1][0] == "v":
    print("No valid version supplied.")
    version = "dev"
    if not input("Would you like to use version 'dev'? (y/N): ") == "y":
        exit()
else:
    version = sys.argv[1]

src_path = "src/alphadb"
setup_path = os.path.join(os.getcwd(), src_path, "Cargo.toml")

new_version_line = f'version = "{version}"\n'

with open(setup_path, "r") as file:
    lines = file.readlines()

for i, line in enumerate(lines):
    if line.startswith("version ="):
        lines[i] = new_version_line

with open(setup_path, "w") as file:
    file.writelines(lines)
