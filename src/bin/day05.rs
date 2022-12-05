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

use aoc_2022::oops::Oops;
use std::borrow::Borrow;
use std::io;
use std::str::FromStr;

struct Entity {
    value: u32,
}

impl FromStr for Entity {
    type Err = Oops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Entity {
            value: s.parse::<u32>()?,
        })
    }
}

fn parse<I>(lines: I) -> Result<Vec<Entity>, Oops>
where
    I: IntoIterator,
    I::Item: Borrow<str>,
{
    lines
        .into_iter()
        .map(|x| x.borrow().trim().parse())
        .collect()
}

fn part1(entities: &[Entity]) -> Result<u32, Oops> {
    Ok(entities.iter().map(|x| x.value).sum())
}

fn part2(entities: &[Entity]) -> Result<u32, Oops> {
    entities
        .iter()
        .map(|x| x.value)
        .max()
        .ok_or_else(|| aoc_2022::oops!("no entities"))
}

fn main() -> Result<(), Oops> {
    let entities = parse(io::stdin().lines().map(|l| l.unwrap()))?;

    println!("{}", part1(&entities)?);
    println!("{}", part2(&entities)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"1
    2
    3"#;

    #[test]
    fn example1() {
        assert_eq!(6, part1(&parse(SAMPLE.lines()).unwrap()).unwrap());
    }

    #[test]
    fn example2() {
        assert_eq!(3, part2(&parse(SAMPLE.lines()).unwrap()).unwrap());
    }
}
