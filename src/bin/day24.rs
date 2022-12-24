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

use aoc_2022::geometry::{Bounds2, Point2, Vector2};
use aoc_2022::{oops, oops::Oops};
use std::collections::{HashMap, HashSet, VecDeque};
use std::io::{self, Read};
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn parse_direction(c: char) -> Direction {
    match c {
        '^' => Direction::North,
        '>' => Direction::East,
        'v' => Direction::South,
        '<' => Direction::West,
        _ => panic!("bad direction character"),
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Blizzard {
    position: Point2,
    direction: Direction,
}

impl Blizzard {
    fn vector(&self) -> Vector2 {
        match self.direction {
            Direction::North => Vector2::new(0, -1),
            Direction::East => Vector2::new(1, 0),
            Direction::South => Vector2::new(0, 1),
            Direction::West => Vector2::new(-1, 0),
        }
    }
}

struct SimState {
    blizzards: Vec<Blizzard>,
    // Technically, this is redundant with blizzards but allows for faster collision testing.
    // An alternate representation would be a HashMap of Vec<Blizzard>, but it is not entirely
    // obvious that this will be a significant improvement, since it would require a lot more small
    // vectors.
    positions: HashSet<Point2>,
}

impl SimState {
    // TODO: Is this something that would be more appropriately implemented using the From trait?
    fn new(blizzards: Vec<Blizzard>) -> Self {
        let positions = blizzards.iter().map(|blizzard| blizzard.position).collect();
        Self {
            blizzards,
            positions,
        }
    }
}

struct Puzzle {
    bounds: Bounds2,
    // Since the blizzards wrap around to the other side of the valley, the states eventually
    // cycle. index = 0 is the initial state, index = 1 is the state after stepping the state
    // forward once, et cetera.
    states: Vec<SimState>,
}

impl Puzzle {
    fn move_blizzards(bounds: &Bounds2, current: &[Blizzard]) -> Vec<Blizzard> {
        current
            .iter()
            .map(|blizzard| {
                let next_position = blizzard.position + blizzard.vector();
                let next_position = if bounds.contains(&next_position) {
                    next_position
                } else {
                    Point2::new(
                        (next_position.x + bounds.width()) % bounds.width(),
                        (next_position.y + bounds.height()) % bounds.height(),
                    )
                };
                Blizzard {
                    position: next_position,
                    direction: blizzard.direction,
                }
            })
            .collect()
    }

    fn compute_states(bounds: &Bounds2, initial: &[Blizzard]) -> Vec<SimState> {
        let mut states = vec![];
        states.push(SimState::new(initial.to_vec()));
        loop {
            let next = Self::move_blizzards(bounds, &states.last().unwrap().blizzards);
            // Since the blizzards wrap around in a rectangular valley, there should eventually be
            // a cycle that maps back to the initial blizzard state.
            if next == initial {
                break;
            }
            states.push(SimState::new(next.clone()));
        }
        states
    }
    fn bfs(&self, start: Point2, end: Point2, starting_state: usize) -> usize {
        #[derive(Clone, Copy, Eq, Hash, PartialEq)]
        struct Search {
            position: Point2,
            state_index: usize,
        }

        // Unlike Search, state_index is wrapped at the cycle length.
        #[derive(Clone, Copy, Eq, Hash, PartialEq)]
        struct MemoizedSearch {
            position: Point2,
            state_index: usize,
        }

        let cycle_length = self.states.len();
        let initial_search = Search {
            position: start,
            state_index: starting_state,
        };
        let mut queue = VecDeque::new();
        queue.push_back(initial_search);

        let memoized_initial_search = MemoizedSearch {
            position: start,
            state_index: starting_state % cycle_length,
        };
        let mut visited = HashSet::new();
        visited.insert(memoized_initial_search);

        while let Some(next) = queue.pop_front() {
            if next.position == end {
                return next.state_index;
            }

            let next_state_index = next.state_index + 1;
            let next_sim_state = &self.states[next_state_index % cycle_length];

            // Get valid moves.
            let mut moves = vec![];
            for neighbor in next.position.neighbors() {
                if !self.bounds.contains(&neighbor) && neighbor != start && neighbor != end {
                    continue;
                }
                if next_sim_state.positions.contains(&neighbor) {
                    continue;
                }
                moves.push(Search {
                    position: neighbor,
                    state_index: next_state_index,
                });
            }
            if !next_sim_state.positions.contains(&next.position) {
                moves.push(Search {
                    position: next.position,
                    state_index: next_state_index,
                });
            }

            // Prune.
            for m in moves {
                let memoized = MemoizedSearch {
                    position: m.position,
                    state_index: m.state_index % cycle_length,
                };
                if visited.contains(&memoized) {
                    continue;
                }
                queue.push_back(m);
                visited.insert(memoized);
            }
        }
        panic!("no path!");
    }

    #[allow(dead_code)]
    fn visualize(&self, state_index: usize) -> String {
        #[derive(Debug)]
        enum Visualization {
            Direction(Direction),
            Count(usize),
        }
        let mut blizzards_and_counts = HashMap::new();
        for blizzard in &self.states[state_index].blizzards {
            blizzards_and_counts
                .entry(blizzard.position)
                .and_modify(|v| match v {
                    Visualization::Direction(_) => *v = Visualization::Count(2),
                    Visualization::Count(x) => *x += 1,
                })
                .or_insert(Visualization::Direction(blizzard.direction));
        }
        let lines = (self.bounds.min.y..=self.bounds.max.y)
            .map(|y| {
                (self.bounds.min.x..=self.bounds.max.x)
                    .map(|x| match blizzards_and_counts.get(&Point2::new(x, y)) {
                        Some(Visualization::Direction(d)) => match d {
                            Direction::North => '^',
                            Direction::East => '>',
                            Direction::South => 'v',
                            Direction::West => '<',
                        },
                        Some(Visualization::Count(c)) if *c < 10 => (*c as u8 + b'0') as char,
                        Some(Visualization::Count(_)) => '!',
                        None => '.',
                    })
                    .collect::<String>()
            })
            .collect::<Vec<_>>();
        lines.join("\n")
    }
}

impl FromStr for Puzzle {
    type Err = Oops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut max_x = 0;
        let mut max_y = 0;
        let mut blizzards = vec![];
        // Start at -1 to just ignore the # borders. In addition, the problems inputs always start
        // one square above the leftmost top square and end one square below the rightmost bottom
        // square. This allows the wraparound logic to be straightforward modular arithmetic.
        for (y, line) in (-1..).zip(s.lines()) {
            for (x, c) in (-1..).zip(line.chars()) {
                max_x = std::cmp::max(max_x, x);
                max_y = std::cmp::max(max_y, y);

                match c {
                    '#' => continue,
                    '.' => continue,
                    c => {
                        let direction = parse_direction(c);
                        blizzards.push(Blizzard {
                            position: Point2::new(x, y),
                            direction,
                        });
                    }
                }
            }
        }

        let bounds = Bounds2 {
            min: Point2::new(0, 0),
            max: Point2::new(max_x - 1, max_y - 1),
        };

        let states = Self::compute_states(&bounds, &blizzards);

        Ok(Puzzle { bounds, states })
    }
}

fn parse(input: &str) -> Result<Puzzle, Oops> {
    input.parse()
}

fn part1(puzzle: &Puzzle) -> usize {
    let start = Point2::new(0, -1);
    let end = Point2::new(puzzle.bounds.width() - 1, puzzle.bounds.height());
    puzzle.bfs(start, end, 0)
}

fn part2(puzzle: &Puzzle) -> usize {
    let start = Point2::new(0, -1);
    let end = Point2::new(puzzle.bounds.width() - 1, puzzle.bounds.height());
    puzzle.bfs(
        start,
        end,
        puzzle.bfs(end, start, puzzle.bfs(start, end, 0)),
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

    const SIMPLE: &str = concat!(
        "#.#####\n",
        "#.....#\n",
        "#>....#\n",
        "#.....#\n",
        "#...v.#\n",
        "#.....#\n",
        "#####.#\n",
    );

    const COMPLEX: &str = concat!(
        "#.######\n",
        "#>>.<^<#\n",
        "#.<..<<#\n",
        "#>v.><>#\n",
        "#<^v^^>#\n",
        "######.#\n",
    );

    #[test]
    fn simple_computed_blizzard_states() {
        let puzzle = parse(SIMPLE).unwrap();
        assert_eq!(5, puzzle.states.len());
        assert_eq!(
            concat!(".....\n", ">....\n", ".....\n", "...v.\n", ".....",),
            puzzle.visualize(0)
        );
        assert_eq!(
            concat!(".....\n", ".>...\n", ".....\n", ".....\n", "...v.",),
            puzzle.visualize(1)
        );
        assert_eq!(
            concat!("...v.\n", "..>..\n", ".....\n", ".....\n", ".....",),
            puzzle.visualize(2)
        );
        assert_eq!(
            concat!(".....\n", "...2.\n", ".....\n", ".....\n", ".....",),
            puzzle.visualize(3)
        );
        assert_eq!(
            concat!(".....\n", "....>\n", "...v.\n", ".....\n", ".....",),
            puzzle.visualize(4)
        );
    }

    #[test]
    fn complex_computed_blizzard_states() {
        let puzzle = parse(COMPLEX).unwrap();
        assert_eq!(12, puzzle.states.len());
        assert_eq!(
            concat!(">>.<^<\n", ".<..<<\n", ">v.><>\n", "<^v^^>",),
            puzzle.visualize(0)
        );
        assert_eq!(
            concat!(".>3.<.\n", "<..<<.\n", ">2.22.\n", ">v..^<",),
            puzzle.visualize(1)
        );

        assert_eq!(
            concat!(".2>2..\n", ".^22^<\n", ".>2.^>\n", ".>..<.",),
            puzzle.visualize(2)
        );

        assert_eq!(
            concat!("<^<22.\n", ".2<.2.\n", "><2>..\n", "..><..",),
            puzzle.visualize(3)
        );

        assert_eq!(
            concat!(".<..22\n", "<<.<..\n", "<2.>>.\n", ".^22^.",),
            puzzle.visualize(4)
        );

        assert_eq!(
            concat!("2.v.<>\n", "<.<..<\n", ".^>^22\n", ".2..2.",),
            puzzle.visualize(5)
        );
    }

    #[test]
    fn example1() {
        assert_eq!(18, part1(&parse(COMPLEX).unwrap()));
    }

    #[test]
    fn example2() {
        assert_eq!(54, part2(&parse(COMPLEX).unwrap()));
    }
}
