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

#[derive(Clone)]
enum Material {
    Air,
    Rock,
    Sand,
}

#[derive(Clone)]
struct Puzzle {
    grid: HashMap<(usize, usize), Material>,
    x_min: usize,
    x_max: usize,
    y_max: usize,
}

impl Puzzle {
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
}

impl FromStr for Puzzle {
    type Err = Oops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid = HashMap::new();
        let mut x_min = usize::MAX;
        let mut x_max = usize::MIN;
        let mut y_max = usize::MIN;
        for line in s.lines() {
            let mut rocks = vec![];
            let mut prev_point = None;
            for point in line.split(" -> ") {
                let (x, y) = point.split_once(',').unwrap();
                let x = x.parse::<usize>().unwrap();
                let y = y.parse::<usize>().unwrap();
                match prev_point {
                    None => (),
                    Some((prev_x, prev_y)) => {
                        let x1 = std::cmp::min(prev_x, x);
                        let x2 = std::cmp::max(prev_x, x);
                        let y1 = std::cmp::min(prev_y, y);
                        let y2 = std::cmp::max(prev_y, y);
                        rocks.extend((x1..=x2).flat_map(|x| (y1..=y2).map(move |y| (x, y))));
                    }
                }
                prev_point = Some((x, y));
            }
            for (x, y) in rocks {
                x_min = std::cmp::min(x, x_min);
                x_max = std::cmp::max(x, x_max);
                y_max = std::cmp::max(y, y_max);
                grid.insert((x, y), Material::Rock);
            }
        }
        for x in x_min..=x_max {
            for y in 0..=y_max {
                grid.entry((x, y)).or_insert(Material::Air);
            }
        }
        Ok(Puzzle {
            grid,
            x_min,
            x_max,
            y_max,
        })
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
    fn get_material(
        y_max: usize,
        grid: &HashMap<(usize, usize), Material>,
        (x, y): (usize, usize),
    ) -> Option<&Material> {
        if y == y_max + 2 {
            Some(&Material::Rock)
        } else {
            grid.get(&(x, y))
        }
    }

    let mut puzzle = puzzle.clone();
    let mut sand_count = 0;

    loop {
        sand_count += 1;
        let mut sand_pos = (500, 0);
        loop {
            let down_pos = (sand_pos.0, sand_pos.1 + 1);
            match get_material(puzzle.y_max, &puzzle.grid, down_pos) {
                Some(Material::Air) | None => {
                    sand_pos = down_pos;
                    continue;
                }
                Some(_) => (),
            }
            let left_pos = (sand_pos.0 - 1, sand_pos.1 + 1);
            match get_material(puzzle.y_max, &puzzle.grid, left_pos) {
                Some(Material::Air) | None => {
                    sand_pos = left_pos;
                    continue;
                }
                Some(_) => (),
            }
            let right_pos = (sand_pos.0 + 1, sand_pos.1 + 1);
            match get_material(puzzle.y_max, &puzzle.grid, right_pos) {
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
