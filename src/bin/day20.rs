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
    out: &mut Vec<i64>,
    mapper: &mut BTreeMap<usize, usize>,
    reverse_mapper: &mut BTreeMap<usize, usize>,
    idx: usize,
    count: usize,
) {
    let mut this_index = idx;
    let mut next_index = idx + 1;

    for _ in 0..count {
        out.swap(this_index, next_index);

        let m = *reverse_mapper.get(&this_index).unwrap();
        let n = *reverse_mapper.get(&next_index).unwrap();

        // Finally, swap m and n.
        let o = *mapper.get(&m).unwrap();
        let p = *mapper.get(&n).unwrap();

        reverse_mapper.insert(this_index, n);
        reverse_mapper.insert(next_index, m);
        mapper.insert(m, p);
        mapper.insert(n, o);

        this_index += 1;
        next_index += 1;
    }
}

fn shift_backwards(
    out: &mut Vec<i64>,
    mapper: &mut BTreeMap<usize, usize>,
    reverse_mapper: &mut BTreeMap<usize, usize>,
    idx: usize,
    count: usize,
) {
    let mut this_index = idx;
    let mut next_index = idx - 1;

    for _ in 0..count {
        out.swap(this_index, next_index);

        let m = *reverse_mapper.get(&this_index).unwrap();
        let n = *reverse_mapper.get(&next_index).unwrap();

        // Finally, swap m and n.
        let o = *mapper.get(&m).unwrap();
        let p = *mapper.get(&n).unwrap();

        reverse_mapper.insert(this_index, n);
        reverse_mapper.insert(next_index, m);
        mapper.insert(m, p);
        mapper.insert(n, o);

        this_index -= 1;
        next_index -= 1;
    }
}

fn part1(puzzle: &Puzzle) -> i64 {
    let mut output = puzzle.values.iter().copied().collect::<Vec<_>>();
    // Map of original indices to new loc mappers.
    let mut mapper: BTreeMap<usize, usize> = (0..output.len()).zip(0..output.len()).collect();
    // Reverse mappings.
    let mut reverse_mapper: BTreeMap<usize, usize> =
        (0..output.len()).zip(0..output.len()).collect();
    for (i, &n) in puzzle.values.iter().enumerate() {
        let idx = *mapper.get(&i).unwrap();
        let n = n % (output.len() - 1) as i64;
        if n > 0 {
            let n = n as usize;
            if idx + n >= output.len() - 1 {
                let count = output.len() - n - 1;
                shift_backwards(&mut output, &mut mapper, &mut reverse_mapper, idx, count);
            } else {
                shift_forwards(&mut output, &mut mapper, &mut reverse_mapper, idx, n);
            }
        } else if n < 0 {
            let n = n.abs() as usize;
            if idx + output.len() - n <= output.len() {
                let count = output.len() - n - 1;
                shift_forwards(&mut output, &mut mapper, &mut reverse_mapper, idx, count);
            } else {
                shift_backwards(&mut output, &mut mapper, &mut reverse_mapper, idx, n);
            }
        }
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

fn part2(puzzle: &Puzzle) -> i64 {
    let decrypted_values = puzzle
        .values
        .iter()
        .map(|v| v * 811589153)
        .collect::<Vec<_>>();
    let mut output = decrypted_values.clone();
    // Map of original indices to new loc mappers.
    let mut mapper: BTreeMap<usize, usize> = (0..output.len()).zip(0..output.len()).collect();
    // Reverse mappings.
    let mut reverse_mapper: BTreeMap<usize, usize> =
        (0..output.len()).zip(0..output.len()).collect();
    for _ in 0..10 {
        for (i, &n) in decrypted_values.iter().enumerate() {
            let idx = *mapper.get(&i).unwrap();
            let n = n % (output.len() - 1) as i64;
            if n > 0 {
                let n = n as usize;
                if idx + n >= output.len() - 1 {
                    let count = output.len() - n - 1;
                    shift_backwards(&mut output, &mut mapper, &mut reverse_mapper, idx, count);
                } else {
                    shift_forwards(&mut output, &mut mapper, &mut reverse_mapper, idx, n);
                }
            } else if n < 0 {
                let n = n.abs() as usize;
                if idx + output.len() - n <= output.len() {
                    let count = output.len() - n - 1;
                    shift_forwards(&mut output, &mut mapper, &mut reverse_mapper, idx, count);
                } else {
                    shift_backwards(&mut output, &mut mapper, &mut reverse_mapper, idx, n);
                }
            }
        }
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
