import os
import sys

if len(sys.argv) == 1 or not sys.argv[1][0] == "v":
    print("No valid version supplied.")
    version = "dev"
    if not input("Would you like to use version 'dev'? (y/N): ") == "y":
        exit()
else:
    version = sys.argv[1]

with open("setup.py", "r") as i:
    c = i.read()
    with open("setup.py", "w") as i2:
        i2.write(c.replace('"indev"', f'"{version}"'))

os.system("python3 -m build")

with open("setup.py", "r") as i:
    c = i.read()
    with open("setup.py", "w") as i2:
        i2.write(c.replace(f'"{version}"', '"indev"'))
