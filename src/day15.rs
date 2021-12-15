use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use pathfinding::prelude::astar;

use crate::utils::neighbors;

#[aoc_generator(day15, part1)]
pub fn generator1(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| char::to_digit(c, 10).unwrap() as usize)
                .collect()
        })
        .collect()
}

fn inc(i: &mut usize) {
    *i += 1;
    if *i > 9 {
        *i -= 9;
    }
}

#[aoc_generator(day15, part2)]
pub fn generator2(input: &str) -> Vec<Vec<usize>> {
    let mut res = input
        .lines()
        .map(|l| {
            let mut row = Vec::new();

            let mut segment = l
                .chars()
                .map(|c| char::to_digit(c, 10).unwrap() as usize)
                .collect_vec();

            for _ in 0..5 {
                row.extend_from_slice(&segment);
                segment.iter_mut().for_each(inc);
            }
            row
        })
        .collect_vec();

    let height = res.len();

    for r in 0..height * 4 {
        let mut row = res[r].clone();
        row.iter_mut().for_each(inc);

        res.push(row);
    }
    res
}

fn solve(map: &[Vec<usize>]) -> usize {
    let height = map.len();
    let width = map[0].len();

    let (_, c) = astar(
        &(0, 0),
        |p| neighbors(p.0, p.1, height, width).map(|(y, x)| ((y, x), map[y][x])),
        |p| map[p.0][p.1],
        |p| *p == (height - 1, width - 1),
    )
    .unwrap();

    c
}

#[aoc(day15, part1)]
pub fn part1(map: &[Vec<usize>]) -> usize {
    solve(map)
}

#[aoc(day15, part2)]
pub fn part2(map: &[Vec<usize>]) -> usize {
    solve(map)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

    #[test]
    pub fn test_input() {
        println!("{:?}", generator2(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator1(SAMPLE)), 40);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator2(SAMPLE)), 315);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2021/day15.txt");
        const ANSWERS: (usize, usize) = (619, 2922);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n'); // Trims trailing newline

            assert_eq!(part1(&generator1(input)), ANSWERS.0);
            assert_eq!(part2(&generator2(input)), ANSWERS.1);
        }
    }
}
