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
use std::collections::HashSet;
use std::io::{self, Read};
use std::str::FromStr;

#[derive(Debug)]
struct Sensor {
    loc: (i64, i64),
    beacon_free_radius: i64,
}

#[derive(Debug)]
struct Puzzle {
    sensors: Vec<Sensor>,
    beacons: HashSet<(i64, i64)>,
}

impl Puzzle {
    fn get_max_skip_sensor(&self, loc: &(i64, i64)) -> Option<(&Sensor, i64)> {
        self.sensors.iter().fold(None, |acc, s| {
            if s.beacon_free_radius - calc_dist(&s.loc, loc) < 0 {
                acc
            } else {
                let skip = s.beacon_free_radius - (s.loc.1 - loc.1).abs();
                if let Some((_, acc_skip)) = acc {
                    if skip > acc_skip {
                        Some((s, skip))
                    } else {
                        acc
                    }
                } else {
                    Some((s, skip))
                }
            }
        })
    }
}

fn calc_dist(p1: &(i64, i64), p2: &(i64, i64)) -> i64 {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
}

fn parse_point(s: &str) -> Option<(i64, i64)> {
    let (x, y) = s.split_once(", ")?;
    let x = x.strip_prefix("x=")?;
    let y = y.strip_prefix("y=")?;
    Some((x.parse().ok()?, y.parse().ok()?))
}

impl FromStr for Puzzle {
    type Err = Oops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut sensor_locs = vec![];
        let mut beacon_locs = HashSet::new();
        for line in s.lines() {
            let (sensor, beacon) = line
                .split_once(": closest beacon is at ")
                .ok_or_else(|| oops!("unexpected line!"))?;
            let Some(sensor) = sensor.strip_prefix("Sensor at ") else {
                return Err(oops!("unexpected sensor format"));
            };
            sensor_locs.push(parse_point(sensor).ok_or_else(|| oops!("no sensor coord"))?);
            beacon_locs.insert(parse_point(beacon).ok_or_else(|| oops!("no beacon coord"))?);
        }
        Ok(Puzzle {
            sensors: sensor_locs
                .iter()
                .map(|s_loc| Sensor {
                    loc: *s_loc,
                    beacon_free_radius: beacon_locs
                        .iter()
                        .map(|b_loc| calc_dist(s_loc, b_loc))
                        .min()
                        .unwrap(),
                })
                .collect(),
            beacons: beacon_locs,
        })
    }
}

fn parse(input: &str) -> Result<Puzzle, Oops> {
    let r = input.parse();
    println!("{r:?}");
    r
}

fn part1(puzzle: &Puzzle, y: i64) -> usize {
    // Given a fixed `y`, find the min `x` and max `x` that can possibly be covered by a sensor.
    let (min_x, max_x) = puzzle.sensors.iter().fold((i64::MAX, i64::MIN), |a, s| {
        let leftover = (s.loc.1 - y).abs();
        if leftover > s.beacon_free_radius {
            a
        } else {
            (
                std::cmp::min(s.loc.0 - leftover, a.0),
                std::cmp::max(s.loc.0 + leftover, a.1),
            )
        }
    });
    let mut x = min_x;
    let mut c = 0;
    while x <= max_x {
        let Some((sensor, skip)) = puzzle.get_max_skip_sensor(&(x, y)) else {
            x += 1;
            continue;
        };
        // TODO: it's unclear that this logic is correct. Sensors can overlap by a square if the
        // same beacon is considered the nearest beacon for multiple sensors..
        c += (sensor.loc.0 + skip + 1) - x;
        x = sensor.loc.0 + skip + 1;
    }
    c -= puzzle.sensors.iter().filter(|s| s.loc.1 == y).count() as i64;
    c -= puzzle.beacons.iter().filter(|b| b.1 == y).count() as i64;
    c as usize
}

fn part2(puzzle: &Puzzle, (max_x, max_y): (i64, i64)) -> Result<usize, Oops> {
    let mut x = 0;
    for y in 0..=max_y {
        while x <= max_x {
            let Some((sensor, skip)) = puzzle.get_max_skip_sensor(&(x, y)) else {
                return Ok((x * 4_000_000 + y) as usize);
            };
            x = sensor.loc.0 + skip + 1;
        }
        x = 0;
    }
    Err(oops!("no answer"))
}

fn main() -> Result<(), Oops> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let input = input;

    let puzzle = parse(&input)?;

    println!("{}", part1(&puzzle, 2_000_000));
    println!("{}", part2(&puzzle, (4_000_000, 4_000_000))?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = concat!(
        "Sensor at x=2, y=18: closest beacon is at x=-2, y=15\n",
        "Sensor at x=9, y=16: closest beacon is at x=10, y=16\n",
        "Sensor at x=13, y=2: closest beacon is at x=15, y=3\n",
        "Sensor at x=12, y=14: closest beacon is at x=10, y=16\n",
        "Sensor at x=10, y=20: closest beacon is at x=10, y=16\n",
        "Sensor at x=14, y=17: closest beacon is at x=10, y=16\n",
        "Sensor at x=8, y=7: closest beacon is at x=2, y=10\n",
        "Sensor at x=2, y=0: closest beacon is at x=2, y=10\n",
        "Sensor at x=0, y=11: closest beacon is at x=2, y=10\n",
        "Sensor at x=20, y=14: closest beacon is at x=25, y=17\n",
        "Sensor at x=17, y=20: closest beacon is at x=21, y=22\n",
        "Sensor at x=16, y=7: closest beacon is at x=15, y=3\n",
        "Sensor at x=14, y=3: closest beacon is at x=15, y=3\n",
        "Sensor at x=20, y=1: closest beacon is at x=15, y=3\n",
    );

    #[test]
    fn example1() {
        assert_eq!(26, part1(&parse(SAMPLE).unwrap(), 10));
    }

    #[test]
    fn example2() {
        assert_eq!(56000011, part2(&parse(SAMPLE).unwrap(), (20, 20)).unwrap());
    }
}
