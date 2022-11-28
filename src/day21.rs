use std::str::FromStr;

use super::common::utils::build_array;
use ahash::AHashMap as HashMap;
use aoc_runner_derive::{aoc, aoc_generator};
//use rustc_hash::FxHashMap as HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Default)]
pub struct Player {
    pos: u8,
    score: u16,
}

impl FromStr for Player {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, x) = s.split_once(':').ok_or("split_once(':') failed")?;
        let pos = x.trim_start().parse().map_err(|_| "u8::parse failed")?;

        Ok(Self::new(pos))
    }
}

struct Dice {
    last: usize,
    count: usize,
}

impl Dice {
    fn new() -> Self {
        Self { last: 7, count: 0 }
    }

    fn roll(&mut self) -> usize {
        self.count += 3;

        self.last -= 1;
        if self.last == 0 {
            self.last = 10;
        }

        self.last
    }

    fn count(&self) -> usize {
        self.count
    }
}

impl Player {
    fn new(pos: u8) -> Self {
        Self { pos, score: 0 }
    }

    fn new_with_score(pos: u8, score: u16) -> Self {
        Self { pos, score }
    }

    fn tick(&mut self, dice: &mut Dice) {
        self.pos = Self::inc_pos(self.pos + Self::roll(dice));
        self.score += u16::from(self.pos);
    }

    fn roll(dice: &mut Dice) -> u8 {
        dice.roll() as u8
    }

    fn inc_pos(pos: u8) -> u8 {
        (pos - 1) % 10 + 1
    }
}

#[aoc_generator(day21)]
pub fn generator(input: &str) -> [Player; 2] {
    build_array(input.lines().map(|l| l.parse().unwrap()))
}

#[aoc(day21, part1)]
pub fn part1(inputs: &[Player; 2]) -> usize {
    let mut player1 = inputs[0];
    let mut player2 = inputs[1];
    let mut dice = Dice::new();

    for _ in 1.. {
        player1.tick(&mut dice);
        if player1.score >= 1000 {
            return usize::from(player2.score) * dice.count();
        }

        player2.tick(&mut dice);
        if player2.score >= 1000 {
            return usize::from(player1.score) * dice.count();
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
        let pos = Player::inc_pos(player1.pos + dice);
        let (v2, v1) = solve2(
            memo,
            player2,
            Player::new_with_score(pos, player1.score + u16::from(pos)),
        );
        ways1 += v1 * ways;
        ways2 += v2 * ways;
    }

    memo.insert((player1, player2), (ways1, ways2));
    (ways1, ways2)
}

#[aoc(day21, part2)]
pub fn part2(inputs: &[Player]) -> usize {
    solve2(&mut HashMap::default(), inputs[0], inputs[1]).0
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
        use itertools::Itertools;

        let v: Vec<_> = (1..=100).cycle().take(300).collect_vec();
        let mut a = Vec::new();

        for chunk in &v.into_iter().chunks(3) {
            a.push((chunk.sum::<i32>() - 1) % 10 + 1);
        }
        println!("{:?}", a);
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
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
