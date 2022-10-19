use aoc_runner_derive::{aoc, aoc_generator};

use crate::utils::parse_range;

#[derive(Debug, PartialEq, Eq)]
pub struct Day17 {
    pub min_x: i32,
    pub max_x: i32,
    pub min_y: i32,
    pub max_y: i32,
}

#[aoc_generator(day17)]
pub fn generator(input: &str) -> Day17 {
    let mut i = input.split(&['=', ','][..]);
    let x = i.nth(1).unwrap();
    let y = i.nth(1).unwrap();
    let (min_x, max_x) = parse_range(x).unwrap();
    let (min_y, max_y) = parse_range(y).unwrap();

    Day17 {
        min_x,
        min_y,
        max_x,
        max_y,
    }
}

struct Probe {
    pos_x: i32,
    pos_y: i32,
    vol_x: i32,
    vol_y: i32,
}

impl Probe {
    fn new(vol_x: i32, vol_y: i32) -> Self {
        Self {
            pos_x: 0,
            pos_y: 0,
            vol_x,
            vol_y,
        }
    }

    fn simulate(&mut self, target: &Day17) -> Option<i32> {
        let mut max = 0;

        loop {
            self.pos_x += self.vol_x;
            self.pos_y += self.vol_y;

            self.vol_x = 0.max(self.vol_x - 1);
            self.vol_y -= 1;

            max = max.max(self.pos_y);
            if target.min_x <= self.pos_x
                && self.pos_x <= target.max_x
                && target.min_y <= self.pos_y
                && self.pos_y <= target.max_y
            {
                return Some(max);
            }

            if self.vol_y < 0 && self.pos_y < target.min_y {
                return None;
            }
        }
    }
}

#[aoc(day17, part1)]
pub fn part1(inputs: &Day17) -> i32 {
    let mut max = i32::MIN;
    for x in 0..250 {
        for y in 0..250 {
            if let Some(y) = Probe::new(x, y).simulate(inputs) {
                max = max.max(y);
            }
        }
    }

    max
}

#[aoc(day17, part2)]
pub fn part2(inputs: &Day17) -> usize {
    let mut count = 0;
    for x in 0..250 {
        for y in -250..250 {
            if Probe::new(x, y).simulate(inputs).is_some() {
                count += 1;
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"target area: x=20..30, y=-10..-5";

    #[test]
    pub fn test_input() {
        println!("{:?}", generator(SAMPLE));

        assert_eq!(
            generator(SAMPLE),
            Day17 {
                min_x: 20,
                max_x: 30,
                min_y: -10,
                max_y: -5
            }
        );
    }

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(SAMPLE)), 45);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(SAMPLE)), 112);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2021/day17.txt");
        const ANSWERS: (i32, usize) = (8911, 4748);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
