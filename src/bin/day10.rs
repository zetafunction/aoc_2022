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
use std::io::{self, Read};
use std::str::FromStr;

#[derive(Debug)]
enum Inst {
    Noop,
    AddX(i32),
}

impl FromStr for Inst {
    type Err = Oops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "noop" {
            Ok(Inst::Noop)
        } else {
            let words = s.split_whitespace().collect::<Vec<_>>();
            match words[0] {
                "addx" => Ok(Inst::AddX(words[1].parse()?)),
                _ => Err(oops!("no match")),
            }
        }
    }
}

struct Puzzle {
    instructions: Vec<Inst>,
}

impl FromStr for Puzzle {
    type Err = Oops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Puzzle {
            instructions: s
                .lines()
                .map(|line| line.trim().parse())
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

fn parse(input: &str) -> Result<Puzzle, Oops> {
    input.parse()
}

fn part1(puzzle: &Puzzle) -> i32 {
    const SPECIAL_CYCLES: &[usize] = &[20, 60, 100, 140, 180, 220];
    let mut reg_x: i32 = 1;
    let mut result: i32 = 0;
    let mut deferred_add = None;
    let mut index = 0;
    let mut cycle = 0;
    while index < puzzle.instructions.len() {
        cycle += 1;
        if SPECIAL_CYCLES.contains(&cycle) {
            let strength = cycle as i32 * reg_x;
            result += strength;
        }
        if let Some(value) = deferred_add.take() {
            reg_x += value;
            continue;
        }
        match puzzle.instructions[index] {
            Inst::Noop => {}
            Inst::AddX(x) => deferred_add = Some(x),
        }
        index += 1;
    }
    result
}

fn part2(puzzle: &Puzzle) -> usize {
    let mut reg_x: i32 = 1;
    let mut result: i32 = 0;
    let mut deferred_add = None;
    let mut index = 0;
    let mut cycle = 0;
    let mut pixels = Vec::new();
    let mut pixel_pos = 0;
    while index < puzzle.instructions.len() {
        cycle += 1;
        if ((pixel_pos % 40) as i32 - reg_x).abs() <= 1 {
            pixels.push('#');
        } else {
            pixels.push('.');
        }
        pixel_pos += 1;
        if let Some(value) = deferred_add.take() {
            reg_x += value;
            continue;
        }
        match puzzle.instructions[index] {
            Inst::Noop => {}
            Inst::AddX(x) => deferred_add = Some(x),
        }
        index += 1;
    }
    for line in pixels.chunks(40) {
        println!("{}", line.iter().collect::<String>());
    }
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

    const SAMPLE: &str = r#"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"#;

    #[test]
    fn example1() {
        assert_eq!(13, part1(&parse(SAMPLE).unwrap()));
    }

    #[test]
    fn example2() {
        assert_eq!(1, part2(&parse(SAMPLE).unwrap()));
    }
}
