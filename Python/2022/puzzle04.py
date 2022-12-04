import os
import sys
sys.path.append(os.path.join(os.path.dirname(__file__), ".."))
from common.Puzzle import Puzzle

class Puzzle04(Puzzle, year=2022, day=4):
    @staticmethod
    def process_input(data):
        return [(tuple(map(int, assignments[0].split('-'))), tuple(map(int, assignments[1].split('-')))) for assignments in [pair.split(',') for pair in data]]

    def solve_part_1(self): # Solution for part 1
        res = 0
        for pair in self.data:
            ar = tuple(range(pair[0][0], pair[0][1]+1))
            br = tuple(range(pair[1][0], pair[1][1]+1))
            if(ar[0] <= br[0] and ar[-1] >= br[-1]):
                res += 1
                continue
            if(br[0] <= ar[0] and br[-1] >= ar[-1]):
                res += 1
                continue
        return res

    def solve_part_2(self): # Solution for part 2
        res = 0
        for pair in self.data:
            br = tuple(range(pair[1][0], pair[1][1]+1))
            for a in range(pair[0][0], pair[0][1]+1):
                if(a in br):
                    res += 1
                    break
        return res

if(__name__ == "__main__"):
    puzzle = Puzzle04()
    part1, part2 = puzzle.solve()
    print(f"Advent of Code {puzzle.year} day {puzzle.day}:")
    print(f"- Part 1: {part1}")
    print(f"- Part 2: {part2}")
