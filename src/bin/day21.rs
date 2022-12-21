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

enum Monkey {
    Literal(i64),
    Add(String, String),
    Sub(String, String),
    Mul(String, String),
    Div(String, String),
}

impl FromStr for Monkey {
    type Err = Oops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(n) = s.parse::<i64>() {
            return Ok(Monkey::Literal(n));
        }
        let mut parser = s.split_whitespace();
        let first_operand = parser
            .next()
            .ok_or_else(|| oops!("bad expression: {s}"))?
            .to_string();
        let op = parser.next().ok_or_else(|| oops!("bad expression: {s}"))?;
        let second_operand = parser
            .next()
            .ok_or_else(|| oops!("bad expression: {s}"))?
            .to_string();
        Ok(match op {
            "+" => Monkey::Add(first_operand, second_operand),
            "-" => Monkey::Sub(first_operand, second_operand),
            "*" => Monkey::Mul(first_operand, second_operand),
            "/" => Monkey::Div(first_operand, second_operand),
            _ => Err(oops!("bad operand {op}"))?,
        })
    }
}

struct Puzzle {
    tree: HashMap<String, Monkey>,
}

#[derive(Debug)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
enum Expr {
    Human,
    Literal(i64),
    BinaryOp(Box<Expr>, Op, Box<Expr>),
}

impl Expr {
    fn new_human() -> Self {
        Expr::Human
    }
    fn new_literal(c: i64) -> Self {
        Expr::Literal(c)
    }
    fn new_add(x: Expr, y: Expr) -> Self {
        Expr::BinaryOp(Box::new(x), Op::Add, Box::new(y))
    }
    fn new_sub(x: Expr, y: Expr) -> Self {
        Expr::BinaryOp(Box::new(x), Op::Sub, Box::new(y))
    }
    fn new_mul(x: Expr, y: Expr) -> Self {
        Expr::BinaryOp(Box::new(x), Op::Mul, Box::new(y))
    }
    fn new_div(x: Expr, y: Expr) -> Self {
        Expr::BinaryOp(Box::new(x), Op::Div, Box::new(y))
    }
}

impl Expr {
    fn solve_for(&self, target: i64) -> Result<i64, Oops> {
        match self {
            Expr::Human => Ok(target),
            Expr::Literal(_) => Err(oops!("solving for literal makes no sense"))?,
            Expr::BinaryOp(x, op, y) => match (&**x, &**y) {
                (Expr::Literal(known), unknown) => match op {
                    Op::Add => unknown.solve_for(target - known),
                    Op::Sub => unknown.solve_for(known - target),
                    Op::Mul => unknown.solve_for(target / known),
                    Op::Div => unknown.solve_for(known / target),
                },
                (unknown, Expr::Literal(known)) => match op {
                    Op::Add => unknown.solve_for(target - known),
                    Op::Sub => unknown.solve_for(known + target),
                    Op::Mul => unknown.solve_for(target / known),
                    Op::Div => unknown.solve_for(known * target),
                },
                _ => Err(oops!("cannot solve for two unknowns"))?,
            },
        }
    }
}

impl Puzzle {
    fn eval(&self, node: &str) -> Result<i64, Oops> {
        Ok(
            match self
                .tree
                .get(node)
                .ok_or_else(|| oops!("no monkey {node}"))?
            {
                Monkey::Literal(c) => *c,
                Monkey::Add(x, y) => self.eval(x)? + self.eval(y)?,
                Monkey::Sub(x, y) => self.eval(x)? - self.eval(y)?,
                Monkey::Mul(x, y) => self.eval(x)? * self.eval(y)?,
                Monkey::Div(x, y) => self.eval(x)? / self.eval(y)?,
            },
        )
    }

    fn symbolic_eval(&self, node: &str) -> Result<Expr, Oops> {
        if node == "humn" {
            return Ok(Expr::new_human());
        }
        Ok(
            match self
                .tree
                .get(node)
                .ok_or_else(|| oops!("no monkey {node}"))?
            {
                Monkey::Literal(n) => Expr::new_literal(*n),
                Monkey::Add(x, y) => Self::simplify(Expr::new_add(
                    self.symbolic_eval(x)?,
                    self.symbolic_eval(y)?,
                )),
                Monkey::Sub(x, y) => Self::simplify(Expr::new_sub(
                    self.symbolic_eval(x)?,
                    self.symbolic_eval(y)?,
                )),
                Monkey::Mul(x, y) => Self::simplify(Expr::new_mul(
                    self.symbolic_eval(x)?,
                    self.symbolic_eval(y)?,
                )),
                Monkey::Div(x, y) => Self::simplify(Expr::new_div(
                    self.symbolic_eval(x)?,
                    self.symbolic_eval(y)?,
                )),
            },
        )
    }

    fn simplify(e: Expr) -> Expr {
        match e {
            Expr::Literal(_) => e,
            Expr::Human => e,
            Expr::BinaryOp(x, op, y) => match (&*x, &*y) {
                (Expr::Literal(x), Expr::Literal(y)) => Expr::new_literal(match op {
                    Op::Add => x + y,
                    Op::Sub => x - y,
                    Op::Mul => x * y,
                    Op::Div => x / y,
                }),
                _ => Expr::BinaryOp(x, op, y),
            },
        }
    }
}

impl FromStr for Puzzle {
    type Err = Oops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Puzzle {
            tree: s
                .lines()
                .map(|line| {
                    let (name, expr) = line
                        .split_once(": ")
                        .ok_or_else(|| oops!("bad line: {line}"))?;
                    Ok::<_, Oops>((name.to_string(), expr.parse()?))
                })
                .collect::<Result<_, _>>()?,
        })
    }
}

fn parse(input: &str) -> Result<Puzzle, Oops> {
    input.parse()
}

fn part1(puzzle: &Puzzle) -> Result<i64, Oops> {
    puzzle.eval("root")
}

fn part2(puzzle: &Puzzle) -> Result<i64, Oops> {
    let root = puzzle.tree.get("root").ok_or_else(|| oops!("no root"))?;
    let (lhs, rhs) = match root {
        Monkey::Add(x, y) => (x, y),
        Monkey::Sub(x, y) => (x, y),
        Monkey::Mul(x, y) => (x, y),
        Monkey::Div(x, y) => (x, y),
        _ => Err(oops!("root monkey not a binary operation"))?,
    };

    let (lhs, rhs) = (puzzle.symbolic_eval(lhs)?, puzzle.symbolic_eval(rhs)?);

    match (&lhs, &rhs) {
        (Expr::Literal(known), unknown) | (unknown, Expr::Literal(known)) => {
            unknown.solve_for(*known)
        }
        _ => Err(oops!("unexpected symbolic_eval result: {lhs:?} vs {rhs:?}"))?,
    }
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
        "root: pppw + sjmn\n",
        "dbpl: 5\n",
        "cczh: sllz + lgvd\n",
        "zczc: 2\n",
        "ptdq: humn - dvpt\n",
        "dvpt: 3\n",
        "lfqf: 4\n",
        "humn: 5\n",
        "ljgn: 2\n",
        "sjmn: drzm * dbpl\n",
        "sllz: 4\n",
        "pppw: cczh / lfqf\n",
        "lgvd: ljgn * ptdq\n",
        "drzm: hmdt - zczc\n",
        "hmdt: 32\n",
    );

    #[test]
    fn example1() {
        assert_eq!(152, part1(&parse(SAMPLE).unwrap()).unwrap());
    }

    #[test]
    fn example2() {
        assert_eq!(301, part2(&parse(SAMPLE).unwrap()).unwrap());
    }
}
