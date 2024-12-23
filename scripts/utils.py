import sys


def replace_line(includes: str, new: str, file: str):
    with open(file, "r") as f:
        lines = f.readlines()

    for i, line in enumerate(lines):
        if includes in line:
            lines[i] = new

    with open(file, "w") as f:
        f.writelines(lines)


def get_version_number() -> str:
    if len(sys.argv) == 1 or not sys.argv[1][0] == "v":
        print("No valid version supplied.")
        exit()

    return sys.argv[1]
