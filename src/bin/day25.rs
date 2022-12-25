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

struct Puzzle {
    values: Vec<Snafu>,
}

enum SnafuDigit {
    DoubleMinus,
    Minus,
    Zero,
    One,
    Two,
}

struct Snafu {
    // Digits stored from least-significant to most-significant digit.
    digits: Vec<SnafuDigit>,
}

impl Snafu {
    fn to_decimal(&self) -> i64 {
        self.digits
            .iter()
            .enumerate()
            .fold(0, |acc, (position, digit)| {
                acc + 5i64.pow(position as u32)
                    * match digit {
                        SnafuDigit::DoubleMinus => -2,
                        SnafuDigit::Minus => -1,
                        SnafuDigit::Zero => 0,
                        SnafuDigit::One => 1,
                        SnafuDigit::Two => 2,
                    }
            })
    }

    fn from_decimal(mut x: i64) -> Self {
        let mut digits = vec![];
        while x != 0 {
            let current = x % 5;
            x = (x - match current {
                0 => 0,
                1 => 1,
                2 => 2,
                3 => -2,
                4 => -1,
                _ => panic!(),
            }) / 5;
            digits.push(match current {
                0 => SnafuDigit::Zero,
                1 => SnafuDigit::One,
                2 => SnafuDigit::Two,
                3 => SnafuDigit::DoubleMinus,
                4 => SnafuDigit::Minus,
                _ => panic!(),
            });
        }
        Self { digits }
    }
}

impl FromStr for Snafu {
    type Err = Oops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let digits = s
            .chars()
            .rev()
            .map(|c| {
                Ok::<_, Oops>(match c {
                    '=' => SnafuDigit::DoubleMinus,
                    '-' => SnafuDigit::Minus,
                    '0' => SnafuDigit::Zero,
                    '1' => SnafuDigit::One,
                    '2' => SnafuDigit::Two,
                    _ => Err(oops!("bad digit {c}"))?,
                })
            })
            .collect::<Result<_, _>>()?;
        Ok(Snafu { digits })
    }
}

impl ToString for Snafu {
    fn to_string(&self) -> String {
        self.digits
            .iter()
            .rev()
            .map(|digit| match digit {
                SnafuDigit::DoubleMinus => '=',
                SnafuDigit::Minus => '-',
                SnafuDigit::Zero => '0',
                SnafuDigit::One => '1',
                SnafuDigit::Two => '2',
            })
            .collect::<String>()
    }
}

impl FromStr for Puzzle {
    type Err = Oops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Puzzle {
            values: s.lines().map(str::parse).collect::<Result<Vec<_>, _>>()?,
        })
    }
}

fn parse(input: &str) -> Result<Puzzle, Oops> {
    input.parse()
}

fn part1(puzzle: &Puzzle) -> String {
    Snafu::from_decimal(puzzle.values.iter().map(|x| x.to_decimal()).sum()).to_string()
}

fn part2(_puzzle: &Puzzle) -> usize {
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
        "1=-0-2\n", "12111\n", "2=0=\n", "21\n", "2=01\n", "111\n", "20012\n", "112\n", "1=-1=\n",
        "1-12\n", "12\n", "1=\n", "122\n",
    );

    #[test]
    fn snafu_to_decimal() {
        assert_eq!(
            4890i64,
            parse(SAMPLE)
                .unwrap()
                .values
                .iter()
                .map(Snafu::to_decimal)
                .sum()
        );
    }

    #[test]
    fn example1() {
        assert_eq!("2=-1=0", part1(&parse(SAMPLE).unwrap()));
    }

    #[test]
    fn example2() {
        assert_eq!(0, part2(&parse(SAMPLE).unwrap()));
    }
}
