use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};

type Coord = (isize, isize);

#[derive(Debug, PartialEq)]
pub struct Day20 {
    enhancement: Vec<bool>,
    image: Image,
}

#[derive(Debug, PartialEq, Clone)]
struct Image {
    image: HashSet<Coord>,
    min_x: isize,
    min_y: isize,
    max_x: isize,
    max_y: isize,
}

impl Image {
    fn get(&self, p: Coord, flip: bool) -> bool {
        (flip && (p.0 < self.min_y || p.0 > self.max_y || p.1 < self.min_x || p.1 > self.max_x))
            || self.image.contains(&p)
    }

    fn extract_zone(&self, row: isize, col: isize, flip: bool) -> usize {
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
        .map(|p| if self.get(p, flip) { 1 } else { 0 })
        .fold(0, |acc, n| acc * 2 + n)
    }
}

#[aoc_generator(day20)]
pub fn generator(input: &str) -> Day20 {
    let (a, b) = input.split_once("\n\n").unwrap();
    let enhancement: Vec<_> = a.chars().map(|c| c == '#').collect();

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
        .collect::<HashSet<_>>();

    let (min_y, max_y) = (0, b.lines().next().unwrap().len() as isize);
    let (min_x, max_x) = (min_y, b.lines().count() as isize);

    Day20 {
        enhancement,
        image: Image {
            image,
            min_x,
            min_y,
            max_x,
            max_y,
        },
    }
}

fn tick(lookup: &[bool], image: &Image, flip: bool) -> Image {
    let mut new_image = HashSet::new();
    let min_y = image.min_y - 1;
    let max_y = image.max_y + 1;
    let min_x = image.min_x - 1;
    let max_x = image.max_x + 1;

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let pos = image.extract_zone(y, x, lookup[0] && flip);

            if lookup[pos] {
                new_image.insert((y, x));
            }
        }
    }

    Image {
        image: new_image,
        min_x,
        max_x,
        min_y,
        max_y,
    }
}

fn solve<const N: usize>(inputs: &Day20) -> usize {
    let mut image = inputs.image.clone();

    for x in 0..N {
        image = tick(&inputs.enhancement, &image, x % 2 == 1);
    }
    image.image.len()
}

#[aoc(day20, part1)]
pub fn part1(inputs: &Day20) -> usize {
    solve::<2>(inputs)
}

#[aoc(day20, part2)]
pub fn part2(inputs: &Day20) -> usize {
    solve::<50>(inputs)
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

        assert_eq!(d.image.extract_zone(2, 2, false), 34)
    }

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(SAMPLE)), 35);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(SAMPLE)), 3351);
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
