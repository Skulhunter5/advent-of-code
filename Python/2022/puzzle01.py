import os
import sys
sys.path.append(os.path.join(os.path.dirname(__file__), ".."))
from common.Puzzle import Puzzle

class Puzzle01(Puzzle, year=2022, day=1):
    @staticmethod
    def process_input(data):
        return data

    def elves(self):
        i = 0
        elves = []
        amount = 0
        while(i < len(self.data)):
            if(self.data[i] == ''):
                elves.append(amount)
                amount = 0
            else:
                amount += int(self.data[i])
            i += 1
        if(amount != 0):
            elves.append(amount)
        return sorted(elves)

    def solve_part_1(self): # Solution for part 1
        return self.elves()[-1]

    def solve_part_2(self): # Solution for part 2
        return sum(self.elves()[-3:])

if(__name__ == "__main__"):
    puzzle = Puzzle01()
    part1, part2 = puzzle.solve()
    print(f"Advent of Code {puzzle.year} day {puzzle.day}:")
    print(f"- Part 1: {part1}")
    print(f"- Part 2: {part2}")
