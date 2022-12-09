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

use aoc_2022::{oops, oops::Oops};
use std::collections::HashSet;
use std::io::{self, Read};
use std::str::FromStr;

#[derive(Debug)]
enum Move {
    Horizontal(isize),
    Vertical(isize),
}

impl FromStr for Move {
    type Err = Oops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        if parts.len() != 2 {
            return Err(oops!("wrong size inputs"));
        }
        let distance: isize = parts[1].parse()?;
        match parts[0] {
            "U" => Ok(Move::Vertical(distance)),
            "R" => Ok(Move::Horizontal(distance)),
            "D" => Ok(Move::Vertical(-distance)),
            "L" => Ok(Move::Horizontal(-distance)),
            _ => Err(oops!("illegal move")),
        }
    }
}

struct Puzzle {
    moves: Vec<Move>,
}

impl FromStr for Puzzle {
    type Err = Oops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Puzzle {
            moves: s
                .lines()
                .map(|line| line.trim().parse())
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

fn parse(input: &str) -> Result<Puzzle, Oops> {
    input.parse()
}

fn reduce(x: isize) -> isize {
    if x > 0 {
        1
    } else {
        -1
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Point(isize, isize);

fn update_position(head: &Point, tail: &Point) -> Point {
    let mut new_tail = *tail;
    if (head.0 - tail.0).abs() > 1 {
        new_tail.0 += reduce(head.0 - tail.0);
        if (head.1 - tail.1) != 0 {
            new_tail.1 += reduce(head.1 - tail.1);
        }
    } else if (head.1 - tail.1).abs() > 1 {
        new_tail.1 += reduce(head.1 - tail.1);
        if (head.0 - tail.0) != 0 {
            new_tail.0 += reduce(head.0 - tail.0);
        }
    }
    new_tail
}

fn solve_puzzle(puzzle: &Puzzle, knot_count: usize) -> usize {
    let mut knots = Vec::<Point>::new();
    knots.resize(knot_count, Point(0, 0));
    let mut visited = HashSet::new();
    for m in &puzzle.moves {
        let (delta_x, delta_y, count) = match m {
            Move::Horizontal(x) => (reduce(*x), 0, x.abs()),
            Move::Vertical(y) => (0, reduce(*y), y.abs()),
        };
        for _ in 0..count {
            knots[0].0 += delta_x;
            knots[0].1 += delta_y;
            for i in 0..knots.len() - 1 {
                let new_position = update_position(&knots[i], &knots[i + 1]);
                knots[i + 1] = new_position;
            }
            visited.insert(knots[knots.len() - 1]);
        }
    }
    visited.len()
}

fn part1(puzzle: &Puzzle) -> usize {
    solve_puzzle(puzzle, 2)
}

fn part2(puzzle: &Puzzle) -> usize {
    solve_puzzle(puzzle, 10)
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

    const SAMPLE1: &str =
        concat!("R 4\n", "U 4\n", "L 3\n", "D 1\n", "R 4\n", "D 1\n", "L 5\n", "R 2\n",);
    const SAMPLE2: &str =
        concat!("R 5\n", "U 8\n", "L 8\n", "D 3\n", "R 17\n", "D 10\n", "L 25\n", "U 20\n",);

    #[test]
    fn example1() {
        assert_eq!(13, part1(&parse(SAMPLE1).unwrap()));
        assert_eq!(88, part1(&parse(SAMPLE2).unwrap()));
    }

    #[test]
    fn example2() {
        assert_eq!(1, part2(&parse(SAMPLE1).unwrap()));
        assert_eq!(36, part2(&parse(SAMPLE2).unwrap()));
    }
}
