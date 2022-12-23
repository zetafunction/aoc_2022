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
use std::collections::{HashMap, HashSet};
use std::io::{self, Read};
use std::str::FromStr;

struct Puzzle {
    elves: Vec<Point2>,
}

impl Puzzle {
    fn find_next_position(
        &self,
        current: Point2,
        occupied: &HashSet<Point2>,
        round: usize,
    ) -> Point2 {
        const DIRECTIONS: [Direction; 4] = [
            Direction::North,
            Direction::South,
            Direction::West,
            Direction::East,
        ];

        const NORTH: Vector2 = Vector2::new(0, -1);
        const NORTHEAST: Vector2 = Vector2::new(1, -1);
        const EAST: Vector2 = Vector2::new(1, 0);
        const SOUTHEAST: Vector2 = Vector2::new(1, 1);
        const SOUTH: Vector2 = Vector2::new(0, 1);
        const SOUTHWEST: Vector2 = Vector2::new(-1, 1);
        const WEST: Vector2 = Vector2::new(-1, 0);
        const NORTHWEST: Vector2 = Vector2::new(-1, -1);

        if !occupied.contains(&(current + NORTH))
            && !occupied.contains(&(current + NORTHEAST))
            && !occupied.contains(&(current + EAST))
            && !occupied.contains(&(current + SOUTHEAST))
            && !occupied.contains(&(current + SOUTH))
            && !occupied.contains(&(current + SOUTHWEST))
            && !occupied.contains(&(current + WEST))
            && !occupied.contains(&(current + NORTHWEST))
        {
            return current;
        }

        for i in 0..DIRECTIONS.len() {
            let direction_to_check = DIRECTIONS[(round + i) % 4];
            match direction_to_check {
                Direction::North => {
                    if occupied.contains(&(current + NORTH))
                        || occupied.contains(&(current + NORTHEAST))
                        || occupied.contains(&(current + NORTHWEST))
                    {
                        continue;
                    }
                    let proposed = current + NORTH;
                    return proposed;
                }
                Direction::South => {
                    if occupied.contains(&(current + SOUTH))
                        || occupied.contains(&(current + SOUTHEAST))
                        || occupied.contains(&(current + SOUTHWEST))
                    {
                        continue;
                    }
                    let proposed = current + SOUTH;
                    return proposed;
                }
                Direction::West => {
                    if occupied.contains(&(current + WEST))
                        || occupied.contains(&(current + NORTHWEST))
                        || occupied.contains(&(current + SOUTHWEST))
                    {
                        continue;
                    }
                    let proposed = current + WEST;
                    return proposed;
                }
                Direction::East => {
                    if occupied.contains(&(current + EAST))
                        || occupied.contains(&(current + NORTHEAST))
                        || occupied.contains(&(current + SOUTHEAST))
                    {
                        continue;
                    }
                    let proposed = current + EAST;
                    return proposed;
                }
            }
        }

        current
    }
}

impl FromStr for Puzzle {
    type Err = Oops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        enum Tile {
            Elf,
            Empty,
        }
        let grid = s
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().map(move |(x, c)| {
                    Ok((
                        Point2::new(x.try_into()?, y.try_into()?),
                        match c {
                            '#' => Tile::Elf,
                            '.' => Tile::Empty,
                            _ => Err(oops!("bad tile"))?,
                        },
                    ))
                })
            })
            .collect::<Result<HashMap<_, _>, Oops>>()?;

        let elves = grid
            .into_iter()
            .filter_map(|(k, v)| match v {
                Tile::Elf => Some(k),
                Tile::Empty => None,
            })
            .collect();

        Ok(Puzzle { elves })
    }
}

fn parse(input: &str) -> Result<Puzzle, Oops> {
    input.parse()
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    North,
    South,
    West,
    East,
}

#[allow(dead_code)]
fn print(positions: &Vec<Point2>) {
    println!("Positions: ");
    let bounds = Bounds2::from_points(positions.iter());
    for y in bounds.min.y..=bounds.max.y {
        println!(
            "{}",
            (bounds.min.x..=bounds.max.x)
                .map(|x| {
                    if positions.contains(&Point2::new(x, y)) {
                        '#'
                    } else {
                        '.'
                    }
                })
                .collect::<String>()
        );
    }
}

fn part1(puzzle: &Puzzle) -> usize {
    let mut positions = puzzle.elves.clone();
    for round in 0..10 {
        let position_set = HashSet::from_iter(positions.iter().copied());
        let mut proposed_positions_counts = HashMap::new();
        let proposed_positions = positions
            .iter()
            .map(|position| {
                let proposed_position = puzzle.find_next_position(*position, &position_set, round);
                proposed_positions_counts
                    .entry(proposed_position)
                    .and_modify(|v| *v += 1)
                    .or_insert(1);
                proposed_position
            })
            .collect::<Vec<_>>();

        let new_positions = proposed_positions
            .into_iter()
            .enumerate()
            .map(
                |(i, proposed_position)| match proposed_positions_counts.get(&proposed_position) {
                    Some(count) if *count == 1 => proposed_position,
                    Some(count) if *count != 1 => positions[i],
                    _ => panic!(),
                },
            )
            .collect::<Vec<_>>();
        positions = new_positions;
    }
    let rectangle = Bounds2::from_points(positions.iter());
    ((rectangle.max.x - rectangle.min.x + 1) * (rectangle.max.y - rectangle.min.y + 1)) as usize
        - positions.len()
}

fn part2(puzzle: &Puzzle) -> usize {
    let mut positions = puzzle.elves.clone();
    for round in 0.. {
        let position_set = HashSet::from_iter(positions.iter().copied());
        let mut proposed_positions_counts = HashMap::new();
        let proposed_positions = positions
            .iter()
            .map(|position| {
                let proposed_position = puzzle.find_next_position(*position, &position_set, round);
                proposed_positions_counts
                    .entry(proposed_position)
                    .and_modify(|v| *v += 1)
                    .or_insert(1);
                proposed_position
            })
            .collect::<Vec<_>>();

        // TODO: Figure out if it is possible to avoid collecting the iterator. Clippy doesn't like
        // this and wants `proposed_positions` and `new_positions` to be chained; however,
        // proposed_positions_counts is mutably borrowed in the former and immutably borrowed in
        // the latter.
        let new_positions = proposed_positions
            .into_iter()
            .enumerate()
            .map(
                |(i, proposed_position)| match proposed_positions_counts.get(&proposed_position) {
                    Some(count) if *count == 1 => proposed_position,
                    Some(count) if *count != 1 => positions[i],
                    _ => panic!(),
                },
            )
            .collect::<Vec<_>>();
        if positions == new_positions {
            return round + 1;
        }
        positions = new_positions;
    }
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
        "..............\n",
        "..............\n",
        ".......#......\n",
        ".....###.#....\n",
        "...#...#.#....\n",
        "....#...##....\n",
        "...#.###......\n",
        "...##.#.##....\n",
        "....#..#......\n",
        "..............\n",
        "..............\n",
        "..............\n",
    );

    #[test]
    fn example1() {
        assert_eq!(110, part1(&parse(SAMPLE).unwrap()));
    }

    #[test]
    fn example2() {
        assert_eq!(20, part2(&parse(SAMPLE).unwrap()));
    }
}