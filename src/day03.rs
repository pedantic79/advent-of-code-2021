use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
pub fn generator(input: &str) -> Vec<String> {
    input.lines().map(|s| s.to_owned()).collect()
}

#[aoc(day3, part1)]
pub fn part1(lines: &[String]) -> usize {
    let mut freq = vec![0; lines[0].len()];

    for l in lines {
        for (b, f) in l.bytes().zip(freq.iter_mut()) {
            if b == b'1' {
                *f += 1;
            }
        }
    }

    let threshold = lines.len() / 2;
    let (gamma, epsilon) = freq.iter().fold((0, 0), |(g, e), &x| {
        let (gx, ge) = if x > threshold { (1, 0) } else { (0, 1) };
        (g * 2 + gx, e * 2 + ge)
    });

    gamma * epsilon
}

fn count_bits<S: AsRef<str>>(data: &[S], pos: usize) -> (usize, usize) {
    data.iter()
        .map(|l| l.as_ref().as_bytes()[pos])
        .fold(
            (0, 0),
            |(x, y), b| if b == b'0' { (x + 1, y) } else { (x, y + 1) },
        )
}

#[aoc(day3, part2)]
pub fn part2(lines: &[String]) -> usize {
    let mut set_generator: Vec<_> = lines.iter().collect();
    let mut set_scrubber: Vec<_> = lines.iter().collect();

    for pos in 0..lines[0].len() {
        let g = count_bits(&set_generator, pos);
        let s = count_bits(&set_scrubber, pos);

        let g_common = if g.1 < g.0 { b'0' } else { b'1' };
        let s_common = if s.1 < s.0 { b'1' } else { b'0' };

        if set_generator.len() > 1 {
            set_generator.retain(|l| l.as_bytes()[pos] == g_common);
        }
        if set_scrubber.len() > 1 {
            set_scrubber.retain(|l| l.as_bytes()[pos] == s_common);
        }
    }

    usize::from_str_radix(set_generator[0], 2).unwrap()
        * usize::from_str_radix(set_scrubber[0], 2).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    pub fn test_input() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(SAMPLE)), 198);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(SAMPLE)), 230);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2021/day3.txt");
        const ANSWERS: (usize, usize) = (2250414, 6085575);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n'); // Trims trailing newline

            assert_eq!(part1(&generator(input)), ANSWERS.0);
            assert_eq!(part2(&generator(input)), ANSWERS.1);
        }
    }
}
