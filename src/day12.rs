use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    hash::{BuildHasher, BuildHasherDefault, Hash, Hasher},
};

use aoc_runner_derive::{aoc, aoc_generator};
use lazy_static::lazy_static;
use nohash_hasher::NoHashHasher;

lazy_static! {
    static ref START: Cave = Cave::new("start");
    static ref END: Cave = Cave::new("end");
}

fn is_little(s: &str) -> bool {
    s.bytes().all(|c| c.is_ascii_lowercase())
}

#[derive(Debug, Eq, Clone, Copy)]
pub struct Cave {
    hash: u64,
    little: bool,
}

impl std::hash::Hash for Cave {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.hash.hash(state);
    }
}

impl PartialEq for Cave {
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash
    }
}

impl Cave {
    fn new(name: &str) -> Self {
        let mut s = DefaultHasher::default();
        name.hash(&mut s);
        let hash = s.finish();
        Self {
            hash,
            little: is_little(name),
        }
    }
}

type HashT = u64;
type CaveMap<K, V> = HashMap<K, V, BuildHasherDefault<NoHashHasher<HashT>>>;

fn make_hashmap<K, V>() -> CaveMap<K, V> {
    HashMap::with_hasher(BuildHasherDefault::<NoHashHasher<HashT>>::default())
}

#[aoc_generator(day12)]
pub fn generator(input: &str) -> CaveMap<Cave, Vec<Cave>> {
    input
        .lines()
        .map(|l| l.split_once('-').unwrap())
        .fold(make_hashmap(), |mut hm, (a, b)| {
            let x = Cave::new(a);
            let y = Cave::new(b);
            hm.entry(x).or_default().push(y);
            hm.entry(y).or_default().push(x);
            hm
        })
}

fn search<S: BuildHasher>(
    graph: &HashMap<Cave, Vec<Cave>, S>,
    visited: &mut HashMap<Cave, usize, S>,
    location: Cave,
    allow_two_visits: bool,
) -> usize {
    if location == *END {
        return 1;
    }

    *visited.entry(location).or_insert(0) += 1;

    let mut total = 0;
    for &cave in &graph[&location] {
        total += match visited.get(&cave).copied().unwrap_or(0) {
            0 => search(graph, visited, cave, allow_two_visits),
            _ if !cave.little => search(graph, visited, cave, allow_two_visits),
            1 if allow_two_visits => search(graph, visited, cave, false),
            _ => 0,
        };
    }

    *visited.entry(location).or_insert(0) -= 1;

    total
}

#[aoc(day12, part1)]
pub fn part1(input: &CaveMap<Cave, Vec<Cave>>) -> usize {
    search(input, &mut make_hashmap(), *START, false)
}

#[aoc(day12, part2)]
pub fn part2(inputs: &CaveMap<Cave, Vec<Cave>>) -> usize {
    let mut visited = make_hashmap();
    visited.insert(*START, 2);

    search(inputs, &mut visited, *START, true)
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
        assert_eq!(part1(&generator(SAMPLE)), 10);
        assert_eq!(part1(&generator(SAMPLE2)), 19);
        assert_eq!(part1(&generator(SAMPLE3)), 226);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(SAMPLE)), 36);
        assert_eq!(part2(&generator(SAMPLE2)), 103);
        assert_eq!(part2(&generator(SAMPLE3)), 3509);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2021/day12.txt");
        const ANSWERS: (usize, usize) = (4912, 150004);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
