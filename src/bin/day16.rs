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
use std::collections::VecDeque;
use std::io::{self, Read};
use std::ops::BitOr;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Label(u64);

#[derive(Debug)]
struct Valve {
    flow: i32,
    next: Vec<Label>,
}

#[derive(Debug)]
struct Puzzle {
    flows: HashMap<Label, i32>,
    distances: HashMap<(Label, Label), i32>,
}

#[derive(Clone, Copy, Debug)]
struct Goal {
    valve: Label,
    left: i32,
}

impl Goal {
    fn new(valve: Label, left: i32) -> Self {
        Goal { valve, left }
    }

    fn set_new_target(&self, p: &Puzzle, valve: Label) -> Self {
        Goal::new(valve, p.distance_between(self.valve, valve) + 1)
    }

    fn next(&self, x: i32) -> Self {
        assert!(self.left >= x);
        Goal::new(self.valve, self.left - x)
    }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct LabelSet(u64);

impl BitOr<Label> for LabelSet {
    type Output = Self;

    fn bitor(self, rhs: Label) -> Self {
        Self(self.0 | rhs.0)
    }
}

fn labels_to_set(labels: &[Label]) -> LabelSet {
    labels.iter().fold(LabelSet(0), |acc, &label| acc | label)
}

impl Puzzle {
    fn find_path<const N: usize>(
        &self,
        targets: &mut Vec<Label>,
        assigned: usize,
        best_seen: &mut HashMap<LabelSet, i32>,
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
                    .map(|g| g.left + self.distance_between(g.valve, *r) + 1)
                    .min()
                    .unwrap();
                std::cmp::max(remaining_time - min_time, 0) * self.flow_for(*r)
            })
            .sum();

        let best_seen_key = labels_to_set(&targets[assigned..]);
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
            .expect("goal should have been reached");

        let mut best = so_far;

        // targets[..assigned] have been visited and activated. Permute through all possible
        // remaining combinations of targets[assigned..].
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

        best_seen
            .entry(best_seen_key)
            .and_modify(|current| *current = std::cmp::max(*current, best))
            .or_insert(best);

        best
    }

    // Note: this counts physical distance and does not include the time to activate a valve.
    fn distance_between(&self, from: Label, to: Label) -> i32 {
        *self
            .distances
            .get(&(from, to))
            .expect("distances should not be missing entries")
        //            .expect(&format!("{:?} <-> {:?} not in distance table", from, to))
    }

    fn flow_for(&self, valve: Label) -> i32 {
        *self
            .flows
            .get(&valve)
            .expect("flows should not be missing entries")
    }

    fn calculate_distances(valves: &HashMap<Label, Valve>) -> HashMap<(Label, Label), i32> {
        let mut distances = HashMap::new();
        for from in valves.keys() {
            let mut frontier = VecDeque::new();
            frontier.push_back(from);
            while let Some(to) = frontier.pop_front() {
                let next_cost = distances.get(&(*from, *to)).unwrap_or(&0) + 1;
                let unvisited_neighbors = valves
                    .get(to)
                    .unwrap()
                    .next
                    .iter()
                    .filter(|x| !distances.contains_key(&(*from, **x)))
                    .collect::<Vec<_>>();
                for n in unvisited_neighbors {
                    frontier.push_back(n);
                    distances.insert((*from, *n), next_cost);
                }
            }
        }
        distances
    }
}

struct StringToLabelMapper {
    mapping: HashMap<String, Label>,
    next_bit: u64,
}

impl StringToLabelMapper {
    fn new() -> Self {
        StringToLabelMapper {
            mapping: HashMap::new(),
            next_bit: 1,
        }
    }
    fn get_label(&mut self, s: &str) -> Label {
        *self.mapping.entry(s.to_string()).or_insert_with(|| {
            let next_bit = self.next_bit;
            self.next_bit *= 2;
            Label(next_bit)
        })
    }
}

impl FromStr for Puzzle {
    type Err = Oops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut mapper = StringToLabelMapper::new();
        //  Reserve the first bit as a placeholder for the initial node, even though it has no flow.
        mapper.get_label("AA");
        let valves = s
            .lines()
            .map(|line| {
                let line = line
                    .strip_prefix("Valve ")
                    .ok_or_else(|| oops!("bad format {}", line))?;
                let (name, line) = line
                    .split_once(" has flow rate=")
                    .ok_or_else(|| oops!("bad format {}", line))?;
                let name = mapper.get_label(name);
                let (flow, valves) = if line.contains("tunnels") {
                    line.split_once("; tunnels lead to valves ")
                        .ok_or_else(|| oops!("bad format {}", line))?
                } else {
                    line.split_once("; tunnel leads to valve ")
                        .ok_or_else(|| oops!("bad format {}", line))?
                };
                let valves = valves.split(", ");
                Ok::<_, Oops>((
                    name,
                    Valve {
                        flow: flow.parse()?,
                        next: valves.map(|s| mapper.get_label(s)).collect(),
                    },
                ))
            })
            .collect::<Result<HashMap<_, _>, _>>()?;
        let distances = Self::calculate_distances(&valves);
        // Drop any remaining nodes with zero flows, as they will never be targetted.
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

fn part1(puzzle: &Puzzle) -> i32 {
    let mut targets = puzzle.flows.keys().copied().collect();
    puzzle.find_path(
        &mut targets,
        0,
        &mut HashMap::new(),
        &[Goal::new(Label(1), 0)],
        0,
        30,
    )
}

fn part2(puzzle: &Puzzle) -> i32 {
    let mut targets = puzzle.flows.keys().copied().collect();
    puzzle.find_path(
        &mut targets,
        0,
        &mut HashMap::new(),
        &[Goal::new(Label(1), 0), Goal::new(Label(1), 0)],
        0,
        26,
    )
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
        assert_eq!(1651, part1(&parse(SAMPLE).unwrap()));
    }

    #[test]
    fn example2() {
        assert_eq!(1707, part2(&parse(SAMPLE).unwrap()));
    }
}
