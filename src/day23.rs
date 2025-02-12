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
        self.0.iter().all(|&amphipod| amphipod == Amphipod::Empty)
    }

    fn is_clear_path_to_slot(&self, pos: usize, l: usize, r: usize) -> bool {
        if pos <= l {
            (pos..=l).all(|x| self.0[x] == Amphipod::Empty)
        } else {
            (r..=pos).all(|x| self.0[x] == Amphipod::Empty)
        }
    }

    // The distance from the entrance to Amphipod room to Slot position
    fn distance(pos: usize, room_name: Amphipod) -> usize {
        match room_name {
            Amphipod::Amber => [2, 1, 1, 3, 5, 7, 8][pos],
            Amphipod::Bronze => [4, 3, 1, 1, 3, 5, 6][pos],
            Amphipod::Copper => [6, 5, 3, 1, 1, 3, 4][pos],
            Amphipod::Desert => [8, 7, 5, 3, 1, 1, 2][pos],
            Amphipod::Empty => unreachable!(),
        }
    }

    // Gets the slot positions left and right of the Amphipod room
    fn room_entrance(room_name: Amphipod) -> (usize, usize) {
        match room_name {
            Amphipod::Amber => (1, 2),
            Amphipod::Bronze => (2, 3),
            Amphipod::Copper => (3, 4),
            Amphipod::Desert => (4, 5),
            Amphipod::Empty => unreachable!(),
        }
    }

    // Gets the range to check between the Amphipod room and the position
    fn slot_paths(pos: usize, room_name: Amphipod) -> Option<(usize, usize)> {
        let (left, right) = Self::room_entrance(room_name);
        if pos == left || pos == right {
            None
        } else if pos < left {
            Some((pos + 1, left))
        } else {
            Some((right, pos - 1))
        }
    }

    fn is_clear_path_to_room(&self, pos: usize, room_name: Amphipod) -> bool {
        Self::slot_paths(pos, room_name)
            .is_none_or(|(l, r)| self.0[l..=r].iter().all(|&x| x == Amphipod::Empty))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Room<const SIZE: usize> {
    slots: [Amphipod; SIZE],
    kind: Amphipod,
}

impl<const SIZE: usize> Room<SIZE> {
    // Checks to see if Room is ready to accept an Amphipod
    fn is_ready(&self) -> bool {
        let mut state = Amphipod::Empty;
        let mut i = 0;

        // State-machine to make sure the top is Empty then matches are particular kind
        while let Some(&amphipod) = self.slots.get(i) {
            if amphipod == state {
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
        self.slots.iter().all(|&amphipod| amphipod == self.kind)
    }

    // Get the top element from the Room, and return new Room, and number of steps
    fn get_top(&self) -> Option<(Amphipod, Self, usize)> {
        for (i, &amphipod) in self.slots.iter().enumerate() {
            if amphipod != Amphipod::Empty {
                let mut new_room = *self;
                new_room.slots[i] = Amphipod::Empty;
                return Some((amphipod, new_room, i + 1));
            }
        }

        None
    }

    // Add Amphipod a to Room, returning number of steps
    fn push(&mut self, amphipod: Amphipod) -> usize {
        for (i, ptr) in self.slots.iter_mut().enumerate().rev() {
            if ptr == &Amphipod::Empty {
                *ptr = amphipod;
                return i + 1;
            }
        }

        unreachable!()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Map<const SIZE: usize> {
    hallway: Hallway,
    rooms: [Room<SIZE>; 4],
}

impl<const SIZE: usize> Display for Map<SIZE> {
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

impl<const SIZE: usize> Debug for Map<SIZE> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self)
    }
}

impl<const SIZE: usize> Map<SIZE> {
    fn new() -> Self {
        Self {
            hallway: Hallway::default(),
            rooms: [
                Room {
                    slots: [Amphipod::Empty; SIZE],
                    kind: Amphipod::Amber,
                },
                Room {
                    slots: [Amphipod::Empty; SIZE],
                    kind: Amphipod::Bronze,
                },
                Room {
                    slots: [Amphipod::Empty; SIZE],
                    kind: Amphipod::Copper,
                },
                Room {
                    slots: [Amphipod::Empty; SIZE],
                    kind: Amphipod::Desert,
                },
            ],
        }
    }

    fn is_done(&self) -> bool {
        self.hallway.is_empty() && self.rooms.iter().all(|room| room.is_done())
    }

    // Generates a list of all moves and their cost
    fn generate_move(&self) -> Vec<(Self, usize)> {
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
                // Generates moves from slot to room
                for (pos, &amphipod) in self.hallway.0.iter().enumerate() {
                    if amphipod == kind && self.hallway.is_clear_path_to_room(pos, kind) {
                        let mut new_map = *self;
                        new_map.hallway.0[pos] = Amphipod::Empty;
                        let steps = Hallway::distance(pos, kind) + new_map.rooms[i].push(kind);
                        res.push((new_map, kind.cost_per() * steps));
                    }
                }
            } else if let Some((amphipod, room, steps)) = self.rooms[i].get_top() {
                // Generates moves from room to slot
                let mut new_map = *self;
                new_map.rooms[i] = room;
                res.extend(generate_room_to_slot(new_map, amphipod, kind, steps));
            }
        }

        res
    }
}

fn generate_room_to_slot<const SIZE: usize>(
    map: Map<SIZE>,
    amphipod: Amphipod,
    room_name: Amphipod,
    steps: usize,
) -> impl Iterator<Item = (Map<SIZE>, usize)> {
    let mut pos = 0;
    let (l, r) = Hallway::room_entrance(room_name);

    // Loop over from pos in 0..7, producing the new state and cost
    from_fn(move || {
        while pos < 7 {
            if map.hallway.0[pos] == Amphipod::Empty && map.hallway.is_clear_path_to_slot(pos, l, r)
            {
                let mut new_map = map;
                new_map.hallway.0[pos] = amphipod;
                let ret = Some((
                    new_map,
                    amphipod.cost_per() * (steps + Hallway::distance(pos, room_name)),
                ));
                pos += 1;

                return ret;
            }
            pos += 1;
        }

        None
    })
}

#[aoc_generator(day23, part1)]
pub fn generator1(input: &str) -> Map<2> {
    let mut map = Map::new();

    for (i, c) in input
        .bytes()
        .filter(|&c| c.is_ascii_alphabetic())
        .enumerate()
    {
        map.rooms[i % 4].slots[i / 4] = Amphipod::parse(c);
    }

    map
}

#[aoc_generator(day23, part2)]
pub fn generator2(input: &str) -> Map<4> {
    let mut map = Map::new();

    let input = input.bytes().filter(|&c| c.is_ascii_alphabetic());
    for (i, c) in input
        .clone()
        .take(4)
        .chain("DCBADBAC".bytes())
        .chain(input.skip(4))
        .enumerate()
    {
        map.rooms[i % 4].slots[i / 4] = Amphipod::parse(c);
    }

    map
}

fn solve<const SIZE: usize>(start: &Map<SIZE>) -> usize {
    dijkstra(start, |m| m.generate_move(), |m| m.is_done())
        .unwrap()
        .1
}

#[aoc(day23, part1)]
pub fn part1(inputs: &Map<2>) -> usize {
    solve(inputs)
}

#[aoc(day23, part2)]
pub fn part2(inputs: &Map<4>) -> usize {
    solve(inputs)
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
            let input = INPUT.trim_end_matches('\n');

            let output1 = generator1(input);
            assert_eq!(part1(&output1), ANSWERS.0);

            let output2 = generator2(input);
            assert_eq!(part2(&output2), ANSWERS.1);
        }
    }
}
