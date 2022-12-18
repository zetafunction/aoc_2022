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
use std::collections::{HashMap, HashSet, VecDeque};
use std::io::{self, Read};
use std::str::FromStr;

#[derive(Clone, Copy, Eq, PartialEq)]
enum Point {
    Side,
    Cube,
}

struct Puzzle {
    points: HashSet<(i64, i64, i64)>,
}

impl FromStr for Puzzle {
    type Err = Oops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Puzzle {
            points: s
                .lines()
                .map(|line| {
                    let mut parser = line.split(',');
                    let x = parser.next().unwrap().parse()?;
                    let y = parser.next().unwrap().parse()?;
                    let z = parser.next().unwrap().parse()?;
                    Ok((x, y, z))
                })
                .collect::<Result<_, Oops>>()?,
        })
    }
}

fn parse(input: &str) -> Result<Puzzle, Oops> {
    input.parse()
}

fn part1(puzzle: &Puzzle) -> Result<i32, Oops> {
    let mut sides = HashMap::<(i64, i64, i64), i32>::new();
    for (x, y, z) in &puzzle.points {
        let mut count = 6;
        if let Some(left) = sides.get_mut(&(*x - 1, *y, *z)) {
            *left -= 1;
            count -= 1;
        }
        if let Some(right) = sides.get_mut(&(*x + 1, *y, *z)) {
            *right -= 1;
            count -= 1;
        }
        if let Some(bottom) = sides.get_mut(&(*x, *y - 1, *z)) {
            *bottom -= 1;
            count -= 1;
        }
        if let Some(top) = sides.get_mut(&(*x, *y + 1, *z)) {
            *top -= 1;
            count -= 1;
        }
        if let Some(front) = sides.get_mut(&(*x, *y, *z - 1)) {
            *front -= 1;
            count -= 1;
        }
        if let Some(back) = sides.get_mut(&(*x, *y, *z + 1)) {
            *back -= 1;
            count -= 1;
        }
        sides.insert((*x, *y, *z), count);
    }
    Ok(sides.iter().map(|(_, v)| v).sum::<i32>())
}

fn part2(puzzle: &Puzzle) -> Result<i32, Oops> {
    let mut sides = HashMap::<(i64, i64, i64), i32>::new();
    let mut min_x = i64::MAX;
    let mut max_x = i64::MIN;
    let mut min_y = i64::MAX;
    let mut max_y = i64::MIN;
    let mut min_z = i64::MAX;
    let mut max_z = i64::MIN;
    for (x, y, z) in &puzzle.points {
        min_x = std::cmp::min(min_x, *x);
        max_x = std::cmp::max(max_x, *x);
        min_y = std::cmp::min(min_y, *y);
        max_y = std::cmp::max(max_y, *y);
        min_z = std::cmp::min(min_z, *z);
        max_z = std::cmp::max(max_z, *z);
        let mut count = 6;
        if let Some(left) = sides.get_mut(&(*x - 1, *y, *z)) {
            *left -= 1;
            count -= 1;
        }
        if let Some(right) = sides.get_mut(&(*x + 1, *y, *z)) {
            *right -= 1;
            count -= 1;
        }
        if let Some(bottom) = sides.get_mut(&(*x, *y - 1, *z)) {
            *bottom -= 1;
            count -= 1;
        }
        if let Some(top) = sides.get_mut(&(*x, *y + 1, *z)) {
            *top -= 1;
            count -= 1;
        }
        if let Some(front) = sides.get_mut(&(*x, *y, *z - 1)) {
            *front -= 1;
            count -= 1;
        }
        if let Some(back) = sides.get_mut(&(*x, *y, *z + 1)) {
            *back -= 1;
            count -= 1;
        }
        sides.insert((*x, *y, *z), count);
    }
    println!(
        "{} {} {} {} {} {}",
        min_x, max_x, min_y, max_y, min_z, max_z
    );
    let mut count = 0;
    min_x -= 1;
    max_x += 1;
    min_y -= 1;
    max_y += 1;
    min_z -= 1;
    max_z += 1;
    let mut frontier = VecDeque::new();
    let mut visited = HashSet::new();
    frontier.push_back((-min_x, -min_y, -min_z));
    let in_bounds =
        |x, y, z| x >= min_x && x <= max_x && y >= min_y && y <= max_y && z >= min_z && z <= max_z;
    while let Some((cur_x, cur_y, cur_z)) = frontier.pop_front() {
        if visited.contains(&(cur_x, cur_y, cur_z)) {
            continue;
        }
        visited.insert((cur_x, cur_y, cur_z));
        let potential_next = vec![
            (cur_x - 1, cur_y, cur_z),
            (cur_x + 1, cur_y, cur_z),
            (cur_x, cur_y - 1, cur_z),
            (cur_x, cur_y + 1, cur_z),
            (cur_x, cur_y, cur_z - 1),
            (cur_x, cur_y, cur_z + 1),
        ];
        let potential_next = potential_next
            .into_iter()
            .filter(|&(x, y, z)| {
                in_bounds(x, y, z)
                    && !visited.contains(&(x, y, z))
                    && !sides.contains_key(&(x, y, z))
            })
            .collect::<Vec<_>>();

        if sides.contains_key(&(cur_x - 1, cur_y, cur_z)) {
            count += 1;
        }
        if sides.contains_key(&(cur_x + 1, cur_y, cur_z)) {
            count += 1;
        }

        if sides.contains_key(&(cur_x, cur_y - 1, cur_z)) {
            count += 1;
        }
        if sides.contains_key(&(cur_x, cur_y + 1, cur_z)) {
            count += 1;
        }
        if sides.contains_key(&(cur_x, cur_y, cur_z - 1)) {
            count += 1;
        }
        if sides.contains_key(&(cur_x, cur_y, cur_z + 1)) {
            count += 1;
        }
        frontier.extend(potential_next.into_iter());
    }
    Ok(count)
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
        "2,2,2\n", "1,2,2\n", "3,2,2\n", "2,1,2\n", "2,3,2\n", "2,2,1\n", "2,2,3\n", "2,2,4\n",
        "2,2,6\n", "1,2,5\n", "3,2,5\n", "2,1,5\n", "2,3,5\n",
    );

    #[test]
    fn example1() {
        assert_eq!(64, part1(&parse(SAMPLE).unwrap()).unwrap());
    }

    #[test]
    fn example2() {
        assert_eq!(58, part2(&parse(SAMPLE).unwrap()).unwrap());
    }
}
