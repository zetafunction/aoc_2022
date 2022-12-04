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

use std::io;
use std::str::FromStr;

#[derive(Copy, Clone, PartialEq)]
enum HandShape {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for HandShape {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(HandShape::Rock),
            "B" | "Y" => Ok(HandShape::Paper),
            "C" | "Z" => Ok(HandShape::Scissors),
            _ => Err("non-matching symbol"),
        }
    }
}

#[derive(Copy, Clone)]
enum Outcome {
    Loss,
    Draw,
    Win,
}

impl FromStr for Outcome {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Outcome::Loss),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => Err("non-matching symbol"),
        }
    }
}

struct ParsedLine {
    opponent_shape: HandShape,
    my_shape: HandShape,
    outcome: Outcome,
}

impl FromStr for ParsedLine {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(' ').collect();
        if parts.len() != 2 {
            return Err("each line must have 2 parts");
        }
        Ok(ParsedLine {
            opponent_shape: parts[0].parse::<HandShape>()?,
            my_shape: parts[1].parse::<HandShape>()?,
            outcome: parts[1].parse::<Outcome>()?,
        })
    }
}

fn determine_outcome(x: &ParsedLine) -> Outcome {
    match (x.opponent_shape, x.my_shape) {
        (HandShape::Rock, HandShape::Paper) => Outcome::Win,
        (HandShape::Paper, HandShape::Scissors) => Outcome::Win,
        (HandShape::Scissors, HandShape::Rock) => Outcome::Win,
        (a, b) if a == b => Outcome::Draw,
        _ => Outcome::Loss,
    }
}

fn determine_shape_score(x: HandShape) -> u32 {
    match x {
        HandShape::Rock => 1,
        HandShape::Paper => 2,
        HandShape::Scissors => 3,
    }
}

fn determine_outcome_score(x: Outcome) -> u32 {
    match x {
        Outcome::Loss => 0,
        Outcome::Draw => 3,
        Outcome::Win => 6,
    }
}

fn determine_shape_from_outcome(x: &ParsedLine) -> HandShape {
    match (x.opponent_shape, x.outcome) {
        (x, Outcome::Draw) => x,
        (HandShape::Rock, Outcome::Win) => HandShape::Paper,
        (HandShape::Paper, Outcome::Win) => HandShape::Scissors,
        (HandShape::Scissors, Outcome::Win) => HandShape::Rock,
        (HandShape::Rock, Outcome::Loss) => HandShape::Scissors,
        (HandShape::Paper, Outcome::Loss) => HandShape::Rock,
        (HandShape::Scissors, Outcome::Loss) => HandShape::Paper,
    }
}

fn main() {
    let parsed_lines: Vec<ParsedLine> = io::stdin()
        .lines()
        .map(|l| l.unwrap().parse::<ParsedLine>().unwrap())
        .collect();
    let mut score = 0;
    let mut score2 = 0;
    for parsed_line in &parsed_lines {
        // Part 1
        score += determine_outcome_score(determine_outcome(parsed_line));
        score += determine_shape_score(parsed_line.my_shape);

        // Part 2
        score2 += determine_shape_score(determine_shape_from_outcome(parsed_line));
        score2 += determine_outcome_score(parsed_line.outcome);
    }
    println!("{}", score);
    println!("{}", score2);
}
