import os
import sys
sys.path.append(os.path.join(os.path.dirname(__file__), ".."))
from common.Puzzle import Puzzle

class Cell():
    def __init__(self, c, pos):
        self.c = c
        self.h = ord('a') if c == 'S' else (ord('z') if c == 'E' else ord(c))
        self.pos = pos
        if(self.is_start()):
            self.visited = True
            self.shortest = 0
        else:
            self.visited = False
            self.shortest = -1
        self.parent = None
        self.children = set()
    
    def __repr__(self):
        return f"('{self.h}', {self.pos[0]}, {self.pos[1]})"

    def is_start(self):
        return self.c == 'S'
    
    def is_goal(self):
        return self.c == 'E'
    
    def update(self, cell):
        if(self.visited):
            if(cell.shortest + 1 >= self.shortest):
                return
        self.visited = True
        self.parent = cell
        self.parent.children.add(self)
        self.shortest = self.parent.shortest + 1
        for child in self.children:
            child.update(self)

class Grid():
    def __init__(self, data):
        self.data = data
        self.width = len(data)
        self.height = len(data[0])
    
    def get_cell(self, pos):
        return self.data[pos[0]][pos[1]]
    
    def get_neighbors(self, pos):
        x = pos[0]
        y = pos[1]
        neighbors = []
        if(x > 0):
            neighbors.append(self.get_cell((x-1, y)))
        if(x < self.width-1):
            neighbors.append(self.get_cell((x+1, y)))
        if(y > 0):
            neighbors.append(self.get_cell((x, y-1)))
        if(y < self.height-1):
            neighbors.append(self.get_cell((x, y+1)))
        return neighbors

class Puzzle12(Puzzle, year=2022, day=12):
    @staticmethod
    def process_input(data):
        grid = Grid([[Cell(data[y][x], (x, y)) for y in range(len(data))] for x in range(len(data[0]))])
        start = goal = None
        for y in range(len(data)):
            for x in range(len(data[y])):
                cell = grid.get_cell((x, y))
                if(cell.is_start()):
                    start = cell
                elif(cell.is_goal()):
                    goal = cell
        return (grid, start, goal)

    def solve_part_1(self): # Solution for part 1
        grid, start, goal = self.data

        queue = grid.get_neighbors(start.pos)
        for neighbor in queue:
            neighbor.update(start)
        while(len(queue) > 0):
            cell = queue.pop(0)
            neighbors = grid.get_neighbors(cell.pos)
            for neighbor in neighbors:
                if(neighbor.h <= cell.h + 1):
                    if(not neighbor.visited):
                        queue.append(neighbor)
                    neighbor.update(cell)
        return goal.shortest

    def solve_part_2(self): # Solution for part 2
        return None

if(__name__ == "__main__"):
    puzzle = Puzzle12()
    part1, part2 = puzzle.solve()
    print(f"Advent of Code {puzzle.year} day {puzzle.day}:")
    print(f"- Part 1: {part1}")
    print(f"- Part 2: {part2}")
