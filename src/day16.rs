use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day16)]
pub fn generator(input: &str) -> Vec<usize> {
    let mut res = Vec::new();

    for n in input.chars().map(|c| c.to_digit(16).unwrap() as usize) {
        res.extend([n >> 3 & 0b1, n >> 2 & 0b1, n >> 1 & 0b1, n & 0b1].into_iter());
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

fn decode_type4(binary: &[usize], pc: &mut usize) -> usize {
    let mut total = 0;

    for chunk in binary.chunks(5) {
        *pc += 5;
        total = to_decimal_init(&chunk[1..], total);
        if chunk[0] == 0 {
            break;
        }
    }

    total
}

fn get_value(program: &[usize], pc: &mut usize, n: usize) -> usize {
    let start = *pc;
    *pc += n;

    to_decimal(&program[start..*pc])
}

fn solve(program: &[usize], pc: &mut usize) -> (usize, usize) {
    let mut version = get_value(program, pc, 3);
    let typeid = get_value(program, pc, 3);

    if typeid == 4 {
        (version, decode_type4(&program[*pc..], pc))
    } else {
        let mut result = Vec::new();
        let length_id = program[*pc];
        *pc += 1;

        if length_id == 0 {
            let length = get_value(program, pc, 15);
            let stop = *pc + length;

            while *pc < stop {
                let (ver, val) = solve(program, pc);
                version += ver;
                result.push(val);
            }
        } else {
            let length = get_value(program, pc, 11);

            for _ in 0..length {
                let (ver, val) = solve(program, pc);
                version += ver;
                result.push(val);
            }
        }

        (
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
    solve(inputs, &mut 0).0
}

#[aoc(day16, part2)]
pub fn part2(inputs: &[usize]) -> usize {
    solve(inputs, &mut 0).1
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
