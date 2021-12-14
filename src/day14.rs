use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

use crate::utils::build_array;

#[derive(Debug, PartialEq)]
pub struct Object {
    start: String,
    rules: HashMap<[u8; 2], char>,
}

#[aoc_generator(day14)]
pub fn generator(input: &str) -> Object {
    let (part1, part2) = input.split_once("\n\n").unwrap();

    let start = part1.to_string();
    let rules = part2
        .lines()
        .map(|l| {
            let (a, b) = l.split_once(" -> ").unwrap();
            (build_array(a.bytes()), b.parse().unwrap())
        })
        .collect();

    Object { start, rules }
}

fn tick(input: &str, lookup: &HashMap<[u8; 2], char>) -> String {
    let v = input.as_bytes();

    let mut output = String::new();
    output.push(v[0] as char);

    for w in v.windows(2) {
        let c = lookup.get(w).unwrap();

        output.push(*c);
        output.push(w[1] as char);
    }

    output
}

fn generate_count(input: &str, lookup: &HashMap<[u8; 2], char>) -> HashMap<[u8; 2], usize> {
    let mut count = HashMap::new();

    for w in input.as_bytes().windows(2) {
        *count.entry(build_array(w.iter().copied())).or_insert(0) += 1;
    }

    count
}

#[aoc(day14, part1)]
pub fn part1(inputs: &Object) -> usize {
    let mut s = inputs.start.clone();

    for _ in 0..10 {
        s = tick(&s, &inputs.rules);
    }

    let freq = s.chars().fold(HashMap::new(), |mut hm, c| {
        *hm.entry(c).or_insert(0) += 1;
        hm
    });

    let (min, max) = freq
        .iter()
        .minmax_by_key(|&(_, &count)| count)
        .into_option()
        .unwrap();

    freq[max.0] - freq[min.0]
}

#[aoc(day14, part2)]
pub fn part2(inputs: &Object) -> usize {
    let mut s = generate_count(&inputs.start, &inputs.rules);

    for _ in 0..40 {
        let mut new_s = HashMap::new();
        for (k, count) in s.iter() {
            let &gen = inputs.rules.get(k).unwrap();
            let (left, right) = ([k[0], gen as u8], [gen as u8, k[1]]);
            *new_s.entry(left).or_default() += count;
            *new_s.entry(right).or_default() += count;
        }

        s = new_s;
    }

    let mut freq = s.iter().fold(HashMap::new(), |mut hm, c| {
        *hm.entry(c.0[1] as char).or_insert(0) += c.1;

        hm
    });

    *freq.entry(inputs.start.as_bytes()[0] as char).or_insert(0) += 1;

    let (min, max) = freq
        .iter()
        .minmax_by_key(|&(_, &count)| count)
        .into_option()
        .unwrap();

    println!("{:?}", freq);
    freq[max.0] - freq[min.0]
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
