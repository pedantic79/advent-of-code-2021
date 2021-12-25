use aoc_runner_derive::{aoc, aoc_generator};

const TRI: [usize; 2048] = precalculate_triangle_seq();

const fn precalculate_triangle_seq<const N: usize>() -> [usize; N] {
    let mut res = [0; N];

    let mut i = 0;
    while i < N {
        res[i] = i * (i + 1) / 2;
        i += 1;
    }

    res
}

#[inline]
fn sub_abs(a: usize, b: usize) -> usize {
    let a = a as isize;
    let b = b as isize;
    (a - b).abs() as usize
}

#[aoc_generator(day7)]
pub fn generator(input: &str) -> Vec<usize> {
    let mut v: Vec<usize> = input.split(',').map(|x| x.parse().unwrap()).collect();
    v.sort_unstable();
    v
}

#[aoc(day7, part1)]
pub fn part1(inputs: &[usize]) -> usize {
    let median = inputs[inputs.len() / 2];

    inputs.iter().map(|&x| sub_abs(x, median)).sum()
}

#[aoc(day7, part2)]
pub fn part2(inputs: &[usize]) -> usize {
    let mean: usize = inputs.iter().sum::<usize>() / inputs.len();

    (mean..=(mean + 1))
        .map(|n| inputs.iter().map(|&x| TRI[sub_abs(x, n)]).sum())
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
        const ANSWERS: (usize, usize) = (352997, 101571302);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
