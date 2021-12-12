use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

fn is_big(s: &str) -> bool {
    s.bytes().all(|c| c.is_ascii_uppercase())
}

#[aoc_generator(day12)]
pub fn generator(input: &str) -> HashMap<String, Vec<String>> {
    input
        .lines()
        .map(|l| {
            let (a, b) = l.split_once('-').unwrap();
            (a.to_string(), b.to_string())
        })
        .fold(HashMap::new(), |mut hm, (k, v)| {
            hm.entry(k.clone()).or_default().push(v.clone());
            hm.entry(v).or_default().push(k);
            hm
        })
}

fn search(
    caves: &HashMap<String, Vec<String>>,
    visited: &mut HashMap<String, usize>,
    location: &str,
    part2: bool,
) -> usize {
    if location == "end" {
        return 1;
    }

    *visited.entry(location.to_string()).or_insert(0) += 1;

    let mut total = 0;
    for v in &caves[location] {
        total += if visited.get(v).unwrap_or(&0) == &0 || is_big(v) {
            search(caves, visited, v, part2)
        } else if visited.get(v).unwrap_or(&0) == &1 && part2 {
            search(caves, visited, v, false)
        } else {
            0
        }
    }

    *visited.entry(location.to_string()).or_insert(0) -= 1;

    total
}

#[aoc(day12, part1)]
pub fn part1(inputs: &HashMap<String, Vec<String>>) -> usize {
    let mut visited = HashMap::new();

    search(inputs, &mut visited, "start", false)
}

#[aoc(day12, part2)]
pub fn part2(inputs: &HashMap<String, Vec<String>>) -> usize {
    let mut visited = HashMap::new();

    visited.insert("start".to_string(), 2);
    search(inputs, &mut visited, "start", true)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"start-A
start-b
A-c
A-b
b-d
A-end
b-end";

    //     const SAMPLE2: &str = r"dc-end
    // HN-start
    // start-kj
    // dc-start
    // dc-HN
    // LN-dc
    // HN-end
    // kj-sa
    // kj-HN
    // kj-dc";

    #[test]
    pub fn test_input() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(SAMPLE)), 10);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(SAMPLE)), 36);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2021/day12.txt");
        const ANSWERS: (usize, usize) = (4912, 150004);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n'); // Trims trailing newline

            assert_eq!(part1(&generator(input)), ANSWERS.0);
            assert_eq!(part2(&generator(input)), ANSWERS.1);
        }
    }
}
