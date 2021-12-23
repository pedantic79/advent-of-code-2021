use std::{
    fmt::{Debug, Display},
    iter::from_fn,
};

use aoc_runner_derive::{aoc, aoc_generator};
use pathfinding::prelude::dijkstra;

// #############
// #01.2.3.4.56#
// ###D#A#C#D###
//   #D#C#B#A#
//   #D#B#A#C#
//   #C#A#B#B#
//   #########

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Amphipod {
    A,
    B,
    C,
    D,
    Empty,
}

impl Default for Amphipod {
    fn default() -> Self {
        Self::Empty
    }
}

impl Display for Amphipod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Amphipod::A => write!(f, "A"),
            Amphipod::B => write!(f, "B"),
            Amphipod::C => write!(f, "C"),
            Amphipod::D => write!(f, "D"),
            Amphipod::Empty => write!(f, "."),
        }
    }
}

impl Amphipod {
    fn cost_per(&self) -> usize {
        match *self {
            Amphipod::A => 1,
            Amphipod::B => 10,
            Amphipod::C => 100,
            Amphipod::D => 1000,
            Amphipod::Empty => todo!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default, Hash)]
pub struct Slots([Amphipod; 7]);
impl Display for Slots {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "#{}{}.{}.{}.{}.{}{}#",
            self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], self.0[5], self.0[6],
        )
    }
}

impl Slots {
    fn is_empty(&self) -> bool {
        self.0.iter().all(|a| a == &Amphipod::Empty)
    }

    fn is_clear(&self, pos: usize, l: usize, r: usize) -> bool {
        if pos <= l {
            (pos..=l).all(|x| self.0[x] == Amphipod::Empty)
        } else {
            (r..=pos).all(|x| self.0[x] == Amphipod::Empty)
        }
    }

    fn distance(pos: usize, a: Amphipod) -> usize {
        match a {
            Amphipod::A => match pos {
                0 => 2,
                1 => 1,
                2 => 1,
                3 => 3,
                4 => 5,
                5 => 7,
                6 => 8,
                _ => unreachable!(),
            },
            Amphipod::B => match pos {
                0 => 4,
                1 => 3,
                2 => 1,
                3 => 1,
                4 => 3,
                5 => 5,
                6 => 6,
                _ => unreachable!(),
            },
            Amphipod::C => match pos {
                0 => 6,
                1 => 5,
                2 => 3,
                3 => 1,
                4 => 1,
                5 => 3,
                6 => 4,
                _ => unreachable!(),
            },
            Amphipod::D => match pos {
                0 => 8,
                1 => 7,
                2 => 5,
                3 => 3,
                4 => 1,
                5 => 1,
                6 => 2,
                _ => unreachable!(),
            },
            Amphipod::Empty => unreachable!(),
        }
    }

