use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

const SIZE: usize = 26;

#[derive(Debug, PartialEq, Eq)]
pub struct Day14 {
    start: String,
    rules: Vec<(u8, u8, u8)>,
}

fn ord(a: u8) -> usize {
    usize::from(a - b'A')
}

fn encode(a: u8, b: u8) -> usize {
    ord(a) * SIZE + ord(b)
}

#[aoc_generator(day14)]
pub fn generator(input: &str) -> Day14 {
    let (part1, part2) = input.split_once("\n\n").unwrap();

    let start = part1.to_string();
    let rules = part2
        .lines()
        .map(|l| {
            let (a, b) = l.split_once(" -> ").unwrap();
            let b = b.parse::<char>().unwrap() as u8;
            let a = a.as_bytes();
            (a[0], a[1], b)
        })
        .collect();

    Day14 { start, rules }
}

fn generate_count(input: &str) -> [usize; SIZE * SIZE] {
    input
        .as_bytes()
        .windows(2)
        .fold([0; SIZE * SIZE], |mut count, w| {
            count[encode(w[0], w[1])] += 1;
            count
        })
}

fn solve<const N: usize>(inputs: &Day14) -> usize {
    let mut s = generate_count(&inputs.start);

    for _ in 0..N {
        let mut new_s = [0; SIZE * SIZE];

        for &(a, b, gen) in inputs.rules.iter() {
            let count = s[encode(a, b)];
            let (left, right) = (encode(a, gen), encode(gen, b));
            new_s[left] += count;
            new_s[right] += count;
        }
        s = new_s;
    }

    let mut freq =
        s.iter()
            .enumerate()
            .filter(|(_, x)| **x > 0)
            .fold([0; SIZE], |mut hm, (k, &c)| {
                hm[k % SIZE] += c;
                hm
            });

    freq[ord(inputs.start.as_bytes()[0])] += 1;

    let (&min, &max) = freq
        .iter()
        .filter(|x| **x > 0)
        .minmax_by_key(|&count| count)
        .into_option()
        .unwrap();

    max - min
}

#[aoc(day14, part1)]
pub fn part1(inputs: &Day14) -> usize {
    solve::<10>(inputs)
}

#[aoc(day14, part2)]
pub fn part2(inputs: &Day14) -> usize {
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
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
