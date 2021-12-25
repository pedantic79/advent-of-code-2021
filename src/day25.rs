use core::panic;
use std::{fmt::Debug, mem::swap};

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(PartialEq, Clone, Copy)]
enum SeaCucumber {
    East,
    South,
    Empty,
}

impl SeaCucumber {
    fn parse_byte(b: u8) -> Self {
        match b {
            b'.' => Self::Empty,
            b'>' => Self::East,
            b'v' => Self::South,
            _ => panic!("invalid character: {}", b as char),
        }
    }
}

impl Debug for SeaCucumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::East => write!(f, ">"),
            Self::South => write!(f, "v"),
            Self::Empty => write!(f, "."),
        }
    }
}

#[derive(PartialEq, Clone)]
pub struct Map(Vec<Vec<SeaCucumber>>);

impl Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.0.iter() {
            for cucumber in row.iter() {
                write!(f, "{:?}", cucumber)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl Map {
    fn new_empty(&self) -> Self {
        Self(vec![
            vec![SeaCucumber::Empty; self.0[0].len()];
            self.0.len()
        ])
    }
    fn tick_east(&self, target: &mut Self) {
        // assume target is clear;

        for (r, row) in self.0.iter().enumerate() {
            for (c, cucumber) in row.iter().enumerate() {
                match *cucumber {
                    SeaCucumber::East => {
                        let (y, x) = self.get_right(r, c);
                        if self.is_empty(y, x) {
                            target.0[y][x] = SeaCucumber::East;
                        } else {
                            target.0[r][c] = SeaCucumber::East;
                        }
                    }
                    SeaCucumber::South => target.0[r][c] = SeaCucumber::South,
                    SeaCucumber::Empty => {}
                }
            }
        }
    }

    fn tick_south(&self, target: &mut Self) {
        for (r, row) in self.0.iter().enumerate() {
            for (c, cucumber) in row.iter().enumerate() {
                match *cucumber {
                    SeaCucumber::South => {
                        let (y, x) = self.get_down(r, c);
                        if self.is_empty(y, x) {
                            target.0[y][x] = SeaCucumber::South;
                        } else {
                            target.0[r][c] = SeaCucumber::South;
                        }
                    }
                    SeaCucumber::East => target.0[r][c] = SeaCucumber::East,
                    SeaCucumber::Empty => {}
                }
            }
        }
    }

    fn is_empty(&self, y: usize, x: usize) -> bool {
        self.0[y][x] == SeaCucumber::Empty
    }

    fn get_down(&self, r: usize, c: usize) -> (usize, usize) {
        ((r + 1) % self.0.len(), c)
    }

    fn get_right(&self, r: usize, c: usize) -> (usize, usize) {
        (r, (c + 1) % self.0[0].len())
    }

    fn clear(&mut self) {
        for row in self.0.iter_mut() {
            for cucumber in row.iter_mut() {
                *cucumber = SeaCucumber::Empty;
            }
        }
    }
}

#[aoc_generator(day25)]
pub fn generator(input: &str) -> Map {
    Map(input
        .lines()
        .map(|l| l.bytes().map(SeaCucumber::parse_byte).collect())
        .collect())
}

#[aoc(day25, part1)]
pub fn part1(inputs: &Map) -> usize {
    let mut inputs = inputs.clone();
    let mut target1 = inputs.new_empty();
    let mut target2 = inputs.new_empty();

    let mut x = 0;
    loop {
        x += 1;
        inputs.tick_east(&mut target1);

        target1.tick_south(&mut target2);
        if target2 == inputs {
            break x;
        }

        swap(&mut inputs, &mut target2);
        target1.clear();
        target2.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>";

    #[test]
    pub fn test_input() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(SAMPLE)), 58);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2021/day25.txt");
        const ANSWERS: (usize, usize) = (432, 0);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');

            let output = generator(input);
            assert_eq!(part1(&output), ANSWERS.0);
        }
    }
}
