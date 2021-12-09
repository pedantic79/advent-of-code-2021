use std::{collections::HashMap, convert::Infallible, str::FromStr};

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, PartialEq)]
pub struct Object {
    before: [(u8, u8); 10],
    after: [(u8, u8); 4],
}

impl FromStr for Object {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.split_once(" | ").unwrap();

        let mut before = build_array(a.split(' ').map(pack_digit));
        let after = build_array(b.split(' ').map(pack_digit));

        before.sort_unstable_by_key(|f| f.1);

        Ok(Object { before, after })
    }
}

fn build_array<T, I, const N: usize>(mut iter: I) -> [T; N]
where
    T: Default + Copy,
    I: Iterator<Item = T>,
{
    let mut res = [T::default(); N];

    // BAD... We don't know if there are enough iterator elements to fill or overfill the array.
    for slot in res.iter_mut() {
        *slot = iter.next().unwrap();
    }

    res
}

fn pack_digit(s: &str) -> (u8, u8) {
    s.bytes()
        .fold((0, 0), |(tot, count), c| (tot | 1 << (c - b'a'), count + 1))
}

fn get_range(len: u8) -> (usize, usize) {
    // constraints are ordered
    // [2, 3, 4, 5, 5, 5, 6, 6, 6, 7]
    match len {
        2 => (0, 1),
        3 => (1, 2),
        4 => (2, 3),
        5 => (3, 6),
        6 => (6, 9),
        7 => (9, 10),
        _ => unreachable!(),
    }
}

fn find(constraints: [(u8, u8); 10], len: u8, predicate: impl Fn(u8) -> bool) -> u8 {
    let (begin, end) = get_range(len);
    constraints[begin..end]
        .iter()
        // .filter(|x| x.1 == len)
        .find(|x| predicate(x.0))
        .unwrap()
        .0
}

fn analyze(constraints: [(u8, u8); 10]) -> HashMap<u8, usize> {
    //  aaaa      2 => [1]
    // b    c     3 => [7]
    // b    c     4 => [4]
    //  dddd      5 => [2, 3, 5]
    // e    f     6 => [0, 6, 9]
    // e    f     7 => [8]
    //  gggg

    let mut res = [0; 10];

    // These are easy, 1, 4, 7, 8 all have unique number of segments
    res[1] = find(constraints, 2, |_| true);
    res[4] = find(constraints, 4, |_| true);
    res[7] = find(constraints, 3, |_| true);
    res[8] = find(constraints, 7, |_| true);

    // Of the possible 5 segment numbers, only 3 share the same segments as 1
    res[3] = find(constraints, 5, |x| x & res[1] == res[1]);

    // 9 is made up of the union of 4 and 7
    let nine_mask = res[4] | res[7];
    res[9] = find(constraints, 6, |x| x & nine_mask == nine_mask);

    // the middle segment is made up of intersection of Ǝ and E (inverse of 1), e.g. ≡
    // and the intersection with 4 (the only number we know that has a middle segment but not top or bottom)
    let middle = (res[3] & !res[1]) & res[4];

    // 0 is the only 6 segment number without a middle
    res[0] = find(constraints, 6, |x| x & middle == 0);
    // 6 is the remaining 6 segment number, that aren't 0 or 9.
    res[6] = find(constraints, 6, |x| x != res[0] && x != res[9]);
    // 5 xor 6 with only 1 segment
    res[5] = find(constraints, 5, |x| (x ^ res[6]).count_ones() == 1);
    // 2 is the remaining 5 segment number that aren't 3 or 5
    res[2] = find(constraints, 5, |x| x != res[3] && x != res[5]);

    res.into_iter().enumerate().map(|(a, b)| (b, a)).collect()
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
        .filter(|x| matches!(x.1, 2 | 3 | 4 | 7))
        // .inspect(|w| println!("**{}", w))
        .count()
}

#[aoc(day8, part2)]
pub fn part2(inputs: &[Object]) -> usize {
    inputs
        .iter()
        .map(|line| {
            let perm = analyze(line.before);
            line.after
                .iter()
                .map(|digit| perm.get(&digit.0).unwrap())
                .fold(0, |acc, &n| acc * 10 + n)
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
