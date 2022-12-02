import sys
import os

def create_new_puzzle():
    if(len(sys.argv) != 3):
        print("Invalid arguments")
        exit(1)

    year = int(sys.argv[1])
    day = int(sys.argv[2])

    templatepath = os.path.join(os.path.dirname(__file__), "template.py")
    with open(templatepath, 'r') as file:
        template = file.read()

    replacements = (
        ("Puzzle__", f"Puzzle{day:02d}"),
        ("year=__", f"year={year}"),
        ("day=__", f"day={day}")
    )

    dirpath = os.path.join(os.path.dirname(__file__), str(year))
    if(not os.path.exists(dirpath)):
        os.makedirs(dirpath)

    for replacement in replacements:
        template = template.replace(replacement[0], replacement[1])

    filepath = os.path.join(dirpath, f"puzzle{day:02d}.py")
    with open(filepath, 'w') as file:
        file.write(template)

    dirpath = os.path.join(os.path.dirname(__file__), str(year), "inputs")
    if(not os.path.exists(dirpath)):
        os.makedirs(dirpath)

    filepath = os.path.join(dirpath, f"{day:02d}-input.txt")
    if(not os.path.exists(filepath)):
        open(filepath, 'w').close()


if(__name__ == "__main__"):
    create_new_puzzle()
