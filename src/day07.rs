use aoc_runner_derive::{aoc, aoc_generator};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

const TRI: [isize; 2048] = precalculate_triangle_seq();

const fn precalculate_triangle_seq<const N: usize>() -> [isize; N] {
    let mut res = [0; N];

    let mut i = 0;
    while i < N {
        let j = i as isize;
        res[i] = j * (j + 1) / 2;
        i += 1;
    }

    res
}

#[aoc_generator(day7)]
pub fn generator(input: &str) -> Vec<isize> {
    input.split(',').map(|x| x.parse().unwrap()).collect()
}

#[aoc(day7, part1)]
pub fn part1(inputs: &[isize]) -> isize {
    let max = *inputs.iter().max().unwrap();

    (0..=max)
        .into_par_iter()
        .map(|n| inputs.iter().map(|x| (x - n).abs()).sum())
        .min()
        .unwrap()
}

#[aoc(day7, part2)]
pub fn part2(inputs: &[isize]) -> isize {
    let max = *inputs.iter().max().unwrap();

    (0..=max)
        .into_par_iter()
        .map(|n| {
            inputs
                .iter()
                .map(|x| TRI[(x - n).abs() as usize])
                .sum::<isize>()
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"16,1,2,0,4,2,7,1,2,14";

    #[test]
    pub fn test_input() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(SAMPLE)), 37);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(SAMPLE)), 168);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2021/day7.txt");
        const ANSWERS: (isize, isize) = (352997, 101571302);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n'); // Trims trailing newline

            assert_eq!(part1(&generator(input)), ANSWERS.0);
            assert_eq!(part2(&generator(input)), ANSWERS.1);
        }
    }
}
