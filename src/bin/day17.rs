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
use std::collections::VecDeque;
use std::io::{self, Read};
use std::str::FromStr;

enum Jet {
    Left,
    Right,
}

struct Puzzle {
    jets: Vec<Jet>,
}

const ROCKS: [[u16; 4]; 5] = [
    // This is actually the last rock, but it goes first since the iterator is double-advanced the
    // first time through the loop.
    [
        0b0001100000000000,
        0b0001100000000000,
        0b0000000000000000,
        0b0000000000000000,
    ],
    [
        0b0001111000000000,
        0b0000000000000000,
        0b0000000000000000,
        0b0000000000000000,
    ],
    [
        0b0000100000000000,
        0b0001110000000000,
        0b0000100000000000,
        0b0000000000000000,
    ],
    [
        0b0001110000000000,
        0b0000010000000000,
        0b0000010000000000,
        0b0000000000000000,
    ],
    [
        0b0001000000000000,
        0b0001000000000000,
        0b0001000000000000,
        0b0001000000000000,
    ],
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
    input.parse()
}

enum State {
    NewRock,
    Fall,
    Jet,
}

const GRID_ROWS: usize = 32768;

struct BitGrid {
    top: usize,
    bot: usize,
    used: usize,
    data: VecDeque<u16>,
}

impl BitGrid {
    fn new() -> Self {
        let mut data = VecDeque::with_capacity(GRID_ROWS);
        data.push_back(0xffff);
        data.extend(std::iter::repeat(0x80ff).take(GRID_ROWS - 1));
        BitGrid {
            top: 0,
            bot: 0,
            used: 1,
            data,
        }
    }

    fn maybe_update_top(&mut self, candidate: usize) {
        if candidate <= self.top {
            return;
        }
        let diff = candidate - self.top;
        self.used += diff;
        self.top = candidate;

        if GRID_ROWS - self.used < 1000 {
            self.data.drain(0..GRID_ROWS / 2);
            self.bot += GRID_ROWS / 2;
            self.used -= GRID_ROWS / 2;
            self.data
                .extend(std::iter::repeat(0x80ff).take(GRID_ROWS / 2));
        }
    }

    fn row(&self, n: usize) -> u16 {
        self.data.get(n - self.bot).copied().unwrap()
    }

    fn mut_row(&mut self, n: usize) -> &mut u16 {
        self.data.get_mut(n - self.bot).unwrap()
    }

    fn render(&self) {
        for i in (self.bot..=self.top).rev() {
            let data = self.row(i);
            println!(
                "{}",
                (7..16)
                    .rev()
                    .map(|pos| if data & (1 << pos) != 0 { '#' } else { '.' })
                    .collect::<String>()
            );
        }
    }
}

fn move_left_if_possible(chamber: &BitGrid, rock_bottom: usize, current_rock: &mut [u16; 4]) {
    if (current_rock[0] << 1) & chamber.row(rock_bottom) == 0
        && (current_rock[1] << 1) & chamber.row(rock_bottom + 1) == 0
        && (current_rock[2] << 1) & chamber.row(rock_bottom + 2) == 0
        && (current_rock[3] << 1) & chamber.row(rock_bottom + 3) == 0
    {
        current_rock[0] <<= 1;
        current_rock[1] <<= 1;
        current_rock[2] <<= 1;
        current_rock[3] <<= 1;
    }
}

fn move_right_if_possible(chamber: &BitGrid, rock_bottom: usize, current_rock: &mut [u16; 4]) {
    if (current_rock[0] >> 1) & chamber.row(rock_bottom) == 0
        && (current_rock[1] >> 1) & chamber.row(rock_bottom + 1) == 0
        && (current_rock[2] >> 1) & chamber.row(rock_bottom + 2) == 0
        && (current_rock[3] >> 1) & chamber.row(rock_bottom + 3) == 0
    {
        current_rock[0] >>= 1;
        current_rock[1] >>= 1;
        current_rock[2] >>= 1;
        current_rock[3] >>= 1;
    }
}

