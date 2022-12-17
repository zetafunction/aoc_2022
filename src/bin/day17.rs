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

type Row = u32;

enum Jet {
    Left,
    Right,
}

struct Puzzle {
    jets: Vec<Jet>,
}

const ROCKS: [[Row; 4]; 5] = [
    // This is actually the last rock, but it goes first since the iterator is double-advanced the
    // first time through the loop.
    [
        0b00011000000000000000000000000000,
        0b00011000000000000000000000000000,
        0b00000000000000000000000000000000,
        0b00000000000000000000000000000000,
    ],
    [
        0b00011110000000000000000000000000,
        0b00000000000000000000000000000000,
        0b00000000000000000000000000000000,
        0b00000000000000000000000000000000,
    ],
    [
        0b00001000000000000000000000000000,
        0b00011100000000000000000000000000,
        0b00001000000000000000000000000000,
        0b00000000000000000000000000000000,
    ],
    [
        0b00011100000000000000000000000000,
        0b00000100000000000000000000000000,
        0b00000100000000000000000000000000,
        0b00000000000000000000000000000000,
    ],
    [
        0b00010000000000000000000000000000,
        0b00010000000000000000000000000000,
        0b00010000000000000000000000000000,
        0b00010000000000000000000000000000,
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
    FallJet,
}

const GRID_ROWS: usize = 1024;

struct BitGrid {
    base: usize,
    used: usize,
    data: [Row; GRID_ROWS],
}

impl BitGrid {
    fn new() -> Self {
        let mut grid = BitGrid {
            base: 0,
            used: 10,
            data: [0; GRID_ROWS],
        };
        grid.data[0] = 0xffffffff;
        for i in 1..GRID_ROWS {
            grid.data[i] = 0x80ffffff;
        }
        grid
    }

    fn mark_new_rows_used(&mut self, delta: usize) {
        self.used += delta;

        if self.used > GRID_ROWS - 10 {
            let capacity_to_free = GRID_ROWS / 2;
            let old_base = self.base;
            let new_base = self.base + capacity_to_free;
            for i in old_base..new_base {
                self.data[i % GRID_ROWS] = 0x80ffffff;
            }
            self.base = new_base % GRID_ROWS;
            self.used -= capacity_to_free;
        }
    }

    fn row(&self, n: usize) -> Row {
        self.data[n % GRID_ROWS]
    }

    fn mut_row(&mut self, n: usize) -> &mut Row {
        &mut self.data[n % GRID_ROWS]
    }

    fn render(&self) {
        println!("drawing from {} to {}", self.base, self.base + self.used);
        for i in (self.base..=self.base + self.used).rev() {
            let data = self.data[i % GRID_ROWS];
            println!(
                "{}",
                (23..32)
                    .rev()
                    .map(|pos| if data & (1 << pos) != 0 { '#' } else { '.' })
                    .collect::<String>()
            );
        }
    }
}

fn move_left_if_possible(chamber: &BitGrid, rock_bottom: usize, current_rock: &mut [Row; 4]) {
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

fn move_right_if_possible(chamber: &BitGrid, rock_bottom: usize, current_rock: &mut [Row; 4]) {
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

fn run_simulation<const MAX_ROCK_COUNT: usize>(puzzle: &Puzzle) -> usize {
    let mut rocks = ROCKS.iter().copied().cycle();
    let mut jets = puzzle.jets.iter().cycle();

    let mut state = State::NewRock;
    let mut chamber = BitGrid::new();

    let mut rock_count = 0;
    let mut current_rock = rocks.next().unwrap();
    // Represents the bottom of the current rock.
    let mut topmost_rock = 0;
    let mut rock_bottom = 1;
    while rock_count < MAX_ROCK_COUNT {
        match state {
            State::NewRock => {
                current_rock = rocks.next().unwrap();
                rock_bottom = topmost_rock + 1;
                // Normally, rocks start at chamber.top + 4. However, it is guaranteed that each
                // rock can shift 4x and fall 3x without hitting anything (other than the side
                // walls), so the falls can be unconditionally simulated, while a lookup table can
                // be used for the side walls.
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
                state = State::FallJet;
                rock_count += 1;
                if rock_count % 100_000_000 == 0 {
                    println!("count: {}", rock_count);
                }
                continue;
            }
            State::FallJet => {
                if current_rock[0] & chamber.row(rock_bottom - 1) != 0
                    || current_rock[1] & chamber.row(rock_bottom) != 0
                {
                    let current_rock_height = current_rock.iter().filter(|&&x| x != 0).count();

                    // rock_bottom is where rocks spawn, which is one above the actual topmost
                    // rock.
                    let possible_new_top = rock_bottom + current_rock_height - 1;
                    if possible_new_top > topmost_rock {
                        chamber.mark_new_rows_used(possible_new_top - topmost_rock);
                        topmost_rock = possible_new_top;
                    }

                    *chamber.mut_row(rock_bottom) |= current_rock[0];
                    *chamber.mut_row(rock_bottom + 1) |= current_rock[1];
                    *chamber.mut_row(rock_bottom + 2) |= current_rock[2];
                    *chamber.mut_row(rock_bottom + 3) |= current_rock[3];
                    state = State::NewRock;
                    continue;
                }
                rock_bottom -= 1;
                match jets.next().unwrap() {
                    Jet::Left => move_left_if_possible(&chamber, rock_bottom, &mut current_rock),
                    Jet::Right => move_right_if_possible(&chamber, rock_bottom, &mut current_rock),
                };
                continue;
            }
        }
    }
    topmost_rock
}

fn part1(puzzle: &Puzzle) -> usize {
    run_simulation::<2023>(puzzle)
}

fn part2(puzzle: &Puzzle) -> usize {
    return run_simulation::<1_000_000_000>(puzzle);
    run_simulation::<1_000_000_000_000>(puzzle)
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

    const SAMPLE: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn example1() {
        assert_eq!(3068, part1(&parse(SAMPLE).unwrap()));
    }

    #[test]
    fn example2() {
        assert_eq!(2468013579, part2(&parse(SAMPLE).unwrap()));
    }
}
