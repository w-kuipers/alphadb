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

NODE_PLATFORMS = {
    "darwin": {
        "node": ["darwin-x64", "darwin-arm64"],
        "rust": ["x86_64-apple-darwin", "aarch64-apple-darwin"],
    },
    "linux": {
        "node": ["linux-x64-gnu"],
        "rust": ["x86_64-unknown-linux-gnu"],
    },
    "win32": {
        "node": ["win32-x64-msvc"],
        "rust": ["x86_64-pc-windows-msvc"],
    },
}


def parse_args():
    parser = argparse.ArgumentParser(description="Build Node binaries for AlphaDB.")
    parser.add_argument("version", help='Release version, for example "v1.0.0".')
    parser.add_argument("engine", choices=PACKAGE_NAMES.keys())
    args = parser.parse_args()

    if not args.version.startswith("v"):
        parser.error('version must start with "v"')

    return args


def run(command, cwd=None):
    subprocess.run(command, cwd=cwd, check=True)


def set_github_env(name, value):
    github_env = os.environ.get("GITHUB_ENV")
    if github_env:
        with open(github_env, "a") as f:
            f.write(f"{name}={value}\n")


def update_package_files(paths, version, engine):
    package_version = version[1:]

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
        "version =",
        f'version = "{package_version}"\n',
        str(paths["alphadb_cargo"]),
    )
    replace_line(
        "version =",
        f'version = "{package_version}-node"\n',
        str(paths["node_cargo"]),
    )


def install_linux_dependencies():
    run(["sudo", "apt", "install", "-y", "gcc-aarch64-linux-gnu"])
    run(["sudo", "apt", "install", "-y", "pkg-config", "libssl-dev"])
    set_github_env("OPENSSL_DIR", "/usr/lib/ssl")


def build_platform_binaries(platform, node_dir, node_bin_dir, engine):
    platform_config = NODE_PLATFORMS[platform]

    for node_platform, rust_target in zip(
        platform_config["node"], platform_config["rust"]
    ):
        run(["rustup", "target", "add", rust_target], cwd=node_dir)
        run(["yarn", f"build:{engine}", "--target", rust_target], cwd=node_dir)

        shutil.move(
            "src/node/index.node",
            node_bin_dir / f"{node_platform}-{engine}.node",
        )


def main():
    args = parse_args()
    root_dir = Path.cwd()
    node_dir = root_dir / "src/node"
    node_bin_dir = root_dir / "src/node/node-bin"
    paths = {
        "package": node_dir / "package.json",
        "alphadb_cargo": root_dir / "src/alphadb/Cargo.toml",
        "node_cargo": node_dir / "crates/alphadb/Cargo.toml",
    }

    node_bin_dir.mkdir()

    print(args.version)
    print(f'version = "{args.version[1:]}-node"')

    update_package_files(paths, args.version, args.engine)
    run(["yarn"], cwd=node_dir)

    platform = "linux" if sys.platform == "linux2" else sys.platform

    if platform == "linux":
        install_linux_dependencies()

    if platform in NODE_PLATFORMS:
        build_platform_binaries(platform, node_dir, node_bin_dir, args.engine)


if __name__ == "__main__":
    main()
