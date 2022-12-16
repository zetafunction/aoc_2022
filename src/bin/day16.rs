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
    flows: HashMap<String, i32>,
    distances: HashMap<String, HashMap<String, i32>>,
}

struct State {
    // Only tracks valves with non-zero flow.
    visited: HashSet<String>,
    remaining: i32,
}

impl State {
    fn new() -> Self {
        Self {
            visited: HashSet::new(),
            remaining: 30,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Goal<'a> {
    valve: &'a str,
    left: i32,
}

impl<'a> Goal<'a> {
    fn new(valve: &'a str, left: i32) -> Self {
        Goal { valve, left }
    }

    fn set_new_target(&self, p: &Puzzle, valve: &'a str) -> Self {
        Goal::new(valve, p.distance_between(self.valve, valve) + 1)
    }

    fn next(&self, x: i32) -> Self {
        assert!(self.left >= x);
        Goal::new(self.valve, self.left - x)
    }
}

impl Puzzle {
    fn try_path(&self, current: &str, state: &mut State) -> i32 {
        // Only need to visit unvisited valves with non-zero flow.
        let target_valves = self
            .flows
            .iter()
            .filter(|(name, &flow)| flow > 0 && !state.visited.contains(*name))
            .map(|(name, _)| name)
            .collect::<Vec<_>>();

        // No more non-zero valves, just return.
        if target_valves.is_empty() {
            return 0;
        }

        let mut best = 0;
        for v in target_valves {
            let distance_to_v = self.distance_between(current, v) + 1;
            if distance_to_v >= state.remaining {
                continue;
            }
            state.remaining -= distance_to_v;
            state.visited.insert(v.clone());
            let flow_from_path = self.try_path(v, state);
            let candidate = flow_from_path + state.remaining * self.flow_for(v);
            best = std::cmp::max(best, candidate);
            state.visited.remove(v);
            state.remaining += distance_to_v;
        }
        best
    }

    fn find_path<'a, const N: usize>(
        &self,
        targets: &mut Vec<&'a str>,
        assigned: usize,
        best_seen: &mut HashMap<Vec<&'a str>, i32>,
        goals: &[Goal; N],
        so_far: i32,
        remaining_time: i32,
    ) -> i32 {
        // Invariant: when calling this function, at least one goal must have been reached, i.e.
        // goal.left == 0.

        // All valves have been visited.
        if assigned == targets.len() {
            return so_far;
        }

        // Calculate an upper bound for the maximum possible pressure that can still be relieved.
        let max_possible_remaining = &targets[assigned..]
            .iter()
            .map(|r| {
                let min_time = goals
                    .iter()
                    .map(|g| g.left + self.distance_between(g.valve, r) + 1)
                    .min()
                    .unwrap();
                std::cmp::max(remaining_time - min_time, 0) * self.flow_for(*r)
            })
            .sum();

        // The best seen key is the remaining unassigned valves ordered lexicographically.
        let mut best_seen_key = targets[assigned..].to_vec();
        best_seen_key.sort();
        if let Some(best_seen) = best_seen.get(&best_seen_key) {
            if *best_seen > max_possible_remaining + so_far {
                // There is already another path that uses the remaining unassigned valves in a
                // more efficient way: no need to waste more time exploring this branch.
                return 0;
            }
        }

        let next_goal_idx = goals
            .iter()
            .enumerate()
            .find_map(|(i, g)| if g.left == 0 { Some(i) } else { None })
            .expect(&format!("no goals reached! {:?}", goals));

        let mut best = so_far;

        for x in assigned..targets.len() {
            targets.swap(x, assigned);

            let mut goals = *goals;
            goals[next_goal_idx] = goals[next_goal_idx].set_new_target(self, targets[assigned]);
            let time_to_next_goal = goals[next_goal_idx].left;
            if time_to_next_goal > remaining_time {
                targets.swap(x, assigned);
                continue;
            }
            let advance_by = goals.iter().map(|g| g.left).min().unwrap();
            for g in goals.iter_mut() {
                *g = g.next(advance_by);
            }
            let result = self.find_path(
                targets,
                assigned + 1,
                best_seen,
                &goals,
                so_far + (remaining_time - time_to_next_goal) * self.flow_for(targets[assigned]),
                remaining_time - advance_by,
            );

            best = std::cmp::max(best, result);

            targets.swap(x, assigned);
        }

        for i in assigned..targets.len() {
            let mut best_seen_key = targets[i..].to_vec();
            best_seen_key.sort();
            best_seen
                .entry(best_seen_key)
                .and_modify(|current| *current = std::cmp::max(*current, best))
                .or_insert(best);
        }

        best
    }

    // Note: this counts physical distance and does not include the time to activate a valve.
    fn distance_between(&self, from: &str, to: &str) -> i32 {
        return *self.distances.get(from).unwrap().get(to).unwrap();
    }

    fn flow_for(&self, valve: &str) -> i32 {
        *self.flows.get(valve).unwrap()
    }

    fn calculate_distances(
        valves: &HashMap<String, Valve>,
    ) -> HashMap<String, HashMap<String, i32>> {
        let mut distances = HashMap::new();
        for from in valves.keys() {
            let mut frontier = VecDeque::<&str>::new();
            let mut from_distances = HashMap::new();
            frontier.push_back(from);
            while let Some(current) = frontier.pop_front() {
                let next_cost = from_distances.get(current).unwrap_or(&0) + 1;
                let unvisited_neighbors = valves
                    .get(current)
                    .unwrap()
                    .next
                    .iter()
                    .filter(|x| !from_distances.contains_key(*x))
                    .collect::<Vec<_>>();
                for n in unvisited_neighbors {
                    frontier.push_back(n);
                    from_distances.insert(n.clone(), next_cost);
                }
            }
            distances.insert(from.clone(), from_distances);
        }
        distances
    }
}

impl FromStr for Puzzle {
    type Err = Oops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let valves = s
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
            .collect::<Result<HashMap<_, _>, _>>()?;
        let distances = Self::calculate_distances(&valves);
        let flows = valves
            .into_iter()
            .filter_map(|(k, v)| if v.flow > 0 { Some((k, v.flow)) } else { None })
            .collect();
        Ok(Puzzle { flows, distances })
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
    let mut targets = puzzle.flows.keys().map(String::as_str).collect();
    Ok(puzzle.find_path(
        &mut targets,
        0,
        &mut HashMap::new(),
        &[Goal::new("AA", 0), Goal::new("AA", 0)],
        0,
        26,
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
