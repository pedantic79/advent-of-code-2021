use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, PartialEq, Clone)]
pub struct Fish {
    days: usize,
}

impl Fish {
    fn tick(&mut self) -> Option<Fish> {
        if self.days == 0 {
            self.days = 6;
            Some(Fish { days: 8 })
        } else {
            self.days -= 1;

            None
        }
    }
}

#[aoc_generator(day6)]
pub fn generator(input: &str) -> Vec<Fish> {
    input
        .split(',')
        .map(|x| Fish {
            days: x.parse().unwrap(),
        })
        .collect()
}

#[aoc(day6, part1)]
pub fn part1(inputs: &[Fish]) -> usize {
    let mut inputs = inputs.to_vec();
    for _ in 0..80 {
        let mut temp = Vec::new();

        for fish in inputs.iter_mut() {
            if let Some(new_fish) = fish.tick() {
                temp.push(new_fish);
            }
        }

        inputs.extend(temp);
    }

    inputs.len()
}

#[aoc(day6, part2)]
pub fn part2(inputs: &[Fish]) -> usize {
    let mut fish = [0; 9];

    for f in inputs {
        fish[f.days] += 1;
    }

    for _ in 0..256 {
        let mut new_fish = [0; 9];

        new_fish[..(9 - 1)].clone_from_slice(&fish[1..9]);
        new_fish[6] += fish[0];
        new_fish[8] += fish[0];

        fish = new_fish;
    }

    fish.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"3,4,3,1,2";

    #[test]
    pub fn test_input() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(SAMPLE)), 5934);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(SAMPLE)), 26984457539);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2021/day6.txt");
        const ANSWERS: (usize, usize) = (386640, 1733403626279);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n'); // Trims trailing newline

            assert_eq!(part1(&generator(input)), ANSWERS.0);
            assert_eq!(part2(&generator(input)), ANSWERS.1);
        }
    }
}
