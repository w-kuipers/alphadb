import os
import shutil
import subprocess
import sys

if len(sys.argv) == 1 or not sys.argv[1][0] == "v":
    print("No valid version supplied.")
    version = "dev"
    if not input("Would you like to use version 'dev'? (y/N): ") == "y":
        exit()
else:
    version = sys.argv[1]

base_dir = "src/node"
package_path = os.path.join(os.getcwd(), base_dir, "package.json")
adb_path = os.path.join(os.getcwd(), "src/alphadb", "Cargo.toml")
node_path = os.path.join(os.getcwd(), base_dir, "crates/alphadb/", "Cargo.toml")
setup_paths = [adb_path, node_path]
node_bin_dir = os.path.join(base_dir, "node-bin")

os.mkdir(node_bin_dir)

new_version_line = f'"version": "{version[1:]}",\n'

with open(package_path, "r") as file:
    lines = file.readlines()

for i, line in enumerate(lines):
    if '"version": "' in line:
        lines[i] = new_version_line

with open(package_path, "w") as file:
    file.writelines(lines)

new_version_line = f'version = "{version[1:]}"\n'
node_version_line = f'version = "{version[1:]}-node"\n'

for path in setup_paths:
    with open(path, "r") as file:
        lines = file.readlines()

    for i, line in enumerate(lines):
        if line.startswith("version ="):

            if path == node_path:
                lines[i] = node_version_line
            else:
                lines[i] = new_version_line

    with open(path, "w") as file:
        file.writelines(lines)

mac = ["darwin-x64", "darwin-arm64"]
win = ["win32-x64-msvc"]
linux = ["linux-x64-gnu"]
# linux = ["linux-x64-gnu", "linux-arm64-gnu"]

mac_r = ["x86_64-apple-darwin", "aarch64-apple-darwin"]
win_r = ["x86_64-pc-windows-msvc"]
linux_r = ["x86_64-unknown-linux-gnu"]
# linux_r = ["x86_64-unknown-linux-gnu", "aarch64-unknown-linux-gnu"]

cwd = os.path.abspath(os.path.join(os.getcwd(), base_dir))

subprocess.Popen(["yarn"], cwd=cwd, shell=True).wait()

if sys.platform == "linux" or sys.platform == "linux2":
    subprocess.Popen(["sudo", "apt", "install", "-y", "gcc-aarch64-linux-gnu"]).wait()
    subprocess.Popen(
        ["sudo", "apt", "install", "-y", "pkg-config", "libssl-dev"]
    ).wait()
    subprocess.Popen(["echo", '"OPENSSL_DIR=/usr/lib/ssl"', ">>", "$GITHUB_ENV"]).wait()
    for i, system in enumerate(linux_r):
        subprocess.Popen(["rustup", "target", "add", system], cwd=cwd).wait()
        subprocess.Popen(["yarn", "build", "--target", system], cwd=cwd).wait()

        shutil.move(
            "src/node/index.node", os.path.join(node_bin_dir, f"{linux[i]}.node")
        )

if sys.platform == "darwin":
    for i, system in enumerate(mac_r):
        subprocess.Popen(["rustup", "target", "add", system], cwd=cwd).wait()
        subprocess.Popen(["yarn", "build", "--target", system], cwd=cwd).wait()

        shutil.move("src/node/index.node", os.path.join(node_bin_dir, f"{mac[i]}.node"))

if sys.platform == "win32":
    for i, system in enumerate(win_r):
        subprocess.Popen(
            ["rustup", "target", "add", system], cwd=cwd, shell=True
        ).wait()
        subprocess.Popen(
            ["yarn", "build", "--target", system], cwd=cwd, shell=True
        ).wait()

        shutil.move("src/node/index.node", os.path.join(node_bin_dir, f"{win[i]}.node"))
