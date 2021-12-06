use aoc_runner_derive::{aoc, aoc_generator};

fn solve<const N: usize>(input: &[usize]) -> usize {
    let mut fish = [0; 9];

    for &f in input {
        fish[f] += 1;
    }

    for _ in 0..N {
        let mut new_fish = [0; 9];

        new_fish[..8].copy_from_slice(&fish[1..9]);
        new_fish[6] += fish[0];
        new_fish[8] += fish[0];

        fish = new_fish;
    }

    fish.iter().sum()
}

#[aoc_generator(day6)]
pub fn generator(input: &str) -> Vec<usize> {
    input.split(',').map(|x| x.parse().unwrap()).collect()
}

#[aoc(day6, part1)]
pub fn part1(inputs: &[usize]) -> usize {
    solve::<80>(inputs)
}

#[aoc(day6, part2)]
pub fn part2(inputs: &[usize]) -> usize {
    solve::<256>(inputs)
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