    fn path_clear(&self, pos: usize, a: Amphipod) -> bool {
        match pos {
            0 => match a {
                Amphipod::A => self.0[1] == Amphipod::Empty,
                Amphipod::B => self.0[1..=2].iter().all(|x| x == &Amphipod::Empty),
                Amphipod::C => self.0[1..=3].iter().all(|x| x == &Amphipod::Empty),
                Amphipod::D => self.0[1..=4].iter().all(|x| x == &Amphipod::Empty),
                Amphipod::Empty => unreachable!(),
            },
            1 => match a {
                Amphipod::A => true,
                Amphipod::B => self.0[2] == Amphipod::Empty,
                Amphipod::C => self.0[2..=3].iter().all(|x| x == &Amphipod::Empty),
                Amphipod::D => self.0[2..=4].iter().all(|x| x == &Amphipod::Empty),
                Amphipod::Empty => unreachable!(),
            },
            2 => match a {
                Amphipod::A => true,
                Amphipod::B => true,
                Amphipod::C => self.0[3] == Amphipod::Empty,
                Amphipod::D => self.0[3..=4].iter().all(|x| x == &Amphipod::Empty),
                Amphipod::Empty => unreachable!(),
            },
            3 => match a {
                Amphipod::A => self.0[2] == Amphipod::Empty,
                Amphipod::B => true,
                Amphipod::C => true,
                Amphipod::D => self.0[4] == Amphipod::Empty,
                Amphipod::Empty => unreachable!(),
            },
            4 => match a {
                Amphipod::A => self.0[2..=3].iter().all(|x| x == &Amphipod::Empty),
                Amphipod::B => self.0[3] == Amphipod::Empty,
                Amphipod::C => true,
                Amphipod::D => true,
                Amphipod::Empty => unreachable!(),
            },
            5 => match a {
                Amphipod::A => self.0[2..=4].iter().all(|x| x == &Amphipod::Empty),
                Amphipod::B => self.0[3..=4].iter().all(|x| x == &Amphipod::Empty),
                Amphipod::C => self.0[4] == Amphipod::Empty,
                Amphipod::D => true,
                Amphipod::Empty => unreachable!(),
            },
            6 => match a {
                Amphipod::A => self.0[2..=5].iter().all(|x| x == &Amphipod::Empty),
                Amphipod::B => self.0[3..=5].iter().all(|x| x == &Amphipod::Empty),
                Amphipod::C => self.0[4..=5].iter().all(|x| x == &Amphipod::Empty),
                Amphipod::D => self.0[5] == Amphipod::Empty,
                Amphipod::Empty => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Room([Amphipod; 4], Amphipod);

impl Room {
    fn is_empty(&self) -> bool {
        let kind = self.1;

        self.0 == [Amphipod::Empty, kind, kind, kind][..]
            || self.0 == [Amphipod::Empty, Amphipod::Empty, kind, kind][..]
            || self.0 == [Amphipod::Empty, Amphipod::Empty, Amphipod::Empty, kind][..]
            || self.0
                == [
                    Amphipod::Empty,
                    Amphipod::Empty,
                    Amphipod::Empty,
                    Amphipod::Empty,
                ][..]
    }

    fn is_done(&self) -> bool {
        self.0.iter().all(|a| a == &self.1)
    }

    fn get_top(&self) -> Option<(Amphipod, Room, usize)> {
        for i in 0..4 {
            if self.0[i] != Amphipod::Empty {
                let mut new_room = *self;
                new_room.0[i] = Amphipod::Empty;
                return Some((self.0[i], new_room, i + 1));
            }
        }

        None
    }

    fn push(&mut self, a: Amphipod) -> usize {
        for p in (0..4).rev() {
            if self.0[p] == Amphipod::Empty {
                self.0[p] = a;
                return p + 1;
            }
        }

        unreachable!()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Map {
    slots: Slots,
    rooms_a: Room,
    rooms_b: Room,
    rooms_c: Room,
    rooms_d: Room,
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "#############")?;
        writeln!(f, "{}", self.slots)?;
        for i in 0..4 {
            writeln!(
                f,
                "{}#{}#{}#{}#{}#{}",
                if i == 0 { "##" } else { "  " },
                self.rooms_a.0[i],
                self.rooms_b.0[i],
                self.rooms_c.0[i],
                self.rooms_d.0[i],
                if i == 0 { "##" } else { "  " },
            )?;
        }

        writeln!(f, "  #########")?;

        Ok(())
    }
}

impl Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self)
    }
}

impl Map {
    fn is_done(&self) -> bool {
        self.slots.is_empty()
            && self.rooms_a.is_done()
            && self.rooms_b.is_done()
            && self.rooms_c.is_done()
            && self.rooms_d.is_done()
    }

