use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day16)]
pub fn generator(input: &str) -> Vec<usize> {
    let mut res = Vec::new();

    for n in input.chars().map(|c| c.to_digit(16).unwrap() as usize) {
        res.extend(
            [
                (n & 0b1000) >> 3,
                (n & 0b0100) >> 2,
                (n & 0b0010) >> 1,
                n & 0b0001,
            ]
            .iter(),
        );
    }

    res
}

fn to_decimal_init(binary: &[usize], init: usize) -> usize {
    binary.iter().fold(init, |n, b| n * 2 + b)
}

fn to_decimal(binary: &[usize]) -> usize {
    to_decimal_init(binary, 0)
}

fn bool2usize(b: bool) -> usize {
    if b {
        1
    } else {
        0
    }
}

fn decode_type4(binary: &[usize]) -> (usize, usize) {
    let mut res = 0;
    let mut count = 0;

    for w in binary.chunks(5) {
        count += 5;
        res = to_decimal_init(&w[1..], res);
        if w[0] == 0 {
            break;
        }
    }

    (count, res)
}

trait Increment {
    fn inc(&mut self, n: Self) -> Self;
}

impl Increment for usize {
    fn inc(&mut self, n: Self) -> Self {
        *self += n;
        *self
    }
}

fn solve(p: &[usize], mut pc: usize) -> (usize, usize, usize) {
    let mut version = to_decimal(&p[pc..pc.inc(3)]);
    let typeid = to_decimal(&p[pc..pc.inc(3)]);

    if typeid == 4 {
        let (a, b) = decode_type4(&p[pc..]);
        (pc + a, version, b)
    } else {
        let mut result = vec![];

        if p[pc] == 0 {
            let l = to_decimal(&p[pc.inc(1)..pc.inc(15)]) as usize;
            let stop = pc + l;

            loop {
                let (npc, ver, val) = solve(p, pc);
                version += ver;
                pc = npc;
                result.push(val);
                if pc >= stop {
                    break;
                }
            }
        } else {
            let l = to_decimal(&p[pc.inc(1)..pc.inc(11)]) as usize;
            for _ in 0..l {
                let (npc, ver, val) = solve(p, pc);
                version += ver;
                pc = npc;
                result.push(val);
            }
        }

        (
            pc,
            version,
            match typeid {
                0 => result.iter().sum::<usize>(),
                1 => result.iter().product::<usize>(),
                2 => result.iter().min().copied().unwrap(),
                3 => result.iter().max().copied().unwrap(),
                5 if result.len() == 2 => bool2usize(result[0] > result[1]),
                6 if result.len() == 2 => bool2usize(result[0] < result[1]),
                7 if result.len() == 2 => bool2usize(result[0] == result[1]),
                _ => unreachable!(),
            },
        )
    }
}

#[aoc(day16, part1)]
pub fn part1(inputs: &[usize]) -> usize {
    solve(inputs, 0).1
}

#[aoc(day16, part2)]
pub fn part2(inputs: &[usize]) -> usize {
    solve(inputs, 0).2
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"EE00D40C823060";

    #[test]
    pub fn test_input() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(SAMPLE)), 14);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(SAMPLE)), 3);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2021/day16.txt");
        const ANSWERS: (usize, usize) = (951, 902198718880);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n'); // Trims trailing newline

            assert_eq!(part1(&generator(input)), ANSWERS.0);
            assert_eq!(part2(&generator(input)), ANSWERS.1);
        }
    }
}
