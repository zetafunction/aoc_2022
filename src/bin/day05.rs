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
    crates: Vec<char>,
}

impl Stack {
    fn new() -> Stack {
        Stack { crates: Vec::new() }
    }
}

#[derive(Debug)]
struct Move {
    count: usize,
    src: usize,
    dst: usize,
}

impl FromStr for Move {
    type Err = Oops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // move <count> from <src> to <dst>
        let mut splitter = s.split_whitespace().skip(1).step_by(2);
        let count = splitter.next().unwrap().parse()?;
        let src = splitter.next().unwrap().parse()?;
        let dst = splitter.next().unwrap().parse()?;
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

    // The blank line delimits the crate stack diagram and the move list.
    let split_idx = lines.iter().take_while(|l| !l.is_empty()).count();

    let crate_lines = &lines[..split_idx - 1];
    let move_lines = &lines[split_idx + 1..];

    // The last number on lines[split_idx - 1] is the number of crate stacks.
    let stack_count = lines[split_idx - 1]
        .split_whitespace()
        .rev()
        .next()
        .unwrap()
        .parse()
        .unwrap();

    let mut stacks: Vec<Stack> = Vec::new();
    stacks.resize_with(stack_count, || Stack::new());

    // Now parse the crate stacks. Scan for alphanumerics; dividing the index by 4 yields the stack
    // index (0-based). Iterate in reverse to build the stack from the bottom up.
    for line in crate_lines.iter().rev() {
        // Another approach is to use chunks() rather than scanning for the alphabetic characters.
        for (i, c) in line.char_indices() {
            if c.is_alphabetic() {
                stacks[i / 4].crates.push(c);
            }
        }
    }

    let moves = move_lines
        .iter()
        .map(|line| line.parse::<Move>())
        .collect::<Result<Vec<_>, _>>()?;

    Ok(Entity { stacks, moves })
}

fn part1(e: &Entity) -> Result<String, Oops> {
    let mut new_stacks = e.stacks.clone();
    for m in &e.moves {
        let src = &mut new_stacks[m.src - 1].crates;
        let mut moved_crates = src.drain(src.len() - m.count..).rev().collect();
        new_stacks[m.dst - 1].crates.append(&mut moved_crates);
    }

    Ok(new_stacks
        .iter()
        .map(|s| *s.crates.last().unwrap())
        .collect())
}

fn part2(e: &Entity) -> Result<String, Oops> {
    let mut new_stacks = e.stacks.clone();
    for m in &e.moves {
        let src = &mut new_stacks[m.src - 1].crates;
        let mut moved_crates = src.drain(src.len() - m.count..).collect();
        new_stacks[m.dst - 1].crates.append(&mut moved_crates);
    }

    Ok(new_stacks
        .iter()
        .map(|s| *s.crates.last().unwrap())
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

    const SAMPLE: &str = concat!(
        "    [D]    \n",
        "[N] [C]    \n",
        "[Z] [M] [P]\n",
        " 1   2   3 \n",
        "\n",
        "move 1 from 2 to 1\n",
        "move 3 from 1 to 3\n",
        "move 2 from 2 to 1\n",
        "move 1 from 1 to 2\n",
    );

    #[test]
    fn example1() {
        assert_eq!("CMZ", part1(&parse(SAMPLE.lines()).unwrap()).unwrap());
    }

    #[test]
    fn example2() {
        assert_eq!("MCD", part2(&parse(SAMPLE.lines()).unwrap()).unwrap());
    }
}
