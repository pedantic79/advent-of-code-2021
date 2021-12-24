use aoc_runner_derive::{aoc, aoc_generator};

fn parse_instruction(s: &str) -> isize {
    s.rsplit_once(' ').unwrap().1.parse().unwrap()
}

#[derive(Debug, PartialEq)]
pub struct Day24 {
    instructions: Vec<[isize; 3]>,
}

#[aoc_generator(day24)]
pub fn generator(input: &str) -> Day24 {
    let ins = input
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
        .collect();

    Day24 { instructions: ins }
}

fn solve(instructions: &[[isize; 3]]) -> (usize, usize) {
    let mut stack = Vec::new();
    let mut max = [0; 14];
    let mut min = [0; 14];

    for (i, &[div_x, add_x, add_y]) in instructions.iter().enumerate() {
        if div_x == 1 {
            stack.push((i, add_y));
        } else if let Some((j, ins_y)) = stack.pop() {
            let delta = ins_y + add_x;

            max[i] = 9.min(9 + delta);
            max[j] = 9.min(9 - delta);
            min[i] = 1.max(1 + delta);
            min[j] = 1.max(1 - delta);
        }
    }

    (
        min.iter().fold(0, |acc, &d| acc * 10 + d as usize),
        max.iter().fold(0, |acc, &d| acc * 10 + d as usize),
    )
}

#[aoc(day24, part1)]
pub fn part1(instructions: &Day24) -> usize {
    solve(&instructions.instructions).1
}

#[aoc(day24, part2)]
pub fn part2(instructions: &Day24) -> usize {
    solve(&instructions.instructions).0
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
            let input = INPUT.trim_end_matches('\n'); // Trims trailing newline

            assert_eq!(part1(&generator(input)), ANSWERS.0);
            assert_eq!(part2(&generator(input)), ANSWERS.1);
        }
    }
}
