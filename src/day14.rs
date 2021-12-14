use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

use crate::utils::build_array;

#[derive(Debug, PartialEq)]
pub struct Object {
    start: String,
    rules: HashMap<[u8; 2], u8>,
}

#[aoc_generator(day14)]
pub fn generator(input: &str) -> Object {
    let (part1, part2) = input.split_once("\n\n").unwrap();

    let start = part1.to_string();
    let rules = part2
        .lines()
        .map(|l| {
            let (a, b) = l.split_once(" -> ").unwrap();
            (build_array(a.bytes()), b.parse::<char>().unwrap() as u8)
        })
        .collect();

    Object { start, rules }
}

fn generate_count(input: &str) -> HashMap<[u8; 2], usize> {
    input
        .as_bytes()
        .windows(2)
        .fold(HashMap::new(), |mut count, w| {
            *count.entry(build_array(w.iter().copied())).or_insert(0) += 1;
            count
        })
}

fn solve<const N: usize>(inputs: &Object) -> usize {
    let mut s = generate_count(&inputs.start);

    for _ in 0..N {
        let mut new_s = HashMap::new();
        for (k, count) in s.iter() {
            let &gen = inputs.rules.get(k).unwrap();
            let (left, right) = ([k[0], gen], [gen, k[1]]);
            *new_s.entry(left).or_default() += count;
            *new_s.entry(right).or_default() += count;
        }

        s = new_s;
    }

    let mut freq = s.iter().fold(HashMap::new(), |mut hm, c| {
        *hm.entry(c.0[1]).or_insert(0) += c.1;

        hm
    });

    *freq.entry(inputs.start.as_bytes()[0]).or_insert(0) += 1;

    let (min, max) = freq
        .iter()
        .minmax_by_key(|&(_, &count)| count)
        .into_option()
        .unwrap();

    freq[max.0] - freq[min.0]
}

#[aoc(day14, part1)]
pub fn part1(inputs: &Object) -> usize {
    solve::<10>(inputs)
}

#[aoc(day14, part2)]
pub fn part2(inputs: &Object) -> usize {
    solve::<40>(inputs)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

    #[test]
    pub fn test_input() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(SAMPLE)), 1588);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(SAMPLE)), 2188189693529);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2021/day14.txt");
        const ANSWERS: (usize, usize) = (5656, 12_271_437_788_530);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n'); // Trims trailing newline

            assert_eq!(part1(&generator(input)), ANSWERS.0);
            assert_eq!(part2(&generator(input)), ANSWERS.1);
        }
    }
}
