// Responses of https://adventofcode.com/2019/day/3
// --- Day 4: Secure Container ---
//
//You arrive at the Venus fuel depot only to discover it's protected by a password. The Elves had written the password on a sticky note, but someone threw it out.
//
//However, they do remember a few key facts about the password:
//
//It is a six-digit number.
//The value is within the range given in your puzzle input.
//Two adjacent digits are the same (like 22 in 122345).
//Going from left to right, the digits never decrease; they only ever increase or stay the same (like 111123 or 135679).
//
//Other than the range rule, the following are true:
//
//111111 meets these criteria (double 11, never decreases).
//223450 does not meet these criteria (decreasing pair of digits 50).
//123789 does not meet these criteria (no double).
//
//How many different passwords within the range given in your puzzle input meet these criteria?
//
//Your puzzle input is 171309-643603.
//
//--- Part Two ---
//
//    An Elf just remembered one more important detail: the two adjacent matching digits are not part of a larger group of matching digits.
//
//    Given this additional criterion, but still ignoring the range rule, the following are now true:
//
//    112233 meets these criteria because the digits never decrease and all repeated digits are exactly two digits long.
//    123444 no longer meets the criteria (the repeated 44 is part of a larger group of 444).
//    111122 meets the criteria (even though 1 is repeated more than twice, it still contains a double 22).
//
//    How many different passwords within the range given in your puzzle input meet all of the criteria?
//
//    Your puzzle input is still 171309-643603.

use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;
use std::collections::HashMap;


fn is_6_digit_number(input: &str) -> bool {
    input.len() == 6 && input.parse::<i32>().is_ok()
}

fn is_two_adjacent_digit_the_same(input: &str) -> bool {
    let mut i = 1;
    let mut chars = input.chars();
    let mut previous_char = chars.next();
    while i < input.chars().count() {
        let c = chars.next();
        if c == previous_char {
            return true
        }
        previous_char = c;
        i += 1;
    }
    return false;
}

fn are_digits_ordered(input: &str) -> bool {
    let mut i = 1;
    let mut chars = input.chars();
    let mut previous_char: char = chars.next().unwrap();
    while i < input.chars().count() {
        let c: char = chars.next().unwrap();
        if previous_char.to_digit(10) > c.to_digit(10) {
            return false
        }
        previous_char = c;
        i += 1;
    }
    return true;
}

fn is_possible_password(input: &str) -> bool {
    return is_6_digit_number(input) && is_two_adjacent_digit_the_same(input) && are_digits_ordered(input)
}

fn two_adjacent_matching_digits_not_part_larger_group(input: &str) -> bool {
    let mut occurence_found = 1;
    let mut chars = input.chars();
    let mut occurences_found: HashMap<char, u32> = HashMap::new();
    let mut previous_char = chars.next().unwrap() ;
    for char in chars {
        if char == previous_char {
            occurence_found += 1;
        } else {
            occurences_found.insert(previous_char, occurence_found);
            occurence_found = 1;
        }
        previous_char = char;
    }
    occurences_found.insert(previous_char, occurence_found);

    let mut res =  false;
    for (_, occurences) in occurences_found {
        if occurences == 2 {
            res = true;
        }
    }
    return res;
}

#[aoc_generator(day4)]
fn parse_input_day4(input: &str) -> Result<Vec<String>, ParseIntError> {
    let range: Vec<String> = input.split("-").map(|input| String::from(input)).collect();
    let mut res: Vec<String> = vec![];
    let mut start: usize = range[0].parse().unwrap();
    let end: usize = range[1].parse().unwrap();

    while start <= end {
        res.push(format!("{}", start));
        start += 1;
    }

    return Ok(res)
}

#[aoc(day4, part1)]
pub fn part1(inputs: &[String]) -> u32 {
    let mut res: u32 = 0;
    for input in inputs {
        if is_possible_password(&input) {
            res += 1;
        }
    }
    return res;
}

#[aoc(day4, part2)]
pub fn part2(inputs: &[String]) -> u32 {
    let mut res: u32 = 0;
    for input in inputs {
        if is_possible_password(&input) && two_adjacent_matching_digits_not_part_larger_group(input) {
            res += 1;
        }
    }
    return res;
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn is_6_digit_number_with_6digit_number() {
        assert!(is_6_digit_number("888888"));
        assert!(is_6_digit_number("111111"));
        assert!(is_6_digit_number("123456"));
    }
    #[test]
    fn is_6_digit_number_with_6chars() {
        assert!(!is_6_digit_number("cccccc"));
        assert!(!is_6_digit_number("11dc12"));
        assert!(!is_6_digit_number("abcdef"));
    }

    #[test]
    fn is_two_adjacent_digit_the_same_with_two_digit_adjacent() {
        assert!(is_two_adjacent_digit_the_same("11"));
        assert!(is_two_adjacent_digit_the_same("137877"));
    }

    #[test]
    fn is_two_adjacent_digit_the_same_without_two_digit_adjacent() {
        assert!(!is_two_adjacent_digit_the_same("12"));
        assert!(!is_two_adjacent_digit_the_same("137897"));
    }

    #[test]
    fn are_digit_ordered_with_ordered_digit() {
        assert!(are_digits_ordered("12"));
        assert!(are_digits_ordered("3489"));
        assert!(are_digits_ordered("1"));
        assert!(are_digits_ordered("89"));
    }

    #[test]
    fn are_digit_ordered_with_unordered_digit() {
        assert!(!are_digits_ordered("90"));
        assert!(!are_digits_ordered("13429"));
        assert!(!are_digits_ordered("73"));
    }

    #[test]
    fn is_possible_password_with_6_same_chars() {
        assert!(is_possible_password("111111"));
    }

    #[test]
    fn is_possible_password_with_unordered_digit() {
        assert!(!is_possible_password("223450"));
    }

    #[test]
    fn is_possible_password_with_no_two_same_digit_adjacent() {
        assert!(!is_possible_password("123789"));
    }

    #[test]
    fn part1_with_empty_vec() {
        assert_eq!(part1(&vec![]), 0);
    }

    #[test]
    fn part1_with_one_possible_password() {
        assert_eq!(part1(&vec![String::from("111111")]), 1);
        assert_eq!(part1(&vec![String::from("111111"), String::from("121110")]), 1);
        assert_eq!(part1(&vec![String::from("121110"), String::from("111111")]), 1);
    }

    #[test]
    fn part_of_bigger_group(){
        assert_eq!(two_adjacent_matching_digits_not_part_larger_group("112233"), true);
        assert_eq!(two_adjacent_matching_digits_not_part_larger_group("123444"), false);
        assert_eq!(two_adjacent_matching_digits_not_part_larger_group("111122"), true);
    }
}
