import argparse
import os
import shutil
import subprocess
import sys
from pathlib import Path

sys.path.append(os.path.abspath(os.path.join(os.path.dirname(__file__), "..")))
from utils import replace_line


PACKAGE_NAMES = {
    "mysql": "alphadb-mysql",
    "postgres": "alphadb-postgres",
}


def parse_args():
    parser = argparse.ArgumentParser(description="Build Python wheels for AlphaDB.")
    parser.add_argument("version", help='Release version, for example "v1.0.0".')
    parser.add_argument("engine", choices=PACKAGE_NAMES.keys())
    args = parser.parse_args()

    if not args.version.startswith("v"):
        parser.error('version must start with "v"')

    return args


def run(command, cwd=None):
    subprocess.run(command, cwd=cwd, check=True)


def update_package_files(paths, version):
    package_version = version[1:]

    replace_line(
        "version =",
        f'version = "{package_version}"\n',
        str(paths["pyproject"]),
    )


def main():
    args = parse_args()
    root_dir = Path.cwd()
    py_dir = root_dir / "src/py"
    package_dir = py_dir / "packages" / args.engine
    wheels_dir = py_dir / "target/wheels"
    paths = {
        "pyproject": package_dir / "pyproject.toml",
    }

    wheels_dir.mkdir(parents=True, exist_ok=True)
    shutil.copyfile(py_dir / "LICENSE", package_dir / "LICENSE")

    print(args.version)
    print(f'package = "{PACKAGE_NAMES[args.engine]}"')
    print(f'version = "{args.version[1:]}"')

    update_package_files(paths, args.version)
    run(["maturin", "build", "--sdist", "--release", "--out", str(wheels_dir)], cwd=package_dir)


if __name__ == "__main__":
    main()
