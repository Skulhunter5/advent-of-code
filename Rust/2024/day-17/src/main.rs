use std::fmt::Write;

mod part2;

fn main() {
    let input = std::fs::read_to_string("../inputs/17-input").unwrap();

    let part1 = solve(&input);
    println!("Part 1: '{}'", &part1);
    let part2 = part2::solve(&input);
    println!("Part 2: {}", &part2);
}

type RegisterValue = usize;

#[derive(Debug, Copy, Clone)]
struct Registers {
    a: RegisterValue,
    b: RegisterValue,
    c: RegisterValue,
}

impl Registers {
    fn new(a: RegisterValue, b: RegisterValue, c: RegisterValue) -> Self {
        Self { a, b, c }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
enum ComputerError {
    InvalidComboOperand(u8),
    OutputError(String),
    InvalidProgram(u8),
}

#[derive(Debug, Clone)]
struct Computer {
    registers: Registers,
    program: Vec<u8>,
}

impl Computer {
    fn new(registers: Registers, program: Vec<u8>) -> Self {
        Self { registers, program }
    }

    fn run_program(mut self) -> Result<String, ComputerError> {
        let mut output = String::new();

        let mut ip = 0;
        loop {
            if ip >= self.program.len() {
                return Ok(output);
            }
            let opcode = self.program[ip];
            let operand = self.program[ip + 1];
            ip += 2;

            match opcode {
                // adv instruction
                // bdv instruction
                // cdv instruction
                0 | 6 | 7 => {
                    let num = self.registers.a;
                    let operand = self.combo_operand(operand)?;
                    let base: RegisterValue = 2;
                    let denom = base.pow(operand as u32);
                    let result = num / denom;
                    match opcode {
                        // adv instruction
                        0 => self.registers.a = result,
                        // bdv instruction
                        6 => self.registers.b = result,
                        // cdv instruction
                        7 => self.registers.c = result,
                        _ => unreachable!(),
                    }
                }
                // bxl instruction
                1 => {
                    let result = self.registers.b ^ operand as RegisterValue;
                    self.registers.b = result;
                }
                // bst instruction
                2 => {
                    let operand = self.combo_operand(operand)?;
                    //let result = operand % 8;
                    let result = operand & 0b111;
                    self.registers.b = result;
                }
                // jnz instruction
                3 => {
                    if self.registers.a != 0 {
                        ip = operand as usize;
                    }
                }
                // bxc instruction
                4 => {
                    let result = self.registers.b ^ self.registers.c;
                    self.registers.b = result;
                }
                // out instruction
                5 => {
                    let operand = self.combo_operand(operand)?;
                    //let result = operand % 8;
                    let result = operand & 0b111;
                    if output.len() > 0 {
                        write!(&mut output, ",")
                            .map_err(|err| ComputerError::OutputError(err.to_string()))?;
                    }
                    write!(&mut output, "{}", result)
                        .map_err(|err| ComputerError::OutputError(err.to_string()))?;
                }
                _ => return Err(ComputerError::InvalidProgram(opcode)),
            }
        }
    }

    fn combo_operand(&self, operand: u8) -> Result<RegisterValue, ComputerError> {
        match operand {
            0..=3 => Ok(operand as RegisterValue),
            4 => Ok(self.registers.a),
            5 => Ok(self.registers.b),
            6 => Ok(self.registers.c),
            7 => Err(ComputerError::InvalidComboOperand(operand)),
            _ => return Err(ComputerError::InvalidProgram(operand)),
        }
    }
}

fn solve(input: &String) -> String {
    let start = std::time::Instant::now();

    let computer = parse_input(&input);
    let output = computer.run_program().unwrap();

    let time = start.elapsed();
    println!("Time part 1: {:?}", &time);

    output
}

fn parse_input(input: &String) -> Computer {
    let mut lines = input.lines();
    let a = lines.next().expect("invalid input")["Register A: ".len()..]
        .parse::<RegisterValue>()
        .expect("invalid input");
    let b = lines.next().expect("invalid input")["Register B: ".len()..]
        .parse::<RegisterValue>()
        .expect("invalid input");
    let c = lines.next().expect("invalid input")["Register C: ".len()..]
        .parse::<RegisterValue>()
        .expect("invalid input");
    assert!(lines.next().expect("invalid input") == "");
    let program_string = lines.next().expect("invalid input")["Program: ".len()..].to_owned();
    let program = program_string
        .split(',')
        .map(|x| {
            let num = x.parse::<u8>().expect("invalid input");
            assert!(num <= 7);
            num
        })
        .collect::<Vec<_>>();
    assert!(lines.next().is_none());

    Computer::new(Registers::new(a, b, c), program)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        let input = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0"
            .to_string();

        assert_eq!(solve(&input), "4,6,3,5,6,3,5,2,1,0".to_string());
    }
}
