import os
import sys
sys.path.append(os.path.join(os.path.dirname(__file__), ".."))
from common.Puzzle import Puzzle

class Puzzle02(Puzzle, year=2021, day=2):
    @staticmethod
    def process_input(data):
        return [(move[0], int(move[1])) for move in [move.split() for move in data]]

    def solve_part_1(self): # Solution for part 1
        position = depth = 0
        for (command, x) in self.data:
            if(command == "forward"):
                position += x
            elif(command == "down"):
                depth += x
            elif(command == "up"):
                depth -= x
        return position * depth
    
    def solve_part_2(self): # Solution for part 2
        aim = position = depth = 0
        for (command, x) in self.data:
            if(command == "forward"):
                position += x
                depth += aim * x
            elif(command == "down"):
                aim += x
            elif(command == "up"):
                aim -= x
        return position * depth

if(__name__ == "__main__"):
    puzzle = Puzzle02()
    part1, part2 = puzzle.solve()
    print(f"Advent of Code {puzzle.year} day {puzzle.day}:")
    print(f"- Part 1: {part1}")
    print(f"- Part 2: {part2}")
