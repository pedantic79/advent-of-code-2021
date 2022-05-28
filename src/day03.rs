use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
pub fn generator(input: &str) -> Vec<String> {
    input.lines().map(|s| s.to_owned()).collect()
}

#[aoc(day3, part1)]
pub fn part1(lines: &[String]) -> usize {
    let mut freq = vec![0; lines[0].len()];

    for l in lines {
        for (b, f) in l.bytes().zip(freq.iter_mut()) {
            if b == b'1' {
                *f += 1;
            }
        }
    }

    let threshold = lines.len() / 2;
    let (gamma, epsilon) = freq.iter().fold((0, 0), |(g, e), &x| {
        let (gx, ge) = if x > threshold { (1, 0) } else { (0, 1) };
        (g * 2 + gx, e * 2 + ge)
    });

    gamma * epsilon
}

fn solve_part2(lines: &[String], prefer: u8, dislike: u8) -> usize {
    let mut data: Vec<&String> = lines.iter().collect();

    for pos in 0.. {
        let counts = data
            .iter()
            .map(|l| l.as_bytes()[pos])
            .filter(|&x| x == b'1')
            .count();

        let common = if counts * 2 < data.len() {
            prefer
        } else {
            dislike
        };

        data.retain(|l| l.as_bytes()[pos] == common);
        if data.len() == 1 {
            break;
        }
    }

    usize::from_str_radix(data[0], 2).unwrap()
}

#[aoc(day3, part2)]
pub fn part2(lines: &[String]) -> usize {
    solve_part2(lines, b'1', b'0') * solve_part2(lines, b'0', b'1')
}

// #[aoc_generator(day3, part1, Int)]
// #[aoc_generator(day3, part2, Int)]
// pub fn generator_int(input: &str) -> Vec<u16> {
//     input
//         .lines()
//         .map(|s| u16::from_str_radix(s, 2).unwrap())
//         .collect::<Vec<_>>()
// }

// #[aoc(day3, part1, Int)]
// pub fn part1_int(nums: &[u16]) -> usize {
//     let threshold = nums.len() / 2;

//     let mut gamma = 0;
//     let mut epsilon = 0;

//     for n in 0.. {
//         let count = nums.iter().filter(|&&x| x >> n & 0b1 > 0).count();

//         if count == 0 {
//             break;
//         }
//         let (dx, de) = if count > threshold { (1, 0) } else { (0, 1) };
//         gamma |= dx << n;
//         epsilon |= de << n;
//     }

//     gamma * epsilon
// }

// fn solve_part2_int(lines: &[u16], prefer: u16, dislike: u16) -> usize {
//     let mut data: Vec<_> = lines.iter().copied().collect();

//     for pos in (0..u16::BITS).rev() {
//         let mask = 1 << pos;
//         let counts = data.iter().filter(|&&n| n & mask > 0).count();
//         if counts == 0 {
//             continue;
//         }
//         let common = if counts * 2 < data.len() {
//             prefer
//         } else {
//             dislike
//         };

//         data.retain(|&n| (n & mask) >> pos == common);
//         if data.len() == 1 {
//             break;
//         }
//     }

//     usize::from(data[0])
// }

// #[aoc(day3, part2, Int)]
// pub fn part2_int(nums: &[u16]) -> usize {
//     solve_part2_int(nums, 1, 0) * solve_part2_int(nums, 0, 1)
// }
#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    pub fn test_input() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(SAMPLE)), 198);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(SAMPLE)), 230);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2021/day3.txt");
        const ANSWERS: (usize, usize) = (2250414, 6085575);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
