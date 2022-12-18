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

type Row = u32;

#[derive(Clone, Copy)]
enum Jet {
    Left = 0,
    Right = 1,
}

struct Puzzle {
    jets: Vec<Jet>,
}

const ROCKS: [u64; 5] = [
    (0b0000000000000000 << 48)
        | (0b0000000000000000 << 32)
        | (0b0000000000000000 << 16)
        | (0b0001111000000000 << 0),
    (0b0000000000000000 << 48)
        | (0b0000100000000000 << 32)
        | (0b0001110000000000 << 16)
        | (0b0000100000000000 << 0),
    (0b0000000000000000 << 48)
        | (0b0000010000000000 << 32)
        | (0b0000010000000000 << 16)
        | (0b0001110000000000 << 0),
    (0b0001000000000000 << 48)
        | (0b0001000000000000 << 32)
        | (0b0001000000000000 << 16)
        | (0b0001000000000000 << 0),
    (0b0000000000000000 << 48)
        | (0b0000000000000000 << 32)
        | (0b0001100000000000 << 16)
        | (0b0001100000000000 << 0),
];

const ROCK_HEIGHTS: [usize; ROCKS.len()] = [1, 3, 3, 4, 2];

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
        grid.data[0] = 0x0000ffff;
        for i in 1..GRID_ROWS {
            grid.data[i] = 0x000080ff;
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
                self.data[i % GRID_ROWS] = 0x000080ff;
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
                (7..16)
                    .rev()
                    .map(|pos| if data & (1 << pos) != 0 { '#' } else { '.' })
                    .collect::<String>()
            );
        }
    }
}

fn move_left_if_possible(chamber: &BitGrid, rock_bottom: usize, current_rock: u64) -> u64 {
    let chamber_rows = ((chamber.row(rock_bottom) as u64) << 0)
        | ((chamber.row(rock_bottom + 1) as u64) << 16)
        | ((chamber.row(rock_bottom + 2) as u64) << 32)
        | ((chamber.row(rock_bottom + 3) as u64) << 48);
    if (current_rock << 1) & chamber_rows == 0 {
        current_rock << 1
    } else {
        current_rock
    }
}

fn move_right_if_possible(chamber: &BitGrid, rock_bottom: usize, current_rock: u64) -> u64 {
    let chamber_rows = ((chamber.row(rock_bottom) as u64) << 0)
        | ((chamber.row(rock_bottom + 1) as u64) << 16)
        | ((chamber.row(rock_bottom + 2) as u64) << 32)
        | ((chamber.row(rock_bottom + 3) as u64) << 48);
    if (current_rock >> 1) & chamber_rows == 0 {
        current_rock >> 1
    } else {
        current_rock
    }
}

fn rock_and_jets_to_index(rock_count: usize, jet1: Jet, jet2: Jet, jet3: Jet, jet4: Jet) -> usize {
    (rock_count << 4)
        | ((jet1 as usize) << 3)
        | ((jet2 as usize) << 2)
        | ((jet3 as usize) << 1)
        | (jet4 as usize)
}

// Entries are indexed by `rock_and_jets_to_index()`.
fn build_new_rock_lookup_table() -> Vec<u64> {
    let mut table = vec![0; ROCKS.len() * 16];
    let grid = BitGrid::new();
    for i in 0..ROCKS.len() {
        for j1 in &[Jet::Left, Jet::Right] {
            for j2 in &[Jet::Left, Jet::Right] {
                for j3 in &[Jet::Left, Jet::Right] {
                    for j4 in &[Jet::Left, Jet::Right] {
                        let rock = ROCKS[i];
                        let rock = match j1 {
                            Jet::Left => move_left_if_possible(&grid, 8, rock),
                            Jet::Right => move_right_if_possible(&grid, 8, rock),
                        };
                        let rock = match j2 {
                            Jet::Left => move_left_if_possible(&grid, 8, rock),
                            Jet::Right => move_right_if_possible(&grid, 8, rock),
                        };
                        let rock = match j3 {
                            Jet::Left => move_left_if_possible(&grid, 8, rock),
                            Jet::Right => move_right_if_possible(&grid, 8, rock),
                        };
                        let rock = match j4 {
                            Jet::Left => move_left_if_possible(&grid, 8, rock),
                            Jet::Right => move_right_if_possible(&grid, 8, rock),
                        };
                        table[rock_and_jets_to_index(i, *j1, *j2, *j3, *j4)] = rock;
                    }
                }
            }
        }
    }
    table
}

