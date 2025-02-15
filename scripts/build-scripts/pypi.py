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

src_path = "src/py"
setup_path = os.path.join(os.getcwd(), src_path, "pyproject.toml")

new_version_line = f'version = "{version[1:]}"\n'

with open(setup_path, "r") as file:
    lines = file.readlines()

for i, line in enumerate(lines):
    if "version = " in line:
        lines[i] = new_version_line

with open(setup_path, "w") as file:
    file.writelines(lines)

subprocess.Popen(["maturin", "build", "--sdist", "--release"], cwd=os.path.join(os.getcwd(), src_path)).wait()

with open(setup_path, "r") as i:
    c = i.read()
    with open(setup_path, "w") as i2:
        i2.write(c.replace(f'"{version}"', '"indev"'))
