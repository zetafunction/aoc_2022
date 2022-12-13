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
use std::cmp::{Ordering, PartialEq, PartialOrd};
use std::io::{self, Read};
use std::str::FromStr;

#[derive(Clone, Debug)]
enum Data {
    List(Vec<Data>),
    Value(u8),
}

impl FromStr for Data {
    type Err = Oops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars = s.chars().collect::<Vec<_>>();
        match chars[0] {
            '[' => {
                let mut contents = vec![];
                let mut idx = 0;
                // Last character must be ']'
                while idx < chars.len() - 1 {
                    idx += 1;
                    match chars[idx] {
                        ']' => {
                            break;
                        }
                        ',' => {
                            continue;
                        }
                        c if c.is_ascii_whitespace() => {
                            continue;
                        }
                        '[' => {
                            // Found a sub-list, need to find matching end.
                            let mut nesting_count = 1;
                            let start_idx = idx;
                            while nesting_count > 0 {
                                idx += 1;
                                match chars[idx] {
                                    '[' => nesting_count += 1,
                                    ']' => nesting_count -= 1,
                                    _ => (),
                                }
                            }
                            // Consume the ']'.
                            idx += 1;
                            contents.push(s[start_idx..idx].parse()?);
                        }

                        c if c.is_ascii_digit() => {
                            // Need to consume until next ] or ,
                            let start_idx = idx;
                            loop {
                                idx += 1;
                                if chars[idx] == ']' || chars[idx] == ',' {
                                    break;
                                }
                            }
                            contents.push(Data::Value(s[start_idx..idx].parse()?));
                        }
                        c => return Err(oops!("unexpected char {} @ {}", c, idx)),
                    }
                }
                return Ok(Data::List(contents));
            }
            c => return Err(oops!("unexpected char {} @ 0", c)),
        }
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
                    let mut lines = chunk.lines();
                    let packet1 = lines.next().ok_or_else(|| oops!("missing line"))?;
                    let packet2 = lines.next().ok_or_else(|| oops!("missing line"))?;
                    Ok((packet1.parse()?, packet2.parse()?))
                })
                .collect::<Result<_, Oops>>()?,
        })
    }
}

fn is_ordered(lhs: &Data, rhs: &Data) -> Ordering {
    match (lhs, rhs) {
        (Data::List(lhs), Data::List(rhs)) => {
            for (x, y) in lhs.iter().zip(rhs) {
                match is_ordered(x, y) {
                    Ordering::Less => {
                        return Ordering::Less;
                    }
                    Ordering::Greater => {
                        return Ordering::Greater;
                    }
                    Ordering::Equal => {
                        continue;
                    }
                }
            }
            match (lhs.len(), rhs.len()) {
                (ll, rl) if ll < rl => Ordering::Less,
                (ll, rl) if ll > rl => Ordering::Greater,
                _ => Ordering::Equal,
            }
        }
        (Data::Value(lhs), Data::Value(rhs)) => lhs.cmp(rhs),
        (Data::List(_), Data::Value(value)) => {
            is_ordered(lhs, &Data::List(vec![Data::Value(*value)]))
        }
        (Data::Value(value), Data::List(_)) => {
            is_ordered(&Data::List(vec![Data::Value(*value)]), rhs)
        }
    }
}

impl Eq for Data {}

impl Ord for Data {
    fn cmp(&self, other: &Data) -> Ordering {
        is_ordered(self, other)
    }
}

impl PartialEq<Data> for Data {
    fn eq(&self, other: &Data) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl PartialOrd<Data> for Data {
    fn partial_cmp(&self, other: &Data) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse(input: &str) -> Result<Puzzle, Oops> {
    input.parse()
}

fn part1(puzzle: &Puzzle) -> Result<usize, Oops> {
    Ok((1..)
        .zip(puzzle.data.iter())
        .filter_map(|(i, (x, y))| {
            if is_ordered(x, y) == Ordering::Less {
                Some(i)
            } else {
                None
            }
        })
        .sum())
}

fn part2(puzzle: &Puzzle) -> Result<usize, Oops> {
    let mut data = puzzle.data.clone();
    let (first, second): (Vec<_>, Vec<_>) = data.drain(..).unzip();
    let mut to_sort = vec![];
    to_sort.extend(first);
    to_sort.extend(second);
    let divider_packet1 = Data::List(vec![Data::List(vec![Data::Value(2)])]);
    let divider_packet2 = Data::List(vec![Data::List(vec![Data::Value(6)])]);
    to_sort.push(divider_packet1.clone());
    to_sort.push(divider_packet2.clone());
    to_sort.sort();
    Ok((1..)
        .zip(to_sort.iter())
        .filter_map(|(i, packet)| {
            if packet == &divider_packet1 || packet == &divider_packet2 {
                Some(i)
            } else {
                None
            }
        })
        .product())
}

fn main() -> Result<(), Oops> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let input = input;

    let puzzle = parse(&input)?;

    println!("{}", part1(&puzzle)?);
    println!("{}", part2(&puzzle)?);

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
        assert_eq!(3579124689, part1(&parse(SAMPLE).unwrap()).unwrap());
    }

    #[test]
    fn example2() {
        assert_eq!(2468013579, part2(&parse(SAMPLE).unwrap()).unwrap());
    }
}
