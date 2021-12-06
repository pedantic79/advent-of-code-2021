use aoc_runner_derive::{aoc, aoc_generator};

fn solve<const N: usize>(mut fish: [usize; 9]) -> usize {
    for x in 0..N {
        let base = x % 9;
        let new_fish = fish[base];
        fish[(base + 7) % 9] += new_fish;
    }

    fish.iter().sum()
}

#[aoc_generator(day6)]
pub fn generator(input: &str) -> [usize; 9] {
    input
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .fold([0; 9], |mut a, n| {
            a[n] += 1;
            a
        })
}

#[aoc(day6, part1)]
pub fn part1(inputs: &[usize; 9]) -> usize {
    solve::<80>(*inputs)
}

#[aoc(day6, part2)]
pub fn part2(inputs: &[usize; 9]) -> usize {
    solve::<256>(*inputs)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"3,4,3,1,2";

    #[test]
    pub fn test_input() {
        println!("{:?}", generator(SAMPLE));

        assert_eq!(generator(SAMPLE), [0, 1, 1, 2, 1, 0, 0, 0, 0]);
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
