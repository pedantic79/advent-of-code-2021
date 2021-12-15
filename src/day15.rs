use std::{cmp::Reverse, collections::BinaryHeap};

use aoc_runner_derive::{aoc, aoc_generator};

use crate::utils::neighbors;

#[aoc_generator(day15)]
pub fn generator(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| char::to_digit(c, 10).unwrap() as usize)
                .collect()
        })
        .collect()
}

fn solve(get: impl Fn((usize, usize)) -> usize, height: usize, width: usize) -> usize {
    let mut costs = vec![vec![usize::MAX; width]; height];
    costs[0][0] = 0;

    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0), 0, 0));

    while let Some((Reverse(_), y, x)) = heap.pop() {
        if y == height - 1 && x == width - 1 {
            return costs[y][x];
        }

        for (r, c) in neighbors(y, x, height, width) {
            let next_cost = costs[y][x] + get((r, c));
            if next_cost < costs[r][c] {
                costs[r][c] = next_cost;
                heap.push((Reverse(next_cost), r, c));
            }
        }
    }

    unreachable!()
}

#[aoc(day15, part1)]
pub fn part1(map: &[Vec<usize>]) -> usize {
    solve(|p| map[p.0][p.1], map.len(), map[0].len())
}

#[aoc(day15, part2)]
pub fn part2(map: &[Vec<usize>]) -> usize {
    const M: usize = 5;
    let h0 = map.len();
    let w0 = map[0].len();

    let get = |(r, c): (usize, usize)| -> usize {
        let ans = map[r % h0][c % h0] + r / h0 + c / h0;
        (ans - 1) % 9 + 1
    };

    solve(get, h0 * M, w0 * M)
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
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(SAMPLE)), 40);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(SAMPLE)), 315);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2021/day15.txt");
        const ANSWERS: (usize, usize) = (619, 2922);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n'); // Trims trailing newline

            assert_eq!(part1(&generator(input)), ANSWERS.0);
            assert_eq!(part2(&generator(input)), ANSWERS.1);
        }
    }
}
