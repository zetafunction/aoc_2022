use std::io;
use std::str::FromStr;

#[derive(Copy, Clone, PartialEq)]
enum HandShape {
    Rock,
    Paper,
    Scissors,
}

#[derive(Copy, Clone)]
enum Outcome {
    Loss,
    Draw,
    Win,
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
        let opponent_shape = match parts[0] {
            "A" => HandShape::Rock,
            "B" => HandShape::Paper,
            "C" => HandShape::Scissors,
            _ => return Err("non-matching symbol"),
        };
        let my_shape = match parts[1] {
            "X" => HandShape::Rock,
            "Y" => HandShape::Paper,
            "Z" => HandShape::Scissors,
            _ => return Err("non-matching symbol"),
        };
        let outcome = match parts[1] {
            "X" => Outcome::Loss,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => return Err("non-matching symbol"),
        };
        Ok(ParsedLine {
            opponent_shape: opponent_shape,
            my_shape: my_shape,
            outcome: outcome,
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
