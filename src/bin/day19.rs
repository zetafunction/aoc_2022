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
use std::io::{self, Read};
use std::str::FromStr;

struct Blueprint {
    ore_robot_ore_cost: i32,
    clay_robot_ore_cost: i32,
    obsidian_robot_ore_cost: i32,
    obsidian_robot_clay_cost: i32,
    geode_robot_ore_cost: i32,
    geode_robot_obsidian_cost: i32,
}

#[derive(Clone, Copy)]
struct Resources {
    ore: i32,
    clay: i32,
    obsidian: i32,
    geode: i32,
}

impl Resources {
    fn new() -> Self {
        Resources {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
        }
    }

    fn consume(&self, ore: i32, clay: i32, obsidian: i32) -> Self {
        Resources {
            ore: self.ore - ore,
            clay: self.clay - clay,
            obsidian: self.obsidian - obsidian,
            geode: self.geode,
        }
    }

    fn maybe_build_ore_robot(&self, blueprint: &Blueprint) -> Option<Resources> {
        if self.ore < blueprint.ore_robot_ore_cost {
            None
        } else {
            Some(self.consume(blueprint.ore_robot_ore_cost, 0, 0))
        }
    }

    fn maybe_build_clay_robot(&self, blueprint: &Blueprint) -> Option<Resources> {
        if self.ore < blueprint.clay_robot_ore_cost {
            None
        } else {
            Some(self.consume(blueprint.clay_robot_ore_cost, 0, 0))
        }
    }

    fn maybe_build_obsidian_robot(&self, blueprint: &Blueprint) -> Option<Resources> {
        if self.ore < blueprint.obsidian_robot_ore_cost
            || self.clay < blueprint.obsidian_robot_clay_cost
        {
            None
        } else {
            Some(self.consume(
                blueprint.obsidian_robot_ore_cost,
                blueprint.obsidian_robot_clay_cost,
                0,
            ))
        }
    }

    fn maybe_build_geode_robot(&self, blueprint: &Blueprint) -> Option<Resources> {
        if self.ore < blueprint.geode_robot_ore_cost
            || self.obsidian < blueprint.geode_robot_obsidian_cost
        {
            None
        } else {
            Some(self.consume(
                blueprint.geode_robot_ore_cost,
                0,
                blueprint.geode_robot_obsidian_cost,
            ))
        }
    }

    fn collect(
        &self,
        ore_robots: i32,
        clay_robots: i32,
        obsidian_robots: i32,
        geode_robots: i32,
    ) -> Resources {
        Resources {
            ore: self.ore + ore_robots,
            clay: self.clay + clay_robots,
            obsidian: self.obsidian + obsidian_robots,
            geode: self.geode + geode_robots,
        }
    }
}

impl Blueprint {
    fn solve_for_max_geodes(
        &self,
        resources: Resources,
        ore_robots: i32,
        clay_robots: i32,
        obsidian_robots: i32,
        geode_robots: i32,
        time_left: i32,
    ) -> i32 {
        if time_left == 0 {
            return resources.geode;
        }

        let mut best = resources.geode;
        if let Some(resources) = resources.maybe_build_ore_robot(self) {
            let new_resources =
                resources.collect(ore_robots, clay_robots, obsidian_robots, geode_robots);
            let result = self.solve_for_max_geodes(
                new_resources,
                ore_robots + 1,
                clay_robots,
                obsidian_robots,
                geode_robots,
                time_left - 1,
            );
            best = std::cmp::max(best, result);
        }
        if let Some(resources) = resources.maybe_build_clay_robot(self) {
            let new_resources =
                resources.collect(ore_robots, clay_robots, obsidian_robots, geode_robots);
            let result = self.solve_for_max_geodes(
                new_resources,
                ore_robots,
                clay_robots + 1,
                obsidian_robots,
                geode_robots,
                time_left - 1,
            );
            best = std::cmp::max(best, result);
        }
        if let Some(resources) = resources.maybe_build_obsidian_robot(self) {
            let new_resources =
                resources.collect(ore_robots, clay_robots, obsidian_robots, geode_robots);
            let result = self.solve_for_max_geodes(
                new_resources,
                ore_robots,
                clay_robots,
                obsidian_robots + 1,
                geode_robots,
                time_left - 1,
            );
            best = std::cmp::max(best, result);
        }
        if let Some(resources) = resources.maybe_build_geode_robot(self) {
            let new_resources =
                resources.collect(ore_robots, clay_robots, obsidian_robots, geode_robots);
            let result = self.solve_for_max_geodes(
                new_resources,
                ore_robots,
                clay_robots,
                obsidian_robots,
                geode_robots + 1,
                time_left - 1,
            );
            best = std::cmp::max(best, result);
        }
        best = std::cmp::max(
            best,
            self.solve_for_max_geodes(
                resources.collect(ore_robots, clay_robots, obsidian_robots, geode_robots),
                ore_robots,
                clay_robots,
                obsidian_robots,
                geode_robots,
                time_left - 1,
            ),
        );
        return best;
    }
}

impl FromStr for Blueprint {
    type Err = Oops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words = s.split_whitespace().collect::<Vec<_>>();
        Ok(Blueprint {
            ore_robot_ore_cost: words[6].parse()?,
            clay_robot_ore_cost: words[12].parse()?,
            obsidian_robot_ore_cost: words[18].parse()?,
            obsidian_robot_clay_cost: words[21].parse()?,
            geode_robot_ore_cost: words[27].parse()?,
            geode_robot_obsidian_cost: words[30].parse()?,
        })
    }
}

struct Puzzle {
    blueprints: Vec<Blueprint>,
}

impl FromStr for Puzzle {
    type Err = Oops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Puzzle {
            blueprints: s
                .lines()
                .map(|s| s.parse())
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

fn parse(input: &str) -> Result<Puzzle, Oops> {
    input.parse()
}

fn part1(puzzle: &Puzzle) -> Result<i32, Oops> {
    Ok(puzzle
        .blueprints
        .iter()
        .enumerate()
        .map(|(i, blueprint)| {
            i as i32 * blueprint.solve_for_max_geodes(Resources::new(), 1, 0, 0, 0, 24)
        })
        .sum())
}

fn part2(puzzle: &Puzzle) -> Result<usize, Oops> {
    Ok(0)
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
        "Blueprint 1:",
        " Each ore robot costs 4 ore.",
        " Each clay robot costs 2 ore.",
        " Each obsidian robot costs 3 ore and 14 clay.",
        " Each geode robot costs 2 ore and 7 obsidian.\n",
        "Blueprint 2:",
        " Each ore robot costs 2 ore.",
        " Each clay robot costs 3 ore.",
        " Each obsidian robot costs 3 ore and 8 clay.",
        " Each geode robot costs 3 ore and 12 obsidian.\n",
    );

    #[test]
    fn example1() {
        assert_eq!(33, part1(&parse(SAMPLE).unwrap()).unwrap());
    }

    #[test]
    fn example2() {
        assert_eq!(2468013579, part2(&parse(SAMPLE).unwrap()).unwrap());
    }
}
