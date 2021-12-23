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
    Amber,
    Bronze,
    Copper,
    Desert,
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
            Amphipod::Amber => write!(f, "A"),
            Amphipod::Bronze => write!(f, "B"),
            Amphipod::Copper => write!(f, "C"),
            Amphipod::Desert => write!(f, "D"),
            Amphipod::Empty => write!(f, "."),
        }
    }
}

impl Amphipod {
    fn cost_per(&self) -> usize {
        match *self {
            Amphipod::Amber => 1,
            Amphipod::Bronze => 10,
            Amphipod::Copper => 100,
            Amphipod::Desert => 1000,
            Amphipod::Empty => todo!(),
        }
    }

    fn parse(c: u8) -> Self {
        match c {
            b'A' => Amphipod::Amber,
            b'B' => Amphipod::Bronze,
            b'C' => Amphipod::Copper,
            b'D' => Amphipod::Desert,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default, Hash)]
pub struct Hallway([Amphipod; 7]);
impl Display for Hallway {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "#{}{}.{}.{}.{}.{}{}#",
            self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], self.0[5], self.0[6],
        )
    }
}

impl Hallway {
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

    // The distance from the enterance to Amphipod Room a to Slot pos
    fn distance(pos: usize, a: Amphipod) -> usize {
        match a {
            Amphipod::Amber => [2, 1, 1, 3, 5, 7, 8][pos],
            Amphipod::Bronze => [4, 3, 1, 1, 3, 5, 6][pos],
            Amphipod::Copper => [6, 5, 3, 1, 1, 3, 4][pos],
            Amphipod::Desert => [8, 7, 5, 3, 1, 1, 2][pos],
            Amphipod::Empty => unreachable!(),
        }
    }

    const fn room_entrance(a: Amphipod) -> (usize, usize) {
        match a {
            Amphipod::Amber => (1, 2),
            Amphipod::Bronze => (2, 3),
            Amphipod::Copper => (3, 4),
            Amphipod::Desert => (4, 5),
            Amphipod::Empty => unreachable!(),
        }
    }

    fn slot_paths(pos: usize, a: Amphipod) -> Option<(usize, usize)> {
        let (left, right) = Self::room_entrance(a);
        if pos == left || pos == right {
            None
        } else if pos < left {
            Some((pos + 1, left))
        } else {
            Some((right, pos - 1))
        }
    }

