import os
import sys
sys.path.append(os.path.join(os.path.dirname(__file__), ".."))
from common.Puzzle import Puzzle

MATERIAL_AIR = 0
MATERIAL_ROCK = 1
MATERIAL_SAND = 2

class Grid():
    def __init__(self, minX, maxX, minY, maxY):
        self.minX = minX
        self.maxX = maxX
        self.minY = minY
        self.maxY = maxY

        self.width = self.maxX - self.minX + 1
        self.height = self.maxY - self.minY + 1

        self.grid = [[MATERIAL_AIR] * self.height for _ in range(self.width)]
        self.offsetX = -minX
        self.offsetY = -minY

    def get_offset_pos(self, pos):
        return (pos[0] + self.offsetX, pos[1] + self.offsetY)

    def contains(self, pos):
        if(pos[0] < self.minX):
            return False
        if(pos[0] > self.maxX):
            return False
        if(pos[1] < self.minY):
            return False
        if(pos[1] > self.maxY):
            return False
        return True

    def set(self, pos, v):
        pos = self.get_offset_pos(pos)
        self.grid[pos[0]][pos[1]] = v

    def get(self, pos):
        pos = self.get_offset_pos(pos)
        return self.grid[pos[0]][pos[1]]

class Grid2(Grid):
    def __init__(self, minX, maxX, minY, maxY):
        # Create Grid with floor
        super().__init__(minX, maxX, minY, maxY+2) # accounting for the floor
        y = self.height-1 # y of the floor
        for x in range(self.width):
            self.grid[x][self.height-1] = MATERIAL_ROCK
    
    def extend(self, x):
        if(x == 1):
            # Create new vertical slice
            new_slice = [MATERIAL_AIR] * self.height
            new_slice[self.height-1] = MATERIAL_ROCK
            # Insert slice into grid on the right side
            self.grid.insert(self.width, new_slice)
            # Correct minX and width
            self.maxX += 1
            self.width += 1
        elif(x == -1):
            # Create new vertical slice
            new_slice = [MATERIAL_AIR] * self.height
            new_slice[self.height-1] = MATERIAL_ROCK
            # Insert slice into grid on the left side
            self.grid.insert(0, new_slice)
            # Correct minX, offsetX and width
            self.minX -= 1
            self.offsetX += 1
            self.width += 1

class Puzzle14(Puzzle, year=2022, day=14):
    @staticmethod
    def process_input(data):
        structures = []
        for line in data:
            structures.append([tuple(map(int, point.split(','))) for point in line.split(' -> ')])
        return structures

    def construct_grid(self, grid_clazz):
        # Initialize grid of correct size
        xs = [500]
        ys = [0]
        for structure in self.data:
            xs.extend([point[0] for point in structure])
            ys.extend([point[1] for point in structure])
        self.grid = grid_clazz(min(xs), max(xs), min(ys), max(ys))
        
        for structure in self.data:
            x = structure[0][0]
            y = structure[0][1]
            self.grid.set((x, y), MATERIAL_ROCK)
            for i in range(1, len(structure)):
                point = structure[i]
                # Get x- and y-delta for line (based on the direction)
                dx = 0
                dy = 0
                if(x < point[0]):
                    dx = 1
                elif(x > point[0]):
                    dx = -1
                elif(y < point[1]):
                    dy = 1
                elif(y > point[1]):
                    dy = -1
                # Draw line of rocks
                while(point[0] != x or point[1] != y):
                    x += dx
                    y += dy
                    self.grid.set((x, y), MATERIAL_ROCK)

    def settle_sand_part_1(self, source_pos):
        pos = source_pos
        while(True):
            pos_d = (pos[0], pos[1]+1)
            if(not self.grid.contains(pos_d)):
                return False
            if(self.grid.get(pos_d) == MATERIAL_AIR):
                pos = pos_d
                continue

            pos_dl = (pos[0]-1, pos[1]+1)
            if(not self.grid.contains(pos_dl)):
                # There can't be any rocks below pos_dl for the sand to land on
                return False
            if(self.grid.get(pos_dl) == MATERIAL_AIR):
                pos = pos_dl
                continue

            pos_dr = (pos[0]+1, pos[1]+1)
            if(not self.grid.contains(pos_dr)):
                # There can't be any rocks below pos_dr for the sand to land on
                return False
            if(self.grid.get(pos_dr) == MATERIAL_AIR):
                pos = pos_dr
                continue

            self.grid.set(pos, MATERIAL_SAND)
            return True

    def solve_part_1(self): # Solution for part 1
        self.construct_grid(Grid)

        source_pos = (500, 0)
        settled = True
        unit_count = 0
        while(settled):
            settled = self.settle_sand_part_1(source_pos)
            unit_count += 1
        
        return unit_count-1

    def settle_sand_part_2(self, source_pos):
        pos = source_pos
        while(True):
            pos_d = (pos[0], pos[1]+1)
            if(self.grid.get(pos_d) == MATERIAL_AIR):
                pos = pos_d
                continue

            pos_dl = (pos[0]-1, pos[1]+1)
            if(not self.grid.contains(pos_dl)):
                self.grid.extend(-1)
            if(self.grid.get(pos_dl) == MATERIAL_AIR):
                pos = pos_dl
                continue

            pos_dr = (pos[0]+1, pos[1]+1)
            if(not self.grid.contains(pos_dr)):
                self.grid.extend(1)
            if(self.grid.get(pos_dr) == MATERIAL_AIR):
                pos = pos_dr
                continue

            self.grid.set(pos, MATERIAL_SAND)

            return pos != source_pos

    def solve_part_2(self): # Solution for part 2
        self.construct_grid(Grid2)

        source_pos = (500, 0)
        settled = True
        unit_count = 0
        while(settled):
            settled = self.settle_sand_part_2(source_pos)
            unit_count += 1

        return unit_count

if(__name__ == "__main__"):
    puzzle = Puzzle14()
    part1, part2 = puzzle.solve()
    print(f"Advent of Code {puzzle.year} day {puzzle.day}:")
    print(f"- Part 1: {part1}")
    print(f"- Part 2: {part2}")
