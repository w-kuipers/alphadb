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
cli_path = os.path.join(os.getcwd(), "src/cli", "Cargo.toml")
setup_paths = [setup_path, cli_path]

new_version_line = f'version = "{version[1:]}"\n'
dep_line = f'alphadb = "{version[1:]}"\n'

for path in setup_paths:
    with open(path, "r") as file:
        lines = file.readlines()

    for i, line in enumerate(lines):
        if line.startswith("version ="):
            lines[i] = new_version_line

    with open(path, "w") as file:
        file.writelines(lines)

## replace AlphaDB dependency for Cargo
with open(cli_path, "r") as file:
    lines = file.readlines()

for i, line in enumerate(lines):
    if 'alphadb = { path = "../alphadb" }' in line:
        lines[i] = dep_line

with open(cli_path, "w") as file:
    file.writelines(lines)
