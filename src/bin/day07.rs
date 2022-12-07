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

use aoc_2022::{oops, oops::Oops};
use std::io::{self, Read};
use std::str::FromStr;

#[derive(Debug)]
enum Info {
    Directory,
    File(usize),
}

#[derive(Debug)]
struct Entry {
    depth: usize,
    info: Info,
}

struct Entity {
    values: Vec<Entry>,
}

impl Entity {
    fn get_directory_sizes(&self) -> Result<Vec<usize>, Oops> {
        let mut current_sizes = Vec::new();
        let mut result = Vec::new();
        for e in &self.values {
            while e.depth < current_sizes.len() {
                result.push(current_sizes.pop().ok_or_else(|| oops!("bad input"))?);
            }
            match e.info {
                Info::Directory => current_sizes.push(0),
                Info::File(size) => {
                    for dir_size in &mut current_sizes {
                        *dir_size += size;
                    }
                }
            }
        }
        current_sizes.reverse();
        result.append(&mut current_sizes);
        Ok(result)
    }
}

impl FromStr for Entity {
    type Err = Oops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut entries = Vec::new();
        let mut current_depth: usize = 0;
        for cmd_and_output in s.split('$').map(|x| x.trim()).filter(|x| !x.is_empty()) {
            let (cmd, remainder) = cmd_and_output
                .split_once(' ')
                .ok_or_else(|| oops!("bad input"))?;
            match cmd {
                "cd" => match remainder {
                    ".." => {
                        current_depth -= 1;
                    }
                    _ => {
                        entries.push(Entry {
                            depth: current_depth,
                            info: Info::Directory,
                        });
                        current_depth += 1;
                    }
                },
                "ls" => {
                    entries.extend(remainder.lines().filter_map(|x| {
                        let (first, _) = x.split_once(' ')?;
                        let size = first.parse().ok()?;
                        Some(Entry {
                            depth: current_depth,
                            info: Info::File(size),
                        })
                    }));
                }
                _ => {
                    return Err(oops!("unexpected command"));
                }
            }
        }
        Ok(Entity { values: entries })
    }
}

fn parse(input: &str) -> Result<Entity, Oops> {
    input.parse()
}

fn part1(entity: &Entity) -> Result<usize, Oops> {
    const MAX_DIR_SIZE: usize = 100_000;

    Ok(entity
        .get_directory_sizes()?
        .iter()
        .filter(|x| **x <= MAX_DIR_SIZE)
        .sum())
}

fn part2(entity: &Entity) -> Result<usize, Oops> {
    const VOLUME_SIZE: usize = 70_000_000;
    const FREE_SPACE_REQUIRED: usize = 30_000_000;

    let directory_sizes = entity.get_directory_sizes()?;
    let root_size = directory_sizes.last().ok_or_else(|| oops!("bad input"))?;

    directory_sizes
        .iter()
        .filter(|x| VOLUME_SIZE - (root_size - **x) >= FREE_SPACE_REQUIRED)
        .min()
        .copied()
        .ok_or_else(|| oops!("bad input"))
}

fn main() -> Result<(), Oops> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let input = input;

    let entity = parse(&input)?;

    println!("{}", part1(&entity)?);
    println!("{}", part2(&entity)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = concat!(
        "$ cd /\n",
        "$ ls\n",
        "dir a\n",
        "14848514 b.txt\n",
        "8504156 c.dat\n",
        "dir d\n",
        "$ cd a\n",
        "$ ls\n",
        "dir e\n",
        "29116 f\n",
        "2557 g\n",
        "62596 h.lst\n",
        "$ cd e\n",
        "$ ls\n",
        "584 i\n",
        "$ cd ..\n",
        "$ cd ..\n",
        "$ cd d\n",
        "$ ls\n",
        "4060174 j\n",
        "8033020 d.log\n",
        "5626152 d.ext\n",
        "7214296 k\n",
    );

    #[test]
    fn example1() {
        assert_eq!(95437, part1(&parse(SAMPLE).unwrap()).unwrap());
    }

    #[test]
    fn example2() {
        assert_eq!(24933642, part2(&parse(SAMPLE).unwrap()).unwrap());
    }
}
