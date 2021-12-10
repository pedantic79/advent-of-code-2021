use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day10)]
pub fn generator(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec()
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
    const VALID: [char; 8] = ['(', ')', '[', ']', '{', '}', '<', '>'];
    let mut stack: Vec<char> = Vec::new();

    for &bracket in line.iter().filter(|x| VALID.contains(x)) {
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
        .filter_map(|line| match process(line) {
            Ok(_) => None,
            Err(e) => {
                // println!("{:?} => {}", line, e);
                convert(e)
            }
        })
        .sum()
}

fn convert(c: char) -> Option<usize> {
    Some(match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => return None,
    })
}

fn convert2(c: char) -> Option<usize> {
    Some(match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => return None,
    })
}

#[aoc(day10, part2)]
pub fn part2(inputs: &[Vec<char>]) -> usize {
    let mut ans = inputs
        .iter()
        .filter_map(|line| match process(line) {
            Ok(v) => Some(
                v.iter()
                    .rev()
                    .filter_map(|&x| convert2(x))
                    .fold(0, |acc, x| acc * 5 + x),
            ),
            Err(_) => None,
        })
        .collect_vec();

    ans.sort_unstable();

    // println!("{:?}", ans);
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
    pub fn test_line() {
        let input: Vec<char> = "[({(<(())[]>[[{[]{<()<>>".chars().collect();

        println!("{:?}", process(&input));
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(SAMPLE)), 336);
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
