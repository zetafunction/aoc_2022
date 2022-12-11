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

#[derive(Clone, Copy)]
enum Op {
    Multiply(Operand),
    Add(Operand),
}

#[derive(Clone)]
struct Monkey {
    items: VecDeque<usize>,
    op: Op,
    test_operand: usize,
    on_true: usize,
    on_false: usize,
    inspections: usize,
}

fn parse_operand(s: Option<&str>) -> Result<Operand, Oops> {
    let Some(s) = s else {
        return Err(oops!("illegal operand"));
    };
    if s == "old" {
        Ok(Operand::Old)
    } else {
        Ok(Operand::Literal(s.parse()?))
    }
}

impl FromStr for Monkey {
    type Err = Oops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parser = s.lines().map(str::trim);
        // Skip monkey index.
        parser.next();
        let Some(items_line) = parser.next() else {
            return Err(oops!("no starting items"));
        };
        let Some(items_list) = items_line.strip_prefix("Starting items: ") else {
            return Err(oops!("invalid starting items format"));
        };
        let items = items_list
            .split(", ")
            .map(|x| x.parse())
            .collect::<Result<_, _>>()?;
        let Some(op_line) = parser.next() else {
            return Err(oops!("no op line"));
        };
        let Some(op_str) = op_line.strip_prefix("Operation: new = old ") else {
            return Err(oops!("invalid op line format"));
        };
        let mut op_parser = op_str.split_whitespace();
        let op = match op_parser.next() {
            Some("*") => Op::Multiply(parse_operand(op_parser.next())?),
            Some("+") => Op::Add(parse_operand(op_parser.next())?),
            _ => Err(oops!("illegal operand"))?,
        };
        let Some(test_line) = parser.next() else {
            return Err(oops!("no test line"));
        };
        let Some(test_operand) = test_line.strip_prefix("Test: divisible by ") else {
            return Err(oops!("illegal test operand"));
        };
        let test_operand = test_operand.parse()?;
        let Some(true_line) = parser.next() else {
            return Err(oops!("no true line"));
        };
        let Some(true_str) = true_line.strip_prefix("If true: throw to monkey ") else {
            return Err(oops!("invalid true line"));
        };
        let on_true = true_str.parse()?;
        let Some(false_line) = parser.next() else {
            return Err(oops!("no false line"));
        };
        let Some(false_str) = false_line.strip_prefix("If false: throw to monkey ") else {
            return Err(oops!("invalid false line"));
        };
        let on_false = false_str.parse()?;
        Ok(Monkey {
            items,
            op,
            test_operand,
            on_true,
            on_false,
            inspections: 0,
        })
    }
}

#[derive(Clone)]
struct Puzzle {
    monkeys: Vec<Monkey>,
    factor: usize,
}

impl Puzzle {
    fn evaluate_monkey(&mut self, i: usize) {
        let op = self.monkeys[i].op;
        while let Some(worry_level) = self.monkeys[i].items.pop_front() {
            let worry_level = match op {
                Op::Add(operand) => match operand {
                    Operand::Old => worry_level + worry_level,
                    Operand::Literal(x) => worry_level + x,
                },
                Op::Multiply(operand) => match operand {
                    Operand::Old => worry_level * worry_level,
                    Operand::Literal(x) => worry_level * x,
                },
            } / 3;
            self.monkeys[i].inspections += 1;
            let target = if worry_level % self.monkeys[i].test_operand == 0 {
                self.monkeys[i].on_true
            } else {
                self.monkeys[i].on_false
            };
            self.monkeys[target].items.push_back(worry_level);
        }
    }

    fn evaluate_monkey2(&mut self, i: usize) {
        let op = self.monkeys[i].op;
        while let Some(worry_level) = self.monkeys[i].items.pop_front() {
            let worry_level = match op {
                Op::Add(operand) => match operand {
                    Operand::Old => worry_level + worry_level,
                    Operand::Literal(x) => worry_level + x,
                },
                Op::Multiply(operand) => match operand {
                    Operand::Old => worry_level * worry_level,
                    Operand::Literal(x) => worry_level * x,
                },
            } % self.factor;
            self.monkeys[i].inspections += 1;
            let target = if worry_level % self.monkeys[i].test_operand == 0 {
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
            .map(|x| x.parse())
            .collect::<Result<Vec<Monkey>, _>>()?;
        let factor = monkeys.iter().map(|x| x.test_operand).product();
        Ok(Puzzle { monkeys, factor })
    }
}

fn parse(input: &str) -> Result<Puzzle, Oops> {
    input.parse()
}

fn part1(puzzle: &Puzzle) -> usize {
    let mut puzzle = (*puzzle).clone();
    let monkey_count = puzzle.monkeys.len();
    for _ in 0..20 {
        for i in 0..monkey_count {
            puzzle.evaluate_monkey(i);
        }
    }

    let mut counts = puzzle
        .monkeys
        .iter()
        .map(|x| x.inspections)
        .collect::<Vec<_>>();
    counts.sort_by(|a, b| b.cmp(a));
    counts[0] * counts[1]
}

fn part2(puzzle: &Puzzle) -> usize {
    let mut puzzle = (*puzzle).clone();
    let monkey_count = puzzle.monkeys.len();
    for _ in 0..10000 {
        for i in 0..monkey_count {
            puzzle.evaluate_monkey2(i);
        }
    }

    let mut counts = puzzle
        .monkeys
        .iter()
        .map(|x| x.inspections)
        .collect::<Vec<_>>();
    counts.sort_by(|a, b| b.cmp(a));
    counts[0] * counts[1]
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
