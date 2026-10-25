import argparse
import hashlib
from pathlib import Path

RELEASE_URL = "https://github.com/w-kuipers/alphadb/releases/download"
TARGETS = ["Darwin-aarch64", "Darwin-x86_64", "Linux-x86_64"]


def parse_args():
    parser = argparse.ArgumentParser(description="Update the Homebrew formula and AUR package for a release.")
    parser.add_argument("version", help='Release version, for example "v1.0.0".')
    parser.add_argument("--dist", default="dist", help="Directory containing the CLI tarballs.")
    parser.add_argument("--homebrew", help="Checkout of the homebrew-alphadb tap.")
    parser.add_argument("--aur", help="Checkout of the alphadb-bin AUR repository.")
    args = parser.parse_args()

    if not args.version.startswith("v"):
        parser.error('version must start with "v"')

    return args


def tarball_name(version, target):
    return f"alphadb-cli_{version}_{target}.tar.gz"


def checksums(dist, version):
    sums = {}
    for target in TARGETS:
        path = Path(dist) / tarball_name(version, target)
        sums[target] = hashlib.sha256(path.read_bytes()).hexdigest()
    return sums


def write_homebrew(path, version, sums):
    formula = f'''class Alphadb < Formula
  desc "SQL Database Version Management"
  homepage "https://alphadb.w-kuipers.com/"
  license "GPL-3.0-or-later"

  version "{version[1:]}"

  on_macos do
    on_arm do
      url "{RELEASE_URL}/v#{{version}}/alphadb-cli_v#{{version}}_Darwin-aarch64.tar.gz"
      sha256 "{sums["Darwin-aarch64"]}"
    end
    on_intel do
      url "{RELEASE_URL}/v#{{version}}/alphadb-cli_v#{{version}}_Darwin-x86_64.tar.gz"
      sha256 "{sums["Darwin-x86_64"]}"
    end
  end

  on_linux do
    url "{RELEASE_URL}/v#{{version}}/alphadb-cli_v#{{version}}_Linux-x86_64.tar.gz"
    sha256 "{sums["Linux-x86_64"]}"
  end

  def install
    libexec.install Dir["*"]
    bin.write_exec_script libexec/"alphadb"
  end

  test do
    system "alphadb", "--version"
  end
end
'''
    (Path(path) / "alphadb.rb").write_text(formula)


def write_aur(path, version, sums):
    # pacman forbids hyphens in pkgver, so "1.0.0-beta.1" becomes "1.0.0_beta.1".
    pkgver = version[1:].replace("-", "_")
    sha = sums["Linux-x86_64"]
    source = f"{RELEASE_URL}/{version}/{tarball_name(version, 'Linux-x86_64')}"

    pkgbuild = f'''# Maintainer: Wibo Kuipers <wibo@w-kuipers.com>
pkgname=alphadb-bin
pkgver={pkgver}
pkgrel=1
pkgdesc="SQL Database Version Management"
arch=('x86_64')
url="https://alphadb.w-kuipers.com/"
license=('GPL-3.0-or-later')
provides=('alphadb')
conflicts=('alphadb')
source_x86_64=("${{pkgname}}-${{pkgver}}.tar.gz::{RELEASE_URL}/v${{pkgver//_beta./-beta.}}/alphadb-cli_v${{pkgver//_beta./-beta.}}_Linux-x86_64.tar.gz")
sha256sums_x86_64=('{sha}')

package() {{
    install -Dm755 "${{srcdir}}/alphadb" "${{pkgdir}}/usr/bin/alphadb"
}}
'''

    srcinfo = f'''pkgbase = alphadb-bin
\tpkgdesc = SQL Database Version Management
\tpkgver = {pkgver}
\tpkgrel = 1
\turl = https://alphadb.w-kuipers.com/
\tarch = x86_64
\tlicense = GPL-3.0-or-later
\tprovides = alphadb
\tconflicts = alphadb
\tsource_x86_64 = alphadb-bin-{pkgver}.tar.gz::{source}
\tsha256sums_x86_64 = {sha}

pkgname = alphadb-bin
'''
    (Path(path) / "PKGBUILD").write_text(pkgbuild)
    (Path(path) / ".SRCINFO").write_text(srcinfo)


def main():
    args = parse_args()
    sums = checksums(args.dist, args.version)

    for target, sha in sums.items():
        print(f"{target}: {sha}")

    if args.homebrew:
        write_homebrew(args.homebrew, args.version, sums)
    if args.aur:
        write_aur(args.aur, args.version, sums)


if __name__ == "__main__":
    main()
