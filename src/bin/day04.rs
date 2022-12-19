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
        !(self.high < other.low || self.low > other.high)
    }
}

impl FromStr for Range {
    type Err = Oops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('-');
        if let (Some(first), Some(second)) = (parts.next(), parts.next()) {
            Ok(Range {
                low: first.parse::<u32>()?,
                high: second.parse::<u32>()?,
            })
        } else {
            Err(aoc_2022::oops!("not enough parts for Range"))
        }
    }
}

struct Entity {
    first: Range,
    second: Range,
}

impl FromStr for Entity {
    type Err = Oops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');
        if let (Some(first), Some(second)) = (parts.next(), parts.next()) {
            Ok(Entity {
                first: first.parse::<Range>()?,
                second: second.parse::<Range>()?,
            })
        } else {
            Err(aoc_2022::oops!("not enough parts for assignment pairs"))
        }
    }
}

fn parse<I>(lines: I) -> Result<Vec<Entity>, Oops>
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    lines
        .into_iter()
        .map(|x| x.as_ref().trim().parse())
        .collect()
}

fn part1(entities: &[Entity]) -> usize {
    entities
        .iter()
        .filter(|e| e.first.contains(&e.second) || e.second.contains(&e.first))
        .count()
}

fn part2(entities: &[Entity]) -> usize {
    entities
        .iter()
        .filter(|e| e.first.overlaps(&e.second))
        .count()
}

fn main() -> Result<(), Oops> {
    let entities = parse(io::stdin().lines().map(Result::unwrap))?;

    println!("{}", part1(&entities));
    println!("{}", part2(&entities));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = concat!(
        "2-4,6-8\n",
        "2-3,4-5\n",
        "5-7,7-9\n",
        "2-8,3-7\n",
        "6-6,4-6\n",
        "2-6,4-8\n"
    );

    #[test]
    fn example1() {
        assert_eq!(2, part1(&parse(SAMPLE.lines()).unwrap()));
    }

    #[test]
    fn example2() {
        assert_eq!(4, part2(&parse(SAMPLE.lines()).unwrap()));
    }
}
