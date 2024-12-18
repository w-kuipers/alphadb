import os
import shutil
import sys
from os.path import join

#### Add all binaries (should also be defined in package.json)
archs = ["linux_x86_64", "win32_x86_64", "darwin_x86_64"]
cwd = os.getcwd()
base_dir = join(cwd, "src/node")
new_dir = join(cwd, "node-dist/")

os.mkdir(new_dir)


def mv(file):
    shutil.copy(join(base_dir, file), join(new_dir, file))


def mvd(directory):
    shutil.copytree(join(base_dir, directory), join(new_dir, directory))


mv("package.json")
mv("LICENSE")
mv("README.md")
mvd("lib")
