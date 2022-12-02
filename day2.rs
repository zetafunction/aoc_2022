use std::io;

#[derive(Copy, Clone)]
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

const POINTS_FOR_LOSS: u32 = 0;
const POINTS_FOR_DRAW: u32 = 3;
const POINTS_FOR_WIN: u32 = 6;

fn parse_hand_shape(input: char, base: char) -> HandShape {
    match input as u32 - base as u32 {
        0 => HandShape::Rock,
        1 => HandShape::Paper,
        2 => HandShape::Scissors,
        _ => unreachable!(),
    }
}

fn determine_outcome(opponent_shape: HandShape, my_shape: HandShape) -> Outcome {
    match opponent_shape {
        HandShape::Rock => match my_shape {
            HandShape::Rock => Outcome::Draw,
            HandShape::Paper => Outcome::Win,
            HandShape::Scissors => Outcome::Loss,
        },
        HandShape::Paper => match my_shape {
            HandShape::Rock => Outcome::Loss,
            HandShape::Paper => Outcome::Draw,
            HandShape::Scissors => Outcome::Win,
        },
        HandShape::Scissors => match my_shape {
            HandShape::Rock => Outcome::Win,
            HandShape::Paper => Outcome::Loss,
            HandShape::Scissors => Outcome::Draw,
        },
    }
}

fn determine_shape_score(shape: HandShape) -> u32 {
    match shape {
        HandShape::Rock => 1,
        HandShape::Paper => 2,
        HandShape::Scissors => 3,
    }
}

fn parse_outcome(input: char, base: char) -> Outcome {
    match input as u32 - base as u32 {
        0 => Outcome::Loss,
        1 => Outcome::Draw,
        2 => Outcome::Win,
        _ => unreachable!(),
    }
}

fn determine_outcome_score(outcome: Outcome) -> u32 {
    match outcome {
        Outcome::Loss => POINTS_FOR_LOSS,
        Outcome::Draw => POINTS_FOR_DRAW,
        Outcome::Win => POINTS_FOR_WIN,
    }
}

fn determine_shape_from_outcome(opponent_shape: HandShape, outcome: Outcome) -> HandShape {
    match opponent_shape {
        HandShape::Rock => match outcome {
            Outcome::Loss => HandShape::Scissors,
            Outcome::Draw => HandShape::Rock,
            Outcome::Win => HandShape::Paper,
        },
        HandShape::Paper => match outcome {
            Outcome::Loss => HandShape::Rock,
            Outcome::Draw => HandShape::Paper,
            Outcome::Win => HandShape::Scissors,
        },
        HandShape::Scissors => match outcome {
            Outcome::Loss => HandShape::Paper,
            Outcome::Draw => HandShape::Scissors,
            Outcome::Win => HandShape::Rock,
        },
    }
}

fn main() {
    let mut score = 0;
    let mut score2 = 0;
    for line in io::stdin().lines() {
        let line = line.unwrap();
        let mut chars = line.chars();

        let elf_shape = parse_hand_shape(chars.next().unwrap(), 'A');
        let my_input = chars.last().unwrap();

        // Part 1
        let my_shape = parse_hand_shape(my_input, 'X');
        score += determine_outcome_score(determine_outcome(elf_shape, my_shape));
        score += determine_shape_score(my_shape);

        // Part 2
        let outcome = parse_outcome(my_input, 'X');
        score2 += determine_shape_score(determine_shape_from_outcome(elf_shape, outcome));
        score2 += determine_outcome_score(outcome);
    }
    println!("{}", score);
    println!("{}", score2);
}
