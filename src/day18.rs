use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    branch::alt,
    character::complete::{char, digit1},
    combinator::map,
    sequence::{delimited, separated_pair},
    IResult,
};

#[derive(Debug, PartialEq, Clone)]
pub enum Day18 {
    Num(usize),
    Pair(Box<Day18>, Box<Day18>),
}

impl Day18 {
    fn add_left(&mut self, b: usize) {
        match self {
            Day18::Num(n) => *n += b,
            Day18::Pair(l, _) => l.add_left(b),
        };
    }

    fn add_right(&mut self, b: usize) {
        match self {
            Day18::Num(n) => *n += b,
            Day18::Pair(_, r) => r.add_right(b),
        };
    }

    fn magnitude(&self) -> usize {
        match self {
            Day18::Num(n) => *n,
            Day18::Pair(a, b) => 3 * a.magnitude() + 2 * b.magnitude(),
        }
    }

    fn split(&mut self) -> bool {
        match self {
            Day18::Num(n) if *n < 10 => false,
            Day18::Num(n) => {
                let (l, r) = (*n / 2, (*n + 1) / 2);
                *self = Day18::Pair(Box::new(Day18::Num(l)), Box::new(Day18::Num(r)));
                true
            }
            Day18::Pair(a, b) => a.split() || b.split(),
        }
    }

    fn reduce(mut self) -> Self {
        loop {
            // println!("{:?}", snail);
            if self.explode(0).is_some() {
                continue;
            }

            if !self.split() {
                break;
            }
        }

        self
    }

    fn explode(&mut self, depth: usize) -> Option<(usize, usize)> {
        match self {
            Day18::Num(_) => None,
            Day18::Pair(a, b) => match (&mut **a, &mut **b) {
                (Day18::Num(x), Day18::Num(y)) if depth == 4 => {
                    let x = *x;
                    let y = *y;
                    *self = Day18::Num(0);
                    Some((x, y))
                }
                (Day18::Num(_), Day18::Num(_)) => None,
                (l, r) => {
                    if let Some((lhs_left, lhs_right)) = l.explode(depth + 1) {
                        b.add_left(lhs_right);
                        Some((lhs_left, 0))
                    } else if let Some((rhs_left, rhs_right)) = r.explode(depth + 1) {
                        a.add_right(rhs_left);
                        Some((0, rhs_right))
                    } else {
                        None
                    }
                }
            },
        }
    }
}

fn parse(s: &str) -> IResult<&str, Day18> {
    delimited(char('['), pair, char(']'))(s)
}

fn pair(s: &str) -> IResult<&str, Day18> {
    map(
        separated_pair(alt((number, parse)), char(','), alt((number, parse))),
        |(left, right)| Day18::Pair(Box::new(left), Box::new(right)),
    )(s)
}

fn number(s: &str) -> IResult<&str, Day18> {
    map(digit1, |n: &str| Day18::Num(n.parse().unwrap()))(s)
}

fn add(left: Day18, right: Day18) -> Day18 {
    Day18::Pair(Box::new(left), Box::new(right))
}

#[aoc_generator(day18)]
pub fn generator(input: &str) -> Vec<Day18> {
    input.lines().map(|x| parse(x).unwrap().1).collect()
}

#[aoc(day18, part1)]
pub fn part1(inputs: &[Day18]) -> usize {
    inputs
        .iter()
        .cloned()
        .reduce(|acc, snailfish| add(acc, snailfish).reduce())
        .unwrap()
        .magnitude()
}

#[aoc(day18, part2)]
pub fn part2(inputs: &[Day18]) -> usize {
    let mut max = 0;

    for a in inputs.iter() {
        for b in inputs.iter() {
            if a == b {
                continue;
            }
            max = max.max(add(a.clone(), b.clone()).reduce().magnitude())
        }
    }

    max
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";

    #[test]
    pub fn test_input() {
        for x in generator(SAMPLE) {
            println!("{:?}", x);
        }

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(SAMPLE)), 4140);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(SAMPLE)), 3993);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2021/day18.txt");
        const ANSWERS: (usize, usize) = (4072, 4483);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n'); // Trims trailing newline

            assert_eq!(part1(&generator(input)), ANSWERS.0);
            assert_eq!(part2(&generator(input)), ANSWERS.1);
        }
    }
}
