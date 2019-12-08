// Responses of https://adventofcode.com/2019/day/3
use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;

#[derive(Clone, Debug, PartialEq, Eq)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn manhattan_distance(&self, position: Position) -> u32 {
        let x1 = position.x.wrapping_abs() as u32;
        let x2 = self.x.wrapping_abs() as u32;
        let y1 = position.y.wrapping_abs() as u32;
        let y2 = self.y.wrapping_abs() as u32;
        (x1 + x2) + (y1 + y2)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
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
    fn execute_from(&self, position: Position) -> Vec<Position> {
        let mut res: Vec<Position> = vec![];
        let mut i = 0;
        while i <= self.length {
            let new_position = match self.direction {
                Direction::Up => Position {
                    x: position.x,
                    y: position.y + i,
                },
                Direction::Down => Position {
                    x: position.x,
                    y: position.y - i,
                },
                Direction::Right => Position {
                    x: position.x + i,
                    y: position.y,
                },
                Direction::Left => Position {
                    x: position.x - i,
                    y: position.y,
                },
            };
            res.push(new_position);
            i += 1;
        }
        return res;
    }
}

#[derive(Debug, PartialEq, Eq)]
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
            let mut new_positions: Vec<Position> = moves[i].execute_from(last_position.clone());
            path.append(&mut new_positions);

            i += 1;
        }
        return Wire { path: path };
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Panel {
    central_port_position: Position,
    wires: Vec<Wire>,
}

impl Panel {
    fn get_intersection_points(&self) -> Vec<Position> {
        let mut res: Vec<Position> = vec![];
        let path1 = self.wires[0].path.clone();
        let path2 = self.wires[1].path.clone();
        for position in path1 {
            if path2.contains(&position) && position != self.central_port_position {
                res.push(position);
            }
        }
        return res;
    }
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
    let move1: Vec<Move> = moves[0].clone();
    let move2: Vec<Move> = moves[1].clone();
    wires.push(Wire::from(move1));
    wires.push(Wire::from(move2));

    let panel = Panel {
        central_port_position: Position { x: 0, y: 0 },
        wires: wires,
    };

    return Ok(panel);
}

#[aoc(day3, part1)]
pub fn part1(panel: &Panel) -> u32 {
    let intersection_points = panel.get_intersection_points();
    let mut shortest_manhattan_distance: u32 = panel
        .central_port_position
        .manhattan_distance(intersection_points[0]);
    for position in intersection_points {
        let manhattan_distance = panel.central_port_position.manhattan_distance(position);

        if shortest_manhattan_distance > manhattan_distance {
            shortest_manhattan_distance = manhattan_distance;
        }
    }
    return shortest_manhattan_distance;
}

#[aoc(day3, part2)]
pub fn part2(panel: &Panel) -> u32 {
    let intersection_points = panel.get_intersection_points();
    let mut shortest_length: u32 = u32::max_value();
    for intersection in intersection_points {
        let wire1_length = panel.wires[0]
            .path
            .iter()
            .position(|&p| p == intersection)
            .unwrap();
        let wire2_length = panel.wires[1]
            .path
            .iter()
            .position(|&p| p == intersection)
            .unwrap();
        let wires_length = (wire1_length + wire1_length) as u32;
        if shortest_length > wires_length {
            shortest_length = wires_length
        }
    }
    return shortest_length;
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_manhattan_distance() {
        let anchor = Position { x: 0, y: 0 };
        let test = Position { x: 3, y: 3 };
        assert_eq!(anchor.manhattan_distance(test), 6)
    }

    #[test]
    fn test_move_from_string() {
        let input = String::from("U78");
        let expected = Move {
            direction: Direction::Up,
            length: 78,
        };
        let result = Move::from(input);
        assert_eq!(result, expected);
        let wrong = Move {
            direction: Direction::Down,
            length: 78,
        };
        assert_ne!(result, wrong);
    }

    #[test]
    fn test_parse_input() {
        let result = parse_input_day3("U2,R2,D1,L1\nR1");
        let expected = Panel {
            central_port_position: Position { x: 0, y: 0 },
            wires: vec![
                Wire {
                    path: vec![
                        Position { x: 0, y: 0 },
                        Position { x: 0, y: 1 },
                        Position { x: 0, y: 2 },
                        Position { x: 1, y: 2 },
                        Position { x: 2, y: 2 },
                        Position { x: 2, y: 1 },
                        Position { x: 1, y: 1 },
                    ],
                },
                Wire {
                    path: vec![Position { x: 0, y: 0 }, Position { x: 1, y: 0 }],
                },
            ],
        };
        let panel: Panel = result.unwrap();
        assert_eq!(panel, expected);
    }

    #[test]
    fn test_panel_get_intersection_points() {
        let input = Panel {
            central_port_position: Position { x: 0, y: 0 },
            wires: vec![
                Wire {
                    path: vec![
                        Position { x: 0, y: 0 },
                        Position { x: 0, y: 1 },
                        Position { x: 0, y: 2 },
                    ],
                },
                Wire {
                    path: vec![Position { x: 0, y: 0 }, Position { x: 0, y: 2 }],
                },
            ],
        };
        let expected = vec![Position { x: 0, y: 2 }];
        assert_eq!(input.get_intersection_points(), expected);
    }

    #[test]
    fn test_panel_get_intersection_points_with_example() {
        let input = parse_input_day3("R8,U5,L5,D3\nU7,R6,D4,L4");
        let panel = input.unwrap();
        let result = panel.get_intersection_points();
        assert!(result.contains(&Position { x: 3, y: 3 }));
        assert!(result.contains(&Position { x: 6, y: 5 }))
    }

    #[test]
    fn test_part1_example() {
        let input = parse_input_day3("R8,U5,L5,D3\nU7,R6,D4,L4");
        let panel = input.unwrap();
        assert_eq!(part1(&panel), 6);
    }

    #[test]
    fn test_part1_input1() {
        let input =
            parse_input_day3("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83");
        let panel = input.unwrap();
        assert_eq!(part1(&panel), 159);
    }

    #[test]
    fn test_part1_input2() {
        let input = parse_input_day3(
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
        );
        let panel = input.unwrap();
        assert_eq!(part1(&panel), 135);
    }

    #[test]
    fn test_part2_example() {
        let input = parse_input_day3("R8,U5,L5,D3\nU7,R6,D4,L4");
        let panel = input.unwrap();
        assert_eq!(part2(&panel), 30);
    }

    #[test]
    fn test_part2_input1() {
        let input =
            parse_input_day3("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83");

        let panel = input.unwrap();
        assert_eq!(part2(&panel), 610);
    }
    #[test]
    fn test_part2_input2() {
        let input = parse_input_day3(
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
        );
        let panel = input.unwrap();
        assert_eq!(part2(&panel), 410);
    }
}
