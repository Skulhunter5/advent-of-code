import os
import sys
sys.path.append(os.path.join(os.path.dirname(__file__), ".."))
from common.Puzzle import Puzzle

class Directory():
    def __init__(self, name, parent=None):
        self.name = name
        self.parent = parent
        self.dirs = []
        self.files = []
        self.size = -1
    
    def add_subdirectory(self, subdir):
        self.dirs.append(subdir)
    
    def add_file(self, file):
        self.files.append(file)

    def total_size(self):
        if(self.size == -1):
            self.size = sum([file[1] for file in self.files]) + sum([subdir.total_size() for subdir in self.dirs])
        return self.size
    
    def accumulate_sizes(self):
        size = 0
        for subdir in self.dirs:
            size += subdir.accumulate_sizes()
        if(self.total_size() <= 100000):
            size += self.total_size()
        return size

    def find_directory_to_delete(self, required_size):
        to_delete = []
        for subdir in self.dirs:
            cur = subdir.find_directory_to_delete(required_size)
            if(cur > 0):
                to_delete.append(cur)
        if(len(to_delete) > 0):
            return min(to_delete)
        elif(self.total_size() >= required_size):
            return self.total_size()
        return 0

class Puzzle07(Puzzle, year=2022, day=7):
    @staticmethod
    def process_input(data):
        filesystem = Directory('/')
        cdir = filesystem # cdir = current directory (starting with outermost directory '/')

        # loop through every line, starting from 1 because line 0 always introduces outermost directory '/'
        for i in range(1, len(data)):
            line = data[i]
            if(line.startswith("$")):
                line = line[2:] # remove '$ '
                if(line.startswith("cd")):
                    line = line[3:] # remove 'cd '
                    if(line == ".."):
                        # cd back to parent directory
                        cdir = cdir.parent
                        continue
                    else:
                        # cd into unexplored subdirectory
                        subdir = Directory(line, cdir)
                        cdir.add_subdirectory(subdir)
                        cdir = subdir
                        continue
                elif(line.startswith("ls")):
                    # ls is irrelevant to parsing the filesystem tree structure
                    continue
            else:
                if(line.startswith("dir")):
                    # irrelevant because the directory will be explored using cd
                    continue
                else:
                    # register file in current directory
                    tokens = line.split()
                    cdir.add_file((tokens[1], int(tokens[0])))
                    continue
        return filesystem

    def solve_part_1(self): # Solution for part 1
        return self.data.accumulate_sizes()

    def solve_part_2(self): # Solution for part 2
        required_size = 30000000 - (70000000 - self.data.total_size())
        print(required_size)
        return self.data.find_directory_to_delete(required_size)

if(__name__ == "__main__"):
    puzzle = Puzzle07()
    part1, part2 = puzzle.solve()
    print(f"Advent of Code {puzzle.year} day {puzzle.day}:")
    print(f"- Part 1: {part1}")
    print(f"- Part 2: {part2}")
