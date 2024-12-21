// !!! WARNING !!!
// Please note that this solution is purely based on my puzzle input. There is no guarantee that
// this program also spits out the correct solution for your puzzle input.
// Because this problem is fairly specific/complex, the puzzle inputs might only differ for the
// already given a-register value for part one. In this case, the solution for part two is the same
// for every participant.
// The puzzle input of participants might also differ in the given program though, in which case
// this solution might still work. If not, my explanation of how I came up with the solution might
// hopefully still be useful in finding your personal solution.
//
// ------ How did I find my solution ------ 
// I first wanted to figure out if you can see a clear trend in the outputs and so I started by
// checking out the first 10 outputs.
// The most obvious thing was that it was outputs of length 1 first and then outputs of length 2,
// conveniently switching at a=1. i.e. at the first instance of a>8, with 8 being an interesting
// number because we're using 3-bit numbers and two of the instructions calculate modulo 8, which
// is the first number too large for a 3-bit number.
//
// Therefore, the next thing I did was confirm my suspicions by checking the outputs when switching
// from a 6-bit to 7-bit number and from a 9-bit to a 10-bit number (i.e. going up 3 bits each
// time).
// This resulted in what I expected. The last number that fit within 6 bits (0b111_111=63=8^2-1)
// resulted in an output with 2 numbers, while the first number that requires 7 bits (i.e. 0b1_000_000=64=8^2)
// resulted in an output with 3 numbers.
// My suspicion now was that the amount of numbers the program outputs with respect to 'a' can be
// determined using the formula floor(log_8(a))+1.
// Since the output I'm looking for is "2,4,1,1,7,5,1,5,4,0,0,3,5,5,3,0", i.e. 16 numbers, that
// would mean that the respective a has to be somewhere between 8^15 and 8^16-1.
// These are still too many numbers to (quickly) brute-force, even if exiting the program's
// execution early as soon as an outputted number does not match the respective number within the
// program's source code.
//
// As a next measure, I wanted to take another look at the specific output of the program based on
// a one after another to see if I could spot some kind of pattern there.
// I first checked the results for the first output length (i.e. from a=0 to a=7) and then for
// outputs of length 2 (i.e. from a=8 to a=63). There I noticed that the last numbers are grouped
// when incrementing a by one between each run, i.e. from a=8 to a=15 the last number is always
// 4, from a=16 to a=23, it's always 6, from a=24 to a=31 it's always 7, and so on...
// i.e. a sub-range of 8 numbers for a between 8 and 63 always has the same number as the last
// number. Each of these sub-ranges also has a unique number as the last number of the output.
// The only number not found as the last number in the output is 5, all other numbers have their
// own sub-range. We have 8 possible 3-bit numbers, 0 through 7, and we only have 7 sub-ranges
// of 8 numbers each. This somehow results in the number 5 not showing up.
// A similar effect can be seen within the numbers 1 through 7, which have sub-ranges of
// length 1 each, missing the 5 again because the number 0 for the range of 1-7 is the requivalent
// of what the 1-7 is for 8-63. My prediction because of this is that for 64..512 there will be 7
// sub-ranges of 8^2 numbers each, the last numbers within each sub-range being the same and the
// overall order of numbers being 4,6,7,0,1,2,3, missing the number 5 again.
// The following piece of code can validate this pattern for every range, which can be determined
// using the parameter k, (2^k)..(2^(k+1))
//let pairs: [(usize, u8); 7] = [(1, 4), (2, 6), (3, 7), (4, 0), (5, 1), (6, 2), (7, 3)];
//let k = 2;
//for (i, x) in pairs {
//    for offset in 0..(8usize.pow(k)) {
//        let a = 8usize.pow(k) * i + offset;
//
//        computer.registers = initial_registers;
//        computer.registers.a = a;
//        let output = computer.run_program().unwrap();
//        if output == computer.program {
//            println!("Solution found: {i}");
//            return;
//        }
//
//        assert_eq!(output[output.len()-1], x);
//    }
//}
//println!("No problems");
// With this, I validated k=1 to k=8, I didn't validate any more than that because k=8 already
// takes like 10 seconds or so on my computer.
// With this, we can further constrain the range that a has to be in by a factor of 8. Since the
// last number in the output we're looking for is the number 0, a now has to be in
// (4*8^15)..(5*8^15).
//
// Next I wanna see if I can see some kind of pattern within the second to last number of the
// output with respect to a.
// What I can see for 8..64 is that the last 2 numbers within the sequence of second-to-last numbers
// within each sub-range are always 2 and 3.
// With the following piece of code, you can check for these kinds of patterns. It groups together
// each second-to-last number for each nth a within each of the 7 blocks on a given region, whose
// outputs are all the same length. Further information has to be added... I haven't done this piece
// of code quite right for what I wanted it to do but I still figured out the correct pattern with
// it and so I never actually corrected it...
//let k = 2;
//let l = 2;
//for i in 0..(8usize.pow(k)) {
//    let mut nums = Vec::new();
//    for j in 1..8 {
//        let a = j * 8usize.pow(k) + i;
//
//        computer.registers = initial_registers;
//        computer.registers.a = a;
//        let output = computer.run_program().unwrap();
//        if output == computer.program {
//            println!("Solution found: {i}");
//            return;
//        }
//        nums.push(output[output.len() - l as usize]);
//    }
//    println!("{:?}", &nums);
//    println!();
//}
// When looking at this for 64..512, the sequence of second-to-last numbers within each sub-range
// will end the numbers 2 and 3 again, this time 8 of each though.
// Now there are 8 sub-ranges of 64 numbers each. Each of these sub-ranges is split into further
// sub-ranges of 8 numbers where the last second-to-last number is the same.
// When investigating each of these sub-sub-ranges, you see that for all third-to-last numbers within
// each sub-sub-range, the 4 last ones are 2,5,2,2...
//
// I finally realized that the thing that happened from the start actually happens everytime and
// can just be done repeatedly until you find the solution.
// When mutliplying a by 8, a..(a+8) will all result in an output that ends with the output for a.
// This is a direct consequence of the block-building. in the beginning, there are 7 different
// numbers that result in an output of one number (ignoring the 0 because it's a special case).
// Looking at the next range (for outputs of length 2), there are 7 blocks of 8 consecutive a's,
// whose outputs only differ in the first element and the rest is simply the output of the a before
// multiplication with 8.
// Because of this, you can simply start at 0, run all 8 consectutive a's and figure out which of
// those results in the end of the program. For each of the one's that output the end of the
// program, multiply a by 8, which gives you the start of a block of 8 consecutive a's whose
// outputs all end in the output of the original a. At this point you can simply repeat from the
// beginning.

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

    fn run_program(&mut self) -> Result<Vec<u8>, ComputerError> {
        let mut output = Vec::new();

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
                    output.push(result as u8);
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

pub fn solve(input: &String) -> RegisterValue {
    let start_time = std::time::Instant::now();

    let mut computer = parse_input(&input);
    let initial_registers = computer.registers;

    let a = {
        let result = solve_recursive(&mut computer, &initial_registers, 0, 1);
        if let Some(result) = result {
            result
        } else {
            panic!("No result found");
        }
    };

    let time = start_time.elapsed();
    println!("Time part 2: {:?}", &time);

    a
}

fn solve_recursive(computer: &mut Computer, initial_registers: &Registers, range_start: usize, i: usize) -> Option<usize> {
    for j in 0..8 {
        let a = range_start + j;

        computer.registers = *initial_registers;
        computer.registers.a = a;
        let output = computer.run_program().unwrap();
        if output[0] == computer.program[computer.program.len() - i] {
            if i == computer.program.len() {
                return Some(a);
            }

            let result = solve_recursive(computer, initial_registers, a * 8, i + 1);
            if let Some(result) = result {
                return Some(result);
            }
        }
    }

    None
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
    fn example_my_input() {
        let input = "Register A: 64196994
Register B: 0
Register C: 0

Program: 2,4,1,1,7,5,1,5,4,0,0,3,5,5,3,0".to_string();

        assert_eq!(solve(&input), 164541160582845);
    }
}
