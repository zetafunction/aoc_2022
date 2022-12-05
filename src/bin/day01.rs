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

use aoc_2022::itertools::IterTools;
use std::{collections::BTreeSet, io};

// Would be nice if rustfmt sorted this...
#[derive(Ord, Eq, PartialOrd, PartialEq)]
struct Elf {
    total_rations: u32,
}

impl Elf {
    fn new(total_rations: u32) -> Elf {
        Elf { total_rations }
    }
}

fn main() {
    let elves: BTreeSet<Elf> = io::stdin()
        .lines()
        .segment(|line| line.as_ref().unwrap().is_empty())
        .map(|group| {
            Elf::new(
                group
                    .iter()
                    .map(|line| line.as_ref().unwrap().parse::<u32>().unwrap())
                    .sum(),
            )
        })
        .collect();
    println!("{}", elves.iter().last().unwrap().total_rations);
    println!(
        "{}",
        elves
            .iter()
            .rev()
            .take(3)
            .map(|elf| elf.total_rations)
            .sum::<u32>()
    );
}
