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
use std::collections::hash_map::{DefaultHasher, HashMap};
use std::io::{self, Read};
use std::str::FromStr;

type Row = u16;

#[derive(Clone, Copy)]
enum Jet {
    Left = 0,
    Right = 1,
}

struct Puzzle {
    jets: Vec<Jet>,
}

#[allow(clippy::eq_op, clippy::identity_op)]
const ROCKS: [u64; 5] = [
    (0b0000_0000_0000_0000 << 48)
        | (0b0000_0000_0000_0000 << 32)
        | (0b0000_0000_0000_0000 << 16)
        | (0b0001_1110_0000_0000),
    (0b0000_0000_0000_0000 << 48)
        | (0b0000_1000_0000_0000 << 32)
        | (0b0001_1100_0000_0000 << 16)
        | (0b0000_1000_0000_0000),
    (0b0000_0000_0000_0000 << 48)
        | (0b0000_0100_0000_0000 << 32)
        | (0b0000_0100_0000_0000 << 16)
        | (0b0001_1100_0000_0000),
    (0b0001_0000_0000_0000 << 48)
        | (0b0001_0000_0000_0000 << 32)
        | (0b0001_0000_0000_0000 << 16)
        | (0b0001_0000_0000_0000),
    (0b0000_0000_0000_0000 << 48)
        | (0b0000_0000_0000_0000 << 32)
        | (0b0001_1000_0000_0000 << 16)
        | (0b0001_1000_0000_0000),
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

// Empirically, smaller sizes seem to produce less throughput.
const GRID_ROWS: usize = 1024;

struct State {
    cycle: usize,
    height: usize,
}

struct Chamber {
    base: usize,
    used: usize,
    data: [Row; GRID_ROWS],
    max_height: usize,
    cycles_elapsed: usize,
    seen: HashMap<u64, Vec<State>>,
}

impl Chamber {
    fn new() -> Self {
        let mut chamber = Chamber {
            base: 0,
            used: 10,
            data: [0; GRID_ROWS],
            max_height: 0,
            cycles_elapsed: 0,
            seen: HashMap::new(),
        };
        chamber.data[0] = 0xffff;
        for i in 1..GRID_ROWS {
            chamber.data[i] = 0x80ff;
        }
        chamber
    }

    fn mark_new_rows_used(&mut self, height: usize) {
        if height > self.max_height {
            let delta = height - self.max_height;
            self.max_height = height;

            self.used += delta;

            if self.used > GRID_ROWS - 10 {
                let capacity_to_free = GRID_ROWS / 2;
                let old_base = self.base;
                let new_base = self.base + capacity_to_free;
                for i in old_base..new_base {
                    self.data[i % GRID_ROWS] = 0x80ff;
                }
                self.base = new_base % GRID_ROWS;
                self.used -= capacity_to_free;
            }
        }
    }

    fn row(&self, n: usize) -> Row {
        self.data[n % GRID_ROWS]
    }

    fn row_mut(&mut self, n: usize) -> &mut Row {
        &mut self.data[n % GRID_ROWS]
    }

    #[allow(dead_code)]
    fn render(&self) {
        println!("drawing from {} to {}", self.base, self.base + self.used);
        for i in (self.base..=self.base + self.used).rev() {
            let data = self.data[i % GRID_ROWS];
            println!(
                "{}",
                (7..16)
                    .rev()
                    .map(|pos| if data & (1 << pos) == 0 { '.' } else { '#' })
                    .collect::<String>()
            );
        }
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
    let walls_bitmask: u64 = 0x80ff_80ff_80ff_80ff;
    for i in 0..ROCKS.len() {
        for j1 in &[Jet::Left, Jet::Right] {
            for j2 in &[Jet::Left, Jet::Right] {
                for j3 in &[Jet::Left, Jet::Right] {
                    for j4 in &[Jet::Left, Jet::Right] {
                        let mut rock = ROCKS[i];
                        // For some inexplicable reason, iterating over &[] instead of [] affects overall
                        // throughput...
                        for j in &[j1, j2, j3, j4] {
                            let next_rock = match j {
                                Jet::Left => rock << 1,
                                Jet::Right => rock >> 1,
                            };
                            if next_rock & walls_bitmask == 0 {
                                rock = next_rock;
                            }
                        }
                        table[rock_and_jets_to_index(i, *j1, *j2, *j3, *j4)] = rock;
                    }
                }
            }
        }
    }
    table
}

fn run_simulation<const MAX_ROCK_COUNT: usize>(puzzle: &Puzzle) -> usize {
    enum State {
        NewRock,
        FallJet,
    }

    let mut rock_heights = ROCK_HEIGHTS.iter().cycle();
    let mut jets = puzzle.jets.iter().cycle();

    let mut state = State::NewRock;
    let mut chamber = Chamber::new();

    let rock_table = build_new_rock_lookup_table();
    let mut rock_count = 0;
    let mut current_rock = ROCKS[0];
    let mut current_rock_height = 0;
    // Represents the bottom of the current rock.
    let mut rock_bottom = 1;
    let mut chamber_rows: u64 = 0;
    while rock_count < MAX_ROCK_COUNT {
        match state {
            State::NewRock => {
                current_rock = unsafe {
                    // jets is a cycled iterator that will never return None.
                    let j1 = jets.next().unwrap_unchecked();
                    let j2 = jets.next().unwrap_unchecked();
                    let j3 = jets.next().unwrap_unchecked();
                    let j4 = jets.next().unwrap_unchecked();
                    // TODO: better scope unsafeness for rock lookup table.
                    *rock_table.get_unchecked(rock_and_jets_to_index(
                        rock_count % ROCKS.len(),
                        *j1,
                        *j2,
                        *j3,
                        *j4,
                    ))
                };
                // rock_heights is a cycled iterator that will never return None.
                current_rock_height = unsafe { *rock_heights.next().unwrap_unchecked() };
                rock_bottom = chamber.max_height + 1;
                // Normally, rocks start at chamber.max_height + 4. However, it is guaranteed that
                // each rock can shift 4x and fall 3x without hitting anything (other than the side
                // walls), so the falls can be unconditionally simulated, while a lookup table can
                // be used for the side walls.
                state = State::FallJet;
                rock_count += 1;
                if rock_count % 100_000_000 == 0 {
                    // println!("count: {}", rock_count);
                }
                if rock_count < 15 {
                    // chamber.render();
                }
                // The new rock always spawns immediately above any non-wall collisions, so the
                // only bits that are set will be for walls.
                chamber_rows = 0x80ff_80ff_80ff_80ff;
                continue;
            }
            State::FallJet => {
                chamber_rows = (chamber_rows << 16) | u64::from(chamber.row(rock_bottom - 1));
                if (current_rock & chamber_rows) != 0 {
                    // rock_bottom is where rocks spawn, which is one above the actual topmost
                    // rock.
                    let possible_new_top = rock_bottom + current_rock_height - 1;
                    chamber.mark_new_rows_used(possible_new_top);

                    *chamber.row_mut(rock_bottom) |= (current_rock & 0xffff) as Row;
                    *chamber.row_mut(rock_bottom + 1) |= (current_rock >> 16 & 0xffff) as Row;
                    *chamber.row_mut(rock_bottom + 2) |= (current_rock >> 32 & 0xffff) as Row;
                    *chamber.row_mut(rock_bottom + 3) |= (current_rock >> 48 & 0xffff) as Row;
                    state = State::NewRock;
                    continue;
                }
                rock_bottom -= 1;
                // jets is a cycled iterator that will never return None.
                let shifted_rock = match unsafe { jets.next().unwrap_unchecked() } {
                    Jet::Left => current_rock << 1,
                    Jet::Right => current_rock >> 1,
                };
                if (shifted_rock & chamber_rows) == 0 {
                    current_rock = shifted_rock;
                }
                continue;
            }
        }
    }
    chamber.max_height
}

fn part1(puzzle: &Puzzle) -> usize {
    run_simulation::<2023>(puzzle)
}

fn part2(puzzle: &Puzzle) -> usize {
    return run_simulation::<1_000_000_000>(puzzle);
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