    fn generate_move(&self) -> Vec<(Map, usize)> {
        let mut res = Vec::new();

        if let Some((a, room, cost)) = self.rooms_a.get_top() {
            let copy = Map {
                rooms_a: room,
                ..*self
            };
            res.extend(generate_ent::<1, 2>(copy, a, Amphipod::A, cost));
        }

        if let Some((b, room, cost)) = self.rooms_b.get_top() {
            let copy = Map {
                rooms_b: room,
                ..*self
            };
            res.extend(generate_ent::<2, 3>(copy, b, Amphipod::B, cost));
        }

        if let Some((c, room, cost)) = self.rooms_c.get_top() {
            let copy = Map {
                rooms_c: room,
                ..*self
            };
            res.extend(generate_ent::<3, 4>(copy, c, Amphipod::C, cost));
        }

        if let Some((d, room, cost)) = self.rooms_d.get_top() {
            let copy = Map {
                rooms_d: room,
                ..*self
            };
            res.extend(generate_ent::<4, 5>(copy, d, Amphipod::D, cost));
        }

        if self.rooms_a.is_empty() {
            for (pos, amp) in self.slots.0.iter().enumerate() {
                if amp == &Amphipod::A && self.slots.path_clear(pos, Amphipod::A) {
                    let mut copy = *self;
                    let cost = Slots::distance(pos, Amphipod::A) + copy.rooms_a.push(Amphipod::A);
                    copy.slots.0[pos] = Amphipod::Empty;
                    res.push((copy, Amphipod::A.cost_per() * cost));
                }
            }
        }

        if self.rooms_b.is_empty() {
            for (pos, amp) in self.slots.0.iter().enumerate() {
                if amp == &Amphipod::B && self.slots.path_clear(pos, Amphipod::B) {
                    let mut copy = *self;
                    let cost = Slots::distance(pos, Amphipod::B) + copy.rooms_b.push(Amphipod::B);
                    copy.slots.0[pos] = Amphipod::Empty;
                    res.push((copy, Amphipod::B.cost_per() * cost));
                }
            }
        }

        if self.rooms_c.is_empty() {
            for (pos, amp) in self.slots.0.iter().enumerate() {
                if amp == &Amphipod::C && self.slots.path_clear(pos, Amphipod::C) {
                    let mut copy = *self;
                    let cost = Slots::distance(pos, Amphipod::C) + copy.rooms_c.push(Amphipod::C);
                    copy.slots.0[pos] = Amphipod::Empty;
                    res.push((copy, Amphipod::C.cost_per() * cost));
                }
            }
        }

        if self.rooms_d.is_empty() {
            for (pos, amp) in self.slots.0.iter().enumerate() {
                if amp == &Amphipod::D && self.slots.path_clear(pos, Amphipod::D) {
                    let mut copy = *self;
                    let cost = Slots::distance(pos, Amphipod::D) + copy.rooms_d.push(Amphipod::D);
                    copy.slots.0[pos] = Amphipod::Empty;
                    res.push((copy, Amphipod::D.cost_per() * cost));
                }
            }
        }

        res
    }
}

fn generate_ent<const L: usize, const R: usize>(
    map: Map,
    a: Amphipod,
    room: Amphipod,
    cost: usize,
) -> impl Iterator<Item = (Map, usize)> {
    let mut pos = 0;

    from_fn(move || loop {
        if pos == 7 {
            break None;
        } else if map.slots.0[pos] == Amphipod::Empty && map.slots.is_clear(pos, L, R) {
            let mut new_map = map;
            new_map.slots.0[pos] = a;
            let ret = Some((new_map, a.cost_per() * (cost + Slots::distance(pos, room))));
            pos += 1;

            break ret;
        } else {
            pos += 1;
        }
    })
}

#[aoc_generator(day23, part1)]
pub fn generator1(_input: &str) -> Map {
    Map {
        slots: Slots::default(),
        rooms_a: Room(
            [Amphipod::D, Amphipod::C, Amphipod::A, Amphipod::A],
            Amphipod::A,
        ),
        rooms_b: Room(
            [Amphipod::A, Amphipod::A, Amphipod::B, Amphipod::B],
            Amphipod::B,
        ),
        rooms_c: Room(
            [Amphipod::C, Amphipod::B, Amphipod::C, Amphipod::C],
            Amphipod::C,
        ),
        rooms_d: Room(
            [Amphipod::D, Amphipod::B, Amphipod::D, Amphipod::D],
            Amphipod::D,
        ),
    }
}

#[aoc_generator(day23, part2)]
pub fn generator2(_input: &str) -> Map {
    Map {
        slots: Slots::default(),
        rooms_a: Room(
            [Amphipod::D, Amphipod::D, Amphipod::D, Amphipod::C],
            Amphipod::A,
        ),
        rooms_b: Room(
            [Amphipod::A, Amphipod::C, Amphipod::B, Amphipod::A],
            Amphipod::B,
        ),
        rooms_c: Room(
            [Amphipod::C, Amphipod::B, Amphipod::A, Amphipod::B],
            Amphipod::C,
        ),
        rooms_d: Room(
            [Amphipod::D, Amphipod::A, Amphipod::C, Amphipod::B],
            Amphipod::D,
        ),
    }
}

#[aoc(day23, part1)]
pub fn part1(inputs: &Map) -> usize {
    let a = dijkstra(inputs, |map| map.generate_move(), |map| map.is_done());
    a.unwrap().1
}

#[aoc(day23, part2)]
pub fn part2(inputs: &Map) -> usize {
    let a = dijkstra(inputs, |map| map.generate_move(), |map| map.is_done());

    // println!("{:?}", a);
    a.unwrap().1
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########";

    #[test]
    pub fn test_input() {
        println!("{}", generator1(SAMPLE));

        // assert_eq!(generator(SAMPLE), Day23());
    }

    #[test]
    pub fn test1() {
        // assert_eq!(part1(&generator(SAMPLE)), 7);
    }

    #[test]
    pub fn test2() {
        // assert_eq!(part2(&generator(SAMPLE)), 336);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2021/day23.txt");
        const ANSWERS: (usize, usize) = (14346, 48984);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n'); // Trims trailing newline

            assert_eq!(part1(&generator1(input)), ANSWERS.0);
            assert_eq!(part2(&generator2(input)), ANSWERS.1);
        }
    }
}
