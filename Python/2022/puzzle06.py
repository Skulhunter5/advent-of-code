import os
import sys
sys.path.append(os.path.join(os.path.dirname(__file__), ".."))
from common.Puzzle import Puzzle

class Puzzle06(Puzzle, year=2022, day=6):
    @staticmethod
    def process_input(data):
        return data[0]

    def find_first_n_unique(self, n):
        for i in range(n-1, len(self.data)):
            sl = self.data[i-(n-1):i+1]
            if(sum([sl.count(c) for c in sl]) == n):
                return i + 1
        raise Exception("Unreachable")

    def solve_part_1(self): # Solution for part 1
        return self.find_first_n_unique(4)

    def solve_part_2(self): # Solution for part 2
        return self.find_first_n_unique(14)

if(__name__ == "__main__"):
    puzzle = Puzzle06()
    part1, part2 = puzzle.solve()
    print(f"Advent of Code {puzzle.year} day {puzzle.day}:")
    print(f"- Part 1: {part1}")
    print(f"- Part 2: {part2}")
