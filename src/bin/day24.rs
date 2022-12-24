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

use aoc_2022::geometry::{Bounds2, Point2, Vector2};
use aoc_2022::{oops, oops::Oops};
use std::collections::HashMap;
use std::io::{self, Read};
use std::str::FromStr;

#[derive(Clone, Copy, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn parse_direction(c: char) -> Direction {
    match c {
        '^' => Direction::North,
        '>' => Direction::East,
        'v' => Direction::South,
        '<' => Direction::West,
        _ => panic!("bad direction character"),
    }
}

fn get_vector_for_direction(d: Direction) -> Vector2 {
    match d {
        Direction::North => Vector2::new(0, -1),
        Direction::East => Vector2::new(1, 0),
        Direction::South => Vector2::new(0, 1),
        Direction::West => Vector2::new(-1, 0),
    }
}

#[derive(Debug)]
struct Blizzard {
    position: Point2,
    direction: Direction,
    vector: Vector2,
}
struct Puzzle {
    blizzards: Vec<Blizzard>,
    bounds: Bounds2,
}

impl Puzzle {
    fn get_next_blizzard_positions(&self, current: &[Blizzard]) -> Vec<Blizzard> {
        current
            .iter()
            .map(|blizzard| {
                let next_position = blizzard.position + blizzard.vector;
                let next_position = if self.bounds.contains(&next_position) {
                    next_position
                } else {
                    let width = self.bounds.max.x - self.bounds.min.x + 1;
                    let height = self.bounds.max.y - self.bounds.min.y + 1;
                    Point2::new(
                        (next_position.x + width) % width,
                        (next_position.y + height) % height,
                    )
                };
                Blizzard {
                    position: next_position,
                    direction: blizzard.direction,
                    vector: blizzard.vector,
                }
            })
            .collect()
    }
}

impl FromStr for Puzzle {
    type Err = Oops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut max_x = 0;
        let mut max_y = 0;
        let mut blizzards = vec![];
        for (y, line) in (-1..).zip(s.lines()) {
            for (x, c) in (-1..).zip(line.chars()) {
                max_x = std::cmp::max(max_x, x);
                max_y = std::cmp::max(max_y, y);

                match c {
                    '#' => continue,
                    '.' => continue,
                    c => {
                        let direction = parse_direction(c);
                        blizzards.push(Blizzard {
                            position: Point2::new(x, y),
                            direction,
                            vector: get_vector_for_direction(direction),
                        });
                    }
                }
            }
        }

        let bounds = Bounds2 {
            min: Point2::new(0, 0),
            max: Point2::new(max_x - 1, max_y - 1),
        };
        println!("{blizzards:?}");
        Ok(Puzzle { blizzards, bounds })
    }
}

fn parse(input: &str) -> Result<Puzzle, Oops> {
    input.parse()
}

#[allow(dead_code)]
fn visualize(bounds: &Bounds2, blizzards: &[Blizzard]) -> String {
    #[derive(Debug)]
    enum Visualization {
        Direction(Direction),
        Count(usize),
    }
    let mut blizzards_and_counts = HashMap::new();
    for blizzard in blizzards {
        blizzards_and_counts
            .entry(blizzard.position)
            .and_modify(|v| match v {
                Visualization::Direction(_) => *v = Visualization::Count(2),
                Visualization::Count(x) => *x += 1,
            })
            .or_insert(Visualization::Direction(blizzard.direction));
        println!("Processed blizzard {blizzard:?}");
    }
    println!("{blizzards_and_counts:?}");
    let lines = (bounds.min.y..=bounds.max.y)
        .map(|y| {
            format!(
                "{}",
                (bounds.min.x..=bounds.max.x)
                    .map(|x| {
                        match blizzards_and_counts.get(&Point2::new(x, y)) {
                            Some(Visualization::Direction(d)) => match d {
                                Direction::North => '^',
                                Direction::East => '>',
                                Direction::South => 'v',
                                Direction::West => '<',
                            },
                            Some(Visualization::Count(c)) if *c < 10 => {
                                (*c as u8 + '0' as u8) as char
                            }
                            Some(Visualization::Count(_)) => '!',
                            None => '.',
                        }
                    })
                    .collect::<String>()
            )
        })
        .collect::<Vec<_>>();
    lines.join("\n")
}

fn part1(puzzle: &Puzzle) -> usize {
    0
}

fn part2(puzzle: &Puzzle) -> usize {
    0
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
        "#.######\n",
        "#>>.<^<#\n",
        "#.<..<<#\n",
        "#>v.><>#\n",
        "#<^v^^>#\n",
        "######.#\n",
    );

    #[test]
    fn simulation() {
        const SIMPLE: &str = concat!(
            "#.#####\n",
            "#.....#\n",
            "#>....#\n",
            "#.....#\n",
            "#...v.#\n",
            "#.....#\n",
            "#####.#\n",
        );
        let puzzle = parse(SIMPLE).unwrap();
        assert_eq!(
            concat!(".....\n", ">....\n", ".....\n", "...v.\n", ".....",),
            visualize(&puzzle.bounds, &puzzle.blizzards)
        );
        let next = puzzle.get_next_blizzard_positions(&puzzle.blizzards);
        assert_eq!(
            concat!(".....\n", ".>...\n", ".....\n", ".....\n", "...v.",),
            visualize(&puzzle.bounds, &next)
        );
        let next = puzzle.get_next_blizzard_positions(&next);
        assert_eq!(
            concat!("...v.\n", "..>..\n", ".....\n", ".....\n", ".....",),
            visualize(&puzzle.bounds, &next)
        );
        let next = puzzle.get_next_blizzard_positions(&next);
        assert_eq!(
            concat!(".....\n", "...2.\n", ".....\n", ".....\n", ".....",),
            visualize(&puzzle.bounds, &next)
        );
        let next = puzzle.get_next_blizzard_positions(&next);
        assert_eq!(
            concat!(".....\n", "....>\n", "...v.\n", ".....\n", ".....",),
            visualize(&puzzle.bounds, &next)
        );
        let next = puzzle.get_next_blizzard_positions(&next);
        assert_eq!(
            concat!(".....\n", ">....\n", ".....\n", "...v.\n", ".....",),
            visualize(&puzzle.bounds, &next)
        );
    }

    #[test]
    fn example1() {
        assert_eq!(18, part1(&parse(SAMPLE).unwrap()));
    }

    #[test]
    fn example2() {
        assert_eq!(2468013579, part2(&parse(SAMPLE).unwrap()));
    }
}
