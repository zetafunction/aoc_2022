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
use std::collections::BTreeMap;
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
    buffer: &mut Vec<i64>,
    forward_map: &mut BTreeMap<usize, usize>,
    reverse_map: &mut BTreeMap<usize, usize>,
    base: usize,
    count: usize,
) {
    for i in 0..count {
        let this_index = base + i;
        let next_index = base + i + 1;

        buffer.swap(this_index, next_index);

        let m = *reverse_map.get(&this_index).unwrap();
        let n = *reverse_map.get(&next_index).unwrap();

        let o = *forward_map.get(&m).unwrap();
        let p = *forward_map.get(&n).unwrap();

        reverse_map.insert(this_index, n);
        reverse_map.insert(next_index, m);
        forward_map.insert(m, p);
        forward_map.insert(n, o);
    }
}

fn shift_backwards(
    buffer: &mut Vec<i64>,
    forward_map: &mut BTreeMap<usize, usize>,
    reverse_map: &mut BTreeMap<usize, usize>,
    base: usize,
    count: usize,
) {
    for i in 0..count {
        let this_index = base - i;
        let next_index = base - i - 1;

        buffer.swap(this_index, next_index);

        let m = *reverse_map.get(&this_index).unwrap();
        let n = *reverse_map.get(&next_index).unwrap();

        let o = *forward_map.get(&m).unwrap();
        let p = *forward_map.get(&n).unwrap();

        reverse_map.insert(this_index, n);
        reverse_map.insert(next_index, m);
        forward_map.insert(m, p);
        forward_map.insert(n, o);
    }
}

fn mix(
    original_values: &Vec<i64>,
    mixed_values: &mut Vec<i64>,
    forward_map: &mut BTreeMap<usize, usize>,
    reverse_map: &mut BTreeMap<usize, usize>,
) {
    for (original_idx, &shift) in original_values.iter().enumerate() {
        let current_idx = *forward_map.get(&original_idx).unwrap();
        // Mixing an element forwards or backwards by size of buffer - 1 leaves the element in the
        // same position it began. To avoid pointlessly shuffling the element around, just process
        // the tail end.
        let shift = shift % (mixed_values.len() - 1) as i64;
        if shift > 0 {
            let shift = shift as usize;
            if current_idx + shift >= mixed_values.len() - 1 {
                let shift = mixed_values.len() - shift - 1;
                shift_backwards(mixed_values, forward_map, reverse_map, current_idx, shift);
            } else {
                shift_forwards(mixed_values, forward_map, reverse_map, current_idx, shift);
            }
        } else if shift < 0 {
            let shift = shift.abs() as usize;
            if current_idx + mixed_values.len() - shift <= mixed_values.len() {
                let shift = mixed_values.len() - shift - 1;
                shift_forwards(mixed_values, forward_map, reverse_map, current_idx, shift);
            } else {
                shift_backwards(mixed_values, forward_map, reverse_map, current_idx, shift);
            }
        }
    }
}

fn part1(puzzle: &Puzzle) -> i64 {
    let mut output = puzzle.values.clone();
    let mut forward_map: BTreeMap<usize, usize> = (0..output.len()).zip(0..output.len()).collect();
    let mut reverse_map: BTreeMap<usize, usize> = forward_map.clone();
    mix(
        &puzzle.values,
        &mut output,
        &mut forward_map,
        &mut reverse_map,
    );
    let mut zero = None;
    for (i, v) in output.iter().enumerate() {
        if *v == 0 {
            zero = Some(i);
            break;
        }
    }
    let zero = zero.unwrap();
    output[(zero + 1000) % output.len()]
        + output[(zero + 2000) % output.len()]
        + output[(zero + 3000) % output.len()]
}

fn part2(puzzle: &Puzzle) -> i64 {
    let decrypted_values = puzzle
        .values
        .iter()
        .map(|v| v * 811589153)
        .collect::<Vec<_>>();
    let mut output = decrypted_values.clone();
    let mut forward_map: BTreeMap<usize, usize> = (0..output.len()).zip(0..output.len()).collect();
    let mut reverse_map: BTreeMap<usize, usize> = forward_map.clone();
    for _ in 0..10 {
        mix(
            &decrypted_values,
            &mut output,
            &mut forward_map,
            &mut reverse_map,
        );
    }
    let mut zero = None;
    for (i, v) in output.iter().enumerate() {
        if *v == 0 {
            zero = Some(i);
            break;
        }
    }
    let zero = zero.unwrap();
    output[(zero + 1000) % output.len()]
        + output[(zero + 2000) % output.len()]
        + output[(zero + 3000) % output.len()]
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

    const SAMPLE: &str = concat!("1\n", "2\n", "-3\n", "3\n", "-2\n", "0\n", "4\n",);

    #[test]
    fn example1() {
        assert_eq!(3, part1(&parse(SAMPLE).unwrap()));
    }

    #[test]
    fn example2() {
        assert_eq!(1623178306, part2(&parse(SAMPLE).unwrap()));
    }
}
