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
use std::collections::BTreeMap;
use std::collections::BTreeSet;
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
    valves: BTreeMap<String, Valve>,
}

struct State {
    // Only tracks valves with non-zero flow.
    visited: BTreeSet<String>,
    remaining: i32,
}

impl State {
    fn new() -> Self {
        Self {
            visited: BTreeSet::new(),
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

        let distances = self.find_distances(current);
        let mut best = 0;
        for v in target_valves {
            let distance_to_v = *distances.get(v).unwrap() + 1;
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

    fn try_elephant_path(
        &self,
        state: &mut State,
        human_goal: Option<(&str, i32)>,
        elephant_goal: Option<(&str, i32)>,
        depth: i32,
    ) -> i32 {
        if state.remaining <= 0 {
            println!("huh");
            return 0;
        }

        match (human_goal, elephant_goal) {
            (Some((h_target, h_left)), Some((e_target, e_left))) if h_left > 0 && e_left > 0 => {
                let min_left = std::cmp::min(h_left, e_left);
                println!(
                    "{:?} {:?} (FF {}, remain: {})",
                    human_goal, elephant_goal, min_left, state.remaining
                );
                state.remaining -= min_left;
                let result = self.try_elephant_path(
                    state,
                    Some((h_target, h_left - min_left)),
                    Some((e_target, e_left - min_left)),
                    depth,
                );
                state.remaining += min_left;
                return result;
            }
            _ => (),
        }

        // Only need to visit unvisited valves with non-zero flow.
        let target_valves = self
            .valves
            .iter()
            .filter(|(name, valve)| valve.flow > 0 && !state.visited.contains(*name))
            .map(|(name, _)| name)
            .collect::<Vec<_>>();

        // println!("Target valves: {:?}", target_valves);
        let human_reached_goal = human_goal.is_some() && human_goal.unwrap().1 == 0;
        let human_can_move = human_reached_goal;
        let elephant_reached_goal = elephant_goal.is_some() && elephant_goal.unwrap().1 == 0;
        let elephant_can_move = elephant_reached_goal;

        // No more non-zero valves, just return.
        if target_valves.is_empty() {
            let human_goal = human_goal.unwrap();
            let elephant_goal = elephant_goal.unwrap();
            println!("Wrap up: {:?} {:?}", human_goal, elephant_goal);
            let r = (state.remaining - human_goal.1) * self.valves.get(human_goal.0).unwrap().flow
                + (state.remaining - elephant_goal.1)
                    * self.valves.get(elephant_goal.0).unwrap().flow;
            println!("Returning {}", r);
            return r;
        }

        let mut best = 0;
        for v in &target_valves {
            state.visited.insert((*v).clone());
            if human_can_move {
                let distances = self.find_distances(human_goal.unwrap().0);
                let distance_to_v = *distances.get(*v).unwrap() + 1;
                if distance_to_v >= state.remaining {
                    continue;
                }
                let result = self.try_elephant_path(
                    state,
                    Some((*v, distance_to_v)),
                    elephant_goal,
                    depth + 1,
                );
                let candidate =
                    result + state.remaining * self.valves.get(human_goal.unwrap().0).unwrap().flow;
                best = std::cmp::max(best, candidate);
            } else {
                assert!(elephant_can_move);
                let distances = self.find_distances(elephant_goal.unwrap().0);
                let distance_to_v = *distances.get(*v).unwrap() + 1;
                if distance_to_v >= state.remaining {
                    continue;
                }
                let result =
                    self.try_elephant_path(state, human_goal, Some((*v, distance_to_v)), depth + 1);
                let candidate = result
                    + state.remaining * self.valves.get(elephant_goal.unwrap().0).unwrap().flow;
                best = std::cmp::max(best, candidate);
            }
            state.visited.remove(*v);
        }
        best
    }

    fn find_distances(&self, from: &str) -> BTreeMap<String, i32> {
        let mut frontier = VecDeque::new();
        let mut visited = BTreeMap::new();
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
        // println!("{:?}", visited);
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
        };
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
    Ok(puzzle.try_elephant_path(&mut state, Some(("AA", 0)), Some(("AA", 0)), 0) as usize)
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
        assert_eq!(3579124689, part1(&parse(SAMPLE).unwrap()).unwrap());
    }

    #[test]
    fn example2() {
        assert_eq!(2468013579, part2(&parse(SAMPLE).unwrap()).unwrap());
    }
}
