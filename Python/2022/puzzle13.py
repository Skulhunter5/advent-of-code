import os
import sys
sys.path.append(os.path.join(os.path.dirname(__file__), ".."))
from common.Puzzle import Puzzle

digits = "0123456789"

import functools

def is_pair_correct(left, right):
    if(type(left) == type(right)):
        if(type(left) == int):
            if(left < right):
                return 1
            elif(left > right):
                return -1
            else:
                return 0
        elif(type(left) == list):
            lmin = min(len(left), len(right))
            for i in range(lmin):
                res = is_pair_correct(left[i], right[i])
                if(res != 0):
                    return res
            if(len(left) == len(right)):
                return 0
            elif(len(left) < len(right)):
                return 1
            else:
                return -1
    else:
        if(type(left) == int):
            left = [left]
        else:
            right = [right]
        return is_pair_correct(left, right)

def pair_compare(x, y):
    return -is_pair_correct(x, y)

class Puzzle13(Puzzle, year=2022, day=13):
    @staticmethod
    def process_input(data):
        return [Puzzle13.parse_line(line, 0)[0] for line in data if line != ""]

    @staticmethod
    def parse_line(line, i):
        if(line[i] in digits):
            if(i+1 < len(line) and line[i+1] in digits):
                return int(line[i:i+2]), i+2
            else:
                return int(line[i]), i+1
        else: # line[i] == '['
            lst = []
            i += 1
            if(line[i] == ']'):
                return lst, i+1
            item, i = Puzzle13.parse_line(line, i)
            lst.append(item)
            while(line[i] == ','):
                i += 1
                item, i = Puzzle13.parse_line(line, i)
                lst.append(item)
            i += 1
            return lst, i

    def solve_part_1(self): # Solution for part 1
        indices_sum = 0
        for i in range(0, len(self.data), 2):
            if(is_pair_correct(self.data[i], self.data[i+1]) == 1):
                indices_sum += i // 2 + 1
        return indices_sum

    def solve_part_2(self): # Solution for part 2
        divider1 = [[2]]
        divider2 = [[6]]
        self.data.append(divider1)
        self.data.append(divider2)
        self.data = sorted(self.data, key=functools.cmp_to_key(pair_compare))
        result = 1
        for i in range(len(self.data)):
            if(self.data[i] == divider1):
                result *= i + 1
            elif(self.data[i] == divider2):
                result *= i + 1
        return result

if(__name__ == "__main__"):
    puzzle = Puzzle13()
    part1, part2 = puzzle.solve()
    print(f"Advent of Code {puzzle.year} day {puzzle.day}:")
    print(f"- Part 1: {part1}")
    print(f"- Part 2: {part2}")
