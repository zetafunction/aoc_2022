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

use aoc_2022::geometry::Point2;
use aoc_2022::{oops, oops::Oops};
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::io::{self, Read};
use std::str::FromStr;

struct Puzzle {
    map: HashMap<Point2, i32>,
    start: Point2,
    end: Point2,
}

impl Puzzle {
    fn bfs(&self, start: &Point2) -> Option<usize> {
        let mut candidates = VecDeque::new();
        let mut distances = HashMap::new();
        candidates.push_back(*start);
        distances.insert(*start, 0);
        while let Some(next) = candidates.pop_front() {
            let current_height = self.map.get(&next).unwrap();
            let current_distance = *distances.get(&next).unwrap();
            for neighbor in next.neighbors() {
                // TODO: try using the Entry API to avoid contains_key followed by insert.
                let Some(height) = self.map.get(&neighbor) else {
                    continue;
                };
                if height - current_height > 1 {
                    continue;
                }
                let Entry::Vacant(v) = distances.entry(neighbor) else {
                    continue;
                };
                candidates.push_back(neighbor);
                v.insert(current_distance + 1);
            }
        }
        distances.get(&self.end).copied()
    }
}

impl FromStr for Puzzle {
    type Err = Oops;

    // Note: x indexes the line and y indexes the character on that line
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start = None;
        let mut end = None;
        let mut map = HashMap::new();
        for (x, line) in (1..).zip(s.lines()) {
            for (y, c) in (1..).zip(line.chars()) {
                let p = Point2::new(x, y);
                map.insert(
                    p,
                    if c == 'S' {
                        start = Some(p);
                        0
                    } else if c == 'E' {
                        end = Some(p);
                        25
                    } else {
                        c as i32 - 'a' as i32
                    },
                );
            }
        }
        Ok(Puzzle {
            map,
            start: start.ok_or_else(|| oops!("no start"))?,
            end: end.ok_or_else(|| oops!("no end"))?,
        })
    }
}

fn parse(input: &str) -> Result<Puzzle, Oops> {
    input.parse()
}

fn part1(puzzle: &Puzzle) -> Result<usize, Oops> {
    puzzle
        .bfs(&puzzle.start)
        .ok_or_else(|| oops!("no solution"))
}

fn part2(puzzle: &Puzzle) -> Result<usize, Oops> {
    puzzle
        .map
        .iter()
        .filter_map(|(k, v)| if *v == 0 { Some(k) } else { None })
        .filter_map(|start| puzzle.bfs(start))
        .min()
        .ok_or_else(|| oops!("no solution"))
}

fn main() -> Result<(), Oops> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let input = input;

    let puzzle = parse(&input)?;

    println!("{}", part1(&puzzle)?);
    println!("{}", part2(&puzzle)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = concat!(
        "Sabqponm\n",
        "abcryxxl\n",
        "accszExk\n",
        "acctuvwj\n",
        "abdefghi\n",
    );

    #[test]
    fn example1() {
        assert_eq!(31, part1(&parse(SAMPLE).unwrap()).unwrap());
    }

    #[test]
    fn example2() {
        assert_eq!(29, part2(&parse(SAMPLE).unwrap()).unwrap());
    }
}
