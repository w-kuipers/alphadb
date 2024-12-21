import os
import shutil
import subprocess
import sys
from os.path import join

if len(sys.argv) == 1 or not sys.argv[1][0] == "v":
    print("No valid version supplied.")
    version = "dev"
    if not input("Would you like to use version 'dev'? (y/N): ") == "y":
        exit()
else:
    version = sys.argv[1]

src_path = "src/cli"
cwd = os.getcwd()
base_dir = join(cwd, src_path)
setup_path = os.path.join(cwd, src_path, "Cargo.toml")
new_dir = join(cwd, "alphadb")
adb_path = join(cwd, "src/alphadb", "Cargo.toml")
setup_paths = [adb_path, setup_path]
dist_path = join(cwd, "dist")

if not os.path.exists(dist_path):
    os.mkdir(dist_path)

new_version_line = f'version = "{version[1:]}"\n'

for path in setup_paths:
    with open(path, "r") as file:
        lines = file.readlines()

    for i, line in enumerate(lines):
        if line.startswith("version ="):
            lines[i] = new_version_line

    with open(path, "w") as file:
        file.writelines(lines)


def mv(file, dst=None):
    if dst == None:
        dst = file

    shutil.copy(join(base_dir, file), join(new_dir, dst))


targets = []
target_names = []
system = ""

if sys.platform == "darwin":
    system = "Darwin"
    targets = ["x86_64-apple-darwin", "aarch64-apple-darwin"]
    target_names = ["x86_64", "aarch64"]

if sys.platform.startswith("linux"):
    system = "Linux"
    # targets = ["x86_64-unknown-linux-gnu", "aarch64-unknown-linux-gnu"]
    # target_names = ["x86_64", "aarch64"]
    targets = ["x86_64-unknown-linux-gnu"]
    target_names = ["x86_64"]

if sys.platform == "win32":
    system = "Windows"
    targets = ["x86_64-pc-windows-msvc", "i686-pc-windows-msvc"]
    target_names = ["x86_64", "i686"]


for i, target in enumerate(targets):
    if not os.path.exists(new_dir):
        os.mkdir(new_dir)

    name = f"alphadb-cli_{version}_{system}-{target_names[i]}"

    subprocess.Popen(["rustup", "target", "add", target], cwd=base_dir).wait()

    if target == "aarch64-unknown-linux-gnu":
        subprocess.Popen(["docker", "build", "-f", join(cwd, "scripts/build-scripts/rust-arm-build.dockerfile"), "-t", "rust-arm-build", "."], cwd=base_dir).wait()
        subprocess.Popen(["docker", "run", "--rm", "-v", f"{cwd}:/app", "rust-arm-build"], cwd=base_dir).wait()
    else:
        subprocess.Popen(["cargo", "build", "--release", "--target", target], cwd=base_dir).wait()

    mv("LICENSE")
    mv("README.md")
    if sys.platform == "linux" or sys.platform == "linux2" or sys.platform == "darwin":
        mv(f"target/{target}/release/alphadb-cli", "alphadb")
        subprocess.Popen(["tar", "-czvf", "release.tar.gz", "alphadb"], cwd=new_dir).wait()
        shutil.move(join(new_dir, "release.tar.gz"), join(cwd, f"dist/{name}.tar.gz"))
    else:
        mv(f"target/{target}/release/alphadb-cli.exe", "alphadb.exe")
        shutil.make_archive(base_name="release", format="zip", root_dir=new_dir)
        shutil.move(join("release.zip"), join(cwd, f"dist/{name}.zip"))

    shutil.rmtree(new_dir)
