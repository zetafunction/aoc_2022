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
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::{self, Read};
use std::str::FromStr;

struct Puzzle {
    map: Vec<Vec<i32>>,
    start: (usize, usize),
    end: (usize, usize),
}

fn advance_search(
    map: &Vec<Vec<i32>>,
    (cur_x, cur_y): (usize, usize),
    visited: &HashSet<(usize, usize)>,
) -> Vec<(usize, usize)> {
    [(1, 0), (-1, 0), (0, 1), (0, -1)]
        .iter()
        .map(|(mod_x, mod_y)| (cur_x as i32 + mod_x, cur_y as i32 + mod_y))
        .filter(|&(x, y)| {
            x >= 0
                && (x as usize) < map.len()
                && y >= 0
                && (y as usize) < map[0].len()
                && !visited.contains(&(x as usize, y as usize))
                && map[x as usize][y as usize] - map[cur_x][cur_y] <= 1
        })
        .map(|(x, y)| (x as usize, y as usize))
        .collect::<Vec<_>>()
}

impl Puzzle {
    fn bfs(&self, start: (usize, usize)) -> Option<usize> {
        let mut visited = HashSet::new();
        let mut candidates = VecDeque::new();
        let mut distances = HashMap::new();
        candidates.push_back(start);
        distances.insert(start, 0);
        while let Some(next) = candidates.pop_front() {
            visited.insert(next);
            let next_candidates = advance_search(&self.map, next, &visited);
            let current_distance = *distances.get(&next).unwrap();
            for c in next_candidates {
                distances.insert(c, current_distance + 1);
                if !candidates.contains(&c) {
                    candidates.push_back(c);
                }
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
        let map = s
            .lines()
            .enumerate()
            .map(|(x, line)| {
                line.chars()
                    .enumerate()
                    .map(|(y, c)| {
                        if c == 'S' {
                            start = Some((x, y));
                            0
                        } else if c == 'E' {
                            end = Some((x, y));
                            25
                        } else {
                            c as i32 - 'a' as i32
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
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
    puzzle.bfs(puzzle.start).ok_or_else(|| oops!("no solution"))
}

fn part2(puzzle: &Puzzle) -> Result<usize, Oops> {
    let candidates = (0..puzzle.map.len())
        .flat_map(|x| (0..puzzle.map[0].len()).map(move |y| (x, y)))
        .filter(|&(x, y)| puzzle.map[x][y] == 0)
        .collect::<Vec<_>>();
    let mut results = candidates
        .iter()
        .filter_map(|&c| puzzle.bfs(c))
        .collect::<Vec<_>>();
    results.sort();
    results.first().copied().ok_or_else(|| oops!("no solution"))
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
