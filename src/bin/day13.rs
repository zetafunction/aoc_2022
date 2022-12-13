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
use std::cmp::Ordering;
use std::io::{self, Read};
use std::iter::Peekable;
use std::str::{Chars, FromStr};

#[derive(Clone, Debug)]
enum Data {
    List(Vec<Data>),
    Integer(u8),
}

impl Data {
    fn parse_integer(chars: &mut Peekable<Chars>) -> Result<u8, Oops> {
        let mut result: u8 = 0;
        while let Some(c) = chars.peek().copied() {
            if !c.is_ascii_digit() {
                break;
            }
            chars.next();
            result = result
                .checked_mul(10)
                .and_then(|x| x.checked_add(c as u8 - '0' as u8))
                .ok_or_else(|| oops!("integer too large"))?;
        }
        Ok(result)
    }

    fn parse_list(chars: &mut Peekable<Chars>) -> Result<Vec<Data>, Oops> {
        enum ParserState {
            Normal,
            WantItemDelimiter,
        }

        if chars.next().ok_or_else(|| oops!("unexpected end"))? != '[' {
            return Err(oops!("expected '['"));
        }
        let mut contents = vec![];
        let mut state = ParserState::Normal;
        loop {
            match (&state, chars.peek()) {
                (ParserState::Normal, Some('[')) => {
                    contents.push(Data::List(Self::parse_list(chars)?));
                    state = ParserState::WantItemDelimiter;
                    continue;
                }
                (ParserState::Normal, Some(c)) if c.is_ascii_digit() => {
                    contents.push(Data::Integer(Self::parse_integer(chars)?));
                    state = ParserState::WantItemDelimiter;
                    continue;
                }
                (ParserState::WantItemDelimiter, Some(',')) => {
                    // Eat the `,`.
                    chars.next();
                    state = ParserState::Normal;
                    continue;
                }
                (_, Some(c)) if c.is_ascii_whitespace() => {
                    // Eat whitespace.
                    chars.next();
                    continue;
                }
                (_, Some(']')) => {
                    // Eat closing `]`.
                    chars.next();
                    return Ok(contents);
                }
                (_, x) => return Err(oops!("unexpected state {:?}", x)),
            }
        }
    }
}

impl Eq for Data {}

impl Ord for Data {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Data::List(lhs), Data::List(rhs)) => lhs.cmp(rhs),
            (Data::Integer(lhs), Data::Integer(rhs)) => lhs.cmp(rhs),
            (Data::List(_), Data::Integer(value)) => {
                self.cmp(&Data::List(vec![Data::Integer(*value)]))
            }
            (Data::Integer(value), Data::List(_)) => {
                Data::List(vec![Data::Integer(*value)]).cmp(other)
            }
        }
    }
}

impl PartialEq for Data {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl PartialOrd for Data {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl FromStr for Data {
    type Err = Oops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Data::List(Self::parse_list(&mut s.chars().peekable())?))
    }
}

#[derive(Debug)]
struct Puzzle {
    data: Vec<(Data, Data)>,
}

impl FromStr for Puzzle {
    type Err = Oops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Puzzle {
            data: s
                .split("\n\n")
                .map(|chunk| {
                    if let Some((first, second)) = chunk.split_once('\n') {
                        Ok((first.parse()?, second.parse()?))
                    } else {
                        Err(oops!("missing line?"))
                    }
                })
                .collect::<Result<_, Oops>>()?,
        })
    }
}

fn parse(input: &str) -> Result<Puzzle, Oops> {
    input.parse()
}

fn part1(puzzle: &Puzzle) -> usize {
    (1..)
        .zip(puzzle.data.iter())
        .filter_map(|(i, (x, y))| {
            if x.cmp(y) == Ordering::Less {
                Some(i)
            } else {
                None
            }
        })
        .sum()
}

fn part2(puzzle: &Puzzle) -> usize {
    let mut packet_pairs = puzzle.data.clone();
    let (first, second): (Vec<_>, Vec<_>) = packet_pairs.drain(..).unzip();
    let mut sorted_packets = vec![];
    sorted_packets.extend(first);
    sorted_packets.extend(second);
    let first_divider = Data::List(vec![Data::List(vec![Data::Integer(2)])]);
    let second_divider = Data::List(vec![Data::List(vec![Data::Integer(6)])]);
    sorted_packets.push(first_divider.clone());
    sorted_packets.push(second_divider.clone());
    sorted_packets.sort();
    (1..)
        .zip(sorted_packets.iter())
        .filter_map(|(i, packet)| {
            if packet == &first_divider || packet == &second_divider {
                Some(i)
            } else {
                None
            }
        })
        .product()
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
        "[1,1,3,1,1]\n",
        "[1,1,5,1,1]\n",
        "\n",
        "[[1],[2,3,4]]\n",
        "[[1],4]\n",
        "\n",
        "[9]\n",
        "[[8,7,6]]\n",
        "\n",
        "[[4,4],4,4]\n",
        "[[4,4],4,4,4]\n",
        "\n",
        "[7,7,7,7]\n",
        "[7,7,7]\n",
        "\n",
        "[]\n",
        "[3]\n",
        "\n",
        "[[[]]]\n",
        "[[]]\n",
        "\n",
        "[1,[2,[3,[4,[5,6,7]]]],8,9]\n",
        "[1,[2,[3,[4,[5,6,0]]]],8,9]\n",
    );

    #[test]
    fn example1() {
        assert_eq!(13, part1(&parse(SAMPLE).unwrap()));
    }

    #[test]
    fn example2() {
        assert_eq!(140, part2(&parse(SAMPLE).unwrap()));
    }
}
