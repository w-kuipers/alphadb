import argparse
import os
import shutil
import subprocess
import sys
from pathlib import Path

sys.path.append(os.path.abspath(os.path.join(os.path.dirname(__file__), "..")))
from utils import replace_line


PACKAGE_NAMES = {
    "mysql": "@w-kuipers/alphadb-mysql",
    "postgres": "@w-kuipers/alphadb-postgres",
}

PACKAGE_FILES = [
    "package.json",
    "postinstall.mjs",
    "LICENSE",
    "README.md",
    "Cargo.toml",
]
PACKAGE_DIRECTORIES = ["lib", "crates"]


def parse_args():
    parser = argparse.ArgumentParser(description="Create the Node package for AlphaDB.")
    parser.add_argument("version", help='Release version, for example "v1.0.0".')
    parser.add_argument("engine", choices=PACKAGE_NAMES.keys())
    args = parser.parse_args()

    if not args.version.startswith("v"):
        parser.error('version must start with "v"')

    return args


def run(command, cwd=None):
    subprocess.run(command, cwd=cwd, check=True)


def update_package_files(paths, version, engine):
    package_version = version[1:]
    release_url = f'https://github.com/w-kuipers/alphadb/releases/download/{version}'

    replace_line(
        '"name":',
        f'\t"name": "{PACKAGE_NAMES[engine]}",\n',
        str(paths["package"]),
    )
    replace_line(
        '"version":',
        f'\t"version": "{package_version}",\n',
        str(paths["package"]),
    )
    replace_line(
        '"engine":',
        f'\t\t"engine": "{engine}"\n',
        str(paths["package"]),
    )
    replace_line(
        "alphadb =",
        f'alphadb = "{package_version}"\n',
        str(paths["node_cargo"]),
    )
    replace_line(
        "const BASE_URL =",
        f'const BASE_URL = "{release_url}";\n',
        str(paths["postinstall"]),
    )


def build_typescript(node_dir):
    run(["npm", "install", "--ignore-scripts"], cwd=node_dir)
    run(["tsc"], cwd=node_dir)


def copy_package_contents(node_dir, dist_dir):
    for file in PACKAGE_FILES:
        shutil.copy(node_dir / file, dist_dir / file)

    for directory in PACKAGE_DIRECTORIES:
        shutil.copytree(node_dir / directory, dist_dir / directory)


def main():
    args = parse_args()
    root_dir = Path.cwd()
    node_dir = root_dir / "src/node"
    dist_dir = root_dir / "node-dist"
    paths = {
        "package": node_dir / "package.json",
        "node_cargo": node_dir / "crates/alphadb/Cargo.toml",
        "postinstall": node_dir / "postinstall.mjs",
    }

    dist_dir.mkdir()

    update_package_files(paths, args.version, args.engine)
    build_typescript(node_dir)
    copy_package_contents(node_dir, dist_dir)


if __name__ == "__main__":
    main()
