use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};

type Scalar = i32;

#[derive(Debug, PartialEq, Eq)]
pub enum Direction {
    Forward(Scalar),
    Down(Scalar),
    Up(Scalar),
}

impl FromStr for Direction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, amount) = s.split_once(' ').ok_or("split error")?;
        let amount = amount.parse().map_err(|_| "parse error")?;

        Ok(match dir {
            "forward" => Direction::Forward(amount),
            "down" => Direction::Down(amount),
            "up" => Direction::Up(amount),
            _ => return Err("invalid direction"),
        })
    }
}

#[aoc_generator(day2)]
pub fn generator(input: &str) -> Vec<Direction> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[aoc(day2, part1)]
pub fn part1(inputs: &[Direction]) -> Scalar {
    let (mut depth, mut hor) = (0, 0);
    for dir in inputs {
        match dir {
            Direction::Forward(amount) => hor += amount,
            Direction::Down(amount) => depth += amount,
            Direction::Up(amount) => depth -= amount,
        }
    }

    depth * hor
}

#[aoc(day2, part2)]
pub fn part2(inputs: &[Direction]) -> Scalar {
    let (mut depth, mut hor, mut aim) = (0, 0, 0);
    for dir in inputs {
        match dir {
            Direction::Forward(amount) => {
                hor += amount;
                depth += aim * amount;
            }
            Direction::Down(amount) => aim += amount,
            Direction::Up(amount) => aim -= amount,
        }
    }

    depth * hor
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"forward 5
down 5
forward 8
up 3
down 8
forward 2";

    #[test]
    pub fn test_input() {
        // println!("{:?}", generator(SAMPLE));
        use super::Direction::*;

        assert_eq!(
            generator(SAMPLE),
            vec![Forward(5), Down(5), Forward(8), Up(3), Down(8), Forward(2)]
        );
    }
    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(SAMPLE)), 150);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(SAMPLE)), 900);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2021/day2.txt");
        const ANSWERS: (i32, i32) = (2322630, 2105273490);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
