import os
import sys
sys.path.append(os.path.join(os.path.dirname(__file__), ".."))
from common.Puzzle import Puzzle

OP_NOOP = 0
OP_ADDX = 1

execution_cycles = [
    1, # OP_NOOP
    2, # OP_ADDX
]

class Puzzle10(Puzzle, year=2022, day=10):
    @staticmethod
    def process_input(data):
        return [((OP_NOOP,) if tokens[0] == "noop" else (OP_ADDX, int(tokens[1]))) for tokens in [line.split() for line in data]]

    def solve_part_1(self): # Solution for part 1
        cycle = 0
        X = 1

        signal_strengths = []

        for op in self.data:
            for _ in range(execution_cycles[op[0]]):
                cycle += 1
                if(cycle % 40 - 20 == 0):
                    signal_strengths.append(cycle * X)
            if(op[0] == OP_ADDX):
                X += op[1]

        return sum(signal_strengths)

    def solve_part_2(self): # Solution for part 2
        cycle = 0
        X = 1

        CRT = [[] for _ in range(6)]

        for op in self.data:
            for _ in range(execution_cycles[op[0]]):
                CRT[cycle // 40].append('#' if (abs(len(CRT[cycle // 40]) - X) <= 1) else '.')
                cycle += 1
            if(op[0] == OP_ADDX):
                X += op[1]

        print("PART 2:") # for now don't have a solution to parse all possible capital letters (and don't even know which ones are possible)
        for row in CRT:
            print(''.join(row))

        return None

if(__name__ == "__main__"):
    puzzle = Puzzle10()
    part1, part2 = puzzle.solve()
    print(f"Advent of Code {puzzle.year} day {puzzle.day}:")
    print(f"- Part 1: {part1}")
    print(f"- Part 2: {part2}")
