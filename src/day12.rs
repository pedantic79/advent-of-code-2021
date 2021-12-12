use std::collections::HashMap;

use aoc_runner_derive::aoc;

fn is_little(s: &str) -> bool {
    s.bytes().all(|c| c.is_ascii_lowercase())
}

// #[aoc_generator(day12)]
pub fn generator(input: &str) -> HashMap<&str, Vec<&str>> {
    input
        .lines()
        .map(|l| l.split_once('-').unwrap())
        .fold(HashMap::new(), |mut hm, (a, b)| {
            hm.entry(a).or_default().push(b);
            hm.entry(b).or_default().push(a);
            hm
        })
}

fn search<'a>(
    graph: &HashMap<&str, Vec<&'a str>>,
    visited: &mut HashMap<&'a str, usize>,
    location: &'a str,
    allow_two_visits: bool,
) -> usize {
    if location == "end" {
        return 1;
    }

    *visited.entry(location).or_insert(0) += 1;

    let mut total = 0;
    for &cave in &graph[location] {
        total += match visited.get(cave).copied().unwrap_or(0) {
            0 => search(graph, visited, cave, allow_two_visits),
            _ if !is_little(cave) => search(graph, visited, cave, allow_two_visits),
            1 if allow_two_visits => search(graph, visited, cave, false),
            _ => 0,
        };
    }

    *visited.entry(location).or_insert(0) -= 1;

    total
}

#[aoc(day12, part1)]
pub fn part1(input: &str) -> usize {
    search(&generator(input), &mut HashMap::new(), "start", false)
}

#[aoc(day12, part2)]
pub fn part2(inputs: &str) -> usize {
    let mut visited = [("start", 2)].into_iter().collect();

    search(&generator(inputs), &mut visited, "start", true)
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

    const SAMPLE2: &str = r"dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";

    const SAMPLE3: &str = r"fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";

    #[test]
    pub fn test_input() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn test1() {
        assert_eq!(part1(SAMPLE), 10);
        assert_eq!(part1(SAMPLE2), 19);
        assert_eq!(part1(SAMPLE3), 226);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(SAMPLE), 36);
        assert_eq!(part2(SAMPLE2), 103);
        assert_eq!(part2(SAMPLE3), 3509);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2021/day12.txt");
        const ANSWERS: (usize, usize) = (4912, 150004);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n'); // Trims trailing newline

            assert_eq!(part1(input), ANSWERS.0);
            assert_eq!(part2(input), ANSWERS.1);
        }
    }
}
