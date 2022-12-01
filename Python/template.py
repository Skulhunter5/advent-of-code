import os
import sys
sys.path.append(os.path.join(os.path.dirname(__file__), ".."))
from common.Puzzle import Puzzle

class Puzzle__(Puzzle, year=__, day=__):
    @staticmethod
    def process_input(data):
        return data

    def solve_part_1(self): # Solution for part 1
        return None
    
    def solve_part_2(self): # Solution for part 2
        return None

if(__name__ == "__main__"):
    puzzle = Puzzle__()
    part1, part2 = puzzle.solve()
    print(f"Advent of Code {puzzle.year} day {puzzle.day}:")
    print(f"- Part 1: {part1}")
    print(f"- Part 2: {part2}")
