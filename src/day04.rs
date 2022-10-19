use std::{convert::Infallible, str::FromStr};

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, PartialEq, Eq)]
pub struct Bingo {
    draw: Vec<usize>,
    boards: Vec<Board>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Board {
    row: [[Option<u8>; 5]; 5],            // row major version of the board
    col: [[Option<u8>; 5]; 5],            // column major version of the board
    index: [Option<(usize, usize)>; 100], // lookup table for the (r,c) location
    score: usize,                         // keep track of the total unmarked score
    winner: bool,                         // keep track if we've won
}

impl FromStr for Board {
    // Technically bad, we should make sure that there are the correct number of numbers, but we silently ignore them
    // so we can handle the extra spaces.
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut row = [[None; 5]; 5];
        let mut col = [[None; 5]; 5];

        let mut index = [None; 100];
        let mut score = 0;

        s.lines().enumerate().for_each(|(r, x)| {
            x.split(' ')
                .filter_map(|nstr| nstr.parse::<u8>().ok())
                .enumerate()
                .for_each(|(c, n)| {
                    // precompute everything to save time when processing
                    index[usize::from(n)] = Some((r, c));
                    row[r][c] = Some(n);
                    col[c][r] = Some(n);
                    score += usize::from(n);
                });
        });

        Ok(Self {
            row,
            col,
            index,
            score,
            winner: false,
        })
    }
}

impl Board {
    // Pass the (r, c) to cheaply check for bingo
    fn check_bingo(&mut self, r: usize, c: usize) -> bool {
        if self.row[r].iter().all(|&cell| cell.is_none())
            || self.col[c].iter().all(|&cell| cell.is_none())
        {
            self.winner = true;
        }

        self.winner
    }

    fn set_num(&mut self, num: usize) -> bool {
        if let Some((r, c)) = self.index[num] {
            self.row[r][c] = None;
            self.col[c][r] = None;
            self.score -= num;
            self.check_bingo(r, c)
        } else {
            false
        }
    }
}

#[aoc_generator(day4)]
pub fn generator(input: &str) -> Bingo {
    // Using unwrap, but that is mostly because it's easy and to fail early rather than hide it in error handling.
    // Error handling could be improved
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
            if b.set_num(n) {
                return b.score * n;
            }
        }
    }

    unreachable!()
}

#[aoc(day4, part2)]
pub fn part2(inputs: &Bingo) -> usize {
    let mut boards = inputs.boards.to_vec();
    let mut len = boards.len();

    for &n in &inputs.draw {
        for b in boards.iter_mut().filter(|b| !b.winner) {
            if b.set_num(n) {
                len -= 1;

                if len == 0 {
                    return b.score * n;
                }
            }
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
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
