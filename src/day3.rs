use aoc_runner_derive::{aoc, aoc_generator};
use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;

enum Instr {
    Mul(usize, usize),
    Do,
    Dont,
}

lazy_static! {
    static ref RE: Regex = Regex::new(r"mul\((?<x>\d+),(?<y>\d+)\)|do\(\)|don't\(\)").unwrap();
}

struct Program {
    instrs: Vec<Instr>,
}

impl FromStr for Program {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut instrs = Vec::new();

        for cap in RE.captures_iter(s) {
            if cap.get(0).unwrap().as_str() == "do()" {
                instrs.push(Instr::Do);
            } else if cap.get(0).unwrap().as_str() == "don't()" {
                instrs.push(Instr::Dont);
            } else {
                let x = cap["x"].parse::<usize>().unwrap();
                let y = cap["y"].parse::<usize>().unwrap();

                instrs.push(Instr::Mul(x, y));
            }
        }

        Ok(Program { instrs })
    }
}

#[aoc_generator(day3)]
fn input_generator(input: &str) -> Program {
    input.parse().unwrap()
}

#[aoc(day3, part1)]
fn part1(input: &Program) -> usize {
    let mut total: usize = 0;

    for instr in &input.instrs {
        match instr {
            Instr::Mul(x, y) => total += x * y,
            _ => {}
        }
    }

    total
}

#[aoc(day3, part2)]
fn part2(input: &Program) -> usize {
    let mut total: usize = 0;
    let mut enabled = true;

    for instr in &input.instrs {
        match instr {
            Instr::Mul(x, y) => {
                if enabled {
                    total += x * y
                }
            }
            Instr::Do => enabled = true,
            Instr::Dont => enabled = false,
        }
    }

    total
}
