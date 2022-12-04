import os
import sys
sys.path.append(os.path.join(os.path.dirname(__file__), ".."))
from common.Puzzle import Puzzle

def priority_of(c):
    if(c.islower()):
        return ord(c) - 96
    return ord(c) - 38

class Puzzle03(Puzzle, year=2022, day=3):
    @staticmethod
    def process_input(data):
        return data


    def solve_part_1(self): # Solution for part 1
        res = 0
        for rucksack in self.data:
            compartment1 = rucksack[:int(len(rucksack)/2)]
            compartment2 = rucksack[int(len(rucksack)/2):]
            for c in compartment1:
                if(c in compartment2):
                    res += priority_of(c)
                    break
        return res

    def solve_part_2(self): # Solution for part 2
        res = 0
        for i in range(0, len(self.data), 3):
            for c in self.data[i]:
                if(c in self.data[i+1] and c in self.data[i+2]):
                    res += priority_of(c)
                    break
        return res

if(__name__ == "__main__"):
    puzzle = Puzzle03()
    part1, part2 = puzzle.solve()
    print(f"Advent of Code {puzzle.year} day {puzzle.day}:")
    print(f"- Part 1: {part1}")
    print(f"- Part 2: {part2}")
