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
use std::io::{self, Read};
use std::str::FromStr;

#[derive(Debug)]
struct Puzzle {
    beacons: Vec<(i64, i64)>,
    sensors: Vec<(i64, i64)>,
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
        let mut beacons = vec![];
        let mut sensors = vec![];
        for line in s.lines() {
            let (sensor, beacon) = line
                .split_once(": closest beacon is at ")
                .ok_or_else(|| oops!("unexpected line!"))?;
            let Some(sensor) = sensor.strip_prefix("Sensor at ") else {
                return Err(oops!("unexpected sensor format"));
            };
            beacons.push(parse_point(beacon).ok_or_else(|| oops!("no beacon coord"))?);
            sensors.push(parse_point(sensor).ok_or_else(|| oops!("no sensor coord"))?);
        }
        Ok(Puzzle { beacons, sensors })
    }
}

fn parse(input: &str) -> Result<Puzzle, Oops> {
    let r = input.parse();
    println!("{:?}", r);
    r
}

fn distance(p1: &(i64, i64), p2: &(i64, i64)) -> i64 {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
}

fn part1(puzzle: &Puzzle, y: i64) -> Result<usize, Oops> {
    let mut grid = HashSet::new();
    let sensors: HashSet<_> = puzzle.sensors.iter().collect();
    let beacons: HashSet<_> = puzzle.beacons.iter().collect();
    for sensor in &puzzle.sensors {
        let min_distance = puzzle
            .beacons
            .iter()
            .map(|beacon| distance(&beacon, &sensor))
            .min()
            .unwrap();
        if y < sensor.1 - min_distance || y > sensor.1 + min_distance {
            continue;
        }
        for x in sensor.0 - min_distance..=sensor.0 + min_distance {
            if min_distance >= distance(&sensor, &(x, y))
                && !sensors.contains(&(x, y))
                && !beacons.contains(&(x, y))
            {
                grid.insert((x, y));
            }
        }
    }
    Ok(grid.len())
}

fn part2(puzzle: &Puzzle, (max_x, max_y): (i64, i64)) -> Result<usize, Oops> {
    let sensor_min_distance = puzzle
        .sensors
        .iter()
        .map(|sensor| {
            (
                sensor,
                puzzle.beacons.iter().fold(i64::MAX, |current, beacon| {
                    std::cmp::min(current, distance(&beacon, &sensor))
                }),
            )
        })
        .collect::<HashMap<_, _>>();
    println!("{:?}", sensor_min_distance);
    let mut x = 0;
    for y in 0..=max_y {
        while x <= max_x {
            let mut closest_sensor = None;
            let mut closest_distance = None;
            for (sensor, sensor_distance) in &sensor_min_distance {
                let d = distance(sensor, &(x, y));
                if d > *sensor_distance {
                    continue;
                }
                let remaining_distance = sensor_distance - d;
                match closest_distance {
                    None => {
                        closest_sensor = Some(*sensor);
                        closest_distance = Some(remaining_distance);
                    }
                    Some(x) => {
                        if remaining_distance > x {
                            closest_sensor = Some(*sensor);
                            closest_distance = Some(remaining_distance);
                        }
                    }
                }
            }
            let Some(closest_distance) = closest_distance else {
                return Ok((x * 4_000_000 + y) as usize);
            };
            let closest_sensor = closest_sensor.unwrap();
            let remaining_x =
                sensor_min_distance.get(&closest_sensor).unwrap() - (closest_sensor.1 - y).abs();
            x = std::cmp::max(closest_sensor.0 + remaining_x, x + 1);
        }
        x = 0;
    }
    Err(oops!("bah"))
}

fn main() -> Result<(), Oops> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let input = input;

    let puzzle = parse(&input)?;

    println!("{}", part1(&puzzle, 2_000_000)?);
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
        assert_eq!(26, part1(&parse(SAMPLE).unwrap(), 10).unwrap());
    }

    #[test]
    fn example2() {
        assert_eq!(56000011, part2(&parse(SAMPLE).unwrap(), (20, 20)).unwrap());
    }
}
