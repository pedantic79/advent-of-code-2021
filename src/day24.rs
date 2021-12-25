use std::cmp::{max, min};

use aoc_runner_derive::{aoc, aoc_generator};

fn parse_instruction(s: &str) -> isize {
    s.rsplit_once(' ').unwrap().1.parse().unwrap()
}

#[aoc_generator(day24)]
pub fn generator(input: &str) -> Vec<[isize; 3]> {
    input
        .split("\ninp")
        .map(|chunk| {
            let mut itr = chunk.lines();

            // get every 4th, 5th, and 15th instruction after an "inp"
            [
                parse_instruction(itr.nth(4).unwrap()),
                parse_instruction(itr.next().unwrap()),
                parse_instruction(itr.nth(9).unwrap()),
            ]
        })
        .collect()
}

fn solve<F, const N: isize>(instructions: &[[isize; 3]], pick: F) -> usize
where
    F: Fn(isize, isize) -> isize,
{
    let mut stack = Vec::new();
    let mut num = [0; 14];

    for (i, &[div_x, add_x, add_y]) in instructions.iter().enumerate() {
        if div_x == 1 {
            stack.push((i, add_y));
        } else if let Some((j, ins_y)) = stack.pop() {
            let delta = ins_y + add_x;

            num[i] = pick(N, N + delta);
            num[j] = pick(N, N - delta);
        }
    }

    num.iter().fold(0, |acc, &d| acc * 10 + d as usize)
}

#[aoc(day24, part1)]
pub fn part1(instructions: &[[isize; 3]]) -> usize {
    solve::<_, 9>(instructions, min)
}

#[aoc(day24, part2)]
pub fn part2(instructions: &[[isize; 3]]) -> usize {
    solve::<_, 1>(instructions, max)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_input() {
        println!("{:?}", generator(regression::INPUT));

        // assert_eq!(generator(SAMPLE), Day24());
    }

    #[test]
    pub fn test1() {
        // assert_eq!(part1(&generator(SAMPLE)), 7);
    }

    #[test]
    pub fn test2() {
        // assert_eq!(part2(&generator(SAMPLE)), 336);
    }

    mod regression {
        use super::*;

        pub const INPUT: &str = include_str!("../input/2021/day24.txt");
        const ANSWERS: (usize, usize) = (99691891979938, 27141191213911);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
