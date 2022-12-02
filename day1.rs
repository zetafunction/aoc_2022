use std::{collections::BTreeSet, io};

// Would be nice if rustfmt sorted this...
#[derive(Ord, Eq, PartialOrd, PartialEq)]
struct Elf {
    total_rations: u32,
}

impl Elf {
    fn new(rations: Vec<u32>) -> Elf {
        Elf {
            total_rations: rations.iter().sum(),
        }
    }
}

fn main() {
    let mut elves = BTreeSet::new();
    let mut rations = Vec::new();
    for line in io::stdin().lines() {
        let line = line.unwrap();
        if line.is_empty() {
            elves.insert(Elf::new(rations));
            rations = Vec::new();
            continue;
        }
        rations.push(line.parse::<u32>().unwrap());
    }
    if !rations.is_empty() {
        elves.insert(Elf::new(rations));
    }
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
