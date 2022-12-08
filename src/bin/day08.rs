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
use std::collections::BTreeSet;
use std::io::{self, Read};
use std::str::FromStr;

struct Matrix<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T: Copy> Matrix<T> {
    fn new(width: usize, height: usize, default: T) -> Matrix<T> {
        Matrix {
            data: vec![default; width * height],
            width,
            height,
        }
    }

    fn get(&self, x: usize, y: usize) -> T {
        self.data[x + y * self.width]
    }

    fn set(&mut self, x: usize, y: usize, v: T) {
        self.data[x + y * self.width] = v;
    }

    fn for_col(&self, x: usize) -> Col<T> {
        Col {
            matrix: self,
            x,
            y_low: 0,
            y_high: self.height,
        }
    }

    fn for_row(&self, y: usize) -> Row<T> {
        Row {
            matrix: self,
            x_low: 0,
            x_high: self.width,
            y,
        }
    }
}

struct Col<'a, T> {
    matrix: &'a Matrix<T>,
    x: usize,
    y_low: usize,
    y_high: usize,
}

impl<'a, T> Iterator for Col<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.y_low < self.y_high {
            let y = self.y_low;
            self.y_low += 1;
            Some(&self.matrix.data[self.x + y * self.matrix.width])
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.y_high - self.y_low;
        (remaining, Some(remaining))
    }
}

impl<'a, T> DoubleEndedIterator for Col<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.y_high > self.y_low {
            self.y_high -= 1;
            let y = self.y_high;
            Some(&self.matrix.data[self.x + y * self.matrix.width])
        } else {
            None
        }
    }
}

impl<'a, T> ExactSizeIterator for Col<'a, T> {}

struct Row<'a, T> {
    matrix: &'a Matrix<T>,
    x_low: usize,
    x_high: usize,
    y: usize,
}

impl<'a, T> Iterator for Row<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.x_low < self.x_high {
            let x = self.x_low;
            self.x_low += 1;
            Some(&self.matrix.data[x + self.y * self.matrix.width])
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.x_high - self.x_low;
        (remaining, Some(remaining))
    }
}

impl<'a, T> DoubleEndedIterator for Row<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.x_high > self.x_low {
            self.x_high -= 1;
            let x = self.x_high;
            Some(&self.matrix.data[x + self.y * self.matrix.width])
        } else {
            None
        }
    }
}

impl<'a, T> ExactSizeIterator for Row<'a, T> {}

struct Puzzle {
    trees: Matrix<i32>,
}

impl FromStr for Puzzle {
    type Err = Oops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Simplifying assumption: input is a square matrix.
        let dim = s.lines().next().ok_or_else(|| oops!("no data"))?.len();
        let mut matrix = Matrix::new(dim, dim, 0);
        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                matrix.set(x, y, c as i32 - '0' as i32);
            }
        }
        Ok(Puzzle { trees: matrix })
    }
}

fn parse(input: &str) -> Result<Puzzle, Oops> {
    input.parse()
}

fn part1(puzzle: &Puzzle) -> Result<usize, Oops> {
    let mut visible = BTreeSet::new();

    for x in 0..puzzle.trees.width {
        let mut seen_tree = -1;
        for (y, &tree) in puzzle.trees.for_col(x).enumerate() {
            if tree > seen_tree {
                seen_tree = tree;
                visible.insert((x, y));
            }
        }

        let mut seen_tree = -1;
        for (y, &tree) in puzzle.trees.for_col(x).enumerate().rev() {
            if tree > seen_tree {
                seen_tree = tree;
                visible.insert((x, y));
            }
        }
    }

    for y in 0..puzzle.trees.height {
        let mut seen_tree = -1;
        for (x, &tree) in puzzle.trees.for_row(y).enumerate() {
            if tree > seen_tree {
                seen_tree = tree;
                visible.insert((x, y));
            }
        }

        let mut seen_tree = -1;
        for (x, &tree) in puzzle.trees.for_row(y).enumerate().rev() {
            if tree > seen_tree {
                seen_tree = tree;
                visible.insert((x, y));
            }
        }
    }

    Ok(visible.len())
}

fn part2(puzzle: &Puzzle) -> Result<usize, Oops> {
    let mut result = 0;
    for x in 0..puzzle.trees.width {
        for y in 0..puzzle.trees.height {
            let current_tree = puzzle.trees.get(x, y);
            let score = (0..x)
                .rev()
                .take_while(|&xc| puzzle.trees.get(xc, y) < current_tree)
                .count()
                * (x + 1..puzzle.trees.width)
                    .take_while(|&xc| puzzle.trees.get(xc, y) < current_tree)
                    .count()
                * (0..y)
                    .rev()
                    .take_while(|&yc| puzzle.trees.get(x, yc) < current_tree)
                    .count()
                * (y + 1..puzzle.trees.height)
                    .take_while(|&yc| puzzle.trees.get(x, yc) < current_tree)
                    .count();
            if score > result {
                result = score;
            }
        }
    }
    Ok(result)
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