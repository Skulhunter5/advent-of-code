import os
import sys
sys.path.append(os.path.join(os.path.dirname(__file__), ".."))
from common.Puzzle import Puzzle

class Cell():
    def __init__(self, c, pos):
        self.c = c
        self.h = ord('a') if c == 'S' else (ord('z') if c == 'E' else ord(c))
        self.pos = pos
        self.visited = False
    
    def __repr__(self):
        return f"({self.h}=>{chr(self.h)}, {self.pos[0]}, {self.pos[1]})"

    def is_start(self):
        return self.c == 'S'
    
    def is_goal(self):
        return self.c == 'E'

class Grid():
    def __init__(self, data):
        self.data = data
        self.width = len(data)
        self.height = len(data[0])
    
    def reset(self):
        for x in range(self.width):
            for y in range(self.height):
                self.data[x][y].visited = False

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
        grid, start, _ = self.data
        grid.reset()

        i = 0
        queue = [start]
        next_queue = []
        while True:
            if(len(queue) == 0):
                raise Exception("No valid path found")

            i += 1
            for cell in queue:
                for neighbor in grid.get_neighbors(cell.pos):
                    if(neighbor.h > cell.h + 1):
                        continue
                    if(neighbor.visited):
                        continue
                    if(neighbor.is_goal()):
                        return i
                    
                    neighbor.visited = True
                    next_queue.append(neighbor)
            
            queue = next_queue
            next_queue = []

    def solve_part_2(self): # Solution for part 2
        grid, _, goal = self.data
        grid.reset()

        i = 0
        queue = [goal]
        next_queue = []
        while True:
            if(len(queue) == 0):
                raise Exception("No valid path found")
            
            i += 1
            for cell in queue:
                for neighbor in grid.get_neighbors(cell.pos):
                    if(cell.h > neighbor.h + 1):
                        continue
                    if(neighbor.visited):
                        continue
                    if(neighbor.h == ord('a')):
                        return i
                    
                    neighbor.visited = True
                    next_queue.append(neighbor)
            
            queue = next_queue
            next_queue = []

if(__name__ == "__main__"):
    puzzle = Puzzle12()
    part1, part2 = puzzle.solve()
    print(f"Advent of Code {puzzle.year} day {puzzle.day}:")
    print(f"- Part 1: {part1}")
    print(f"- Part 2: {part2}")
