use crate::utils::AddIsize;
use aoc_runner_derive::{aoc, aoc_generator};

const SIZE: usize = 10;

fn neighbors(r: usize, c: usize, r_m: usize, c_m: usize) -> impl Iterator<Item = (usize, usize)> {
    [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ]
    .iter()
    .filter_map(move |&(y, x)| {
        let r_new = r.checked_add_isize_clamp(y, r_m)?;
        let c_new = c.checked_add_isize_clamp(x, c_m)?;

        Some((r_new, c_new))
    })
}

fn build_array<T, I, const N: usize>(mut iter: I) -> [T; N]
where
    T: Default + Copy,
    I: Iterator<Item = T>,
{
    let mut res = [T::default(); N];

    // BAD... We don't know if there are enough iterator elements to fill or overfill the array.
    for slot in res.iter_mut() {
        *slot = iter.next().unwrap();
    }

    res
}

#[aoc_generator(day11)]
pub fn generator(input: &str) -> [[u8; SIZE]; SIZE] {
    build_array(
        input
            .lines()
            .map(|l| build_array(l.chars().map(|c| c.to_digit(10).unwrap() as u8))),
    )
}

fn inc(slot: &mut u8) -> bool {
    *slot += 1;
    *slot == 10
}

fn step(m: &mut [[u8; SIZE]; SIZE]) -> usize {
    let mut flashing = Vec::new();

    for (r, row) in m.iter_mut().enumerate() {
        for (c, cell) in row.iter_mut().enumerate() {
            if inc(cell) {
                flashing.push((r, c));
            }
        }
    }

    let mut flashed = Vec::new();
    while let Some((r, c)) = flashing.pop() {
        flashed.push((r, c));
        for (y, x) in neighbors(r, c, SIZE, SIZE) {
            if inc(&mut m[y][x]) {
                flashing.push((y, x));
            }
        }
    }

    for &(r, c) in flashed.iter() {
        m[r][c] = 0;
    }

    flashed.len()
}

#[aoc(day11, part1)]
pub fn part1(inputs: &[[u8; SIZE]; SIZE]) -> usize {
    let mut m = *inputs;
    let mut amount = 0;
    for _ in 0..100 {
        amount += step(&mut m);
    }

    amount
}

#[aoc(day11, part2)]
pub fn part2(inputs: &[[u8; SIZE]; SIZE]) -> usize {
    let mut m = *inputs;
    for x in 1.. {
        if step(&mut m) == 100 {
            return x;
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    //     const SAMPLE: &str = r"11111
    // 19991
    // 19191
    // 19991
    // 11111";

    #[test]
    pub fn test_input() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(SAMPLE)), 1656);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(SAMPLE)), 195);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2021/day11.txt");
        const ANSWERS: (usize, usize) = (1640, 312);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n'); // Trims trailing newline

            assert_eq!(part1(&generator(input)), ANSWERS.0);
            assert_eq!(part2(&generator(input)), ANSWERS.1);
        }
    }
}
