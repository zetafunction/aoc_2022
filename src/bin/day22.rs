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
use std::collections::{HashMap, HashSet};
use std::io::{self, Read};
use std::str::FromStr;

enum Tile {
    Wall,
    Open,
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
    step: i32,
}

#[derive(Clone, Copy, Debug)]
struct LocalPoint2 {
    p: Point2,
    square_x: i32,
    square_y: i32,
}

impl LocalPoint2 {
    fn square(&self) -> i32 {
        // TODO(this should depend on whether it is 4x3 or 3x4.
        // Or we should just assign IDs based on a hash map (better).
        self.square_y * 3 + self.square_x
    }
}

impl Puzzle {
    fn global_to_local(&self, g: Point2) -> LocalPoint2 {
        let square_x = g.x / self.step;
        let square_y = g.y / self.step;
        LocalPoint2 {
            p: Point2::new(g.x % self.step, g.y % self.step),
            square_x,
            square_y,
        }
    }

    fn local_to_global(&self, l: LocalPoint2) -> Point2 {
        Point2::new(
            l.square_x * self.step + l.p.x,
            l.square_y * self.step + l.p.y,
        )
    }

    fn get_next2(&self, p: Point2, d: Direction) -> Option<(Point2, Direction)> {
        let candidate = match d {
            Direction::North => Point2::new(p.x, p.y - 1),
            Direction::East => Point2::new(p.x + 1, p.y),
            Direction::South => Point2::new(p.x, p.y + 1),
            Direction::West => Point2::new(p.x - 1, p.y),
        };

        match self.map.get(&candidate) {
            Some(Tile::Open) => return Some((candidate, d)),
            Some(Tile::Wall) => return None,
            None => {
                let local = self.global_to_local(p);
                let (new_local, new_direction) = match d {
                    Direction::North => match local.square() {
                        1 => {
                            let new_local = LocalPoint2 {
                                p: Point2::new(local.p.y, local.p.x),
                                square_x: 0,
                                square_y: 3,
                            };
                            let direction = Direction::East;
                            (new_local, direction)
                        }
                        2 => {
                            let new_local = LocalPoint2 {
                                p: Point2::new(local.p.x, (self.step - 1) - local.p.y),
                                square_x: 0,
                                square_y: 3,
                            };
                            let direction = Direction::North;
                            (new_local, direction)
                        }
                        6 => {
                            let new_local = LocalPoint2 {
                                p: Point2::new(local.p.y, local.p.x),
                                square_x: 1,
                                square_y: 1,
                            };
                            let direction = Direction::East;
                            (new_local, direction)
                        }
                        _ => panic!(),
                    },
                    Direction::East => match local.square() {
                        2 => {
                            let new_local = LocalPoint2 {
                                p: Point2::new(local.p.x, (self.step - 1) - local.p.y),
                                square_x: 1,
                                square_y: 2,
                            };
                            let direction = Direction::West;
                            (new_local, direction)
                        }
                        4 => {
                            let new_local = LocalPoint2 {
                                p: Point2::new(local.p.y, local.p.x),
                                square_x: 2,
                                square_y: 0,
                            };
                            let direction = Direction::North;
                            (new_local, direction)
                        }
                        7 => {
                            let new_local = LocalPoint2 {
                                p: Point2::new(local.p.x, (self.step - 1) - local.p.y),
                                square_x: 2,
                                square_y: 0,
                            };
                            let direction = Direction::West;
                            (new_local, direction)
                        }
                        9 => {
                            let new_local = LocalPoint2 {
                                p: Point2::new(local.p.y, local.p.x),
                                square_x: 1,
                                square_y: 2,
                            };
                            let direction = Direction::North;
                            (new_local, direction)
                        }
                        _ => panic!(),
                    },
                    Direction::South => match local.square() {
                        2 => {
                            let new_local = LocalPoint2 {
                                p: Point2::new(local.p.y, local.p.x),
                                square_x: 1,
                                square_y: 1,
                            };
                            let direction = Direction::West;
                            (new_local, direction)
                        }
                        7 => {
                            let new_local = LocalPoint2 {
                                p: Point2::new(local.p.y, local.p.x),
                                square_x: 0,
                                square_y: 3,
                            };
                            let direction = Direction::West;
                            (new_local, direction)
                        }
                        9 => {
                            let new_local = LocalPoint2 {
                                p: Point2::new(local.p.x, (self.step - 1) - local.p.y),
                                square_x: 2,
                                square_y: 0,
                            };
                            let direction = Direction::South;
                            (new_local, direction)
                        }
                        _ => panic!(),
                    },
                    Direction::West => match local.square() {
                        1 => {
                            let new_local = LocalPoint2 {
                                p: Point2::new(local.p.x, (self.step - 1) - local.p.y),
                                square_x: 0,
                                square_y: 2,
                            };
                            let direction = Direction::East;
                            (new_local, direction)
                        }
                        4 => {
                            let new_local = LocalPoint2 {
                                p: Point2::new(local.p.y, local.p.x),
                                square_x: 0,
                                square_y: 2,
                            };
                            let direction = Direction::South;
                            (new_local, direction)
                        }
                        6 => {
                            let new_local = LocalPoint2 {
                                p: Point2::new(local.p.x, (self.step - 1) - local.p.y),
                                square_x: 1,
                                square_y: 0,
                            };
                            let direction = Direction::East;
                            (new_local, direction)
                        }
                        9 => {
                            let new_local = LocalPoint2 {
                                p: Point2::new(local.p.y, local.p.x),
                                square_x: 1,
                                square_y: 0,
                            };
                            let direction = Direction::South;
                            (new_local, direction)
                        }
                        _ => panic!(),
                    },
                };
                let new_global = self.local_to_global(new_local);
                match self.map.get(&new_global) {
                    None => panic!(),
                    Some(Tile::Open) => return Some((new_global, new_direction)),
                    _ => return None,
                }
            }
        }
    }

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
            None => match d {
                Direction::North => {
                    for y in (0..=self.max_y).rev() {
                        match self.map.get(&Point2::new(p.x, y)) {
                            Some(Tile::Open) => return Some(Point2::new(p.x, y)),
                            Some(Tile::Wall) => return None,
                            None => continue,
                        }
                    }
                }
                Direction::East => {
                    for x in 0.. {
                        match self.map.get(&Point2::new(x, p.y)) {
                            Some(Tile::Open) => return Some(Point2::new(x, p.y)),
                            Some(Tile::Wall) => return None,
                            None => continue,
                        }
                    }
                }
                Direction::South => {
                    for y in 0.. {
                        match self.map.get(&Point2::new(p.x, y)) {
                            Some(Tile::Open) => return Some(Point2::new(p.x, y)),
                            Some(Tile::Wall) => return None,
                            None => continue,
                        }
                    }
                }
                Direction::West => {
                    for x in (0..=self.max_x).rev() {
                        match self.map.get(&Point2::new(x, p.y)) {
                            Some(Tile::Open) => return Some(Point2::new(x, p.y)),
                            Some(Tile::Wall) => return None,
                            None => continue,
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
            n = None;
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
                let p = Point2::new(x.try_into()?, y.try_into()?);
                match c {
                    '#' => {
                        map.insert(p, Tile::Wall);
                    }
                    '.' => {
                        map.insert(p, Tile::Open);
                    }
                    _ => (),
                }
            }
        }
        let max_x = map.keys().map(|p| p.x).max().unwrap();
        let max_y = map.keys().map(|p| p.y).max().unwrap();
        let moves = parse_moves(moves_str);
        let step = if max_x > max_y {
            (max_x + 1) / 4
        } else {
            (max_y + 1) / 4
        };
        Ok(Puzzle {
            map,
            moves,
            max_x,
            max_y,
            step,
        })
    }
}

fn parse(input: &str) -> Result<Puzzle, Oops> {
    input.parse()
}

#[derive(Clone, Copy, Debug)]
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

fn print(puzzle: &Puzzle, current_pos: Point2, d: Direction) {
    println!("Current state:");
    for y in 0..puzzle.max_y {
        println!(
            "{}",
            (0..puzzle.max_x)
                .map(|x| {
                    let p = Point2::new(x, y);
                    if p == current_pos {
                        return match d {
                            Direction::North => '^',
                            Direction::East => '>',
                            Direction::South => 'v',
                            Direction::West => '<',
                        };
                    }
                    if let Some(tile) = puzzle.map.get(&p) {
                        match tile {
                            Tile::Wall => '#',
                            Tile::Open => '.',
                        }
                    } else {
                        ' '
                    }
                })
                .collect::<String>()
        );
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
    1000 * (current_pos.y + 1)
        + 4 * (current_pos.x + 1)
        + match direction {
            Direction::East => 0,
            Direction::South => 1,
            Direction::West => 2,
            Direction::North => 3,
        }
}

fn part2(puzzle: &Puzzle) -> i32 {
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
                    if let Some(next) = puzzle.get_next2(current_pos, direction) {
                        current_pos = next.0;
                        direction = next.1;
                    } else {
                        break;
                    }
                }
            }
        }
    }
    1000 * (current_pos.y + 1)
        + 4 * (current_pos.x + 1)
        + match direction {
            Direction::East => 0,
            Direction::South => 1,
            Direction::West => 2,
            Direction::North => 3,
        }
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
