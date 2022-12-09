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
use std::ops::{Add, AddAssign, Sub};
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

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Point(isize, isize);

#[derive(Clone, Copy)]
struct Delta(isize, isize);

impl Delta {
    fn signum(&self) -> Delta {
        Delta(self.0.signum(), self.1.signum())
    }
}

impl Add<Delta> for Point {
    type Output = Point;
    fn add(self, dir: Delta) -> Self::Output {
        return Point(self.0 + dir.0, self.1 + dir.1);
    }
}

impl AddAssign<Delta> for Point {
    fn add_assign(&mut self, dir: Delta) {
        self.0 += dir.0;
        self.1 += dir.1;
    }
}

impl Sub for Point {
    type Output = Delta;
    fn sub(self, rhs: Self) -> Self::Output {
        Delta(self.0 - rhs.0, self.1 - rhs.1)
    }
}

fn update_position(head: &Point, tail: &Point) -> Point {
    //*
    let delta = *head - *tail;
    if delta.0.abs() > 1 || delta.1.abs() > 1 {
        *tail + delta.signum()
    } else {
        *tail
    }
}

fn solve_puzzle(puzzle: &Puzzle, knot_count: usize) -> usize {
    let mut knots: Vec<_> = std::iter::repeat(Point(0, 0)).take(knot_count).collect();
    let mut visited = HashSet::new();
    for m in &puzzle.moves {
        let (delta, count) = match m {
            Move::Horizontal(x) => (Delta(x.signum(), 0), x.abs()),
            Move::Vertical(y) => (Delta(0, y.signum()), y.abs()),
        };
        for _ in 0..count {
            knots[0] += delta;
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
