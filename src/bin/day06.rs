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

use aoc_2022::oops::Oops;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io;

fn part1(s: &str) -> Result<usize, Oops> {
    let mut recent: VecDeque<char> = VecDeque::new();
    for (i, c) in s.chars().enumerate() {
        recent.push_back(c);
        if recent.len() > 3 {
            let u: HashSet<char> = recent.iter().copied().collect();
            if u.len() == 4 {
                return Ok(i + 1);
            }
            recent.pop_front();
        }
    }
    Err(aoc_2022::oops!("no answer"))
}

fn part2(s: &str) -> Result<usize, Oops> {
    let mut recent: VecDeque<char> = VecDeque::new();
    for (i, c) in s.chars().enumerate() {
        recent.push_back(c);
        if recent.len() > 13 {
            let u: HashSet<char> = recent.iter().copied().collect();
            if u.len() == 14 {
                return Ok(i + 1);
            }
            recent.pop_front();
        }
    }
    Err(aoc_2022::oops!("no answer"))
}

fn main() -> Result<(), Oops> {
    let entities = io::stdin().lines().next().unwrap().unwrap();

    println!("{}", part1(&entities)?);
    println!("{}", part2(&entities)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE1: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    const SAMPLE2: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    const SAMPLE3: &str = "nppdvjthqldpwncqszvftbrmjlhg";
    const SAMPLE4: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    const SAMPLE5: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    #[test]
    fn example1() {
        assert_eq!(7, part1(SAMPLE1).unwrap());
        assert_eq!(5, part1(SAMPLE2).unwrap());
        assert_eq!(6, part1(SAMPLE3).unwrap());
        assert_eq!(10, part1(SAMPLE4).unwrap());
        assert_eq!(11, part1(SAMPLE5).unwrap());
    }

    #[test]
    fn example2() {
        assert_eq!(19, part2(SAMPLE1).unwrap());
        assert_eq!(23, part2(SAMPLE2).unwrap());
        assert_eq!(23, part2(SAMPLE3).unwrap());
        assert_eq!(29, part2(SAMPLE4).unwrap());
        assert_eq!(26, part2(SAMPLE5).unwrap());
    }
}
