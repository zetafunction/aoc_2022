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
    label: String,
    info: Info,
}

impl FromStr for Entry {
    type Err = Oops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        /*
        println!("{}", s);
        let depth = s.find('/').unwrap() / 2;
        let mut pieces = s.split_whitespace();
        pieces.next(); // Skip the `-`
        let label = pieces.next().unwrap().to_string();
        let info = if pieces.next().unwrap() == "(dir)" {
            Info::Directory
        } else {
            let size_as_string = pieces.next().unwrap();
            let size = size_as_string[5..size_as_string.len() - 2].parse::<usize>()?;
            Info::File(size)
        };
        */
        Err(oops!("not implemented"))
    }
}

struct Entity {
    values: Vec<Entry>,
}

impl FromStr for Entity {
    type Err = Oops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut entries = Vec::new();
        let mut current_depth: usize = 0;
        // Skip $ cd /
        let mut lines = s.lines();
        loop {
            let l = lines.next();
            if l.is_none() {
                break;
            }
            let l = l.unwrap();
            if l.starts_with('$') {
                let mut cmd = l.split_whitespace().skip(1);
                let verb = cmd.next().unwrap();
                if verb == "cd" {
                    let arg = cmd.next().unwrap();
                    if arg == ".." {
                        current_depth -= 1;
                    } else {
                        entries.push(Entry {
                            depth: current_depth,
                            label: arg.to_string(),
                            info: Info::Directory,
                        });
                        current_depth += 1;
                    }
                }
            } else if l.starts_with("dir") {
                continue;
            } else {
                let mut file_info = l.split_whitespace();
                let size = file_info.next().unwrap().parse()?;
                let label = file_info.next().unwrap();
                entries.push(Entry {
                    depth: current_depth,
                    label: label.to_string(),
                    info: Info::File(size),
                });
            }
        }
        /*
        Ok(Entity {
            values: s
                .lines()
                .map(|s| s.parse::<Entry>())
                .collect::<Result<Vec<_>, _>>()?,
        })
        */
        Ok(Entity { values: entries })
    }
}

fn parse(input: &str) -> Result<Entity, Oops> {
    input.parse()
}

fn part1(entity: &Entity) -> Result<usize, Oops> {
    let mut current_sizes = Vec::new();
    let mut result = 0;
    for e in &entity.values {
        while e.depth < current_sizes.len() {
            let dir_size = current_sizes.pop().unwrap();
            if dir_size < 100000 {
                result += dir_size;
            }
        }
        match e.info {
            Info::Directory => {
                current_sizes.push(0);
                ()
            }
            Info::File(size) => {
                for dir_size in &mut current_sizes {
                    *dir_size += size;
                }
            }
        }
    }
    Ok(result)
}

fn part2(entity: &Entity) -> Result<usize, Oops> {
    let mut current_sizes = Vec::new();
    let mut result = 0;
    for e in &entity.values {
        while e.depth < current_sizes.len() {
            let dir_size = current_sizes.pop().unwrap();
            if dir_size < 100000 {
                result += dir_size;
            }
        }
        match e.info {
            Info::Directory => {
                current_sizes.push(0);
                ()
            }
            Info::File(size) => {
                for dir_size in &mut current_sizes {
                    *dir_size += size;
                }
            }
        }
    }
    let current_size = current_sizes[0];
    const VOLUME_SIZE: usize = 70000000;
    const REQUIRED_SIZE: usize = 30000000;
    let mut candidate_size = VOLUME_SIZE;
    for e in &entity.values {
        while e.depth < current_sizes.len() {
            let dir_size = current_sizes.pop().unwrap();
            if (VOLUME_SIZE - (current_size - dir_size) >= REQUIRED_SIZE)
                && dir_size < candidate_size
            {
                candidate_size = dir_size;
            }
        }
        match e.info {
            Info::Directory => {
                current_sizes.push(0);
                ()
            }
            Info::File(size) => {
                for dir_size in &mut current_sizes {
                    *dir_size += size;
                }
            }
        }
    }
    Ok(candidate_size)
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
