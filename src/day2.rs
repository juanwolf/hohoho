// Responses of https://adventofcode.com/2019/day/2
use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;

/// OPCODE_ELEMENTS_NUMBER represents the number of elements to define an opcode.
const OPCODE_ELEMENTS_NUMBER: usize = 4;
const PART2_MISTERY_OUTPUT: i32 = 19690720;


enum Operation {
    Add,
    Mul,
    Halt,
}

fn operation_from_opcode(opcode: i32) -> Operation {
    match opcode {
        1 => Operation::Add,
        2 => Operation::Mul,
        99 => Operation::Halt,
        _ => panic!("Unknown operation for opcode {}", opcode),
    }
}

fn apply_operation(operation: Operation, input1: i32, input2: i32) -> i32 {
    match operation {
        Operation::Add => input1 + input2,
        Operation::Mul => input1 * input2,
        Operation::Halt => 0,
    }
}


fn store(input: &[i32], operation: Operation, input1: i32, input2: i32, output: usize) -> Vec<i32> {
    let operation_result: i32 = apply_operation(operation, input1, input2);
    let mut result: Vec<i32> = Vec::from(input);

    result[output] = operation_result;
    return result.clone()
}

#[aoc_generator(day2)]
fn parse_input_day2(input: &str) -> Result<Vec<i32>, ParseIntError> {
    input.split(",").map(|i| i.parse()).collect()
}

fn intcode_program(mut input: Vec<i32>, noun: i32, verb: i32) -> Vec<i32> {
    input[1] = noun;
    input[2] = verb;
    let mut line_parsed: usize = 0;
    let mut line_start_index: usize = 0;
    while line_start_index < input.len() {
        line_start_index = line_parsed * OPCODE_ELEMENTS_NUMBER;
        let operation_opcode = input[line_start_index];
        let operation: Operation = operation_from_opcode(operation_opcode);


        match operation {
            Operation::Halt => {
                return input;
            },
            _ => {
                let first_input_position = input[line_start_index + 1];
                let second_input_position = input[line_start_index + 2];
                let output = input[line_start_index + 3];
                let first_input = input[first_input_position as usize];
                let second_input = input[second_input_position as usize];

                input = store(&input, operation, first_input, second_input, output as usize);
            },
        }
        line_parsed += 1;
    }
    return input;
}

#[aoc(day2, part1)]
pub fn part1(input: &[i32]) -> String {
    let mut result: Vec<i32> = Vec::from(input);
    let result = intcode_program(result, 12, 2);
    return format!("{:?}", result);
}

#[aoc(day2, part2)]
pub fn part2(input: &[i32]) -> String {
    let mut noun = 0;
    while noun <= 99 {
        let mut verb = 0;
        while verb <= 99 {
            let mut result: Vec<i32> = Vec::from(input);
            result = intcode_program(result, noun, verb);
            if result[0] == PART2_MISTERY_OUTPUT {
                return format!("Result found! noun: {}, verb: {}. Solution: {}", noun, verb, noun*100+verb);
            }
            verb += 1;
        }
        noun += 1;
    }
    return String::from("ERROR");
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_intcode_program() {
        assert_eq!(intcode_program(vec![1,0,0,0,99], 0, 0), vec![2, 0, 0, 0, 99]);
        assert_eq!(intcode_program(vec![ 2,3,0,3,99 ],3,0), vec![2, 3, 0, 6, 99]);
        assert_eq!(intcode_program(vec![ 2,4,4,5,99,0 ], 4, 4), vec![2, 4, 4, 5, 99, 9801]);
        assert_eq!(intcode_program(vec![ 1,1,1,4,99,5,6,0,99 ], 1, 1), vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }

    #[test]
    fn test_store() {
        assert_eq!(store(&mut [1, 2, 3, 3], Operation::Add, 2, 3, 3), &[1, 2, 3, 5]);
    }
}
