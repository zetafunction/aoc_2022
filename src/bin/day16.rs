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

#[derive(Debug)]
struct Valve {
    flow: i32,
    next: Vec<String>,
}

#[derive(Debug)]
struct Puzzle {
    valves: HashMap<String, Valve>,
    distance_table: HashMap<String, HashMap<String, i32>>,
}

struct State {
    // Only tracks valves with non-zero flow.
    visited: HashSet<String>,
    remaining: i32,
}

#[derive(Copy, Clone, Debug)]
struct Goal<'a> {
    target: &'a str,
    left: i32,
}

impl<'a> Goal<'a> {
    fn new(target: &'a str, left: i32) -> Self {
        Goal { target, left }
    }

    fn advance(&self, by: i32) -> Goal {
        Goal {
            target: self.target,
            left: self.left - by,
        }
    }
}

impl State {
    fn new() -> Self {
        Self {
            visited: HashSet::new(),
            remaining: 30,
        }
    }
}

impl Puzzle {
    fn try_path(&self, current: &str, state: &mut State) -> i32 {
        // Only need to visit unvisited valves with non-zero flow.
        let target_valves = self
            .valves
            .iter()
            .filter(|(name, valve)| valve.flow > 0 && !state.visited.contains(*name))
            .map(|(name, _)| name)
            .collect::<Vec<_>>();

        // No more non-zero valves, just return.
        if target_valves.is_empty() {
            return 0;
        }

        // println!("Target valves: {:?}", target_valves);

        let mut best = 0;
        for v in target_valves {
            let distance_to_v = self.distance_table.get(current).unwrap().get(v).unwrap() + 1;
            if distance_to_v >= state.remaining {
                continue;
            }
            state.remaining -= distance_to_v;
            state.visited.insert(v.clone());
            let flow_from_path = self.try_path(v, state);
            /*
            println!(
                "could get {} from opening valve {} at {}",
                state.remaining * self.valves.get(v).unwrap().flow,
                v,
                state.remaining
            );
            */
            let candidate = flow_from_path + state.remaining * self.valves.get(v).unwrap().flow;
            best = std::cmp::max(best, candidate);
            state.visited.remove(v);
            state.remaining += distance_to_v;
        }
        best
    }

    fn try_elephant_path(&self, state: &mut State, human_goal: &Goal, elephant_goal: &Goal) -> i32 {
        assert!(state.remaining >= 0);

        if human_goal.left > 0 && elephant_goal.left > 0 {
            let advance_by = std::cmp::min(human_goal.left, elephant_goal.left);
            state.remaining -= advance_by;
            let x = self.try_elephant_path(
                state,
                &human_goal.advance(advance_by),
                &elephant_goal.advance(advance_by),
            );
            state.remaining += advance_by;
            return x;
        }

        // Only need to visit unvisited valves with non-zero flow.
        let target_valves = self
            .valves
            .iter()
            .filter(|(name, valve)| valve.flow > 0 && !state.visited.contains(*name))
            .map(|(name, _)| name)
            .collect::<Vec<_>>();

        // No more non-zero valves, just return.
        if target_valves.is_empty() {
            return (state.remaining - human_goal.left)
                * self.valves.get(human_goal.target).unwrap().flow
                + (state.remaining - elephant_goal.left)
                    * self.valves.get(elephant_goal.target).unwrap().flow;
        }

        let mut best = 0;

        if human_goal.left == 0 {
            let base_flow = state.remaining * self.valves.get(human_goal.target).unwrap().flow;
            for v in &target_valves {
                let distance = self
                    .distance_table
                    .get(human_goal.target)
                    .unwrap()
                    .get(*v)
                    .unwrap()
                    + 1;
                if distance > state.remaining {
                    continue;
                }
                state.visited.insert((*v).clone());
                let result = base_flow
                    + self.try_elephant_path(
                        state,
                        &Goal {
                            target: *v,
                            left: distance,
                        },
                        elephant_goal,
                    );
                best = std::cmp::max(result, best);
                state.visited.remove(*v);
            }
        } else if elephant_goal.left == 0 {
            let base_flow = state.remaining * self.valves.get(elephant_goal.target).unwrap().flow;
            for v in &target_valves {
                let distance = self
                    .distance_table
                    .get(elephant_goal.target)
                    .unwrap()
                    .get(*v)
                    .unwrap()
                    + 1;
                if distance > state.remaining {
                    continue;
                }
                state.visited.insert((*v).clone());
                let result = base_flow
                    + self.try_elephant_path(
                        state,
                        human_goal,
                        &Goal {
                            target: *v,
                            left: distance,
                        },
                    );
                state.visited.remove(*v);
                best = std::cmp::max(result, best);
            }
        }
        best
    }

