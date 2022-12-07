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

struct Entity {
    values: Vec<usize>,
}

impl FromStr for Entity {
    type Err = Oops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Entity {
            values: s
                .lines()
                .map(|s| s.parse::<usize>())
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

fn parse(input: &str) -> Result<Entity, Oops> {
    input.parse()
}

fn part1(entity: &Entity) -> Result<usize, Oops> {
    Ok(entity.values.iter().sum())
}

fn part2(entity: &Entity) -> Result<usize, Oops> {
    entity
        .values
        .iter()
        .max()
        .ok_or_else(|| oops!("no entities"))
        .copied()
}

fn main() -> Result<(), Oops> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let input = input;

    let entity = parse(&input)?;

    println!("{}", part1(&entity)?);
    println!("{}", part2(&entity)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = concat!(
        // First line!
        "123456789\n",
        // Second line!
        "987654321\n",
        // Third line!
        "2468013579\n"
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
