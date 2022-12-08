import os
import sys
sys.path.append(os.path.join(os.path.dirname(__file__), ".."))
from common.Puzzle import Puzzle

class Puzzle05(Puzzle, year=2022, day=5):
    @staticmethod
    def process_input(data):
        for i in range(len(data)):
            if(data[i] == ""):
                stacks = [[] for _ in range(int(data[i-1].strip()[-1]))]
                for j in range(i-2, -1, -1):
                    l = 0
                    for k in range(1, len(data[j]), 4):
                        if(data[j][k] != ' '):
                            stacks[l].append(data[j][k])
                        l += 1
                moves = [(int(line[1]), int(line[3])-1, int(line[5])-1) for line in [line.split() for line in data[i+1:]]]
                return (stacks, moves)
        raise Exception("Unreachable")

    def solve_part_1(self): # Solution for part 1
        (stacks, moves) = self.data
        stacks = [stack.copy() for stack in stacks]
        for (n, a, b) in moves:
            for i in range(n):
                stacks[b].append(stacks[a].pop())
        return ''.join([stack[-1] for stack in stacks])

    def solve_part_2(self): # Solution for part 2
        (stacks, moves) = self.data
        stacks = [stack.copy() for stack in stacks]
        for (n, a, b) in moves:
            moved = stacks[a][-n:]
            stacks[a] = stacks[a][:-n]
            stacks[b].extend(moved)
        return ''.join([stack[-1] for stack in stacks])

if(__name__ == "__main__"):
    puzzle = Puzzle05()
    part1, part2 = puzzle.solve()
    print(f"Advent of Code {puzzle.year} day {puzzle.day}:")
    print(f"- Part 1: {part1}")
    print(f"- Part 2: {part2}")
