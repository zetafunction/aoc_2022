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
use std::io;
use std::str::FromStr;

type Error = &'static str;

struct Rucksack {
    compartment_one: HashSet<char>,
    compartment_two: HashSet<char>,
    contents: HashSet<char>,
}

impl FromStr for Rucksack {
    type Err = Error;

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

fn get_priority(c: char) -> Result<u32, Error> {
    match c {
        'a'..='z' => Ok(c as u32 - 'a' as u32 + 1),
        'A'..='Z' => Ok(c as u32 - 'A' as u32 + 27),
        _ => Err("invalid item"),
    }
}

fn main() -> Result<(), Error> {
    let rucksacks = io::stdin()
        .lines()
        .map(|x| x.unwrap().parse::<Rucksack>())
        .collect::<Result<Vec<_>, _>>()?;

    let mut item_priorities = 0;
    for rucksack in &rucksacks {
        for x in rucksack
            .compartment_one
            .intersection(&rucksack.compartment_two)
        {
            item_priorities += get_priority(*x)?;
        }
    }

    let mut badge_priorities = 0;
    for i in 0..rucksacks.len() / 3 {
        let common_items = rucksacks[3 * i]
            .contents
            .iter()
            .filter(|x| rucksacks[3 * i + 1].contents.contains(x))
            .filter(|x| rucksacks[3 * i + 2].contents.contains(x));
        for x in common_items {
            badge_priorities += get_priority(*x)?;
        }
    }

    println!("{}", item_priorities);
    println!("{}", badge_priorities);

    Ok(())
}
