use std::{convert::Infallible, str::FromStr};

use aoc_runner_derive::{aoc, aoc_generator};
use rustc_hash::FxHashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Variable {
    Number(isize),
    RegisterX,
    RegisterY,
    RegisterZ,
    RegisterW,
}

impl FromStr for Variable {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "w" => Variable::RegisterW,
            "x" => Variable::RegisterX,
            "y" => Variable::RegisterY,
            "z" => Variable::RegisterZ,
            _ => Variable::Number(s.parse().unwrap()),
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Instruction {
    Inp(Variable),
    Add(Variable, Variable),
    Mul(Variable, Variable),
    Div(Variable, Variable),
    Mod(Variable, Variable),
    Eql(Variable, Variable),
}

impl FromStr for Instruction {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (ins, variables) = s.split_once(' ').unwrap();
        if s.starts_with("inp") {
            Ok(Instruction::Inp(variables.parse().unwrap()))
        } else {
            let (a, b) = variables.split_once(' ').unwrap();
            let a = a.parse().unwrap();
            let b = b.parse().unwrap();
            Ok(match ins {
                "add" => Instruction::Add(a, b),
                "mul" => Instruction::Mul(a, b),
                "div" => Instruction::Div(a, b),
                "mod" => Instruction::Mod(a, b),
                "eql" => Instruction::Eql(a, b),
                _ => panic!("invalid instruction: {}", ins),
            })
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Hash)]
struct Alu {
    x: isize,
    y: isize,
    z: isize,
    w: isize,
}

impl Alu {
    fn get_variable(&self, variable: Variable) -> isize {
        match variable {
            Variable::Number(x) => x,
            Variable::RegisterX => self.x,
            Variable::RegisterY => self.y,
            Variable::RegisterZ => self.z,
            Variable::RegisterW => self.w,
        }
    }

    fn set_variable(&mut self, variable: Variable, value: isize) {
        match variable {
            Variable::Number(_) => panic!("you cannot set a Number"),
            Variable::RegisterX => self.x = value,
            Variable::RegisterY => self.y = value,
            Variable::RegisterZ => self.z = value,
            Variable::RegisterW => self.w = value,
        }
    }

    fn is_valid(&self) -> bool {
        self.z == 0
    }

    fn process_ins(&mut self, ins: &Instruction, input: usize) {
        match ins {
            Instruction::Inp(v) => {
                self.set_variable(*v, input as isize);
            }
            Instruction::Add(a, b) => {
                let x = self.get_variable(*a);
                let y = self.get_variable(*b);
                self.set_variable(*a, x + y);
            }
            Instruction::Mul(a, b) => {
                let x = self.get_variable(*a);
                let y = self.get_variable(*b);
                self.set_variable(*a, x * y);
            }
            Instruction::Div(a, b) => {
                let x = self.get_variable(*a);
                let y = self.get_variable(*b);
                self.set_variable(*a, x / y);
            }
            Instruction::Mod(a, b) => {
                let x = self.get_variable(*a);
                let y = self.get_variable(*b);
                self.set_variable(*a, x % y);
            }
            Instruction::Eql(a, b) => {
                let x = self.get_variable(*a);
                let y = self.get_variable(*b);
                self.set_variable(*a, if x == y { 1 } else { 0 });
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Day24 {
    ins: Vec<Instruction>,
}

#[aoc_generator(day24)]
pub fn generator(input: &str) -> Day24 {
    Day24 {
        ins: input.lines().map(|l| l.parse().unwrap()).collect(),
    }
}

fn solve(
    program: &[Instruction],
    pc: usize,
    alu: Alu,
    digits: [usize; 9],
    seen: &mut FxHashMap<(Alu, usize), Option<usize>>,
    acc: usize,
) -> Option<usize> {
    if let Some(ans) = seen.get(&(alu, pc)) {
        return *ans;
    }

    'inputs: for input in digits {
        let mut alu = alu;
        let mut pc = pc;
        alu.process_ins(&program[pc], input);
        pc += 1;

        while let Some(inst) = program.get(pc) {
            if let Instruction::Inp(_) = inst {
                if let Some(ans) = solve(program, pc, alu, digits, seen, acc * 10 + input) {
                    seen.insert((alu, pc), Some(ans));
                    return Some(ans);
                } else {
                    continue 'inputs;
                }
            } else {
                alu.process_ins(inst, input);
                pc += 1;
            }
        }

        if alu.is_valid() {
            let total = Some(acc * 10 + input);
            seen.insert((alu, pc), total);
            return total;
        }
    }

    seen.insert((alu, pc), None);
    None
}

#[aoc(day24, part1)]
pub fn part1(instructions: &Day24) -> usize {
    solve(
        &instructions.ins,
        0,
        Alu::default(),
        [9, 8, 7, 6, 5, 4, 3, 2, 1],
        &mut FxHashMap::default(),
        0,
    )
    .unwrap()
}

#[aoc(day24, part2)]
pub fn part2(instructions: &Day24) -> usize {
    solve(
        &instructions.ins,
        0,
        Alu::default(),
        [1, 2, 7, 3, 4, 9, 9, 9, 9],
        &mut FxHashMap::default(),
        0,
    )
    .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_input() {
        println!("{:?}", generator(regression::INPUT));

        // assert_eq!(generator(SAMPLE), Day24());
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

        pub const INPUT: &str = include_str!("../input/2021/day24.txt");
        const ANSWERS: (usize, usize) = (99691891979938, 27141191213911);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n'); // Trims trailing newline

            assert_eq!(part1(&generator(input)), ANSWERS.0);
            assert_eq!(part2(&generator(input)), ANSWERS.1);
        }
    }
}
