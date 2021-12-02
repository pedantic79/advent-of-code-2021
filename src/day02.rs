use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2)]
pub fn generator(input: &str) -> Vec<(u8, i32)> {
    input
        .lines()
        .map(|line| {
            let (dir, amount) = line.split_once(' ').unwrap();
            let amount: i32 = amount.parse().unwrap();
            (dir.as_bytes()[0], amount)
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn part1(inputs: &[(u8, i32)]) -> i32 {
    let (mut depth, mut hor) = (0, 0);
    for (cmd, amount) in inputs {
        match cmd {
            b'f' => hor += amount,
            b'd' => depth += amount,
            b'u' => depth -= amount,
            _ => unreachable!(),
        }
    }

    depth * hor
}

#[aoc(day2, part2)]
pub fn part2(inputs: &[(u8, i32)]) -> i32 {
    let (mut depth, mut hor, mut aim) = (0, 0, 0);
    for (cmd, amount) in inputs {
        match cmd {
            b'f' => {
                hor += amount;
                depth += aim * amount;
            }
            b'd' => aim += amount,
            b'u' => aim -= amount,
            _ => unreachable!(),
        }
    }

    depth * hor
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"forward 5
down 5
forward 8
up 3
down 8
forward 2";

    #[test]
    pub fn test_input() {
        // println!("{:?}", generator(SAMPLE));

        assert_eq!(
            generator(SAMPLE),
            vec![
                (b'f', 5),
                (b'd', 5),
                (b'f', 8),
                (b'u', 3),
                (b'd', 8),
                (b'f', 2)
            ]
        );
    }
    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(SAMPLE)), 150);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(SAMPLE)), 900);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2021/day2.txt");
        const ANSWERS: (i32, i32) = (2322630, 2105273490);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n'); // Trims trailing newline

            assert_eq!(part1(&generator(input)), ANSWERS.0);
            assert_eq!(part2(&generator(input)), ANSWERS.1);
        }
    }
}
