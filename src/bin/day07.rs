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
use std::collections::HashMap;
use std::io::{self, Read};
use std::str::FromStr;

struct PathTree {
    size: usize,
    children: HashMap<String, PathTree>,
}

impl PathTree {
    fn new() -> PathTree {
        PathTree {
            size: 0,
            children: HashMap::new(),
        }
    }

    fn add_size_for_path(&mut self, path: &[&str], size: usize) {
        self.size += size;
        if path.is_empty() {
            return;
        }
        // TODO: Is it possible to avoid calling to_string() here?
        self.children
            .entry(path[0].to_string())
            .or_insert_with(PathTree::new)
            .add_size_for_path(&path[1..], size);
    }

    // TODO: It would be nice if this didn't require a FnMut.
    fn walk<F: FnMut(&PathTree)>(&self, f: &mut F) {
        f(self);
        for child in self.children.values() {
            child.walk(f);
        }
    }
}

struct Entity {
    tree: PathTree,
}

impl FromStr for Entity {
    type Err = Oops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tree = PathTree::new();
        let mut current_path = Vec::new();
        for cmd_and_output in s.split('$').map(str::trim).filter(|x| !x.is_empty()) {
            let (cmd, remainder) = cmd_and_output
                .split_once([' ', '\n'])
                .ok_or_else(|| oops!("bad input"))?;
            match cmd {
                "cd" => match remainder {
                    ".." => {
                        current_path.pop();
                    }
                    name => {
                        current_path.push(name);
                    }
                },
                "ls" => {
                    for line in remainder.lines() {
                        if line.starts_with("dir ") {
                            continue;
                        }
                        let (first, _) = line
                            .split_once(' ')
                            .ok_or_else(|| oops!("expected file size"))?;
                        tree.add_size_for_path(&current_path, first.parse()?);
                    }
                }
                huh => {
                    return Err(oops!("unexpected command {}", huh));
                }
            }
        }
        Ok(Entity { tree })
    }
}

fn parse(input: &str) -> Result<Entity, Oops> {
    input.parse()
}

fn part1(entity: &Entity) -> usize {
    const MAX_DIR_SIZE: usize = 100_000;

    let mut result = 0;
    entity.tree.walk(&mut |x: &PathTree| {
        if x.size <= MAX_DIR_SIZE {
            result += x.size;
        }
    });
    result
}

fn part2(entity: &Entity) -> usize {
    const VOLUME_SIZE: usize = 70_000_000;
    const FREE_SPACE_REQUIRED: usize = 30_000_000;

    let mut result = VOLUME_SIZE;
    entity.tree.walk(&mut |x: &PathTree| {
        if VOLUME_SIZE - (entity.tree.size - x.size) >= FREE_SPACE_REQUIRED && x.size < result {
            result = x.size;
        }
    });
    result
}

fn main() -> Result<(), Oops> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let input = input;

    let entity = parse(&input)?;

    println!("{}", part1(&entity));
    println!("{}", part2(&entity));

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
        assert_eq!(95437, part1(&parse(SAMPLE).unwrap()));
    }

    #[test]
    fn example2() {
        assert_eq!(24933642, part2(&parse(SAMPLE).unwrap()));
    }
}
