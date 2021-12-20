use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

type Coord = (isize, isize);

#[derive(Debug, PartialEq)]
pub struct Day20 {
    enhancement: Vec<bool>,
    image: HashSet<Coord>,
}

fn convert(c: char) -> bool {
    c == '#'
}

#[aoc_generator(day20)]
pub fn generator(input: &str) -> Day20 {
    let (a, b) = input.split_once("\n\n").unwrap();
    let enhancement: Vec<_> = a.chars().map(convert).collect();
    assert_eq!(enhancement.len(), 512);
    let image = b
        .lines()
        .enumerate()
        .flat_map(|(r, l)| {
            l.chars().enumerate().filter_map(move |(c, ch)| {
                if ch == '#' {
                    Some((r as isize, c as isize))
                } else {
                    None
                }
            })
        })
        .collect();

    Day20 { enhancement, image }
}

fn extract_zone(
    image: &HashSet<Coord>,
    row: isize,
    col: isize,
    get: impl Fn(&HashSet<Coord>, Coord) -> bool,
) -> usize {
    [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 0),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ]
    .iter()
    .map(|(dy, dx)| (row + dy, col + dx))
    .map(|p| if get(image, p) { 1 } else { 0 })
    .fold(0, |acc, n| acc * 2 + n)
}

fn tick(lookup: &[bool], image: &HashSet<Coord>, flip: bool) -> HashSet<Coord> {
    let mut new_image = HashSet::new();
    let (min_y, max_y) = image
        .iter()
        .map(|(y, _)| *y)
        .minmax()
        .into_option()
        .unwrap();
    let (min_x, max_x) = image
        .iter()
        .map(|(_, x)| *x)
        .minmax()
        .into_option()
        .unwrap();

    let get = |image: &HashSet<Coord>, p: Coord| {
        if lookup[0] && flip && (p.0 < min_y || p.0 > max_y || p.1 < min_x || p.1 > max_x) {
            true
        } else {
            image.contains(&p)
        }
    };

    for y in (min_y - 1)..=(max_y + 1) {
        for x in (min_x - 1)..=(max_x + 1) {
            let pos = extract_zone(image, y, x, get);

            if lookup[pos] {
                new_image.insert((y, x));
            }
        }
    }

    new_image
}

#[aoc(day20, part1)]
pub fn part1(inputs: &Day20) -> usize {
    let mut image = inputs.image.clone();

    image = tick(&inputs.enhancement, &image, false);
    image = tick(&inputs.enhancement, &image, true);

    image.len()
}

#[aoc(day20, part2)]
pub fn part2(inputs: &Day20) -> usize {
    let mut image = inputs.image.clone();

    for x in 0..50 {
        image = tick(&inputs.enhancement, &image, x % 2 == 1);
    }
    image.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";

    #[test]
    pub fn test_input() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn test_extract() {
        let d = generator(SAMPLE);

        assert_eq!(extract_zone(&d.image, 2, 2, |img, p| img.contains(&p)), 34)
    }

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(SAMPLE)), 35);
    }

    #[test]
    pub fn test2() {
        // assert_eq!(part2(&generator(SAMPLE)), 336);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2021/day20.txt");
        const ANSWERS: (usize, usize) = (5622, 20395);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n'); // Trims trailing newline

            assert_eq!(part1(&generator(input)), ANSWERS.0);
            assert_eq!(part2(&generator(input)), ANSWERS.1);
        }
    }
}
