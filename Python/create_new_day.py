import sys
import os

def create_new_day():
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
    dir_exists = os.path.exists(dirpath)
    if(not dir_exists):
        os.makedirs(dirpath)

    for replacement in replacements:
        template = template.replace(replacement[0], replacement[1])
    
    filepath = os.path.join(dirpath, f"puzzle{day:02d}.py")
    with open(filepath, 'w') as file:
        file.write(template)


if(__name__ == "__main__"):
    create_new_day()
