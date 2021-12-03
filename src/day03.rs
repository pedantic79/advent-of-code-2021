use std::collections::HashSet;

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

    let mut gamma = 0;
    let mut epsilon = 0;
    let threshold = lines.len() / 2;
    for &x in freq.iter() {
        gamma = gamma * 2 + if x > threshold { 1 } else { 0 };
        epsilon = epsilon * 2 + if x < threshold { 1 } else { 0 };
    }

    gamma * epsilon
}

#[aoc(day3, part2)]
pub fn part2(lines: &[String]) -> usize {
    let mut set_generator: HashSet<&str> = lines.iter().map(|s| s.as_ref()).collect();
    let mut set_scrubber: HashSet<&str> = lines.iter().map(|s| s.as_ref()).collect();

    let mut pos = 0;
    while pos < lines[0].len() {
        let g_0 = set_generator
            .iter()
            .map(|l| l.as_bytes()[pos])
            .filter(|b| *b == b'0')
            .count();
        let g_1 = set_generator
            .iter()
            .map(|l| l.as_bytes()[pos])
            .filter(|b| *b == b'1')
            .count();
        let s_0 = set_scrubber
            .iter()
            .map(|l| l.as_bytes()[pos])
            .filter(|b| *b == b'0')
            .count();
        let s_1 = set_scrubber
            .iter()
            .map(|l| l.as_bytes()[pos])
            .filter(|b| *b == b'1')
            .count();

        let g_common = if g_1 < g_0 { b'0' } else { b'1' };
        let s_common = if s_0 <= s_1 { b'0' } else { b'1' };

        if set_generator.len() > 1 {
            set_generator.retain(|l| l.as_bytes()[pos] == g_common);
        }
        if set_scrubber.len() > 1 {
            set_scrubber.retain(|l| l.as_bytes()[pos] == s_common);
        }

        pos += 1;
    }

    let g = set_generator.iter().next().unwrap();
    let s = set_scrubber.iter().next().unwrap();

    let mut g_num = 0;
    for d in g.bytes() {
        g_num = g_num * 2 + if d == b'1' { 1 } else { 0 }
    }

    let mut s_num = 0;
    for d in s.bytes() {
        s_num = s_num * 2 + if d == b'1' { 1 } else { 0 }
    }

    g_num * s_num
}

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
            let input = INPUT.trim_end_matches('\n'); // Trims trailing newline

            assert_eq!(part1(&generator(input)), ANSWERS.0);
            assert_eq!(part2(&generator(input)), ANSWERS.1);
        }
    }
}
