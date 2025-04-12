import os
import shutil
import subprocess
import sys
from os.path import join

sys.path.append(os.path.abspath(os.path.join(os.path.dirname(__file__), "..")))
from utils import get_version_number, replace_line

version = get_version_number()

cwd = os.getcwd()
base_dir = join(cwd, "src/node")
new_dir = join(cwd, "node-dist/")
package_path = os.path.join(cwd, base_dir, "package.json")
cargo_path = os.path.join(cwd, base_dir, "crates/alphadb/Cargo.toml")
postinstalljs_path = os.path.join(cwd, base_dir, "postinstall.mjs")

os.mkdir(new_dir)
new_version_line = f'"version": "{version[1:]}",\n'
cargo_version_line = f'alphadb = "{version[1:]}"\n'
postinstalljs_line = (
    'const BASE_URL = "https://github.com/w-kuipers/alphadb/releases/download/'
    + version
    + '"'
)


replace_line('"version":', new_version_line, package_path)
replace_line("alphadb =", cargo_version_line, cargo_path)
replace_line("const BASE_URL =", postinstalljs_line, postinstalljs_path)

subprocess.Popen(
    ["npm", "install", "--ignore-scripts"], cwd=os.path.join(cwd, base_dir)
).wait()
subprocess.Popen(["tsc"], cwd=os.path.join(cwd, base_dir)).wait()


def mv(file):
    shutil.copy(join(base_dir, file), join(new_dir, file))


def mvd(directory):
    shutil.copytree(join(base_dir, directory), join(new_dir, directory))


mv("package.json")
mv("postinstall.mjs")
mv("LICENSE")
mv("README.md")
mv("Cargo.toml")
mvd("lib")
mvd("crates")
