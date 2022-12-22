//  Copyright 2022 Google LLC
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//      https://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.

use aoc_2022::geometry::Point2;
use aoc_2022::{oops, oops::Oops};
use std::collections::HashMap;
use std::io::{self, Read};
use std::str::FromStr;

enum Tile {
    Wall,
    Open,
    Nothing,
}

enum Move {
    Ahead(i32),
    Left,
    Right,
}

struct Puzzle {
    map: HashMap<Point2, Tile>,
    moves: Vec<Move>,
    max_x: i32,
    max_y: i32,
}

impl Puzzle {
    fn get_next(&self, p: Point2, d: Direction) -> Option<Point2> {
        let candidate = match d {
            Direction::North => Point2::new(p.x, p.y - 1),
            Direction::East => Point2::new(p.x + 1, p.y),
            Direction::South => Point2::new(p.x, p.y + 1),
            Direction::West => Point2::new(p.x - 1, p.y),
        };

        match self.map.get(&candidate) {
            Some(Tile::Open) => return Some(candidate),
            Some(Tile::Wall) => return None,
            Some(Tile::Nothing) | None => match d {
                Direction::North => {
                    for y in (0..=self.max_y).rev() {
                        match self.map.get(&Point2::new(p.x, y)) {
                            Some(Tile::Open) => return Some(Point2::new(p.x, y)),
                            Some(Tile::Wall) => return None,
                            Some(Tile::Nothing) | None => continue,
                        }
                    }
                }
                Direction::East => {
                    for x in 0.. {
                        match self.map.get(&Point2::new(x, p.y)) {
                            Some(Tile::Open) => return Some(Point2::new(x, p.y)),
                            Some(Tile::Wall) => return None,
                            Some(Tile::Nothing) | None => continue,
                        }
                    }
                }
                Direction::South => {
                    for y in 0.. {
                        match self.map.get(&Point2::new(p.x, y)) {
                            Some(Tile::Open) => return Some(Point2::new(p.x, y)),
                            Some(Tile::Wall) => return None,
                            Some(Tile::Nothing) | None => continue,
                        }
                    }
                }
                Direction::West => {
                    for x in (0..=self.max_x).rev() {
                        match self.map.get(&Point2::new(x, p.y)) {
                            Some(Tile::Open) => return Some(Point2::new(x, p.y)),
                            Some(Tile::Wall) => return None,
                            Some(Tile::Nothing) | None => continue,
                        }
                    }
                }
            },
        }

        None
    }
}

fn parse_moves(s: &str) -> Vec<Move> {
    let mut moves = vec![];
    let mut n = None;
    for c in s.chars() {
        if c.is_ascii_digit() {
            n = match n {
                Some(n) => Some(n * 10 + c as i32 - '0' as i32),
                None => Some(c as i32 - '0' as i32),
            };
        } else {
            if let Some(n) = n {
                moves.push(Move::Ahead(n));
            }
            match c {
                'L' => {
                    moves.push(Move::Left);
                }
                'R' => {
                    moves.push(Move::Right);
                }
                _ => (),
            }
        }
    }
    moves
}

impl FromStr for Puzzle {
    type Err = Oops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = HashMap::new();
        let (map_str, moves_str) = s
            .split_once("\n\n")
            .ok_or_else(|| oops!("invalid format"))?;
        for (y, line) in map_str.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    '#' => {
                        map.insert(Point2::new(x as i32, y as i32), Tile::Wall);
                    }
                    '.' => {
                        map.insert(Point2::new(x as i32, y as i32), Tile::Open);
                    }
                    _ => (),
                }
            }
        }
        let max_x = map.keys().map(|p| p.x).max().unwrap();
        let max_y = map.keys().map(|p| p.y).max().unwrap();
        let moves = parse_moves(moves_str);
        Ok(Puzzle {
            map,
            moves,
            max_x,
            max_y,
        })
    }
}

fn parse(input: &str) -> Result<Puzzle, Oops> {
    input.parse()
}

#[derive(Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn turn_left(d: Direction) -> Direction {
    match d {
        Direction::North => Direction::West,
        Direction::East => Direction::North,
        Direction::South => Direction::East,
        Direction::West => Direction::South,
    }
}

fn turn_right(d: Direction) -> Direction {
    match d {
        Direction::North => Direction::East,
        Direction::East => Direction::South,
        Direction::South => Direction::West,
        Direction::West => Direction::North,
    }
}

fn part1(puzzle: &Puzzle) -> i32 {
    // Find the starting point.
    let mut current_pos = Point2::new(0, 0);
    for x in 0.. {
        let p = Point2::new(x, 0);
        if let Some(tile) = puzzle.map.get(&p) {
            match tile {
                Tile::Open => {
                    current_pos = p;
                    break;
                }
                _ => continue,
            }
        }
    }

    let mut direction = Direction::East;

    for m in &puzzle.moves {
        match m {
            Move::Left => direction = turn_left(direction),
            Move::Right => direction = turn_right(direction),
            Move::Ahead(n) => {
                for _ in 0..*n {
                    if let Some(next) = puzzle.get_next(current_pos, direction) {
                        current_pos = next;
                    } else {
                        break;
                    }
                }
            }
        }
    }
    1000 * current_pos.y
        + 4 * current_pos.x
        + match direction {
            Direction::East => 0,
            Direction::South => 1,
            Direction::West => 2,
            Direction::North => 3,
        }
}

fn part2(puzzle: &Puzzle) -> usize {
    0
}

fn main() -> Result<(), Oops> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let input = input;

    let puzzle = parse(&input)?;

    println!("{}", part1(&puzzle));
    println!("{}", part2(&puzzle));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = concat!(
        "        ...#\n",
        "        .#..\n",
        "        #...\n",
        "        ....\n",
        "...#.......#\n",
        "........#...\n",
        "..#....#....\n",
        "..........#.\n",
        "        ...#....\n",
        "        .....#..\n",
        "        .#......\n",
        "        ......#.\n",
        "\n",
        "10R5L5R10L4R5L5\n",
    );

    #[test]
    fn example1() {
        assert_eq!(6032, part1(&parse(SAMPLE).unwrap()));
    }

    #[test]
    fn example2() {
        assert_eq!(2468013579, part2(&parse(SAMPLE).unwrap()));
    }
}
