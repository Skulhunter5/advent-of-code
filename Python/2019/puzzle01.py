import os
import sys
sys.path.append(os.path.join(os.path.dirname(__file__), ".."))
from common.Puzzle import Puzzle

def required_fuel(mass):
    fuel = mass // 3 - 2
    if(fuel <= 0):
        return 0
    return fuel + required_fuel(fuel)

class Puzzle01(Puzzle, year=2019, day=1):
    @staticmethod
    def process_input(data):
        return [int(line) for line in data]

    def solve_part_1(self): # Solution for part 1
        return sum([(mass // 3 - 2) for mass in self.data])

    def solve_part_2(self): # Solution for part 2
        return sum([required_fuel(mass) for mass in self.data])

if(__name__ == "__main__"):
    puzzle = Puzzle01()
    part1, part2 = puzzle.solve()
    print(f"Advent of Code {puzzle.year} day {puzzle.day}:")
    print(f"- Part 1: {part1}")
    print(f"- Part 2: {part2}")
