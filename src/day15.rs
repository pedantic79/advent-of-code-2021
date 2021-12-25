use aoc_runner_derive::{aoc, aoc_generator};
use pathfinding::prelude::dijkstra;

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
    dijkstra(
        &(0, 0),
        |p| neighbors(p.0, p.1, height, width).map(|p| (p, get(p))),
        //   |p| absdiff(height - 1, p.0) + absdiff(p.1, width - 1),
        |p| *p == (height - 1, width - 1),
    )
    .unwrap()
    .1
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
        let ans = map[r % h0][c % w0] + r / h0 + c / w0;
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
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
