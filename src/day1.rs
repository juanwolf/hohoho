// Responses of https://adventofcode.com/2019/day/1
use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;


/// fuel_calculation returns the fuel needed for one specific mass
fn fuel_calculation(mass: i32) -> i32 {
    mass / 3 - 2
}

// fuel_calculation_for_fuel returns the fuel needed for the original fuel measurement of a module.
fn fuel_calculation_for_fuel(fuel: i32) -> i32 {
    let mut res = fuel_calculation(fuel);
    if res <= 0 {
        return 0;
    } else {
        res += fuel_calculation_for_fuel(res)
    }
    return res;
}

#[aoc_generator(day1)]
fn parse_input_day1(input: &str) -> Result<Vec<i32>, ParseIntError> {
    input.lines().map(|l| l.parse()).collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[i32]) -> i32 {
    input
        .iter()
        .map(|mass| fuel_calculation(*mass))
        .sum()
}

#[aoc(day1, part2)]
pub fn part2(input: &[i32]) -> i32 {
    input
        .iter()
        .map(|mass| fuel_calculation(*mass) + fuel_calculation_for_fuel(fuel_calculation(*mass)))
        .sum()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_fuel_calculation() {
        assert_eq!(fuel_calculation(12), 2);
        assert_eq!(fuel_calculation(14), 2);
        assert_eq!(fuel_calculation(1969), 654);
        assert_eq!(fuel_calculation(100756), 33583);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&[10, 9]), 2);
        assert_eq!(part1(&[12]), 2);
    }

    #[test]
    fn test_fuel_calculation_for_fuel() {
        assert_eq!(fuel_calculation_for_fuel(2), 0);
        assert_eq!(fuel_calculation_for_fuel(654), 312);
        assert_eq!(fuel_calculation_for_fuel(33583), 16763);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&[100756]), 50346);
        assert_eq!(part2(&[1969]), 966);
        assert_eq!(part2(&[100756, 1969]), 50346 + 966);
    }
}