fn part1(puzzle: &Puzzle) -> Result<usize, Oops> {
    let mut rocks = ROCKS.iter().copied().cycle();
    let mut jets = puzzle.jets.iter().cycle();

    let mut state = State::NewRock;
    let mut chamber = BitGrid::new();

    let mut rock_count = 0;
    let mut current_rock = rocks.next().unwrap();
    // Represents the bottom of the current rock.
    let mut rock_bottom = 0;
    while rock_count < 2023 {
        match state {
            State::NewRock => {
                current_rock = rocks.next().unwrap();
                // Shortcut the simulation. Mainly saves some falling checks.
                // Each rock will (potentially) shift 4x and fall 3x.
                rock_bottom = chamber.top + 1;
                for _ in 0..4 {
                    match jets.next().unwrap() {
                        Jet::Left => {
                            move_left_if_possible(&chamber, rock_bottom, &mut current_rock)
                        }
                        Jet::Right => {
                            move_right_if_possible(&chamber, rock_bottom, &mut current_rock)
                        }
                    };
                }
                state = State::Fall;
                rock_count += 1;
                continue;
            }
            State::Fall => {
                if current_rock[0] & chamber.row(rock_bottom - 1) != 0
                    || current_rock[1] & chamber.row(rock_bottom) != 0
                    || current_rock[2] & chamber.row(rock_bottom + 1) != 0
                    || current_rock[3] & chamber.row(rock_bottom + 2) != 0
                {
                    chamber.maybe_update_top(
                        rock_bottom + current_rock.iter().filter(|&&x| x != 0).count() - 1,
                    );
                    *chamber.mut_row(rock_bottom) |= current_rock[0];
                    *chamber.mut_row(rock_bottom + 1) |= current_rock[1];
                    *chamber.mut_row(rock_bottom + 2) |= current_rock[2];
                    *chamber.mut_row(rock_bottom + 3) |= current_rock[3];
                    state = State::NewRock;
                    continue;
                }
                rock_bottom -= 1;
                state = State::Jet;
                continue;
            }
            State::Jet => {
                match jets.next().unwrap() {
                    Jet::Left => move_left_if_possible(&chamber, rock_bottom, &mut current_rock),
                    Jet::Right => move_right_if_possible(&chamber, rock_bottom, &mut current_rock),
                };
                state = State::Fall;
                continue;
            }
        }
    }
    Ok(chamber.top as usize)
}

fn part2(puzzle: &Puzzle) -> Result<usize, Oops> {
    let mut rocks = ROCKS.iter().copied().cycle();
    let mut jets = puzzle.jets.iter().cycle();

    let mut state = State::NewRock;
    let mut chamber = BitGrid::new();

    let mut rock_count = 0;
    let mut current_rock = rocks.next().unwrap();
    // Represents the bottom of the current rock.
    let mut rock_bottom = 0;
    while rock_count < 1_000_000_000_000u64 {
        match state {
            State::NewRock => {
                current_rock = rocks.next().unwrap();
                // Shortcut the simulation. Mainly saves some falling checks.
                // Each rock will (potentially) shift 4x and fall 3x.
                rock_bottom = chamber.top + 1;
                for _ in 0..4 {
                    match jets.next().unwrap() {
                        Jet::Left => {
                            move_left_if_possible(&chamber, rock_bottom, &mut current_rock)
                        }
                        Jet::Right => {
                            move_right_if_possible(&chamber, rock_bottom, &mut current_rock)
                        }
                    };
                }
                state = State::Fall;
                rock_count += 1;
                if rock_count % 100_000_000 == 0 {
                    println!("count: {}", rock_count);
                }
                continue;
            }
            State::Fall => {
                if current_rock[0] & chamber.row(rock_bottom - 1) != 0
                    || current_rock[1] & chamber.row(rock_bottom) != 0
                    || current_rock[2] & chamber.row(rock_bottom + 1) != 0
                    || current_rock[3] & chamber.row(rock_bottom + 2) != 0
                {
                    chamber.maybe_update_top(
                        rock_bottom + current_rock.iter().filter(|&&x| x != 0).count() - 1,
                    );
                    *chamber.mut_row(rock_bottom) |= current_rock[0];
                    *chamber.mut_row(rock_bottom + 1) |= current_rock[1];
                    *chamber.mut_row(rock_bottom + 2) |= current_rock[2];
                    *chamber.mut_row(rock_bottom + 3) |= current_rock[3];
                    state = State::NewRock;
                    continue;
                }
                rock_bottom -= 1;
                state = State::Jet;
                continue;
            }
            State::Jet => {
                match jets.next().unwrap() {
                    Jet::Left => move_left_if_possible(&chamber, rock_bottom, &mut current_rock),
                    Jet::Right => move_right_if_possible(&chamber, rock_bottom, &mut current_rock),
                };
                state = State::Fall;
                continue;
            }
        }
    }
    Ok(chamber.top as usize)
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
        assert_eq!(3068, part1(&parse(SAMPLE).unwrap()).unwrap());
    }

    #[test]
    fn example2() {
        assert_eq!(2468013579, part2(&parse(SAMPLE).unwrap()).unwrap());
    }
}
