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
use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::io;
use std::str::FromStr;

#[derive(Debug)]
struct Oops {
    message: String,
}

impl Oops {
    fn new(message: &str) -> Box<Oops> {
        Box::new(Oops {
            message: message.to_string(),
        })
    }
}

impl Display for Oops {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.message)?;
        Ok(())
    }
}

impl Error for Oops {}

struct Rucksack {
    compartment_one: HashSet<char>,
    compartment_two: HashSet<char>,
    contents: HashSet<char>,
}

impl FromStr for Rucksack {
    type Err = Box<dyn Error>;

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

fn get_priority(c: char) -> Result<u32, Box<dyn Error>> {
    match c {
        'a'..='z' => Ok(c as u32 - 'a' as u32 + 1),
        'A'..='Z' => Ok(c as u32 - 'A' as u32 + 27),
        _ => Err(Oops::new("invalid item")),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let rucksacks = io::stdin()
        .lines()
        .map(|x| x?.parse::<Rucksack>())
        .collect::<Result<Vec<_>, _>>()?;

    let mut item_priorities = 0;
    for rucksack in &rucksacks {
        let common_items = rucksack
            .compartment_one
            .intersection(&rucksack.compartment_two);
        for x in common_items {
            item_priorities += get_priority(*x)?;
        }
    }

    let mut badge_priorities = 0;
    let chunks = rucksacks.chunks_exact(3);
    for group in chunks {
        let (x, y, z) = (&group[0].contents, &group[1].contents, &group[2].contents);
        let common_items = x.iter().filter(|x| y.contains(x)).filter(|x| z.contains(x));
        for x in common_items {
            badge_priorities += get_priority(*x)?;
        }
    }

    println!("{}", item_priorities);
    println!("{}", badge_priorities);

    Ok(())
}
