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

enum Jet {
    Left,
    Right,
}

struct Puzzle {
    jets: Vec<Jet>,
}

// All rocks are assumed to be anchored at 0, 0
const ROCKS: &[&[(i64, i64)]] = &[
    // This is actually the last rock but we call next() 2x at the start.
    &[(0, 0), (0, 1), (1, 1), (1, 0)],
    &[(0, 0), (1, 0), (2, 0), (3, 0)],
    &[(0, 1), (1, 2), (1, 1), (1, 0), (2, 1)],
    &[(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
    &[(0, 0), (0, 1), (0, 2), (0, 3)],
];

impl FromStr for Puzzle {
    type Err = Oops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Puzzle {
            jets: s
                .trim()
                .chars()
                .map(|c| {
                    Ok::<_, Oops>(match c {
                        '<' => Jet::Left,
                        '>' => Jet::Right,
                        c => Err(oops!("unexpected jet {}", c))?,
                    })
                })
                .collect::<Result<_, _>>()?,
        })
    }
}

fn parse(input: &str) -> Result<Puzzle, Oops> {
    let p = input.parse::<Puzzle>()?;
    println!("jet count: {}", p.jets.len());
    Ok(p)
}

enum State {
    NewRock,
    Fall,
    Jet,
}

fn render_chamber(chamber: &HashSet<(i64, i64)>, top: i64) {
    println!(".......");
    println!(".......");
    for y in (0..=top).rev() {
        println!(
            "{}",
            (0..=6)
                .map(|x| if chamber.contains(&(x, y)) { '#' } else { '.' })
                .collect::<String>()
        );
    }
    println!("+-----+");
}

fn part1(puzzle: &Puzzle) -> Result<usize, Oops> {
    let mut rocks = ROCKS.iter().cycle();
    let mut jets = puzzle.jets.iter().cycle();
    let mut rock_pos = (0, 0);
    let mut rock_count = 0;
    let mut state = State::NewRock;
    let mut chamber = HashSet::new();
    chamber.extend((0..7).into_iter().map(|x| ((x, 0))));
    let mut top = 0;
    let mut current_rock = rocks.next().unwrap();
    while rock_count < 2023 {
        match state {
            State::NewRock => {
                current_rock = rocks.next().unwrap();
                rock_pos = (2, top + 4);
                state = State::Jet;
                rock_count += 1;
                continue;
            }
            State::Fall => {
                if let Some(_) = current_rock.iter().find_map(|&pos| {
                    let pos = (pos.0 + rock_pos.0, pos.1 + rock_pos.1 - 1);
                    chamber.get(&pos)
                }) {
                    for pos in current_rock.iter() {
                        top = std::cmp::max(top, pos.1 + rock_pos.1);
                        chamber.insert((pos.0 + rock_pos.0, pos.1 + rock_pos.1));
                    }
                    state = State::NewRock;
                    continue;
                }
                rock_pos = (rock_pos.0, rock_pos.1 - 1);
                state = State::Jet;
                continue;
            }
            State::Jet => {
                let offset = match jets.next().unwrap() {
                    Jet::Left => -1,
                    Jet::Right => 1,
                };
                if let None = current_rock.iter().find_map(|&pos| {
                    if pos.0 + rock_pos.0 + offset < 0 {
                        Some(true)
                    } else if pos.0 + rock_pos.0 + offset >= 7 {
                        Some(true)
                    } else if let Some(_) = current_rock.iter().find_map(|&pos| {
                        let pos = (pos.0 + rock_pos.0 + offset, pos.1 + rock_pos.1);
                        chamber.get(&pos)
                    }) {
                        Some(true)
                    } else {
                        None
                    }
                }) {
                    rock_pos = (rock_pos.0 + offset, rock_pos.1);
                }
                state = State::Fall;
                continue;
            }
        }
    }
    Ok(top as usize)
}

fn part2(puzzle: &Puzzle) -> Result<usize, Oops> {
    let mut rocks = ROCKS.iter().cycle();
    let mut jets = puzzle.jets.iter().cycle();
    let mut rock_pos = (0, 0);
    let mut rock_count = 0i64;
    let mut state = State::NewRock;
    let mut chamber = HashSet::new();
    chamber.extend((0..7).into_iter().map(|x| ((x, 0))));
    let mut top = 0;
    let mut current_rock = rocks.next().unwrap();
    while rock_count < 1_000_000_000_000 {
        match state {
            State::NewRock => {
                current_rock = rocks.next().unwrap();
                rock_pos = (2, top + 4);
                state = State::Jet;
                if rock_count % puzzle.jets.len() as i64 == 0 {
                    println!("current top (iteration {}): {}", rock_count, top);
                }
                rock_count += 1;
                continue;
            }
            State::Fall => {
                if let Some(_) = current_rock.iter().find_map(|&pos| {
                    let pos = (pos.0 + rock_pos.0, pos.1 + rock_pos.1 - 1);
                    chamber.get(&pos)
                }) {
                    for pos in current_rock.iter() {
                        top = std::cmp::max(top, pos.1 + rock_pos.1);
                        chamber.insert((pos.0 + rock_pos.0, pos.1 + rock_pos.1));
                    }
                    state = State::NewRock;
                    continue;
                }
                rock_pos = (rock_pos.0, rock_pos.1 - 1);
                state = State::Jet;
                continue;
            }
            State::Jet => {
                let offset = match jets.next().unwrap() {
                    Jet::Left => -1,
                    Jet::Right => 1,
                };
                if let None = current_rock.iter().find_map(|&pos| {
                    if pos.0 + rock_pos.0 + offset < 0 {
                        Some(true)
                    } else if pos.0 + rock_pos.0 + offset >= 7 {
                        Some(true)
                    } else if let Some(_) = current_rock.iter().find_map(|&pos| {
                        let pos = (pos.0 + rock_pos.0 + offset, pos.1 + rock_pos.1);
                        chamber.get(&pos)
                    }) {
                        Some(true)
                    } else {
                        None
                    }
                }) {
                    rock_pos = (rock_pos.0 + offset, rock_pos.1);
                }
                state = State::Fall;
                continue;
            }
        }
    }
    Ok(top as usize)
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

    const SAMPLE: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn example1() {
        assert_eq!(3579124689, part1(&parse(SAMPLE).unwrap()).unwrap());
    }

    #[test]
    fn example2() {
        assert_eq!(2468013579, part2(&parse(SAMPLE).unwrap()).unwrap());
    }
}
