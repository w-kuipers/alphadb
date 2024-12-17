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

with open(setup_path, "r") as i:
    c = i.read()
    with open(setup_path, "w") as i2:
        i2.write(c.replace('"indev"', f'"{version}"'))

# os.system("maturin build --release")

if sys.platform == "linux" or sys.platform == "linux2":
    # docker_command = [
    #     "docker",
    #     "run",
    #     "--rm",
    #     "-v",
    #     "./:/io",  # Replace with your local project path
    #     "-w",
    #     "/io",
    #     "quay.io/pypa/manylinux2014_x86_64",
    #     "/bin/bash",
    #     "-c",
    #     """
    #         cd src/py\
    #
    #         curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && \
    #         source $HOME/.cargo/env && \
    #
    #         /opt/python/cp310-cp310/bin/pip install maturin && \
    #         /opt/python/cp310-cp310/bin/maturin build --release --manylinux 2014
    #     """,
    # ]
    # subprocess.Popen(docker_command).wait()

    subprocess.Popen(["maturin", "build", "--release"], cwd=os.path.join(os.getcwd(), src_path)).wait()
else:
    subprocess.Popen(["maturin", "build", "--release", "--manylinux", "off"], cwd=os.path.join(os.getcwd(), src_path)).wait()

subprocess.Popen(["maturin", "build", "--sdist", "--release"], cwd=os.path.join(os.getcwd(), src_path)).wait()

with open(setup_path, "r") as i:
    c = i.read()
    with open(setup_path, "w") as i2:
        i2.write(c.replace(f'"{version}"', '"indev"'))
