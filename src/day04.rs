use std::{convert::Infallible, str::FromStr};

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, PartialEq)]
pub struct Bingo {
    draw: Vec<u8>,
    boards: Vec<Board>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Board {
    data: [[Option<u8>; 5]; 5],
}

impl FromStr for Board {
    type Err = Infallible; // Bad. We're going to unwrap, to catch errors rather than trying to handle them.

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut data = [[None; 5]; 5];

        s.lines().enumerate().for_each(|(row, x)| {
            x.split(' ')
                .filter_map(|nstr| nstr.parse().ok())
                .enumerate()
                .for_each(|(col, n)| data[row][col] = Some(n));
        });

        Ok(Self { data })
    }
}

impl Board {
    fn check_bingo(&self) -> bool {
        self.data
            .iter()
            .any(|row| row.iter().all(|&cell| cell.is_none()))
            || (0..self.data.len())
                .any(|col_num| self.data.iter().all(|row| row[col_num].is_none()))
    }

    fn set_num(&mut self, num: u8) -> bool {
        self.data
            .iter_mut()
            .flat_map(|row| row.iter_mut())
            .find(|x| x == &&Some(num))
            .map(|o| *o = None)
            .is_some()
    }

    fn score(&self) -> usize {
        self.data
            .iter()
            .flat_map(|row| row.iter())
            .filter_map(|&cell| cell)
            .map(usize::from)
            .sum()
    }
}

#[aoc_generator(day4)]
pub fn generator(input: &str) -> Bingo {
    let mut itr = input.split("\n\n");
    let first = itr.next().unwrap();
    let draw = first.split(',').map(|x| x.parse().unwrap()).collect();
    let boards = itr.map(|s| s.parse::<Board>().unwrap()).collect();

    Bingo { draw, boards }
}

#[aoc(day4, part1)]
pub fn part1(inputs: &Bingo) -> usize {
    let mut boards = inputs.boards.to_vec();
    for &n in &inputs.draw {
        for b in boards.iter_mut() {
            if b.set_num(n) && b.check_bingo() {
                return b.score() * usize::from(n);
            }
        }
    }

    unreachable!()
}

#[aoc(day4, part2)]
pub fn part2(inputs: &Bingo) -> usize {
    let mut boards = inputs.boards.to_vec();
    for &n in &inputs.draw {
        for b in boards.iter_mut() {
            b.set_num(n);
        }

        if boards.len() > 1 {
            boards.retain(|b| !b.check_bingo());
        } else {
            // we seem to be able to skip this bingo check
            // if boards[0].check_bingo()
            return boards[0].score() * usize::from(n);
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    #[test]
    pub fn test_input() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(SAMPLE)), 4512);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(SAMPLE)), 1924);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2021/day4.txt");
        const ANSWERS: (usize, usize) = (8580, 9576);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n'); // Trims trailing newline

            assert_eq!(part1(&generator(input)), ANSWERS.0);
            assert_eq!(part2(&generator(input)), ANSWERS.1);
        }
    }
}