    fn path_clear(&self, pos: usize, a: Amphipod) -> bool {
        Self::slot_paths(pos, a).map_or(true, |(l, r)| {
            self.0[l..=r].iter().all(|x| x == &Amphipod::Empty)
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Room {
    slots: [Amphipod; 4],
    kind: Amphipod,
}

impl Room {
    // Checks to see if Room is ready to accept an Amphipod
    fn is_ready(&self) -> bool {
        let mut state = Amphipod::Empty;
        let mut i = 0;

        while let Some(&a) = self.slots.get(i) {
            if a == state {
                i += 1;
            } else if state == Amphipod::Empty {
                state = self.kind;
            } else {
                return false;
            }
        }

        true
    }

    fn is_done(&self) -> bool {
        self.slots.iter().all(|a| a == &self.kind)
    }

    fn get_top(&self) -> Option<(Amphipod, Room, usize)> {
        for (i, &a) in self.slots.iter().enumerate() {
            if a != Amphipod::Empty {
                let mut new_room = *self;
                new_room.slots[i] = Amphipod::Empty;
                return Some((a, new_room, i + 1));
            }
        }

        None
    }

    // Add Amphipod a to room, returning number of steps
    fn push(&mut self, a: Amphipod) -> usize {
        for (i, p) in self.slots.iter_mut().enumerate().rev() {
            if p == &Amphipod::Empty {
                *p = a;
                return i + 1;
            }
        }

        unreachable!()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Map {
    hallway: Hallway,
    rooms: [Room; 4],
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "#############")?;
        writeln!(f, "{}", self.hallway)?;
        for i in 0..self.rooms[0].slots.len() {
            writeln!(
                f,
                "{}#{}#{}#{}#{}#{}",
                if i == 0 { "##" } else { "  " },
                self.rooms[0].slots[i],
                self.rooms[1].slots[i],
                self.rooms[2].slots[i],
                self.rooms[3].slots[i],
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
    fn new() -> Self {
        Self {
            hallway: Hallway::default(),
            rooms: [
                Room {
                    slots: [Amphipod::Empty; 4],
                    kind: Amphipod::Amber,
                },
                Room {
                    slots: [Amphipod::Empty; 4],
                    kind: Amphipod::Bronze,
                },
                Room {
                    slots: [Amphipod::Empty; 4],
                    kind: Amphipod::Copper,
                },
                Room {
                    slots: [Amphipod::Empty; 4],
                    kind: Amphipod::Desert,
                },
            ],
        }
    }

    fn is_done(&self) -> bool {
        self.hallway.is_empty() && self.rooms.iter().all(|r| r.is_done())
    }

    fn generate_move(&self) -> Vec<(Map, usize)> {
        let mut res = Vec::new();

        for (i, &kind) in [
            Amphipod::Amber,
            Amphipod::Bronze,
            Amphipod::Copper,
            Amphipod::Desert,
        ]
        .iter()
        .enumerate()
        {
            if self.rooms[i].is_ready() {
                for (pos, amp) in self.hallway.0.iter().enumerate() {
                    if amp == &kind && self.hallway.path_clear(pos, kind) {
                        let mut copy = *self;
                        copy.hallway.0[pos] = Amphipod::Empty;
                        let cost = Hallway::distance(pos, kind) + copy.rooms[i].push(kind);
                        res.push((copy, kind.cost_per() * cost));
                    }
                }
            } else if let Some((a, room, cost)) = self.rooms[i].get_top() {
                let mut copy = *self;
                copy.rooms[i] = room;
                res.extend(generate_ent(copy, a, kind, cost));
            }
        }

        res
    }
}

fn generate_ent(
    map: Map,
    a: Amphipod,
    room: Amphipod,
    cost: usize,
) -> impl Iterator<Item = (Map, usize)> {
    let mut pos = 0;
    let (l, r) = Hallway::room_entrance(room);

    from_fn(move || loop {
        if pos == 7 {
            break None;
        } else if map.hallway.0[pos] == Amphipod::Empty && map.hallway.is_clear(pos, l, r) {
            let mut new_map = map;
            new_map.hallway.0[pos] = a;
            let ret = Some((
                new_map,
                a.cost_per() * (cost + Hallway::distance(pos, room)),
            ));
            pos += 1;

            break ret;
        } else {
            pos += 1;
        }
    })
}

#[aoc_generator(day23, part1)]
pub fn generator1(input: &str) -> Map {
    let mut map = Map::new();

    for (i, c) in input
        .bytes()
        .filter(|&c| c.is_ascii_alphabetic())
        .enumerate()
    {
        map.rooms[i % 4].slots[i / 4] = Amphipod::parse(c);
    }

    for i in 8..16 {
        map.rooms[i % 4].slots[i / 4] = Amphipod::parse(b'A' + (i % 4) as u8);
    }

    map
}

#[aoc_generator(day23, part2)]
pub fn generator2(input: &str) -> Map {
    let mut map = Map::new();

    for (i, c) in input
        .bytes()
        .filter(|&c| c.is_ascii_alphabetic())
        .take(4)
        .chain("DCBADBAC".bytes())
        .chain(input.bytes().filter(|&c| c.is_ascii_alphabetic()).skip(4))
        .enumerate()
    {
        map.rooms[i % 4].slots[i / 4] = Amphipod::parse(c);
    }

    map
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
        assert_eq!(part1(&generator1(SAMPLE)), 12521);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator2(SAMPLE)), 44169);
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