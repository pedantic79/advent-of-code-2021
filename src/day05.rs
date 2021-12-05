use aoc_runner_derive::{aoc, aoc_generator};

use crate::utils::{dec, inc};

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

    fn points_part2(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        let r_inc = if self.start.0 < self.end.0 { inc } else { dec };
        let c_inc = if self.start.1 < self.end.1 { inc } else { dec };
        let (mut r, mut c) = self.start;

        std::iter::from_fn(move || {
            let res = (r, c);

            if res == self.end {
                None
            } else {
                r_inc(&mut r);
                c_inc(&mut c);
                Some(res)
            }
        })
        .chain(std::iter::once(self.end))
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

fn solve(inputs: &[Line]) -> (Vec<[u8; 1000]>, usize) {
    let mut map = vec![[0; 1000]; 1000];
    let mut count = 0;

    for l in inputs.iter().filter(|x| x.for_part1()) {
        for coord in l.points_part1() {
            map[coord.0][coord.1] += 1;
            if map[coord.0][coord.1] == 2 {
                count += 1;
            }
        }
    }

    (map, count)
}

#[aoc(day5, part1)]
pub fn part1(inputs: &[Line]) -> usize {
    solve(inputs).1
}

#[aoc(day5, part2)]
pub fn part2(inputs: &[Line]) -> usize {
    let (mut map, mut count) = solve(inputs);

    for l in inputs.iter().filter(|x| !x.for_part1()) {
        for coord in l.points_part2() {
            map[coord.0][coord.1] += 1;
            if map[coord.0][coord.1] == 2 {
                count += 1;
            }
        }
    }

    count
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
        fn sorted_equal<T>(mut a: Vec<T>, mut b: Vec<T>)
        where
            T: Ord + std::fmt::Debug,
        {
            a.sort_unstable();
            b.sort_unstable();

            assert_eq!(a, b)
        }

        let t = Line {
            start: (1, 1),
            end: (3, 3),
        };

        sorted_equal(
            t.points_part2().collect::<Vec<_>>(),
            vec![(1, 1), (2, 2), (3, 3)],
        );

        let t = Line {
            start: (9, 7),
            end: (7, 9),
        };
        sorted_equal(
            t.points_part2().collect::<Vec<_>>(),
            vec![(9, 7), (8, 8), (7, 9)],
        );
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
