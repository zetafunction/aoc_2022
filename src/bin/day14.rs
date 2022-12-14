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
    fn draw_line(&mut self, p1: &Point, p2: &Point, material: Material) {
        let x1 = std::cmp::min(p1.x, p2.x);
        let x2 = std::cmp::max(p1.x, p2.x);
        let y1 = std::cmp::min(p1.y, p2.y);
        let y2 = std::cmp::min(p1.y, p2.y);

        // This assumes the lines are always strictly horizontal or strictly vertical.
        for x in x1..=x2 {
            for y in y1..=y2 {
                self.grid.insert(Point::new(x, y), material);
            }
        }

        self.top_left.x = std::cmp::min(self.top_left.x, x1);
        self.top_left.y = std::cmp::min(self.top_left.y, y1);
        self.bottom_right.x = std::cmp::max(self.bottom_right.x, x2);
        self.bottom_right.y = std::cmp::max(self.bottom_right.y, y2);
    }

    fn fill(&mut self, material: Material) {
        for x in self.top_left.x..=self.bottom_right.x {
            for y in self.top_left.y..=self.bottom_right.y {
                self.grid.entry(Point::new(x, y)).or_insert(Material::Air);
            }
        }
    }

    /*
    fn print(&self) {
        for y in 0..=self.y_max {
            println!(
                "{}",
                (self.x_min..=self.x_max)
                    .map(|x| match self.grid.get(&(x, y)).unwrap() {
                        Material::Air => '.',
                        Material::Rock => '#',
                        Material::Sand => 'O',
                    })
                    .collect::<String>()
            );
        }
    }
    */
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
            let prev_pt = None;
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
        puzzle.fill(Material::Air);
        Ok(puzzle)
    }
}

fn parse(input: &str) -> Result<Puzzle, Oops> {
    input.parse()
}

fn part1(puzzle: &Puzzle) -> usize {
    let mut grid = puzzle.grid.clone();
    let mut sand_count = 0;
    loop {
        let mut sand_pos = (500, 0);
        loop {
            let down_pos = (sand_pos.0, sand_pos.1 + 1);
            match grid.get(&down_pos) {
                Some(Material::Air) => {
                    sand_pos = down_pos;
                    continue;
                }
                Some(_) => (),
                None => return sand_count,
            }
            let left_pos = (sand_pos.0 - 1, sand_pos.1 + 1);
            match grid.get(&left_pos) {
                Some(Material::Air) => {
                    sand_pos = left_pos;
                    continue;
                }
                Some(_) => (),
                None => return sand_count,
            }
            let right_pos = (sand_pos.0 + 1, sand_pos.1 + 1);
            match grid.get(&right_pos) {
                Some(Material::Air) => {
                    sand_pos = right_pos;
                    continue;
                }
                Some(_) => (),
                None => return sand_count,
            }
            // Otherwise, the sand cannot fall any further. Mark it.
            grid.insert(sand_pos, Material::Sand);
            break;
        }
        sand_count += 1;
    }
}

fn part2(puzzle: &Puzzle) -> usize {
    fn get_material<'a>(
        y_max: &i64,
        grid: &'a HashMap<(i64, i64), Material>,
        (x, y): &(i64, i64),
    ) -> Option<&'a Material> {
        if *y == y_max + 2 {
            Some(&Material::Rock)
        } else {
            grid.get(&(*x, *y))
        }
    }

    let mut puzzle = puzzle.clone();
    let mut sand_count = 0;

    loop {
        sand_count += 1;
        let mut sand_pos = (500, 0);
        loop {
            let down_pos = (sand_pos.0, sand_pos.1 + 1);
            match get_material(&puzzle.y_max, &puzzle.grid, &down_pos) {
                Some(Material::Air) | None => {
                    sand_pos = down_pos;
                    continue;
                }
                Some(_) => (),
            }
            let left_pos = (sand_pos.0 - 1, sand_pos.1 + 1);
            match get_material(&puzzle.y_max, &puzzle.grid, &left_pos) {
                Some(Material::Air) | None => {
                    sand_pos = left_pos;
                    continue;
                }
                Some(_) => (),
            }
            let right_pos = (sand_pos.0 + 1, sand_pos.1 + 1);
            match get_material(&puzzle.y_max, &puzzle.grid, &right_pos) {
                Some(Material::Air) | None => {
                    sand_pos = right_pos;
                    continue;
                }
                Some(_) => (),
            }
            if sand_pos == (500, 0) {
                return sand_count;
            }
            // Otherwise, the sand cannot fall any further. Mark it.
            puzzle.grid.insert(sand_pos, Material::Sand);
            break;
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
