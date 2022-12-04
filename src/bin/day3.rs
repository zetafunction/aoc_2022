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

use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
use std::str::FromStr;

struct Rucksack {
    compartment_one: HashMap<char, u32>,
    compartment_two: HashMap<char, u32>,
}

impl FromStr for Rucksack {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first_str, second_str) = s.split_at(s.len() / 2);

        let mut first_compartment = HashMap::new();
        for c in first_str.chars() {
            first_compartment
                .entry(c)
                .and_modify(|x| *x += 1)
                .or_insert(1);
        }

        let mut second_compartment = HashMap::new();
        for c in second_str.chars() {
            second_compartment
                .entry(c)
                .and_modify(|x| *x += 1)
                .or_insert(1);
        }

        Ok(Rucksack {
            compartment_one: first_compartment,
            compartment_two: second_compartment,
        })
    }
}

fn get_priority(c: char) -> Option<u32> {
    match c {
        'a'..='z' => Some(c as u32 - 'a' as u32 + 1),
        'A'..='Z' => Some(c as u32 - 'A' as u32 + 27),
        _ => None,
    }
}

fn main() {
    let rucksacks: Vec<_> = io::stdin()
        .lines()
        .map(|l| l.unwrap().parse::<Rucksack>().unwrap())
        .collect();

    let mut common_priority_sum = 0;

    for rucksack in &rucksacks {
        let compartment_one_items = HashSet::<_>::from_iter(rucksack.compartment_one.keys());
        let compartment_two_items = HashSet::<_>::from_iter(rucksack.compartment_two.keys());
        for x in compartment_one_items.intersection(&compartment_two_items) {
            common_priority_sum += get_priority(**x).unwrap();
        }
    }
    println!("{}", common_priority_sum);
}