    fn find_distances(&self, from: &str) -> HashMap<String, i32> {
        let mut frontier = VecDeque::new();
        let mut visited = HashMap::new();
        frontier.push_back(from.to_string());
        visited.insert(from.to_string(), 0);
        while !frontier.is_empty() {
            let current = frontier.pop_front().unwrap();
            let next_cost = visited.get(&current).unwrap() + 1;
            let nexts: Vec<_> = self
                .valves
                .get(&current)
                .unwrap()
                .next
                .iter()
                .filter(|x| !visited.contains_key(*x))
                .collect();

            for next in nexts {
                frontier.push_back(next.clone());
                visited.insert(next.clone(), next_cost);
            }
        }
        visited
    }
}

impl FromStr for Puzzle {
    type Err = Oops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut puzzle = Puzzle {
            valves: s
                .lines()
                .map(|line| {
                    let line = line
                        .strip_prefix("Valve ")
                        .ok_or_else(|| oops!("bad format {}", line))?;
                    let (name, line) = line
                        .split_once(" has flow rate=")
                        .ok_or_else(|| oops!("bad format {}", line))?;
                    let (flow, valves) = if line.contains("tunnels") {
                        line.split_once("; tunnels lead to valves ")
                            .ok_or_else(|| oops!("bad format {}", line))?
                    } else {
                        line.split_once("; tunnel leads to valve ")
                            .ok_or_else(|| oops!("bad format {}", line))?
                    };
                    let valves = valves.split(", ");
                    Ok::<_, Oops>((
                        name.to_string(),
                        Valve {
                            flow: flow.parse()?,
                            next: valves.map(str::to_string).collect(),
                        },
                    ))
                })
                .collect::<Result<_, _>>()?,
            distance_table: HashMap::new(),
        };
        puzzle.distance_table = puzzle
            .valves
            .iter()
            .map(|(name, _)| (name.clone(), puzzle.find_distances(name)))
            .collect();
        Ok(puzzle)
    }
}

fn parse(input: &str) -> Result<Puzzle, Oops> {
    input.parse()
}

fn part1(puzzle: &Puzzle) -> Result<usize, Oops> {
    let mut state = State::new();
    Ok(puzzle.try_path("AA", &mut state) as usize)
}

fn part2(puzzle: &Puzzle) -> Result<usize, Oops> {
    let mut state = State::new();
    state.remaining = 26;
    Ok(puzzle.try_elephant_path(
        &mut state,
        &Goal {
            target: "AA",
            left: 0,
        },
        &Goal {
            target: "AA",
            left: 0,
        },
    ) as usize)
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
        "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB\n",
        "Valve BB has flow rate=13; tunnels lead to valves CC, AA\n",
        "Valve CC has flow rate=2; tunnels lead to valves DD, BB\n",
        "Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE\n",
        "Valve EE has flow rate=3; tunnels lead to valves FF, DD\n",
        "Valve FF has flow rate=0; tunnels lead to valves EE, GG\n",
        "Valve GG has flow rate=0; tunnels lead to valves FF, HH\n",
        "Valve HH has flow rate=22; tunnel leads to valve GG\n",
        "Valve II has flow rate=0; tunnels lead to valves AA, JJ\n",
        "Valve JJ has flow rate=21; tunnel leads to valve II\n",
    );

    #[test]
    fn example1() {
        assert_eq!(1651, part1(&parse(SAMPLE).unwrap()).unwrap());
    }

    #[test]
    fn example2() {
        assert_eq!(1707, part2(&parse(SAMPLE).unwrap()).unwrap());
    }
}
