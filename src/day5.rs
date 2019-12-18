// https://adventofcode.com/2019/day/5
use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;
use std::str::FromStr;
use std::io::{self, BufRead};

const ADD_ARGS_EXPECTED:usize  = 3;
const MUL_ARGS_EXPECTED:usize  = 3;
const INS_ARGS_EXPECTED:usize  = 1;
const OUTPUT_ARGS_EXPECTED: usize = 1;
const HALT_ARGS_EXPECTED: usize = 0;

const POSITION_MODE: i32 = 0;
// const INSERT_MODE: i32 = 1;


fn add(mut instructions: Vec<i32>, args: Vec<i32>) -> Vec<i32> {
    assert_eq!(args.len(), 3);
    let input1 = args[0];
    let input2 = args[1];
    let to_store_position = args[2] as usize;
    instructions[to_store_position] = input1 + input2;
    instructions
}

fn mul(mut instructions: Vec<i32>, args: Vec<i32>) -> Vec<i32> {
    assert_eq!(args.len(), 3);
    let input1 = args[0];
    let input2 = args[1];
    let to_store_position = args[2] as usize;
    instructions[to_store_position] = input1 * input2;
    instructions
}

fn ins(mut  instructions: Vec<i32>, args: Vec<i32>) -> Vec<i32> {
    assert_eq!(args.len(), 1);
    let input: i32 = args[0];
    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut buffer).expect("Could not read line");
    instructions[input as usize] = buffer.trim().parse().unwrap();
    instructions
}

fn output(instructions: Vec<i32>, args: Vec<i32>) -> Vec<i32> {
    println!("{}", args[0]);
    instructions
}

fn halt(instructions: Vec<i32>, _inputs: Vec<i32>) -> Vec<i32> {
    instructions
}

#[derive(Debug, PartialEq, Eq)]
enum Operation {
    Add,
    Mul,
    Ins,
    Out,
    Halt,
}

impl From<i32> for Operation {
    fn from(input: i32) -> Self {
        match input {
            1 => Operation::Add,
            2 => Operation::Mul,
            3 => Operation::Ins,
            4 => Operation::Out,
            99 => Operation::Halt,
            _ => panic!("Operation not known!"),
        }
    }
}

impl Operation {
    fn get_args_expected(&self) -> usize {
        match self {
            Operation::Add => ADD_ARGS_EXPECTED,
            Operation::Mul => MUL_ARGS_EXPECTED,
            Operation::Ins => INS_ARGS_EXPECTED,
            Operation::Out => OUTPUT_ARGS_EXPECTED,
            Operation::Halt => HALT_ARGS_EXPECTED,
        }
    }

    fn apply(&self, inputs: Vec<i32>, instructions: Vec<i32>) -> Vec<i32> {
        match self {
            Operation::Add => add(inputs, instructions),
            Operation::Mul => mul(inputs, instructions),
            Operation::Ins => ins(inputs, instructions),
            Operation::Out => output(inputs, instructions),
            Operation::Halt => halt(inputs, instructions)
        }
    }

    fn is_write_parameter(&self, parameter_position: usize) -> bool {
        match (self, parameter_position) {
            (Operation::Add, 2) => true,
            (Operation::Mul, 2) => true,
            (Operation::Ins, 0) => true,
            _ => false,
        }
    }
}

fn read_arguments(inputs: &Vec<i32>, progress: usize) -> (Operation, Vec<i32>, usize) {
    // hole contains opcode + instruction mode
    let hole: i32 = inputs[progress];
    let opcode: i32 = hole % 100;
    let mut instruction_modes: i32 = hole / 100;
    let operation: Operation = Operation::from(opcode);
    let args_expected: usize = operation.get_args_expected();
    let mut instructions_to_process = args_expected;
    let mut res: Vec<i32> = vec![];
    while instructions_to_process != 0 {
        let mode = instruction_modes % 10;
        let input: i32 = inputs[progress + 1 + args_expected - instructions_to_process];

        if !operation.is_write_parameter(args_expected - instructions_to_process) && mode == POSITION_MODE {
            res.push(inputs[input as usize]);
        } else {
            res.push(input);
        }
        instructions_to_process -= 1;
        instruction_modes = instruction_modes / 10;
    }
    (operation, res, progress + args_expected + 1)
}

#[aoc_generator(day5)]
fn parse_input(input: &str) -> Result<Vec<i32>, ParseIntError> {
    input.split(",").map(|i| i32::from_str(i)).collect()
}

fn intcode_program(mut input: Vec<i32>, _noun: i32, _verb: i32) -> Vec<i32> {
    //input[1] = noun;
    //input[2] = verb;
    let mut line_start_index: usize = 0;
    while line_start_index < input.len() {
        let (operation, instruction_parameters, new_index) = read_arguments(&input, line_start_index);
        line_start_index = new_index;

        match operation {
            Operation::Halt => {
                return input;
            },
            _ => {
                input = operation.apply(input, instruction_parameters);
            }
        }
    }
    return input;
}

#[aoc(day5, part1)]
pub fn part1(input: &[i32]) -> String {
    let mut result: Vec<i32> = Vec::from(input);
    result = intcode_program(result, 1, 1);
    return format!("{:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_operation() {
        let op = Operation::Add;
        assert_eq!(op.apply(vec![1,1,1], vec![4, 1, 2]), vec![1, 1, 5]);
    }

    #[test]
    fn test_mul_operation() {
        let op = Operation::Mul;
        assert_eq!(op.apply(vec![1,1,1], vec![4, 2, 0]), vec![8, 1, 1]);
    }

    #[test]
    fn test_read_arguments() {
        let (operation, instruction_args, index) = read_arguments(&vec![1002, 1, 2, 3], 0);
        assert_eq!(instruction_args, vec![1, 2, 3]);
        assert_eq!(operation, Operation::Mul);
        assert_eq!(index, 4);
    }

    #[test]
    fn test_read_arguments_with_one_opcode() {
        let (operation, instruction_args, index) = read_arguments(&vec![1, 2, 0, 2], 0);
        assert_eq!(instruction_args, vec![0, 1, 2]);
        assert_eq!(operation, Operation::Add);
        assert_eq!(index, 4);
    }

    #[test]
    fn test_intcode_program() {
        assert_eq!(intcode_program(vec![1,0,0,0,99], 0, 0), vec![2, 0, 0, 0, 99]);
        assert_eq!(intcode_program(vec![2,3,0,3,99 ],3,0), vec![2, 3, 0, 6, 99]);
        assert_eq!(intcode_program(vec![2,4,4,5,99,0 ], 4, 4), vec![2, 4, 4, 5, 99, 9801]);
        assert_eq!(intcode_program(vec![1,1,1,4,99,5,6,0,99 ], 1, 1), vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }
}
