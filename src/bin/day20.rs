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
use std::str::FromStr;

struct Puzzle {
    values: Vec<i64>,
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

fn shift_forwards(
    buffer: &mut [i64],
    forward_map: &mut [usize],
    reverse_map: &mut [usize],
    base: usize,
    count: usize,
) {
    for i in 0..count {
        let this_index = base + i;
        let next_index = base + i + 1;

        buffer.swap(this_index, next_index);

        forward_map.swap(reverse_map[this_index], reverse_map[next_index]);
        reverse_map.swap(this_index, next_index);
    }
}

fn shift_backwards(
    buffer: &mut [i64],
    forward_map: &mut [usize],
    reverse_map: &mut [usize],
    base: usize,
    count: usize,
) {
    for i in 0..count {
        let this_index = base - i;
        let next_index = base - i - 1;

        buffer.swap(this_index, next_index);

        forward_map.swap(reverse_map[this_index], reverse_map[next_index]);
        reverse_map.swap(this_index, next_index);
    }
}

fn mix(
    original_values: &[i64],
    mixed_values: &mut [i64],
    forward_map: &mut [usize],
    reverse_map: &mut [usize],
) {
    // When moving an element forwards or backwards a position, there are only `mixed_values.len() - 1`
    // other elements in the list, and thus, only `mixed_values.len() - 1` positions to arrange it
    // in front or behind of.
    let positions = mixed_values.len() - 1;
    for (original_idx, &shift) in original_values.iter().enumerate() {
        let current_idx = forward_map[original_idx];
        // Mixing an element forwards or backwards by `positions` leaves the list of values
        // unchanged. Avoid pointless shuffling and just process the leftover amount.
        let shift = shift % positions as i64;
        match shift.cmp(&0) {
            Ordering::Greater => {
                let shift = shift as usize;
                if current_idx + shift >= positions {
                    // The remaining amount will push the mixed element off the end of the list. Just
                    // shift it backwards instead.
                    let shift = positions - shift;
                    shift_backwards(mixed_values, forward_map, reverse_map, current_idx, shift);
                } else {
                    shift_forwards(mixed_values, forward_map, reverse_map, current_idx, shift);
                }
            }
            Ordering::Less => {
                let shift = shift.abs() as usize;
                // Slightly trickier arithmetic here since these are unsigned types. `shift <
                // positions` must be true given the modulo operation above, but `shift` may still be
                // greater than `current_idx`. To avoid it going negative, add `position` to both sides
                // before performing the comparison.
                if current_idx + positions - shift <= positions {
                    // The remaining amount will push the mixed element off the front of the list. Just
                    // shift it forwards instead.
                    let shift = positions - shift;
                    shift_forwards(mixed_values, forward_map, reverse_map, current_idx, shift);
                } else {
                    shift_backwards(mixed_values, forward_map, reverse_map, current_idx, shift);
                }
            }
            Ordering::Equal => (),
        }
    }
}

fn part1(puzzle: &Puzzle) -> Result<i64, Oops> {
    let mut output = puzzle.values.clone();
    let mut forward_map: Vec<_> = (0..output.len()).collect();
    let mut reverse_map = forward_map.clone();
    mix(
        &puzzle.values,
        &mut output,
        &mut forward_map,
        &mut reverse_map,
    );
    let zero = output
        .iter()
        .enumerate()
        .find_map(|(i, x)| if *x == 0 { Some(i) } else { None })
        .ok_or_else(|| oops!("no zero element"))?;
    Ok(output[(zero + 1000) % output.len()]
        + output[(zero + 2000) % output.len()]
        + output[(zero + 3000) % output.len()])
}

fn part2(puzzle: &Puzzle) -> Result<i64, Oops> {
    let decrypted_values = puzzle
        .values
        .iter()
        // Magic decryption key.
        .map(|v| v * 811_589_153)
        .collect::<Vec<_>>();
    let mut output = decrypted_values.clone();
    let mut forward_map: Vec<_> = (0..output.len()).collect();
    let mut reverse_map = forward_map.clone();
    for _ in 0..10 {
        mix(
            &decrypted_values,
            &mut output,
            &mut forward_map,
            &mut reverse_map,
        );
    }
    let zero = output
        .iter()
        .enumerate()
        .find_map(|(i, x)| if *x == 0 { Some(i) } else { None })
        .ok_or_else(|| oops!("no zero element"))?;
    Ok(output[(zero + 1000) % output.len()]
        + output[(zero + 2000) % output.len()]
        + output[(zero + 3000) % output.len()])
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

    const SAMPLE: &str = concat!("1\n", "2\n", "-3\n", "3\n", "-2\n", "0\n", "4\n",);

    #[test]
    fn example1() {
        assert_eq!(3, part1(&parse(SAMPLE).unwrap()).unwrap());
    }

    #[test]
    fn example2() {
        assert_eq!(1623178306, part2(&parse(SAMPLE).unwrap()).unwrap());
    }
}
