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
use std::collections::VecDeque;
use std::io::{self, Read};
use std::str::FromStr;

#[derive(Clone, Copy)]
enum Operand {
    Literal(usize),
    Old,
}

impl FromStr for Operand {
    type Err = Oops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "old" => Operand::Old,
            s => Operand::Literal(s.parse()?),
        })
    }
}

#[derive(Clone, Copy)]
enum Op {
    Add,
    Multiply,
}

impl FromStr for Op {
    type Err = Oops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "+" => Op::Add,
            "*" => Op::Multiply,
            _ => Err(oops!("bad op"))?,
        })
    }
}

#[derive(Clone)]
struct Monkey {
    items: VecDeque<usize>,
    op: Op,
    operand: Operand,
    divisor_test: usize,
    on_true: usize,
    on_false: usize,
}

impl FromStr for Monkey {
    type Err = Oops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Skip monkey index.
        let mut parser = s.lines().skip(1).map(str::trim);
        let items = parser
            .next()
            .and_then(|s| s.strip_prefix("Starting items: "))
            .ok_or_else(|| oops!("no starting items"))?
            .split(", ")
            .map(str::parse)
            .collect::<Result<_, _>>()?;
        let Some(op_str) = parser.next().and_then(|s| s.strip_prefix("Operation: new = old ")) else {
            return Err(oops!("no operation"));
        };
        let mut op_parser = op_str.split_whitespace();
        let op = op_parser
            .next()
            .ok_or_else(|| oops!("no operator"))?
            .parse()?;
        let operand = op_parser
            .next()
            .ok_or_else(|| oops!("no operand"))?
            .parse()?;
        let divisor_test = parser
            .next()
            .and_then(|s| s.strip_prefix("Test: divisible by "))
            .ok_or_else(|| oops!("no test"))?
            .parse()?;
        let on_true = parser
            .next()
            .and_then(|s| s.strip_prefix("If true: throw to monkey "))
            .ok_or_else(|| oops!("no if true"))?
            .parse()?;
        let on_false = parser
            .next()
            .and_then(|s| s.strip_prefix("If false: throw to monkey "))
            .ok_or_else(|| oops!("no if false"))?
            .parse()?;
        Ok(Monkey {
            items,
            op,
            operand,
            divisor_test,
            on_true,
            on_false,
        })
    }
}

#[derive(Clone)]
struct Puzzle {
    monkeys: Vec<Monkey>,
}

impl Puzzle {
    fn calculate_mbl<F: Fn(usize) -> usize>(&mut self, rounds: usize, mitigate_worry: &F) -> usize {
        let mut inspections = (0..rounds).fold(vec![0; self.monkeys.len()], |mut acc, _| {
            for i in 0..self.monkeys.len() {
                acc[i] += self.monkeys[i].items.len();
                self.process_monkey(i, mitigate_worry);
            }
            acc
        });

        inspections.sort_by(|a, b| b.cmp(a));
        inspections[0] * inspections[1]
    }

    fn process_monkey<F: Fn(usize) -> usize>(&mut self, i: usize, mitigate_worry: &F) {
        let op = self.monkeys[i].op;
        let operand = self.monkeys[i].operand;
        let apply_op = |current_worry| {
            let operand = match operand {
                Operand::Literal(x) => x,
                Operand::Old => current_worry,
            };
            match op {
                Op::Add => current_worry + operand,
                Op::Multiply => current_worry * operand,
            }
        };

        while let Some(worry_level) = self.monkeys[i].items.pop_front() {
            let worry_level = mitigate_worry(apply_op(worry_level));
            let target = if worry_level % self.monkeys[i].divisor_test == 0 {
                self.monkeys[i].on_true
            } else {
                self.monkeys[i].on_false
            };
            self.monkeys[target].items.push_back(worry_level);
        }
    }
}

impl FromStr for Puzzle {
    type Err = Oops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let monkeys = s
            .split("\n\n")
            .map(str::parse)
            .collect::<Result<Vec<Monkey>, _>>()?;
        Ok(Puzzle { monkeys })
    }
}

fn parse(input: &str) -> Result<Puzzle, Oops> {
    input.parse()
}

fn part1(puzzle: &Puzzle) -> usize {
    let mut puzzle = (*puzzle).clone();
    let mitigate_worry = |x| x / 3;
    puzzle.calculate_mbl(20, &mitigate_worry)
}

fn part2(puzzle: &Puzzle) -> usize {
    let mut puzzle = (*puzzle).clone();
    let factor: usize = puzzle.monkeys.iter().map(|x| x.divisor_test).product();
    let mitigate_worry = |x| x % factor;
    puzzle.calculate_mbl(10000, &mitigate_worry)
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
        "Monkey 0:\n",
        "  Starting items: 79, 98\n",
        "  Operation: new = old * 19\n",
        "  Test: divisible by 23\n",
        "    If true: throw to monkey 2\n",
        "    If false: throw to monkey 3\n",
        "\n",
        "Monkey 1:\n",
        "  Starting items: 54, 65, 75, 74\n",
        "  Operation: new = old + 6\n",
        "  Test: divisible by 19\n",
        "    If true: throw to monkey 2\n",
        "    If false: throw to monkey 0\n",
        "\n",
        "Monkey 2:\n",
        "  Starting items: 79, 60, 97\n",
        "  Operation: new = old * old\n",
        "  Test: divisible by 13\n",
        "    If true: throw to monkey 1\n",
        "    If false: throw to monkey 3\n",
        "\n",
        "Monkey 3:\n",
        "  Starting items: 74\n",
        "  Operation: new = old + 3\n",
        "  Test: divisible by 17\n",
        "    If true: throw to monkey 0\n",
        "    If false: throw to monkey 1\n",
    );

    #[test]
    fn example1() {
        assert_eq!(10605, part1(&parse(SAMPLE).unwrap()));
    }

    #[test]
    fn example2() {
        assert_eq!(2713310158, part2(&parse(SAMPLE).unwrap()));
    }
}
