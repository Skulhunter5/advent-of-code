import os
import sys
sys.path.append(os.path.join(os.path.dirname(__file__), ".."))
from common.Puzzle import Puzzle

class Puzzle01(Puzzle, year=2021, day=1):
    @staticmethod
    def process_input(data):
        return [int(entry) for entry in data]

    def solve_part_1(self): # Solution for part 1
        return sum([1 for i in range(1, len(self.data)) if self.data[i] > self.data[i-1]])
    
    def solve_part_2(self): # Solution for part 2
        return sum([1 for i in range(1, len(self.data)-2) if sum(self.data[i:i+3]) > sum(self.data[i-1:i+2])])

if(__name__ == "__main__"):
    puzzle = Puzzle01()
    part1, part2 = puzzle.solve()
    print(f"Advent of Code {puzzle.year} day {puzzle.day}:")
    print(f"- Part 1: {part1}")
    print(f"- Part 2: {part2}")
