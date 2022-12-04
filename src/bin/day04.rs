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

struct Range {
    low: u32,
    high: u32,
}

impl Range {
    fn contains(&self, other: &Range) -> bool {
        self.low <= other.low && self.high >= other.high
    }

    fn overlaps(&self, other: &Range) -> bool {
        (self.low >= other.low && self.low <= other.high)
            || (self.high >= other.low && self.high <= other.high)
    }
}

impl FromStr for Range {
    type Err = Oops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split("-");
        Ok(Range {
            low: parts
                .next()
                .ok_or_else(|| aoc_2022::oops!("no part"))?
                .parse::<u32>()?,
            high: parts
                .next()
                .ok_or_else(|| aoc_2022::oops!("no part"))?
                .parse::<u32>()?,
        })
    }
}

struct Entity {
    first: Range,
    second: Range,
}

impl FromStr for Entity {
    type Err = Oops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(",");
        Ok(Entity {
            first: parts
                .next()
                .ok_or_else(|| aoc_2022::oops!("no part"))?
                .parse::<Range>()?,
            second: parts
                .next()
                .ok_or_else(|| aoc_2022::oops!("no part"))?
                .parse::<Range>()?,
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
    Ok(entities
        .iter()
        .filter(|e| e.first.contains(&e.second) || e.second.contains(&e.first))
        .count() as u32)
}

fn part2(entities: &[Entity]) -> Result<u32, Oops> {
    Ok(entities
        .iter()
        .filter(|e| e.first.overlaps(&e.second) || e.second.overlaps(&e.first))
        .count() as u32)
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

    const SAMPLE: &str = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#;

    #[test]
    fn example1() {
        assert_eq!(2, part1(&parse(SAMPLE.lines()).unwrap()).unwrap());
    }

    #[test]
    fn example2() {
        assert_eq!(4, part2(&parse(SAMPLE.lines()).unwrap()).unwrap());
    }
}
