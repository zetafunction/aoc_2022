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

#[derive(Clone, Copy)]
enum Op {
    Nop,
    AddX(i32),
}

struct Puzzle {
    ops: Vec<Op>,
}

#[derive(Clone, Copy, Debug)]
struct CpuState {
    x: i32,
}

impl CpuState {
    fn new() -> Self {
        CpuState { x: 1 }
    }
}

fn parse_instruction(s: &str) -> Vec<Result<Op, Oops>> {
    let mut parser = s.split_whitespace();
    let failed = || vec![Err(oops!("decode failed: {}", s))];
    let Some(instruction) = parser.next() else {
        return failed();
    };
    match instruction {
        "noop" => vec![Ok(Op::Nop)],
        "addx" => {
            let Some(addend) = parser.next().and_then(|x| x.parse().ok()) else {
                return failed();
            };
            vec![Ok(Op::Nop), Ok(Op::AddX(addend))]
        }
        _ => failed(),
    }
}

impl FromStr for Puzzle {
    type Err = Oops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Puzzle {
            ops: s
                .lines()
                .flat_map(|line| parse_instruction(line.trim()))
                .collect::<Result<_, _>>()?,
        })
    }
}

fn parse(input: &str) -> Result<Puzzle, Oops> {
    input.parse()
}

fn execute_program(ops: &[Op]) -> Vec<CpuState> {
    ops.iter()
        .scan(CpuState::new(), |state, next| {
            let current_state = *state;
            match next {
                Op::Nop => {}
                Op::AddX(delta) => {
                    state.x += delta;
                }
            };
            Some(current_state)
        })
        .collect()
}

fn part1(puzzle: &Puzzle) -> i32 {
    let states = execute_program(&puzzle.ops);
    (1..)
        .zip(states)
        .skip(19)
        .step_by(40)
        .map(|(i, state)| i as i32 * state.x)
        .sum()
}

fn part2(puzzle: &Puzzle) -> String {
    let states = execute_program(&puzzle.ops);
    let pixels: String = (0..40)
        .cycle()
        .zip(states.iter())
        .flat_map(|(cursor, state)| {
            if cursor == 0 { Some('\n') } else { None }
                .into_iter()
                .chain(if ((cursor % 40) as i32 - state.x).abs() <= 1 {
                    Some('#').into_iter()
                } else {
                    Some('.').into_iter()
                })
        })
        // Skip the initial newline.
        .skip(1)
        .chain(Some('\n').into_iter())
        .collect();
    pixels
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
        "addx 15\n",
        "addx -11\n",
        "addx 6\n",
        "addx -3\n",
        "addx 5\n",
        "addx -1\n",
        "addx -8\n",
        "addx 13\n",
        "addx 4\n",
        "noop\n",
        "addx -1\n",
        "addx 5\n",
        "addx -1\n",
        "addx 5\n",
        "addx -1\n",
        "addx 5\n",
        "addx -1\n",
        "addx 5\n",
        "addx -1\n",
        "addx -35\n",
        "addx 1\n",
        "addx 24\n",
        "addx -19\n",
        "addx 1\n",
        "addx 16\n",
        "addx -11\n",
        "noop\n",
        "noop\n",
        "addx 21\n",
        "addx -15\n",
        "noop\n",
        "noop\n",
        "addx -3\n",
        "addx 9\n",
        "addx 1\n",
        "addx -3\n",
        "addx 8\n",
        "addx 1\n",
        "addx 5\n",
        "noop\n",
        "noop\n",
        "noop\n",
        "noop\n",
        "noop\n",
        "addx -36\n",
        "noop\n",
        "addx 1\n",
        "addx 7\n",
        "noop\n",
        "noop\n",
        "noop\n",
        "addx 2\n",
        "addx 6\n",
        "noop\n",
        "noop\n",
        "noop\n",
        "noop\n",
        "noop\n",
        "addx 1\n",
        "noop\n",
        "noop\n",
        "addx 7\n",
        "addx 1\n",
        "noop\n",
        "addx -13\n",
        "addx 13\n",
        "addx 7\n",
        "noop\n",
        "addx 1\n",
        "addx -33\n",
        "noop\n",
        "noop\n",
        "noop\n",
        "addx 2\n",
        "noop\n",
        "noop\n",
        "noop\n",
        "addx 8\n",
        "noop\n",
        "addx -1\n",
        "addx 2\n",
        "addx 1\n",
        "noop\n",
        "addx 17\n",
        "addx -9\n",
        "addx 1\n",
        "addx 1\n",
        "addx -3\n",
        "addx 11\n",
        "noop\n",
        "noop\n",
        "addx 1\n",
        "noop\n",
        "addx 1\n",
        "noop\n",
        "noop\n",
        "addx -13\n",
        "addx -19\n",
        "addx 1\n",
        "addx 3\n",
        "addx 26\n",
        "addx -30\n",
        "addx 12\n",
        "addx -1\n",
        "addx 3\n",
        "addx 1\n",
        "noop\n",
        "noop\n",
        "noop\n",
        "addx -9\n",
        "addx 18\n",
        "addx 1\n",
        "addx 2\n",
        "noop\n",
        "noop\n",
        "addx 9\n",
        "noop\n",
        "noop\n",
        "noop\n",
        "addx -1\n",
        "addx 2\n",
        "addx -37\n",
        "addx 1\n",
        "addx 3\n",
        "noop\n",
        "addx 15\n",
        "addx -21\n",
        "addx 22\n",
        "addx -6\n",
        "addx 1\n",
        "noop\n",
        "addx 2\n",
        "addx 1\n",
        "noop\n",
        "addx -10\n",
        "noop\n",
        "noop\n",
        "addx 20\n",
        "addx 1\n",
        "addx 2\n",
        "addx 2\n",
        "addx -6\n",
        "addx -11\n",
        "noop\n",
        "noop\n",
        "noop\n",
    );

    #[test]
    fn example1() {
        assert_eq!(13140, part1(&parse(SAMPLE).unwrap()));
    }

    #[test]
    fn example2() {
        const OUTPUT: &str = concat!(
            "##..##..##..##..##..##..##..##..##..##..\n",
            "###...###...###...###...###...###...###.\n",
            "####....####....####....####....####....\n",
            "#####.....#####.....#####.....#####.....\n",
            "######......######......######......####\n",
            "#######.......#######.......#######.....\n",
        );
        assert_eq!(OUTPUT, part2(&parse(SAMPLE).unwrap()));
    }
}
