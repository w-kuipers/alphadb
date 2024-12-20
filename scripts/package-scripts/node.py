import os
import shutil
import subprocess
from os.path import join
import sys

if len(sys.argv) == 1 or not sys.argv[1][0] == "v":
    print("No valid version supplied.")
    version = "dev"
    if not input("Would you like to use version 'dev'? (y/N): ") == "y":
        exit()
else:
    version = sys.argv[1]

cwd = os.getcwd()
base_dir = join(cwd, "src/node")
new_dir = join(cwd, "node-dist/")
package_path = os.path.join(cwd, base_dir, "package.json")

os.mkdir(new_dir)
new_version_line = f'"version": "{version[1:]}",\n'

with open(package_path, "r") as file:
    lines = file.readlines()

for i, line in enumerate(lines):
    if '"version": "' in line:
        lines[i] = new_version_line

with open(package_path, "w") as file:
    file.writelines(lines)


subprocess.Popen(["npm", "install"], cwd=os.path.join(cwd, base_dir)).wait()
subprocess.Popen(["tsc"], cwd=os.path.join(cwd, base_dir)).wait()

def mv(file):
    shutil.copy(join(base_dir, file), join(new_dir, file))


def mvd(directory):
    shutil.copytree(join(base_dir, directory), join(new_dir, directory))


mv("package.json")
mv("LICENSE")
mv("README.md")
mvd("lib")
