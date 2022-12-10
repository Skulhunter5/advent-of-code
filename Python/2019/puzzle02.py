import os
import sys
sys.path.append(os.path.join(os.path.dirname(__file__), ".."))
from common.Puzzle import Puzzle

OPCODE_ADD = 1
OPCODE_MULT = 2
OPCODE_HALT = 99

class IntcodeComputer():
    def __init__(self, program):
        self.ram = program
    
    def execute_program(self):
        IP = 0 # instruction pointer
        while(True):
            opcode = self.ram[IP]
            if(opcode == OPCODE_ADD):
                a = self.ram[self.ram[IP+1]]
                b = self.ram[self.ram[IP+2]]
                self.ram[self.ram[IP+3]] = a + b
                IP += 4
            elif(opcode == OPCODE_MULT):
                a = self.ram[self.ram[IP+1]]
                b = self.ram[self.ram[IP+2]]
                self.ram[self.ram[IP+3]] = a * b
                IP += 4
            elif(opcode == OPCODE_HALT):
                return
            else:
                raise Exception("Invalid opcode:", opcode)
    
    def read_ram(self, position):
        return self.ram[position]

class Puzzle02(Puzzle, year=2019, day=2):
    @staticmethod
    def process_input(data):
        return [int(x) for x in data[0].split(',')]

    def solve_part_1(self): # Solution for part 1
        program = self.data[:]
        program[1] = 12
        program[2] = 2
        computer = IntcodeComputer(program)
        computer.execute_program()
        return computer.read_ram(0)

    def solve_part_2(self): # Solution for part 2
        for noun in range(100):
            for verb in range(100):
                program = self.data[:]
                program[1] = noun
                program[2] = verb
                computer = IntcodeComputer(program)
                computer.execute_program()
                result = computer.read_ram(0)
                if(result == 19690720):
                    return 100 * noun + verb
        raise Exception("Unreachable")

if(__name__ == "__main__"):
    puzzle = Puzzle02()
    part1, part2 = puzzle.solve()
    print(f"Advent of Code {puzzle.year} day {puzzle.day}:")
    print(f"- Part 1: {part1}")
    print(f"- Part 2: {part2}")