fn run_simulation<const MAX_ROCK_COUNT: usize>(puzzle: &Puzzle) -> usize {
    let mut rock_heights = ROCK_HEIGHTS.iter().cycle();
    let mut jets = puzzle.jets.iter().cycle();

    let mut state = State::NewRock;
    let mut chamber = BitGrid::new();

    let rock_table = build_new_rock_lookup_table();
    let mut rock_count = 0;
    let mut current_rock = ROCKS[0];
    let mut current_rock_height = 0;
    // Represents the bottom of the current rock.
    let mut topmost_rock = 0;
    let mut rock_bottom = 1;
    while rock_count < MAX_ROCK_COUNT {
        match state {
            State::NewRock => {
                unsafe {
                    let j1 = jets.next().unwrap_unchecked();
                    let j2 = jets.next().unwrap_unchecked();
                    let j3 = jets.next().unwrap_unchecked();
                    let j4 = jets.next().unwrap_unchecked();
                    current_rock = *rock_table.get_unchecked(rock_and_jets_to_index(
                        rock_count % ROCKS.len(),
                        *j1,
                        *j2,
                        *j3,
                        *j4,
                    ));
                }
                current_rock_height = unsafe { *rock_heights.next().unwrap_unchecked() };
                rock_bottom = topmost_rock + 1;
                // Normally, rocks start at topmost_rock + 4. However, it is guaranteed that each
                // rock can shift 4x and fall 3x without hitting anything (other than the side
                // walls), so the falls can be unconditionally simulated, while a lookup table can
                // be used for the side walls.
                state = State::FallJet;
                rock_count += 1;
                if rock_count % 100_000_000 == 0 {
                    println!("count: {}", rock_count);
                }
                if rock_count < 15 {
                    // chamber.render();
                }
                continue;
            }
            State::FallJet => {
                if ((current_rock & 0xffff) as Row) & chamber.row(rock_bottom - 1) != 0
                    || ((current_rock >> 16 & 0xffff) as Row) & chamber.row(rock_bottom) != 0
                {
                    // rock_bottom is where rocks spawn, which is one above the actual topmost
                    // rock.
                    let possible_new_top = rock_bottom + current_rock_height - 1;
                    if possible_new_top > topmost_rock {
                        chamber.mark_new_rows_used(possible_new_top - topmost_rock);
                        topmost_rock = possible_new_top;
                    }

                    *chamber.mut_row(rock_bottom) |= (current_rock & 0xffff) as Row;
                    *chamber.mut_row(rock_bottom + 1) |= (current_rock >> 16 & 0xffff) as Row;
                    *chamber.mut_row(rock_bottom + 2) |= (current_rock >> 32 & 0xffff) as Row;
                    *chamber.mut_row(rock_bottom + 3) |= (current_rock >> 48 & 0xffff) as Row;
                    state = State::NewRock;
                    continue;
                }
                rock_bottom -= 1;
                current_rock = match unsafe { jets.next().unwrap_unchecked() } {
                    Jet::Left => move_left_if_possible(&chamber, rock_bottom, current_rock),
                    Jet::Right => move_right_if_possible(&chamber, rock_bottom, current_rock),
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
    // return run_simulation::<1_000_000_000>(puzzle);
    run_simulation::<1_000_000_000_001>(puzzle)
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
        assert_eq!(1514285714288, part2(&parse(SAMPLE).unwrap()));
    }
}
