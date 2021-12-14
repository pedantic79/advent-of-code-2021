use std::{collections::HashMap, mem::swap};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use nohash_hasher::IntMap;

#[derive(Debug, PartialEq)]
pub struct Object {
    start: String,
    rules: IntMap<u16, u8>,
}

fn encode(a: u8, b: u8) -> u16 {
    ((a as u16) << 8) + b as u16
}

fn decode(x: u16) -> (u8, u8) {
    ((x >> 8) as u8, (x & 0xff) as u8)
}

fn encode_iter(mut i: impl Iterator<Item = u8>) -> u16 {
    encode(i.next().unwrap(), i.next().unwrap())
}

#[aoc_generator(day14)]
pub fn generator(input: &str) -> Object {
    let (part1, part2) = input.split_once("\n\n").unwrap();

    let start = part1.to_string();
    let rules = part2
        .lines()
        .map(|l| {
            let (a, b) = l.split_once(" -> ").unwrap();
            (encode_iter(a.bytes()), b.parse::<char>().unwrap() as u8)
        })
        .collect();

    Object { start, rules }
}

fn generate_count(input: &str) -> HashMap<u16, usize> {
    input
        .as_bytes()
        .windows(2)
        .fold(HashMap::new(), |mut count, w| {
            *count.entry(encode(w[0], w[1])).or_insert(0) += 1;
            count
        })
}

fn solve<const N: usize>(inputs: &Object) -> usize {
    let mut s = generate_count(&inputs.start);
    let mut new_s = HashMap::new();

    for _ in 0..N {
        for (k, count) in s.drain() {
            let &gen = inputs.rules.get(&k).unwrap();
            let (a, b) = decode(k);

            let (left, right) = (encode(a, gen), encode(gen, b));
            *new_s.entry(left).or_default() += count;
            *new_s.entry(right).or_default() += count;
        }

        swap(&mut s, &mut new_s);
    }

    let mut freq = s.iter().fold(HashMap::new(), |mut hm, c| {
        let (_, b) = decode(*c.0);
        *hm.entry(b).or_insert(0) += c.1;

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
