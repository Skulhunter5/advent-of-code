import os

class Puzzle():
    def __init_subclass__(cls, year, day) -> None:
        super().__init_subclass__()
        cls.year = year
        cls.day = day
    
    def __init__(self):
        self.get_input()

    def get_input(self):
        filename = f"{self.day:02d}-input.txt"
        print(os.path.dirname(__file__))
        path = os.path.join(os.path.dirname(__file__), "..", str(self.year), "inputs")
        with open(os.path.join(path, filename)) as fp:
            self.data = self.process_input([line.rstrip() for line in fp.readlines()])

    def solve_part_1(self):
        raise NotImplementedError(f"Puzzle__{self.day:02}.solve_part_1() has not been implemented.")
    
    def solve_part_2(self):
        raise NotImplementedError(f"Puzzle__{self.day:02}.solve_part_2() has not been implemented.")

    def solve(self):
        part1 = self.solve_part_1()
        part2 = self.solve_part_2()
        return part1, part2

    @staticmethod
    def process_input(data):
        return data
