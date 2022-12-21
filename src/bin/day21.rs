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

enum Op {
    Add(String, String),
    Sub(String, String),
    Mul(String, String),
    Div(String, String),
}

enum Expr {
    Literal(i64),
    Operation(Op),
}

impl FromStr for Expr {
    type Err = Oops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(n) = s.parse::<i64>() {
            return Ok(Expr::Literal(n));
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
        Ok(Expr::Operation(match op {
            "+" => Op::Add(first_operand, second_operand),
            "-" => Op::Sub(first_operand, second_operand),
            "*" => Op::Mul(first_operand, second_operand),
            "/" => Op::Div(first_operand, second_operand),
            _ => Err(oops!("bad operand {op}"))?,
        }))
    }
}
struct Puzzle {
    tree: HashMap<String, Expr>,
}

enum Sym {
    Human,
    Literal(i64),
    Add(Box<Sym>, Box<Sym>),
    Sub(Box<Sym>, Box<Sym>),
    Mul(Box<Sym>, Box<Sym>),
    Div(Box<Sym>, Box<Sym>),
}

impl Sym {
    fn evaluate(&self) -> Option<i64> {
        match self {
            Sym::Human => None,
            Sym::Literal(n) => Some(*n),
            Sym::Add(x, y) => Some(x.evaluate()? + y.evaluate()?),
            Sym::Sub(x, y) => Some(x.evaluate()? - y.evaluate()?),
            Sym::Mul(x, y) => Some(x.evaluate()? * y.evaluate()?),
            Sym::Div(x, y) => Some(x.evaluate()? / y.evaluate()?),
        }
    }

    fn simplify(&self) -> Sym {
        match self {
            Sym::Literal(n) => Sym::Literal(*n),
            Sym::Human => Sym::Human,
            Sym::Add(x, y) => {
                let x = x.simplify();
                let y = y.simplify();
                match (&x, &y) {
                    (Sym::Literal(x), Sym::Literal(y)) => Sym::Literal(x + y),
                    _ => Sym::Add(Box::new(x), Box::new(y)),
                }
            }
            Sym::Sub(x, y) => {
                let x = x.simplify();
                let y = y.simplify();
                match (&x, &y) {
                    (Sym::Literal(x), Sym::Literal(y)) => Sym::Literal(x - y),
                    _ => Sym::Sub(Box::new(x), Box::new(y)),
                }
            }
            Sym::Mul(x, y) => {
                let x = x.simplify();
                let y = y.simplify();
                match (&x, &y) {
                    (Sym::Literal(x), Sym::Literal(y)) => Sym::Literal(x * y),
                    _ => Sym::Mul(Box::new(x), Box::new(y)),
                }
            }
            Sym::Div(x, y) => {
                let x = x.simplify();
                let y = y.simplify();
                match (&x, &y) {
                    (Sym::Literal(x), Sym::Literal(y)) => Sym::Literal(x / y),
                    _ => Sym::Div(Box::new(x), Box::new(y)),
                }
            }
        }
    }

    fn solve_for(&self, target: i64) -> i64 {
        match self {
            Sym::Human => target,
            Sym::Add(x, y) => match (&**x, &**y) {
                (Sym::Literal(x), y) => y.solve_for(target - x),
                (x, Sym::Literal(y)) => x.solve_for(target - y),
                _ => panic!(),
            },
            Sym::Sub(x, y) => match (&**x, &**y) {
                (Sym::Literal(x), y) => y.solve_for(x - target),
                (x, Sym::Literal(y)) => x.solve_for(y + target),
                _ => panic!(),
            },
            Sym::Mul(x, y) => match (&**x, &**y) {
                (Sym::Literal(x), y) => y.solve_for(target / x),
                (x, Sym::Literal(y)) => x.solve_for(target / y),
                _ => panic!(),
            },
            Sym::Div(x, y) => match (&**x, &**y) {
                (Sym::Literal(x), y) => y.solve_for(x / target),
                (x, Sym::Literal(y)) => x.solve_for(y * target),
                _ => panic!(),
            },
            _ => panic!(),
        }
    }
}

impl Puzzle {
    fn eval(&self, node: &str) -> i64 {
        match self.tree.get(node).unwrap() {
            Expr::Literal(n) => *n,
            Expr::Operation(op) => match op {
                Op::Add(x, y) => self.eval(x) + self.eval(y),
                Op::Sub(x, y) => self.eval(x) - self.eval(y),
                Op::Mul(x, y) => self.eval(x) * self.eval(y),
                Op::Div(x, y) => self.eval(x) / self.eval(y),
            },
        }
    }

    fn eval2(&self, node: &str) -> i64 {
        let (expr1, expr2) = match self.tree.get(node).unwrap() {
            Expr::Operation(op) => match op {
                Op::Add(x, y) => (self.eval3(x), self.eval3(y)),
                Op::Sub(x, y) => (self.eval3(x), self.eval3(y)),
                Op::Mul(x, y) => (self.eval3(x), self.eval3(y)),
                Op::Div(x, y) => (self.eval3(x), self.eval3(y)),
            },
            _ => panic!(),
        };

        if let Some(result) = expr1.evaluate() {
            let simplified = expr2.simplify();
            simplified.solve_for(result)
        } else if let Some(result) = expr2.evaluate() {
            let simplified = expr1.simplify();
            simplified.solve_for(result)
        } else {
            panic!()
        }
    }

    fn eval3(&self, node: &str) -> Sym {
        if node == "humn" {
            Sym::Human
        } else {
            match self.tree.get(node).unwrap() {
                Expr::Literal(n) => Sym::Literal(*n),
                Expr::Operation(op) => match op {
                    Op::Add(x, y) => Sym::Add(Box::new(self.eval3(x)), Box::new(self.eval3(y))),
                    Op::Sub(x, y) => Sym::Sub(Box::new(self.eval3(x)), Box::new(self.eval3(y))),
                    Op::Mul(x, y) => Sym::Mul(Box::new(self.eval3(x)), Box::new(self.eval3(y))),
                    Op::Div(x, y) => Sym::Div(Box::new(self.eval3(x)), Box::new(self.eval3(y))),
                },
            }
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

fn part1(puzzle: &Puzzle) -> i64 {
    puzzle.eval("root")
}

fn part2(puzzle: &Puzzle) -> i64 {
    puzzle.eval2("root")
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
        assert_eq!(152, part1(&parse(SAMPLE).unwrap()));
    }

    #[test]
    fn example2() {
        assert_eq!(301, part2(&parse(SAMPLE).unwrap()));
    }
}
