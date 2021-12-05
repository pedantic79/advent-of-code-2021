use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, PartialEq)]
pub struct Line {
    start: (usize, usize),
    end: (usize, usize),
}

impl Line {
    fn for_part1(&self) -> bool {
        self.start.0 == self.end.0 || self.start.1 == self.end.1
    }

    fn points_part1(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        let startr = self.start.0.min(self.end.0);
        let stopr = self.start.0.max(self.end.0);

        let startc = self.start.1.min(self.end.1);
        let stopc = self.start.1.max(self.end.1);

        (startr..=stopr).flat_map(move |r| (startc..=stopc).map(move |c| (r, c)))
    }

    fn points_part2(&self) -> Vec<(usize, usize)> {
        let mut ret = Vec::new();

        let r_inc = self.start.0 < self.end.0;
        let c_inc = self.start.1 < self.end.1;

        let (mut r, mut c) = self.start;
        ret.push((r, c));

        while (r, c) != self.end {
            if r_inc {
                r += 1;
            } else {
                r -= 1;
            }

            if c_inc {
                c += 1;
            } else {
                c -= 1;
            }

            ret.push((r, c));
        }

        ret
    }
}

#[aoc_generator(day5)]
pub fn generator(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(|x| {
            let (start, end) = x.split_once(" -> ").unwrap();
            let start = start
                .split_once(',')
                .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
                .unwrap();
            let end = end
                .split_once(',')
                .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
                .unwrap();
            Line { start, end }
        })
        .collect()
}

#[aoc(day5, part1)]
pub fn part1(inputs: &[Line]) -> usize {
    let mut map = HashMap::new();

    for l in inputs.iter().filter(|x| x.for_part1()) {
        for coord in l.points_part1() {
            *map.entry(coord).or_insert(0) += 1;
        }
    }

    map.values().filter(|x| **x > 1).count()
}

#[aoc(day5, part2)]
pub fn part2(inputs: &[Line]) -> usize {
    let mut map = HashMap::new();

    for l in inputs.iter().filter(|x| x.for_part1()) {
        // println!("{:?}", l);
        for coord in l.points_part1() {
            *map.entry(coord).or_insert(0) += 1;
        }
    }

    for l in inputs.iter().filter(|x| !x.for_part1()) {
        // println!("{:?}", l);
        for coord in l.points_part2() {
            *map.entry(coord).or_insert(0) += 1;
        }
    }

    map.values().filter(|x| **x > 1).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    pub fn test_input() {
        println!("{:?}", generator(SAMPLE));
    }

    #[test]
    pub fn test_diagonal() {
        let t = Line {
            start: (1, 1),
            end: (3, 3),
        };
        assert_eq!(t.points_part2(), vec![(1, 1), (2, 2), (3, 3)]);

        let t = Line {
            start: (9, 7),
            end: (7, 9),
        };
        assert_eq!(t.points_part2(), vec![(9, 7), (8, 8), (7, 9)]);
    }

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(SAMPLE)), 5);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(SAMPLE)), 12);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2021/day5.txt");
        const ANSWERS: (usize, usize) = (6856, 20666);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n'); // Trims trailing newline

            assert_eq!(part1(&generator(input)), ANSWERS.0);
            assert_eq!(part2(&generator(input)), ANSWERS.1);
        }
    }
}
