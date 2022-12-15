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

#[derive(Clone, Copy)]
enum Material {
    Air,
    Rock,
    Sand,
    Source,
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Point { x, y }
    }
}

#[derive(Clone)]
struct Puzzle {
    grid: HashMap<Point, Material>,
    top_left: Point,
    bottom_right: Point,
}

impl Puzzle {
    fn draw_line(&mut self, p1: &Point, p2: &Point, m: Material) {
        let x1 = std::cmp::min(p1.x, p2.x);
        let x2 = std::cmp::max(p1.x, p2.x);
        let y1 = std::cmp::min(p1.y, p2.y);
        let y2 = std::cmp::max(p1.y, p2.y);

        // This assumes the lines are always strictly horizontal or strictly vertical.
        for x in x1..=x2 {
            for y in y1..=y2 {
                self.grid.insert(Point::new(x, y), m);
            }
        }

        self.top_left.x = std::cmp::min(self.top_left.x, x1);
        self.top_left.y = std::cmp::min(self.top_left.y, y1);
        self.bottom_right.x = std::cmp::max(self.bottom_right.x, x2);
        self.bottom_right.y = std::cmp::max(self.bottom_right.y, y2);
    }

    fn fill(&mut self, m: Material) {
        for x in self.top_left.x..=self.bottom_right.x {
            for y in self.top_left.y..=self.bottom_right.y {
                self.grid.entry(Point::new(x, y)).or_insert(m);
            }
        }
    }

    fn set(&mut self, p: Point, m: Material) {
        self.grid.insert(p, m);
        self.top_left.x = std::cmp::min(self.top_left.x, p.x);
        self.top_left.y = std::cmp::min(self.top_left.y, p.y);
        self.bottom_right.x = std::cmp::max(self.bottom_right.x, p.x);
        self.bottom_right.y = std::cmp::max(self.bottom_right.y, p.y);
    }
}

impl std::fmt::Debug for Puzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in self.top_left.y..=self.bottom_right.y {
            writeln!(
                f,
                "{}",
                (self.top_left.x..=self.bottom_right.x)
                    .map(|x| match self.grid.get(&Point::new(x, y)).unwrap() {
                        Material::Air => '.',
                        Material::Rock => '#',
                        Material::Sand => 'O',
                        Material::Source => '+',
                    })
                    .collect::<String>()
            )?;
        }
        Ok(())
    }
}

impl FromStr for Puzzle {
    type Err = Oops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut puzzle = Puzzle {
            grid: HashMap::new(),
            top_left: Point::new(i64::MAX, i64::MAX),
            bottom_right: Point::new(i64::MIN, i64::MIN),
        };
        for line in s.lines() {
            let mut prev_pt = None;
            for point in line.split(" -> ") {
                let (x, y) = point
                    .split_once(',')
                    .ok_or_else(|| oops!("invalid point"))?;
                let cur_pt = Point::new(x.parse()?, y.parse()?);
                if let Some(prev_pt) = prev_pt {
                    puzzle.draw_line(&prev_pt, &cur_pt, Material::Rock);
                }
                prev_pt = Some(cur_pt);
            }
        }
        // Per the problem, (500, 0) is the source of sand.
        puzzle.set(Point::new(500, 0), Material::Source);
        puzzle.fill(Material::Air);
        Ok(puzzle)
    }
}

fn parse(input: &str) -> Result<Puzzle, Oops> {
    match input.parse() {
        Ok(p) => {
            println!("{:?}", p);
            Ok(p)
        }
        e => e,
    }
}

fn drop_sand(puzzle: &Puzzle, position: &Point) -> Option<Point> {
    let mut current = *position;
    loop {
        let down = Point::new(current.x, current.y + 1);
        match puzzle.grid.get(&down) {
            Some(Material::Air) => {
                current = down;
                continue;
            }
            None => return None,
            _ => (),
        }
        let down_left = Point::new(current.x - 1, current.y + 1);
        match puzzle.grid.get(&down_left) {
            Some(Material::Air) => {
                current = down_left;
                continue;
            }
            None => return None,
            _ => (),
        }
        let down_right = Point::new(current.x + 1, current.y + 1);
        match puzzle.grid.get(&down_right) {
            Some(Material::Air) => {
                current = down_right;
                continue;
            }
            None => return None,
            _ => (),
        }
        if current == *position {
            return None;
        }
        return Some(current);
    }
}

fn part1(puzzle: &Puzzle) -> usize {
    let mut puzzle = puzzle.clone();
    loop {
        for x in 0.. {
            let Some(position) = drop_sand(&puzzle, &Point::new(500, 0)) else {
                println!("{:?}", puzzle);
                return x;
            };
            puzzle.set(position, Material::Sand);
        }
    }
}

fn part2(puzzle: &Puzzle) -> usize {
    let mut puzzle = puzzle.clone();
    let bottom = puzzle.bottom_right.y + 2;
    let height = bottom - puzzle.top_left.y;
    puzzle.draw_line(
        &Point::new(500 - height, bottom),
        &Point::new(500 + height, bottom),
        Material::Rock,
    );
    puzzle.fill(Material::Air);

    loop {
        for x in 1.. {
            let Some(position) = drop_sand(&puzzle, &Point::new(500, 0)) else {
                println!("{:?}", puzzle);
                return x;
            };
            puzzle.set(position, Material::Sand);
        }
    }
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
        "498,4 -> 498,6 -> 496,6\n",
        "503,4 -> 502,4 -> 502,9 -> 494,9\n",
    );

    #[test]
    fn example1() {
        assert_eq!(24, part1(&parse(SAMPLE).unwrap()));
    }

    #[test]
    fn example2() {
        assert_eq!(93, part2(&parse(SAMPLE).unwrap()));
    }
}
