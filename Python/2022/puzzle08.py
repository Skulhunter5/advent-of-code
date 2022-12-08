import os
import sys
sys.path.append(os.path.join(os.path.dirname(__file__), ".."))
from common.Puzzle import Puzzle

class Puzzle08(Puzzle, year=2022, day=8):
    @staticmethod
    def process_input(data):
        return [[int(data[i][j]) for i in range(len(data[0]))] for j in range(len(data))]

    def is_visible(self, x, y):
        if(self.data[x][y] == 0):
            return False
        
        # LEFT
        visible = True
        for nx in range(0, x):
            if(self.data[nx][y] >= self.data[x][y]):
                visible = False
                break
        if(visible):
            return True
        # RIGHT
        visible = True
        for nx in range(x+1, len(self.data)):
            if(self.data[nx][y] >= self.data[x][y]):
                visible = False
                break
        if(visible):
            return True
        # UP
        visible = True
        for ny in range(0, y):
            if(self.data[x][ny] >= self.data[x][y]):
                visible = False
                break
        if(visible):
            return True
        # DOWN
        visible = True
        for ny in range(y+1, len(self.data[0])):
            if(self.data[x][ny] >= self.data[x][y]):
                visible = False
                break
        if(visible):
            return True
        
        return False

    def scenic_score(self, x, y):
        scores = []

        # LEFT
        score = 0
        for nx in range(x-1, -1, -1):
            score += 1
            if(self.data[nx][y] >= self.data[x][y]):
                break
        scores.append(score)
        # RIGHT
        score = 0
        for nx in range(x+1, len(self.data)):
            score += 1
            if(self.data[nx][y] >= self.data[x][y]):
                break
        scores.append(score)
        # UP
        score = 0
        for ny in range(y-1, -1, -1):
            score += 1
            if(self.data[x][ny] >= self.data[x][y]):
                break
        scores.append(score)
        # DOWN
        score = 0
        for ny in range(y+1, len(self.data[0])):
            score += 1
            if(self.data[x][ny] >= self.data[x][y]):
                break
        scores.append(score)

        return scores[0] * scores[1] * scores[2] * scores[3]

    def solve_part_1(self): # Solution for part 1
        grid = self.data
        n_visible = len(grid) * 2 + (len(grid[0]) - 2) * 2
        n_visible += sum([1 for x in range(1, len(grid)-1) for y in range(1, len(grid[0])-1) if self.is_visible(x, y)])
        return n_visible

    def solve_part_2(self): # Solution for part 2
        grid = self.data
        scores = [self.scenic_score(x, y) for x in range(1, len(grid)-1) for y in range(1, len(grid[0])-1)]
        return max(scores)

if(__name__ == "__main__"):
    puzzle = Puzzle08()
    part1, part2 = puzzle.solve()
    print(f"Advent of Code {puzzle.year} day {puzzle.day}:")
    print(f"- Part 1: {part1}")
    print(f"- Part 2: {part2}")
