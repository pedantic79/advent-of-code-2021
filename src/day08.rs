use std::{convert::Infallible, str::FromStr};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[derive(Debug, PartialEq)]
pub struct Object {
    before: Vec<String>,
    after: Vec<String>,
}

impl FromStr for Object {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.split_once(" | ").unwrap();
        let before = a.split(' ').map(sort_chars).collect();
        let after = b.split(' ').map(sort_chars).collect();

        Ok(Object { before, after })
    }
}

fn sort_chars(s: &str) -> String {
    let mut s = s.to_string();

    // SAFETY: input string only contains ascii characters, so it is safe to sort them.
    unsafe {
        s.as_bytes_mut().sort_unstable();
    }
    s
}

fn find_match(s: &str) -> bool {
    matches!(s.len(), 2 | 3 | 4 | 7)
}

const PACKED: [u8; 10] = [
    parse_lights("abcefg"),
    parse_lights("cf"),
    parse_lights("acdeg"),
    parse_lights("acdfg"),
    parse_lights("bcdf"),
    parse_lights("abdfg"),
    parse_lights("abdefg"),
    parse_lights("acf"),
    parse_lights("abcdefg"),
    parse_lights("abcdfg"),
];

const fn parse_lights(num: &str) -> u8 {
    let mut r = 0;
    let mut pos = 0;
    let bytes = num.as_bytes();

    while pos < bytes.len() {
        r |= 1 << (bytes[pos] - b'a');
        pos += 1;
    }

    r
}

fn permute_num(permutation: &[usize], num: &str) -> Option<usize> {
    let num = parse_lights(num);
    let n = permutation
        .iter()
        .enumerate()
        .filter(|(_, p)| num & (1 << **p) > 0)
        .fold(0, |acc, (i, _)| acc | 1 << i);

    PACKED.iter().position(|&x| x == n)
}

fn analyze(left: &[String]) -> Vec<usize> {
    (0..7)
        .permutations(7)
        .find_map(|permutation| {
            if left
                .iter()
                .all(|digit| permute_num(&permutation, digit).is_some())
            {
                Some(permutation)
            } else {
                None
            }
        })
        .unwrap()
}

#[aoc_generator(day8)]
pub fn generator(input: &str) -> Vec<Object> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[aoc(day8, part1)]
pub fn part1(inputs: &[Object]) -> usize {
    inputs
        .iter()
        .flat_map(|l| l.after.iter())
        .filter(|x| find_match(*x))
        // .inspect(|w| println!("**{}", w))
        .count()
}

#[aoc(day8, part2)]
pub fn part2(inputs: &[Object]) -> usize {
    inputs
        .iter()
        .map(|line| {
            let perm = analyze(&line.before);
            line.after
                .iter()
                .map(|digit| permute_num(&perm, digit).unwrap())
                .fold(0, |acc, n| acc * 10 + n)
        })
        // .inspect(|n| println!("{:?}", n))
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    pub fn test_input() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(SAMPLE)), 26);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(SAMPLE)), 61229);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2021/day8.txt");
        const ANSWERS: (usize, usize) = (294, 973292);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n'); // Trims trailing newline

            assert_eq!(part1(&generator(input)), ANSWERS.0);
            assert_eq!(part2(&generator(input)), ANSWERS.1);
        }
    }
}
