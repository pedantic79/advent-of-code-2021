use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

use crate::utils::build_array;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Default)]
pub struct Player {
    pos: usize,
    score: usize,
}

struct Dice {
    last: usize,
}

impl Dice {
    fn roll(&mut self) -> usize {
        self.last = self.last % 100 + 1;

        self.last
    }
}

impl Player {
    fn new(pos: usize) -> Self {
        Player { pos, score: 0 }
    }

    fn tick(&mut self, dice: &mut Dice) {
        let n = dice.roll() + dice.roll() + dice.roll();
        self.pos += n;
        while self.pos > 10 {
            self.pos -= 10;
        }
        self.score += self.pos;
    }
}

#[aoc_generator(day21)]
pub fn generator(input: &str) -> [Player; 2] {
    build_array(
        input
            .lines()
            .map(|l| Player::new(l.split_once(':').unwrap().1.trim_start().parse().unwrap())),
    )
}

#[aoc(day21, part1)]
pub fn part1(inputs: &[Player; 2]) -> usize {
    let mut player1 = inputs[0];
    let mut player2 = inputs[1];
    let mut dice = Dice { last: 0 };

    let mut count = 0;
    for _ in 1.. {
        player1.tick(&mut dice);
        count += 3;

        if player1.score >= 1000 {
            return player2.score * count;
        }

        player2.tick(&mut dice);
        count += 3;
        if player2.score >= 1000 {
            return player1.score * count;
        }
    }

    unreachable!()
}

fn solve2(
    memo: &mut HashMap<(Player, Player), (usize, usize)>,
    player1: Player,
    player2: Player,
) -> (usize, usize) {
    if let Some(ans) = memo.get(&(player1, player2)) {
        return *ans;
    }

    if player2.score >= 21 {
        memo.insert((player1, player2), (0, 1));
        return (0, 1);
    }

    let (mut ways1, mut ways2) = (0, 0);
    for (dice, ways) in [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)] {
        let pos = (player1.pos + dice - 1) % 10 + 1;
        let (v2, v1) = solve2(
            memo,
            player2,
            Player {
                pos,
                score: player1.score + pos,
            },
        );
        ways1 += v1 * ways;
        ways2 += v2 * ways;
    }

    memo.insert((player1, player2), (ways1, ways2));
    (ways1, ways2)
}

#[aoc(day21, part2)]
pub fn part2(inputs: &[Player]) -> usize {
    solve2(&mut HashMap::new(), inputs[0], inputs[1]).0
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"Player 1 starting position: 4
Player 2 starting position: 8";

    #[test]
    pub fn test_input() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(SAMPLE)), 739785);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(SAMPLE)), 444356092776315);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2021/day21.txt");
        const ANSWERS: (usize, usize) = (921585, 911090395997650);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n'); // Trims trailing newline

            assert_eq!(part1(&generator(input)), ANSWERS.0);
            assert_eq!(part2(&generator(input)), ANSWERS.1);
        }
    }
}
