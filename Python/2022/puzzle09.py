import os
import sys
sys.path.append(os.path.join(os.path.dirname(__file__), ".."))
from common.Puzzle import Puzzle

def move(head, direction):
    if(direction == 'L'):
        head[0] -= 1
    elif(direction == 'R'):
        head[0] += 1
    elif(direction == 'U'):
        head[1] -= 1
    elif(direction == 'D'):
        head[1] += 1

def resolve(head, tail):
    if(head[1] == tail[1]):
        if(head[0] < tail[0]-1):
            tail[0] -= 1
        elif(head[0] > tail[0]+1):
            tail[0] += 1
    elif(head[0] == tail[0]):
        if(head[1] < tail[1]-1):
            tail[1] -= 1
        elif(head[1] > tail[1]+1):
            tail[1] += 1
    else:
        apart_x = abs(head[0] - tail[0]) > 1
        apart_y = abs(head[1] - tail[1]) > 1
        if(apart_x and apart_y):
            tail[0] += (head[0] - tail[0]) // 2
            tail[1] += (head[1] - tail[1]) // 2
        elif(apart_x):
            tail[0] += (head[0] - tail[0]) // 2
            tail[1] += head[1] - tail[1]
        elif(apart_y):
            tail[1] += (head[1] - tail[1]) // 2
            tail[0] += head[0] - tail[0]

def print_knots(knots): # debug purposes
    xs = [knot[0] for knot in knots]
    minX = min(min(xs), 0)
    maxX = max(max(xs), 0)
    ys = [knot[1] for knot in knots]
    minY = min(min(ys), 0)
    maxY = max(max(ys), 0)
    grid = [['.' for x in range(minX, maxX+1)] for y in range(minY, maxY+1)]
    grid[-minY][-minX] = 's'
    for i in range(len(knots)-1, -1, -1):
        if(i == 0):
            grid[knots[i][1] - minY][knots[i][0] - minX] = str('H')
        else:
            grid[knots[i][1] - minY][knots[i][0] - minX] = str(i)
    for i in range(len(grid)):
        print(''.join(grid[i]))

class Puzzle09(Puzzle, year=2022, day=9):
    @staticmethod
    def process_input(data):
        return [(tokens[0], int(tokens[1])) for tokens in [line.split() for line in data]]

    def solve_part_1(self): # Solution for part 1
        head = [0, 0]
        tail = [0, 0]

        self.visited_positions = set(((0, 0),))

        for (direction, distance) in self.data:
            for _ in range(distance):
                move(head, direction)
                resolve(head, tail)
                self.visited_positions.add(tuple(tail))

        return len(self.visited_positions)

    def solve_part_2(self): # Solution for part 2
        knots = [[0, 0] for _ in range(10)]

        self.visited_positions = set(((0, 0),))

        for (direction, distance) in self.data:
            for _ in range(distance):
                move(knots[0], direction)
                for i in range(1, len(knots)):
                    resolve(knots[i-1], knots[i])
                self.visited_positions.add(tuple(knots[-1]))

        return len(self.visited_positions)

if(__name__ == "__main__"):
    puzzle = Puzzle09()
    part1, part2 = puzzle.solve()
    print(f"Advent of Code {puzzle.year} day {puzzle.day}:")
    print(f"- Part 1: {part1}")
    print(f"- Part 2: {part2}")
