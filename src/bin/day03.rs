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

use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::io::{self, Read};
use std::str::FromStr;

#[derive(Debug)]
enum Oops {
    Message(String),
    RealError(Box<dyn std::error::Error>),
}

impl Display for Oops {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Oops::Message(s) => write!(f, "oops: {}", s)?,
            Oops::RealError(e) => e.fmt(f)?,
        }
        Ok(())
    }
}

impl<E> From<E> for Oops
where
    E: std::error::Error + 'static,
{
    fn from(error: E) -> Self {
        Oops::RealError(Box::new(error))
    }
}

macro_rules! oops {
    ($($e:expr),*) => {
        Oops::Message(format!($($e,)*))
    };
}

struct Rucksack {
    compartment_one: HashSet<char>,
    compartment_two: HashSet<char>,
    contents: HashSet<char>,
}

impl FromStr for Rucksack {
    type Err = Oops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first_str, second_str) = s.split_at(s.len() / 2);
        let compartment_one = first_str.chars().collect();
        let compartment_two = second_str.chars().collect();
        let contents = s.chars().collect();
        Ok(Rucksack {
            compartment_one,
            compartment_two,
            contents,
        })
    }
}

fn get_priority(c: char) -> Result<u32, Oops> {
    match c {
        'a'..='z' => Ok(c as u32 - 'a' as u32 + 1),
        'A'..='Z' => Ok(c as u32 - 'A' as u32 + 27),
        _ => Err(oops!("invalid item")),
    }
}

fn parse(input: &str) -> Result<Vec<Rucksack>, Oops> {
    input
        .lines()
        .map(|x| x.trim().parse::<Rucksack>())
        .collect()
}

fn part1(rucksacks: &[Rucksack]) -> Result<u32, Oops> {
    let mut item_priorities = 0;
    for rucksack in rucksacks {
        let common_items = rucksack
            .compartment_one
            .intersection(&rucksack.compartment_two);
        for x in common_items {
            item_priorities += get_priority(*x)?;
        }
    }
    Ok(item_priorities)
}

fn part2(rucksacks: &[Rucksack]) -> Result<u32, Oops> {
    let mut badge_priorities = 0;
    let mut chunks = rucksacks.chunks_exact(3);
    for group in &mut chunks {
        let (x, y, z) = (&group[0].contents, &group[1].contents, &group[2].contents);
        let common_items = x.iter().filter(|x| y.contains(x)).filter(|x| z.contains(x));
        for x in common_items {
            badge_priorities += get_priority(*x)?;
        }
    }
    if !chunks.remainder().is_empty() {
        Err(oops!("leftover rucksacks"))
    } else {
        Ok(badge_priorities)
    }
}

fn main() -> Result<(), Oops> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let rucksacks = parse(&input)?;

    println!("{}", part1(&rucksacks)?);
    println!("{}", part2(&rucksacks)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw"#;

    #[test]
    fn example1() {
        assert_eq!(157, part1(&parse(SAMPLE).unwrap()).unwrap());
    }

    #[test]
    fn example2() {
        assert_eq!(70, part2(&parse(SAMPLE).unwrap()).unwrap());
    }
}
