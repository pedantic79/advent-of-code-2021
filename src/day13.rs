use std::mem::swap;

use super::common::utils::parse_pair;
use ahash::AHashSet as HashSet;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[derive(Debug, PartialEq, Eq)]
pub struct Object {
    dots: HashSet<(usize, usize)>,
    ins: Vec<(char, usize)>,
}

#[aoc_generator(day13)]
pub fn generator(input: &str) -> Object {
    let (chunk_a, chunk_b) = input.split_once("\n\n").unwrap();

    let dots = chunk_a.lines().map(|l| parse_pair(l).unwrap()).collect();

    let ins = chunk_b
        .lines()
        .map(|l| {
            let mut it = l.split([' ', '='].as_ref());
            let dir = it.nth(2).unwrap();
            let num = it.next().unwrap();

            (dir.chars().last().unwrap(), num.parse().unwrap())
        })
        .collect();

    Object { dots, ins }
}

fn fold_x(input: &mut HashSet<(usize, usize)>, temp: &mut HashSet<(usize, usize)>, pos: usize) {
    temp.extend(
        input
            .drain()
            .map(|(x, y)| if x > pos { (2 * pos - x, y) } else { (x, y) }),
    )
}

fn fold_y(input: &mut HashSet<(usize, usize)>, temp: &mut HashSet<(usize, usize)>, pos: usize) {
    temp.extend(
        input
            .drain()
            .map(|(x, y)| if y > pos { (x, 2 * pos - y) } else { (x, y) }),
    );
}

fn fold(
    input: &mut HashSet<(usize, usize)>,
    temp: &mut HashSet<(usize, usize)>,
    axis: char,
    pos: usize,
) {
    if axis == 'y' {
        fold_y(input, temp, pos);
    } else {
        fold_x(input, temp, pos);
    }

    swap(input, temp);
}

#[aoc(day13, part1)]
pub fn part1(Object { dots, ins }: &Object) -> usize {
    let mut dots = dots.clone();
    let mut temp = HashSet::default();

    fold(&mut dots, &mut temp, ins[0].0, ins[0].1);
    dots.len()
}

#[aoc(day13, part2)]
pub fn part2(Object { dots, ins }: &Object) -> String {
    let mut dots = dots.clone();
    let mut temp = HashSet::default();

    for (a, b) in ins {
        fold(&mut dots, &mut temp, *a, *b);
    }

    let (y_min, y_max) = dots.iter().map(|(_, y)| *y).minmax().into_option().unwrap();
    let (x_min, x_max) = dots.iter().map(|(x, _)| *x).minmax().into_option().unwrap();

    let mut res = String::new();
    res.push('\n');

    for y in y_min..=y_max {
        let mut s = String::with_capacity(x_max - x_min + 1);
        for x in x_min..=x_max {
            if dots.contains(&(x, y)) {
                s.push('#');
            } else {
                s.push(' ');
            }
        }
        res.push_str(&s);
        res.push('\n');
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

    #[test]
    pub fn test_input() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(SAMPLE)), 17);
    }

    // #[test]
    // pub fn test2() {
    //     assert_eq!(part2(&generator(SAMPLE)), 336);
    // }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2021/day13.txt");
        const ANSWERS: (usize, &str) = (
            669,
            "
#  # #### #### ####  ##  #  #  ##    ##
#  # #    #       # #  # #  # #  #    #
#  # ###  ###    #  #    #  # #       #
#  # #    #     #   #    #  # #       #
#  # #    #    #    #  # #  # #  # #  #
 ##  #### #    ####  ##   ##   ##   ## \n",
        );

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
