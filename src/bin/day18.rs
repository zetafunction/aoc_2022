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
use std::borrow::Borrow;
use std::collections::{HashMap, HashSet, VecDeque};
use std::io::{self, Read};
use std::ops::Add;
use std::str::FromStr;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Point3 {
    x: i32,
    y: i32,
    z: i32,
}

const NEIGHBOR_VECTORS: &[Vector3] = &[
    Vector3::new(-1, 0, 0),
    Vector3::new(1, 0, 0),
    Vector3::new(0, -1, 0),
    Vector3::new(0, 1, 0),
    Vector3::new(0, 0, -1),
    Vector3::new(0, 0, 1),
];

struct Neighbors<'a> {
    p: &'a Point3,
    iter: std::slice::Iter<'static, Vector3>,
}

impl<'a> Iterator for Neighbors<'a> {
    type Item = Point3;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(v) = self.iter.next() {
            Some(*self.p + *v)
        } else {
            None
        }
    }
}

impl Point3 {
    const fn new(x: i32, y: i32, z: i32) -> Self {
        Point3 { x, y, z }
    }

    fn neighbors(&self) -> Neighbors {
        Neighbors {
            p: self,
            iter: NEIGHBOR_VECTORS.iter(),
        }
    }
}

impl Add<Vector3> for Point3 {
    type Output = Point3;

    fn add(self, rhs: Vector3) -> Self::Output {
        Point3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Vector3 {
    x: i32,
    y: i32,
    z: i32,
}

impl Vector3 {
    const fn new(x: i32, y: i32, z: i32) -> Self {
        Vector3 { x, y, z }
    }
}

// TODO: Maybe this should be a cube class?
struct Bounds3 {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
    min_z: i32,
    max_z: i32,
}

impl Bounds3 {
    fn contains(&self, p: &Point3) -> bool {
        p.x >= self.min_x
            && p.x <= self.max_x
            && p.y >= self.min_y
            && p.y <= self.max_y
            && p.z >= self.min_z
            && p.z <= self.max_z
    }

    fn from_point3s<I>(i: I) -> Self
    where
        I: IntoIterator,
        I::Item: Borrow<Point3>,
    {
        i.into_iter()
            .fold(Self::new_uninitialized(), |b, p| Bounds3 {
                min_x: std::cmp::min(b.min_x, p.borrow().x - 1),
                max_x: std::cmp::max(b.max_x, p.borrow().x + 1),
                min_y: std::cmp::min(b.min_y, p.borrow().y - 1),
                max_y: std::cmp::max(b.max_y, p.borrow().y + 1),
                min_z: std::cmp::min(b.min_z, p.borrow().z - 1),
                max_z: std::cmp::max(b.max_z, p.borrow().z + 1),
            })
    }

    fn new_uninitialized() -> Self {
        Bounds3 {
            min_x: i32::MAX,
            max_x: i32::MIN,
            min_y: i32::MAX,
            max_y: i32::MIN,
            min_z: i32::MAX,
            max_z: i32::MIN,
        }
    }
}

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
    counts.iter().map(|(_, v)| v).sum()
}

fn part2(puzzle: &Puzzle) -> i32 {
    // Find the bounding box for the puzzle points.
    let bounds = Bounds3::from_point3s(puzzle.points.iter());
    let points: HashSet<_> = puzzle.points.iter().collect();

    // Do a BFS from a point guaranteed to be outside the solid and find all reachable surfaces.
    let mut count = 0;
    let mut frontier = VecDeque::new();
    let mut visited = HashSet::new();
    frontier.push_back(Point3::new(bounds.min_x, bounds.min_y, bounds.min_z));
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
