use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
pub fn generator(input: &str) -> Vec<usize> {
    input.lines().map(|s| s.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn part1(inputs: &[usize]) -> usize {
    inputs.windows(2).filter(|w| w[0] < w[1]).count()
}

#[aoc(day1, part2)]
pub fn part2(inputs: &[usize]) -> usize {
    inputs
        .windows(4)
        .filter(|w| {
            let a: usize = w[..3].iter().sum();
            let b: usize = w[1..].iter().sum();
            a < b
        })
        .count()
}

fn solve_fastest<const N: usize>(inputs: &[usize]) -> usize {
    let mut count = 0;

    for i in N..inputs.len() {
        if inputs[i - N] < inputs[i] {
            count += 1;
        }
    }

    count
}

#[aoc(day1, part1, fastest)]
pub fn part1_fastest(inputs: &[usize]) -> usize {
    solve_fastest::<1>(inputs)
}

#[aoc(day1, part2, fastest)]
pub fn part2_fastest(inputs: &[usize]) -> usize {
    solve_fastest::<3>(inputs)
}

// fn solve_fast<const N: usize>(inputs: &[usize]) -> usize {
//     inputs.windows(N + 1).filter(|w| w[0] < w[N]).count()
// }

// #[aoc(day1, part1, fast)]
// pub fn part1_fast(inputs: &[usize]) -> usize {
//     solve_fast::<1>(inputs)
// }

// #[aoc(day1, part2, fast)]
// pub fn part2_fast(inputs: &[usize]) -> usize {
//     solve_fast::<3>(inputs)
// }

// #[aoc(day1, part2, fold)]
// pub fn part2_fold(inputs: &[usize]) -> usize {
//     inputs
//         .windows(4)
//         .fold((inputs[..3].iter().sum::<usize>(), 0), |(sum, count), w| {
//             let new_sum = sum - w[0] + w[3];
//             if new_sum > sum {
//                 (new_sum, count + 1)
//             } else {
//                 (new_sum, count)
//             }
//         })
//         .1
// }

// #[aoc(day1, part2, queue)]
// pub fn part2_queue(inputs: &[usize]) -> usize {
//     assert!(inputs.len() > 3);
//     let mut windows = std::collections::VecDeque::with_capacity(3);
//     windows.extend(&inputs[..3]);
//     let first3: usize = inputs[..3].iter().sum();

//     inputs[3..]
//         .iter()
//         .scan(first3, |sum, n| {
//             let old = *sum;
//             *sum = *sum - windows.pop_front().unwrap() + n;
//             windows.push_back(n);
//             Some(old < *sum)
//         })
//         .filter(|&x| x)
//         .count()
// }

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"199
200
208
210
200
207
240
269
260
263";

    #[test]
    pub fn test_input() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn test1() {
        assert_eq!(part1_fastest(&generator(SAMPLE)), 7);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2_fastest(&generator(SAMPLE)), 5);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2021/day1.txt");
        const ANSWERS: (usize, usize) = (1676, 1706);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1_fastest(&output), ANSWERS.0);
            assert_eq!(part2_fastest(&output), ANSWERS.1);
        }
    }
}
