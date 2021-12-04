use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, PartialEq)]
pub struct Object {
    draw: Vec<usize>,
    boards: Vec<Board>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Board {
    state: [[bool; 5]; 5],
    data: Vec<Vec<usize>>,
}

impl FromStr for Board {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = s
            .lines()
            .map(|x| {
                x.split(' ')
                    .filter_map(|n| n.parse::<usize>().ok())
                    .collect()
            })
            .collect();

        Ok(Self {
            state: [[false; 5]; 5],
            data,
        })
    }
}

impl Board {
    fn check_bingo(&self) -> bool {
        self.state.iter().any(|row| row.iter().all(|&cell| cell))
            || (0..5).any(|col_num| self.state.iter().all(|row| row[col_num]))
    }

    fn set_num(&mut self, num: usize) -> bool {
        for r in 0..5 {
            for c in 0..5 {
                if self.data[r][c] == num {
                    self.state[r][c] = true;
                    return true;
                }
            }
        }

        false
    }

    fn score(&self) -> usize {
        let mut total = 0;
        for r in 0..5 {
            for c in 0..5 {
                if !self.state[r][c] {
                    total += self.data[r][c]
                }
            }
        }

        total
    }
}

#[aoc_generator(day4)]
pub fn generator(input: &str) -> Object {
    let mut itr = input.split("\n\n");
    let first = itr.next().unwrap();
    let draw = first.split(',').map(|x| x.parse().unwrap()).collect();
    let boards = itr.map(|s| s.parse::<Board>().unwrap()).collect();

    Object { draw, boards }
}

#[aoc(day4, part1)]
pub fn part1(inputs: &Object) -> usize {
    let mut boards = inputs.boards.to_vec();
    for &n in &inputs.draw {
        for b in boards.iter_mut() {
            if b.set_num(n) && b.check_bingo() {
                return b.score() * n;
            }
        }
    }

    unimplemented!()
}

#[aoc(day4, part2)]
pub fn part2(inputs: &Object) -> usize {
    let mut boards = inputs.boards.to_vec();
    for &n in &inputs.draw {
        if boards.len() == 1 {
            boards[0].set_num(n);
            println!("{:?}", boards[0]);
            return boards[0].score() * n;
        }

        loop {
            let mut flag = false;
            for (pos, b) in boards.iter_mut().enumerate() {
                if b.set_num(n) && b.check_bingo() {
                    boards.remove(pos);
                    flag = true;
                    break;
                }
            }
            if !flag {
                break;
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
            let input = INPUT.trim_end_matches('\n'); // Trims trailing newline

            assert_eq!(part1(&generator(input)), ANSWERS.0);
            assert_eq!(part2(&generator(input)), ANSWERS.1);
        }
    }
}
