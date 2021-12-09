use std::{cmp::Reverse, iter::from_fn};

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day9)]
pub fn generator(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|l| l.chars().map(|x| x.to_digit(10).unwrap() as u8).collect())
        .collect()
}

fn neighbors(r: usize, c: usize, r_m: usize, c_m: usize) -> impl Iterator<Item = (usize, usize)> {
    let mut kind = 0;

    from_fn(move || {
        if kind == 0 {
            kind += 1;
            if r > 0 {
                return Some((r - 1, c));
            }
        }

        if kind == 1 {
            kind += 1;
            if c > 0 {
                return Some((r, c - 1));
            }
        }

        if kind == 2 {
            kind += 1;
            if c + 1 < c_m {
                return Some((r, c + 1));
            }
        }

        if kind == 3 {
            kind += 1;
            if r + 1 < r_m {
                return Some((r + 1, c));
            }
        }

        None
    })
}

#[aoc(day9, part1)]
pub fn part1(inputs: &[Vec<u8>]) -> usize {
    let r_max = inputs.len();
    let c_max = inputs[0].len();
    (0..r_max)
        .flat_map(|row| (0..c_max).map(move |col| (row, col)))
        .filter_map(|(r, c)| {
            let cell = inputs[r][c];
            if neighbors(r, c, r_max, c_max).all(|(y, x)| inputs[y][x] > cell) {
                Some(usize::from(cell) + 1)
            } else {
                None
            }
        })
        // .inspect(|x| println!("{}", x - 1))
        .sum()
}

#[aoc(day9, part2)]
pub fn part2(inputs: &[Vec<u8>]) -> usize {
    let r_max = inputs.len();
    let c_max = inputs[0].len();
    let mut v = inputs.to_vec();
    let mut areas = Vec::new();

    for (row, col) in (0..r_max).flat_map(|row| (0..c_max).map(move |col| (row, col))) {
        if v[row][col] < 9 {
            areas.push(find_basin(&mut v, row, col, r_max, c_max));
        }
    }

    areas.sort_by_key(|&x| Reverse(x));
    areas[0..3].iter().product()
}

fn find_basin(v: &mut Vec<Vec<u8>>, row: usize, col: usize, r_max: usize, c_max: usize) -> usize {
    let mut total = 1;
    v[row][col] = 10;

    for (r, c) in neighbors(row, col, r_max, c_max) {
        if v[r][c] < 9 {
            total += find_basin(v, r, c, r_max, c_max);
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    pub fn test_input() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(SAMPLE)), 15);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(SAMPLE)), 1134);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2021/day9.txt");
        const ANSWERS: (usize, usize) = (535, 1122700);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n'); // Trims trailing newline

            assert_eq!(part1(&generator(input)), ANSWERS.0);
            assert_eq!(part2(&generator(input)), ANSWERS.1);
        }
    }
}
