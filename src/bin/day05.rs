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

#[derive(Debug, Clone)]
struct Stack {
    value: Vec<char>,
}

impl Stack {
    fn new() -> Stack {
        Stack { value: Vec::new() }
    }
}

#[derive(Debug)]
struct Move {
    count: u32,
    src: usize,
    dst: usize,
}

impl FromStr for Move {
    type Err = Oops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splitter = s.split_whitespace();
        splitter.next();
        let count = splitter.next().unwrap().parse::<u32>().unwrap();
        splitter.next();
        let src = splitter.next().unwrap().parse::<usize>().unwrap();
        splitter.next();
        let dst = splitter.next().unwrap().parse::<usize>().unwrap();
        Ok(Move { count, src, dst })
    }
}

struct Entity {
    stacks: Vec<Stack>,
    moves: Vec<Move>,
}

fn parse<I>(lines: I) -> Result<Entity, Oops>
where
    I: IntoIterator,
    I::Item: Borrow<str>,
{
    let lines: Vec<String> = lines.into_iter().map(|l| l.borrow().to_string()).collect();
    let mut split_idx = 0;
    for line in &lines {
        split_idx += 1;
        if line.is_empty() {
            break;
        }
    }

    let mut max_col: usize = 0;
    for num in lines[split_idx - 2].trim().split_whitespace() {
        max_col = num.parse::<usize>().unwrap()
    }
    let mut stacks: Vec<Stack> = Vec::new();
    stacks.resize_with(max_col, || Stack::new());
    for line in &lines[..split_idx - 2] {
        let mut idx = 0;
        for c in line.chars() {
            idx += 1;
            if c.is_alphabetic() {
                stacks[idx / 4].value.push(c);
            }
        }
    }
    stacks = stacks
        .iter()
        .map(|s| Stack {
            value: s.value.iter().rev().copied().collect(),
        })
        .collect();

    let mut moves: Vec<Move> = Vec::new();
    for line in &lines[split_idx..] {
        moves.push(line.parse::<Move>()?)
    }

    Ok(Entity { stacks, moves })
}

fn part1(e: &Entity) -> Result<String, Oops> {
    let mut new_stacks = e.stacks.clone();
    for m in &e.moves {
        for _ in 0..m.count {
            let moved = new_stacks[m.src - 1].value.pop().unwrap();
            new_stacks[m.dst - 1].value.push(moved);
        }
    }

    Ok(new_stacks
        .iter()
        .map(|s| *s.value.last().unwrap())
        .collect())
}

fn part2(e: &Entity) -> Result<String, Oops> {
    let mut new_stacks = e.stacks.clone();
    for m in &e.moves {
        let mut moved_crates = Vec::new();
        for _ in 0..m.count {
            moved_crates.push(new_stacks[m.src - 1].value.pop().unwrap())
        }
        for c in moved_crates.iter().rev().copied() {
            new_stacks[m.dst - 1].value.push(c);
        }
    }

    Ok(new_stacks
        .iter()
        .map(|s| *s.value.last().unwrap())
        .collect())
}

fn main() -> Result<(), Oops> {
    let entity = parse(io::stdin().lines().map(|l| l.unwrap()))?;

    println!("{}", part1(&entity)?);
    println!("{}", part2(&entity)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;

    #[test]
    fn example1() {
        assert_eq!("CMZ", part1(&parse(SAMPLE.lines()).unwrap()).unwrap());
    }

    #[test]
    fn example2() {
        assert_eq!("MCD", part2(&parse(SAMPLE.lines()).unwrap()).unwrap());
    }
}
