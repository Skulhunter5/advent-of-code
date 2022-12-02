import os
import sys
sys.path.append(os.path.join(os.path.dirname(__file__), ".."))
from common.Puzzle import Puzzle

scores1 = {
    "A X": 1 + 3,
    "A Y": 2 + 6,
    "A Z": 3 + 0,
    "B X": 1 + 0,
    "B Y": 2 + 3,
    "B Z": 3 + 6,
    "C X": 1 + 6,
    "C Y": 2 + 0,
    "C Z": 3 + 3,
}

scores2 = {
    "A X": 3 + 0,
    "A Y": 1 + 3,
    "A Z": 2 + 6,
    "B X": 1 + 0,
    "B Y": 2 + 3,
    "B Z": 3 + 6,
    "C X": 2 + 0,
    "C Y": 3 + 3,
    "C Z": 1 + 6,
}


class Puzzle02(Puzzle, year=2022, day=2):
    @staticmethod
    def process_input(data):
        return data

    def solve_part_1(self): # Solution for part 1
        score = 0
        for round in self.data:
            score += scores1[round]
        return score

    def solve_part_2(self): # Solution for part 2
        score = 0
        for round in self.data:
            score += scores2[round]
        return score

if(__name__ == "__main__"):
    puzzle = Puzzle02()
    part1, part2 = puzzle.solve()
    print(f"Advent of Code {puzzle.year} day {puzzle.day}:")
    print(f"- Part 1: {part1}")
    print(f"- Part 2: {part2}")
