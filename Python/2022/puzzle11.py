import os
import sys
sys.path.append(os.path.join(os.path.dirname(__file__), ".."))
from common.Puzzle import Puzzle

import re

import time

FACTOR_OLD = 0
FACTOR_NUMBER = 1

INDEX_MONKEY_ID = 0
INDEX_ITEMS = 1
INDEX_OPERATOR = 2
INDEX_FACTOR = 3
INDEX_DIVISOR = 4
INDEX_TRUE_ID = 5
INDEX_FALSE_ID = 6

class Puzzle11(Puzzle, year=2022, day=11):
    @staticmethod
    def process_input(data):
        data = '\n'.join(data)

        data = """
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

    Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

    Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

    Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
        """

        p = re.compile("Monkey (\d+):\n  Starting items: (\d+(?:, \d+)*)\n  Operation: new = old ([*+]) (old|\d+)\n  Test: divisible by (\d+)\n    If true: throw to monkey (\d+)\n    If false: throw to monkey (\d+)")
        return [(int(monkeyId), list(map(int, items.split(", "))), operator, (FACTOR_OLD, ) if factor == "old" else (FACTOR_NUMBER, int(factor)), int(divisor), int(trueId), int(falseId)) for (monkeyId, items, operator, factor, divisor, trueId, falseId) in [match.groups() for match in p.finditer(data)]]

    def create_data_copy(self):
        return [(monkeyId, items.copy(), operator, factor, divisor, trueId, falseId) for (monkeyId, items, operator, factor, divisor, trueId, falseId) in self.data]

    def solve_part_1(self): # Solution for part 1
        data = self.create_data_copy()
        inspection_counter = [0] * len(data)
        for _ in range(20):
            for (monkeyId, items, operator, factor, divisor, trueId, falseId) in data:
                inspection_counter[monkeyId] += len(items)
                for item in items:
                    f = item if factor[0] == FACTOR_OLD else factor[1]
                    r = (item + f) if operator == '+' else (item * f)
                    r = r // 3
                    nextId = trueId if (r % divisor == 0) else falseId
                    data[nextId][INDEX_ITEMS].append(r)
                items.clear()

        # returning the product of the two highest inspection-counts
        result = max(inspection_counter)
        inspection_counter.remove(result)
        return result * max(inspection_counter)

    def solve_part_2(self): # Solution for part 2
        data = self.create_data_copy()
        inspection_counter = [0] * len(data)
        for i in range(10000):
            for (monkeyId, items, operator, factor, divisor, trueId, falseId) in data:
                inspection_counter[monkeyId] += len(items)
                for item in items:
                    f = item if factor[0] == FACTOR_OLD else factor[1]
                    r = (item + f) if operator == '+' else (item * f)
                    divisible = r % divisor == 0
                    r %= divisor
                    nextId = trueId if divisible else falseId
                    data[nextId][INDEX_ITEMS].append(r)
                items.clear()
            
            if((i + 1) % 1000 == 0 or (i+1) == 1 or (i+1) == 20):
                if(i > 1):
                    print()
                print(f"== After round {i+1} ==")
                for monkeyId, inspections in enumerate(inspection_counter):
                    print(f"Monkey {monkeyId} inspected items {inspections} times.")
                #print(f"After round {i+1}, the monkeys are holding items with these worry levels:")
                #for (monkeyId, items, _, _, _, _, _) in data:
                #    print(f"Monkey {monkeyId}: {', '.join([str(item) for item in items])}")

        # returning the product of the two highest inspection-counts
        print()
        print(inspection_counter)
        print()
        result = max(inspection_counter)
        inspection_counter.remove(result)
        return result * max(inspection_counter)

if(__name__ == "__main__"):
    puzzle = Puzzle11()
    part1, part2 = puzzle.solve()
    print(f"Advent of Code {puzzle.year} day {puzzle.day}:")
    print(f"- Part 1: {part1}")
    print(f"- Part 2: {part2}")
