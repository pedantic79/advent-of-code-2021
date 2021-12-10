use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day10)]
pub fn generator(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn get_match(bracket: char) -> Option<char> {
    match bracket {
        '(' => Some(')'),
        '{' => Some('}'),
        '[' => Some(']'),
        '<' => Some('>'),
        _ => None,
    }
}

fn process(line: &[char]) -> Result<Vec<char>, char> {
    let mut stack: Vec<char> = Vec::new();

    for &bracket in line.iter() {
        if let Some(right) = get_match(bracket) {
            stack.push(right);
        } else if Some(bracket) != stack.pop() {
            return Err(bracket);
        }
    }

    Ok(stack)
}

#[aoc(day10, part1)]
pub fn part1(inputs: &[Vec<char>]) -> usize {
    inputs
        .iter()
        .map(|line| {
            if let Err(e) = process(line) {
                score1(e)
            } else {
                0
            }
        })
        .sum()
}

fn score1(c: char) -> usize {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

fn score2(c: char) -> usize {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => 0,
    }
}

#[aoc(day10, part2)]
pub fn part2(inputs: &[Vec<char>]) -> usize {
    let ans = inputs
        .iter()
        .filter_map(|line| match process(line) {
            Ok(v) => Some(
                v.iter()
                    .rev()
                    .map(|&x| score2(x))
                    .fold(0, |acc, x| acc * 5 + x),
            ),
            Err(_) => None,
        })
        .sorted_unstable()
        .collect_vec();

    ans[ans.len() / 2]
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    pub fn test_input() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(SAMPLE)), 26397);
    }

    #[test]
    pub fn test_lines_part1() {
        let input = generator(SAMPLE);
        for &(index, expected) in [(2, '}'), (4, ')'), (5, ']'), (7, ')'), (8, '>')].iter() {
            assert_eq!(process(&input[index]).unwrap_err(), expected);
        }
    }

    #[test]
    pub fn test_lines_part2() {
        let input = generator(SAMPLE);

        for &(index, expected) in [
            (0, "}}]])})]"),
            (1, ")}>]})"),
            (3, "}}>}>))))"),
            (6, "]]}}]}]}>"),
            (9, "])}>"),
        ]
        .iter()
        {
            let mut ans = process(&input[index]).unwrap();
            ans.reverse();

            let expected = expected.chars().collect_vec();
            assert_eq!(ans, expected);
        }
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(SAMPLE)), 288957);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2021/day10.txt");
        const ANSWERS: (usize, usize) = (388713, 3539961434);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n'); // Trims trailing newline

            assert_eq!(part1(&generator(input)), ANSWERS.0);
            assert_eq!(part2(&generator(input)), ANSWERS.1);
        }
    }
}
