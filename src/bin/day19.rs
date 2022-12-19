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

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
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

    fn collect(&self, robots: &Robots) -> Resources {
        Resources {
            ore: self.ore + robots.ore,
            clay: self.clay + robots.clay,
            obsidian: self.obsidian + robots.obsidian,
            geode: self.geode + robots.geode,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Robots {
    ore: i32,
    clay: i32,
    obsidian: i32,
    geode: i32,
}

impl Robots {
    fn new() -> Self {
        Robots {
            ore: 1,
            clay: 0,
            obsidian: 0,
            geode: 0,
        }
    }

    fn add_ore(&self) -> Self {
        Robots {
            ore: self.ore + 1,
            clay: self.clay,
            obsidian: self.obsidian,
            geode: self.geode,
        }
    }

    fn add_clay(&self) -> Self {
        Robots {
            ore: self.ore,
            clay: self.clay + 1,
            obsidian: self.obsidian,
            geode: self.geode,
        }
    }

    fn add_obsidian(&self) -> Self {
        Robots {
            ore: self.ore,
            clay: self.clay,
            obsidian: self.obsidian + 1,
            geode: self.geode,
        }
    }

    fn add_geode(&self) -> Self {
        Robots {
            ore: self.ore,
            clay: self.clay,
            obsidian: self.obsidian,
            geode: self.geode + 1,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Stuff {
    resources: Resources,
    robots: Robots,
}

#[derive(Debug)]
struct Blueprint {
    ore_robot_ore_cost: i32,
    clay_robot_ore_cost: i32,
    obsidian_robot_ore_cost: i32,
    obsidian_robot_clay_cost: i32,
    geode_robot_ore_cost: i32,
    geode_robot_obsidian_cost: i32,
    construct_ore_robot_cycle: i32,
}

impl Blueprint {
    fn solve_for_max_geodes(
        &self,
        resources: Resources,
        robots: Robots,
        time_left: i32,
        seen: &mut HashMap<Robots, i32>,
        did_build: bool,
    ) -> i32 {
        if time_left == 0 {
            return resources.geode;
        }

        if did_build {
            if let Some(so_far) = seen.get_mut(&robots) {
                if *so_far > time_left {
                    return 0;
                }
                *so_far = time_left;
            } else {
                seen.insert(robots, time_left);
            }
        }

        let mut could_build_geode_or_obsidian = false;
        let mut best = resources.geode;
        if let Some(resources) = resources.maybe_build_geode_robot(self) {
            could_build_geode_or_obsidian = true;
            let new_resources = resources.collect(&robots);
            let result = self.solve_for_max_geodes(
                new_resources,
                robots.add_geode(),
                time_left - 1,
                seen,
                true,
            );
            best = std::cmp::max(best, result);
        } else {
            if let Some(resources) = resources.maybe_build_ore_robot(self) {
                // println!("building ore robot at {}", time_left);
                let new_resources = resources.collect(&robots);
                let result = self.solve_for_max_geodes(
                    new_resources,
                    robots.add_ore(),
                    time_left - 1,
                    seen,
                    true,
                );
                best = std::cmp::max(best, result);
            }
            if (robots.clay as f32 / robots.ore as f32)
                < (self.obsidian_robot_clay_cost as f32 / self.obsidian_robot_ore_cost as f32)
            {
                if let Some(resources) = resources.maybe_build_clay_robot(self) {
                    // println!("building clay robot at {}", time_left);
                    let new_resources = resources.collect(&robots);
                    let result = self.solve_for_max_geodes(
                        new_resources,
                        robots.add_clay(),
                        time_left - 1,
                        seen,
                        true,
                    );
                    best = std::cmp::max(best, result);
                }
            }
            if (robots.obsidian as f32 / robots.ore as f32)
                < (self.geode_robot_obsidian_cost as f32 / self.geode_robot_ore_cost as f32)
            {
                if let Some(resources) = resources.maybe_build_obsidian_robot(self) {
                    could_build_geode_or_obsidian = true;
                    let new_resources = resources.collect(&robots);
                    let result = self.solve_for_max_geodes(
                        new_resources,
                        robots.add_obsidian(),
                        time_left - 1,
                        seen,
                        true,
                    );
                    best = std::cmp::max(best, result);
                }
            }
        }
        if !could_build_geode_or_obsidian {
            best = std::cmp::max(
                best,
                self.solve_for_max_geodes(
                    resources.collect(&robots),
                    robots,
                    time_left - 1,
                    seen,
                    false,
                ),
            );
        }
        return best;
    }
}

impl FromStr for Blueprint {
    type Err = Oops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words = s.split_whitespace().collect::<Vec<_>>();
        let ore_robot_ore_cost = words[6].parse()?;
        let clay_robot_ore_cost = words[12].parse()?;
        let obsidian_robot_ore_cost = words[18].parse()?;
        let obsidian_robot_clay_cost = words[21].parse()?;
        let geode_robot_ore_cost = words[27].parse()?;
        let geode_robot_obsidian_cost = words[30].parse()?;
        let construct_ore_robot_cycle = std::cmp::max(
            std::cmp::min(
                (geode_robot_obsidian_cost as f32 / geode_robot_ore_cost as f32).floor() as i32,
                (obsidian_robot_clay_cost as f32 / obsidian_robot_ore_cost as f32).floor() as i32,
            ) - 1,
            1,
        );
        Ok(Blueprint {
            ore_robot_ore_cost,
            clay_robot_ore_cost,
            obsidian_robot_ore_cost,
            obsidian_robot_clay_cost,
            geode_robot_ore_cost,
            geode_robot_obsidian_cost,
            construct_ore_robot_cycle,
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
            println!("{:?}", blueprint);
            (i + 1) as i32
                * blueprint.solve_for_max_geodes(
                    Resources::new(),
                    Robots::new(),
                    24,
                    &mut HashMap::new(),
                    false,
                )
        })
        .inspect(|x| println!("{}", x))
        .sum())
}

fn part2(puzzle: &Puzzle) -> Result<i32, Oops> {
    Ok(puzzle
        .blueprints
        .iter()
        .take(3)
        .map(|blueprint| {
            println!("{:?}", blueprint);
            blueprint.solve_for_max_geodes(
                Resources::new(),
                Robots::new(),
                32,
                &mut HashMap::new(),
                false,
            )
        })
        .inspect(|x| println!("{}", x))
        .product())
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
        assert_eq!(56 * 62, part2(&parse(SAMPLE).unwrap()).unwrap());
    }
}
