use std::{
    fmt::{Debug, Display},
    hash::Hash,
    ops::{Add, Sub},
};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use rustc_hash::FxHashSet;

type C = i32;

#[derive(Eq, Clone, Copy, PartialOrd, Ord)]
pub struct Coord3(C, C, C);

impl Hash for Coord3 {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_i32(self.0 ^ self.1 ^ self.2);
    }
}

impl PartialEq for Coord3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1 && self.2 == other.2
    }
}

impl Display for Coord3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{},{},{}]", self.0, self.1, self.2)
    }
}

impl Debug for Coord3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Add for Coord3 {
    type Output = Coord3;

    fn add(self, rhs: Self) -> Self::Output {
        Coord3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Sub for Coord3 {
    type Output = Coord3;

    fn sub(self, rhs: Self) -> Self::Output {
        Coord3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl Coord3 {
    fn distance(&self, other: &Self) -> C {
        (self.0 - other.0).abs() + (self.1 - other.1).abs() + (self.2 - other.2).abs()
    }

    fn rot_xy(&self) -> Self {
        Coord3(self.1, -self.0, self.2)
    }

    fn rot_xz(&mut self) -> Self {
        Coord3(self.2, self.1, -self.0)
    }

    fn transforms(&self) -> [Coord3; 24] {
        let mut r = [Coord3(0, 0, 0); 24];

        r[0] = *self;
        r[1] = r[0].rot_xy();
        r[2] = r[1].rot_xy();
        r[3] = r[2].rot_xy();

        for x in 0..5 {
            r[x * 4 + 4] = r[x].rot_xz();
            r[x * 4 + 5] = r[x * 4 + 4].rot_xy();
            r[x * 4 + 6] = r[x * 4 + 5].rot_xy();
            r[x * 4 + 7] = r[x * 4 + 6].rot_xy();
        }

        r
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Default)]
pub struct Scanner(Vec<Coord3>);

impl Scanner {
    fn transforms(&self) -> [Scanner; 24] {
        let mut res = [
            Scanner::default(),
            Scanner::default(),
            Scanner::default(),
            Scanner::default(),
            Scanner::default(),
            Scanner::default(),
            Scanner::default(),
            Scanner::default(),
            Scanner::default(),
            Scanner::default(),
            Scanner::default(),
            Scanner::default(),
            Scanner::default(),
            Scanner::default(),
            Scanner::default(),
            Scanner::default(),
            Scanner::default(),
            Scanner::default(),
            Scanner::default(),
            Scanner::default(),
            Scanner::default(),
            Scanner::default(),
            Scanner::default(),
            Scanner::default(),
        ];

        for beacon in self.0.iter().map(|b| b.transforms()) {
            for (i, b) in beacon.iter().enumerate() {
                res[i].0.push(*b);
            }
        }

        res
    }
}

fn parse_vector(s: &str) -> Coord3 {
    let mut it = s.split(',').map(|x| x.parse::<C>().unwrap());

    Coord3(it.next().unwrap(), it.next().unwrap(), it.next().unwrap())
}

fn parse_chunk(s: &str) -> Scanner {
    Scanner(s.lines().skip(1).map(parse_vector).collect())
}

#[aoc_generator(day19)]
pub fn generator(input: &str) -> Vec<Scanner> {
    let chunk = input.split("\n\n");
    chunk.map(parse_chunk).collect()
}

fn solve(
    scanners: &[Scanner],
    total: &FxHashSet<Coord3>,
) -> Option<(Coord3, FxHashSet<Coord3>, usize)> {
    (0..scanners.len()).find_map(|i| {
        scanners[i]
            .transforms()
            .par_iter()
            .find_map_any(|r| merge(total, r))
            .map(|(d, s)| (d, s, i))
    })
}

fn merge(t: &FxHashSet<Coord3>, r: &Scanner) -> Option<(Coord3, FxHashSet<Coord3>)> {
    let mut nt = t.clone();

    for d in nt
        .iter()
        .cartesian_product(r.0.iter())
        .map(|(&t, &c)| t - c)
    {
        let mut count = 0;
        let mut remain = r.0.len();

        for c in r.0.iter() {
            let v: Coord3 = *c + d;
            remain -= 1;

            if t.contains(&v) {
                count += 1;

                if count == 12 {
                    nt.extend(r.0.iter().map(|&c| c + d));

                    return Some((d, nt));
                }
            } else if remain + count < 12 {
                // We can't ever reach 12, so stop

                break;
            }
        }
    }

    None
}

#[aoc(day19, part1)]
pub fn part1(inputs: &[Scanner]) -> usize {
    let mut inputs: Vec<_> = inputs.iter().rev().cloned().collect();

    let mut t = inputs.pop().unwrap().0.into_iter().collect();
    while let Some((_, nt, i)) = solve(&inputs, &t) {
        t = nt;
        inputs.swap_remove(i);
    }

    t.len()
}

#[aoc(day19, part2)]
pub fn part2(inputs: &[Scanner]) -> C {
    let mut inputs: Vec<_> = inputs.iter().rev().cloned().collect();
    let mut dists = Vec::new();

    let mut t = inputs.pop().unwrap().0.into_iter().collect();
    while let Some((dist, nt, i)) = solve(&inputs, &t) {
        t = nt;
        inputs.swap_remove(i);
        dists.push(dist);
    }

    dists
        .iter()
        .tuple_combinations()
        .map(|(a, b)| a.distance(b))
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {

    use super::*;

    mod input {
        pub const SAMPLE: &str = r"--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14";
    }

    #[test]
    pub fn test_input() {
        // assert_eq!(generator(SAMPLE), Day19());
    }

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(input::SAMPLE)), 79);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(input::SAMPLE)), 3621);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2021/day19.txt");
        const ANSWERS: (usize, C) = (465, 12149);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n'); // Trims trailing newline

            assert_eq!(part1(&generator(input)), ANSWERS.0);
            assert_eq!(part2(&generator(input)), ANSWERS.1);
        }
    }
}
