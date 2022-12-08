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
use std::collections::HashSet;
use std::io::{self, Read};
use std::str::FromStr;

struct Puzzle {
    trees: Vec<Vec<i32>>,
}

impl FromStr for Puzzle {
    type Err = Oops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result = Puzzle { trees: Vec::new() };
        for line in s.lines() {
            result
                .trees
                .push(line.chars().map(|c| c as i32 - '0' as i32).collect());
        }
        Ok(result)
    }
}

fn parse(input: &str) -> Result<Puzzle, Oops> {
    input.parse()
}

fn part1(puzzle: &Puzzle) -> Result<usize, Oops> {
    let xsize = puzzle.trees[0].len();
    let ysize = puzzle.trees.len();
    let mut visible = HashSet::new();

    // Check left
    let mut col = Vec::new();
    col.resize(ysize, -1);
    for x in 0..xsize {
        for y in 0..ysize {
            if puzzle.trees[x][y] > col[y] {
                visible.insert((x, y));
                col[y] = puzzle.trees[x][y];
            }
        }
    }
    // Check right
    let mut col = Vec::new();
    col.resize(ysize, -1);
    for x in (0..xsize).rev() {
        for y in 0..ysize {
            if puzzle.trees[x][y] > col[y] {
                visible.insert((x, y));
                col[y] = puzzle.trees[x][y];
            }
        }
    }
    // Check top
    let mut row = Vec::new();
    row.resize(xsize, -1);
    for y in 0..ysize {
        for x in 0..xsize {
            if puzzle.trees[x][y] > row[x] {
                visible.insert((x, y));
                row[x] = puzzle.trees[x][y];
            }
        }
    }
    // Check bottom
    let mut row = Vec::new();
    row.resize(xsize, -1);
    for y in (0..ysize).rev() {
        for x in 0..xsize {
            if puzzle.trees[x][y] > row[x] {
                visible.insert((x, y));
                row[x] = puzzle.trees[x][y];
            }
        }
    }
    Ok(visible.len())
}

fn part2(puzzle: &Puzzle) -> Result<usize, Oops> {
    let xsize = puzzle.trees[0].len();
    let ysize = puzzle.trees.len();

    let mut candidate_score = 0;

    for x in 0..xsize {
        for y in 0..ysize {
            let mut up_score = 0;
            let this_height = puzzle.trees[x][y];
            // look up
            for y2 in (0..y).rev() {
                up_score += 1;
                if puzzle.trees[x][y2] >= this_height {
                    break;
                }
            }
            // look down
            let mut down_score = 0;
            for y2 in y + 1..ysize {
                down_score += 1;
                if puzzle.trees[x][y2] >= this_height {
                    break;
                }
            }
            // look left
            let mut left_score = 0;
            for x2 in (0..x).rev() {
                left_score += 1;
                if puzzle.trees[x2][y] >= this_height {
                    break;
                }
            }
            // look right
            let mut right_score = 0;
            for x2 in x + 1..xsize {
                right_score += 1;
                if puzzle.trees[x2][y] >= this_height {
                    break;
                }
            }
            let score = up_score * left_score * right_score * down_score;
            if score > candidate_score {
                candidate_score = score;
            }
        }
    }
    Ok(candidate_score.try_into().unwrap())
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

    const SAMPLE: &str = concat!("30373\n", "25512\n", "65332\n", "33549\n", "35390\n",);

    #[test]
    fn example1() {
        assert_eq!(21, part1(&parse(SAMPLE).unwrap()).unwrap());
    }

    #[test]
    fn example2() {
        assert_eq!(8, part2(&parse(SAMPLE).unwrap()).unwrap());
    }
}
