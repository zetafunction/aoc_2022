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

use aoc_2022::geometry::{Bounds3, Outsets3, Point3};
use aoc_2022::{oops, oops::Oops};
use std::collections::{HashMap, HashSet, VecDeque};
use std::io::{self, Read};
use std::str::FromStr;

struct Puzzle {
    points: Vec<Point3>,
}

impl FromStr for Puzzle {
    type Err = Oops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Puzzle {
            points: s
                .lines()
                .map(|line| {
                    let mut parser = line.split(',');
                    let x = parser.next().ok_or_else(|| oops!("missing x"))?.parse()?;
                    let y = parser.next().ok_or_else(|| oops!("missing y"))?.parse()?;
                    let z = parser.next().ok_or_else(|| oops!("missing z"))?.parse()?;
                    Ok(Point3::new(x, y, z))
                })
                .collect::<Result<_, Oops>>()?,
        })
    }
}

fn parse(input: &str) -> Result<Puzzle, Oops> {
    input.parse()
}

fn part1(puzzle: &Puzzle) -> i32 {
    let mut counts = HashMap::<Point3, i32>::new();
    for p in &puzzle.points {
        let count: i32 = p
            .neighbors()
            .map(|neighbor| {
                if let Some(neighbor_count) = counts.get_mut(&neighbor) {
                    *neighbor_count -= 1;
                    0
                } else {
                    1
                }
            })
            .sum();
        counts.insert(*p, count);
    }
    counts.values().sum()
}

fn part2(puzzle: &Puzzle) -> i32 {
    // Find the bounding box for the puzzle points.
    let bounds = Bounds3::from_points(puzzle.points.iter());
    // Expand the bounds by 1 in each dimension so the BFS can go around the edges of the solid.
    let outsets = Outsets3::new(1);
    let bounds = bounds + &outsets;
    let points: HashSet<_> = puzzle.points.iter().collect();

    // Do a BFS from a point guaranteed to be outside the solid and find all reachable surfaces.
    let mut count = 0;
    let mut frontier = VecDeque::new();
    let mut visited = HashSet::new();
    frontier.push_back(bounds.min);
    visited.insert(*frontier.front().unwrap());

    while let Some(p) = frontier.pop_front() {
        for neighbor in p.neighbors() {
            if !bounds.contains(&neighbor) || visited.contains(&neighbor) {
                continue;
            }
            if points.contains(&neighbor) {
                count += 1;
                continue;
            }
            frontier.push_back(neighbor);
            visited.insert(neighbor);
        }
    }
    count
}

fn main() -> Result<(), Oops> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let input = input;

    let puzzle = parse(&input)?;

    println!("{}", part1(&puzzle));
    println!("{}", part2(&puzzle));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = concat!(
        "2,2,2\n", "1,2,2\n", "3,2,2\n", "2,1,2\n", "2,3,2\n", "2,2,1\n", "2,2,3\n", "2,2,4\n",
        "2,2,6\n", "1,2,5\n", "3,2,5\n", "2,1,5\n", "2,3,5\n",
    );

    #[test]
    fn example1() {
        assert_eq!(64, part1(&parse(SAMPLE).unwrap()));
    }

    #[test]
    fn example2() {
        assert_eq!(58, part2(&parse(SAMPLE).unwrap()));
    }
}
