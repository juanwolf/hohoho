// Responses of https://adventofcode.com/2019/day/3
use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(string: char) -> Self {
        match string {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Unknown direction..."),
        }
    }
}

struct Move {
    direction: Direction,
    length: i32,
}

impl From<String> for Move {
    fn from(string: String) -> Self {
        let (dir_str, length) = string.split_at(1);
        let dir = dir_str.chars().next().unwrap();
        let direction = Direction::from(dir);
        return Move {
            direction: direction,
            length: length.parse().unwrap(),
        };
    }
}

impl Move {
    fn execute_from(&self, position: Position) -> Position {
        match self.direction {
            Direction::Up => Position {
                x: position.x + self.length,
                y: position.y,
            },
            Direction::Down => Position {
                x: position.x - self.length,
                y: position.y,
            },
            Direction::Right => Position {
                x: position.x,
                y: position.y + self.length,
            },
            Direction::Left => Position {
                x: position.x,
                y: position.y - self.length,
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

struct Wire {
    path: Vec<Position>,
}

impl From<Vec<Move>> for Wire {
    fn from(moves: Vec<Move>) -> Self {
        let mut path: Vec<Position> = vec![];
        let start: Position = Position { x: 0, y: 0 };
        path.push(start);
        let mut i = 0;
        while i < moves.len() {
            let last_position = match path.pop() {
                Some(p) => p,
                None => panic!("NO ELEMENT IN Vector"),
            };
            let new_position = moves[i].execute_from(last_position.clone());
            path.append(&mut vec![last_position, new_position]);

            i += 1;
        }
        return Wire { path: path };
    }
}

pub struct Panel<'a> {
    central_port_position: Position,
    wires: &'a [Wire],
}

#[aoc_generator(day3)]
fn parse_input_day3(input: &str) -> Result<Panel, ParseIntError> {
    let moves: Vec<Vec<Move>> = input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|mouvement| Move::from(String::from(mouvement)))
                .collect()
        })
        .collect();

    let mut wires = vec![];
    let move1 = moves[0];
    let move2 = moves[1];
    wires.push(Wire::from(move1));
    wires.push(Wire::from(move2));

    return Ok(Panel {
        central_port_position: Position { x: 0, y: 0 },
        wires: &wires,
    });
}

#[aoc(day3, part1)]
pub fn part1(input: Panel) -> String {
    return String::from("test");
}
