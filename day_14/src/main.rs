use io::Result;
use regex::Regex;
use std::iter::FromIterator;
use std::{collections::HashMap, convert::TryInto};
use std::{collections::HashSet, io};
use std::{env, io::prelude::*, iter::Inspect};

#[macro_use]
extern crate lazy_static;

#[derive(Debug, Clone, Copy)]
struct Mask([char; 36]);
impl Mask {
    fn apply(&self, value: u64) -> u64 {
        let (and_mask, or_mask) =
            self.0
                .iter()
                .fold((0 as u64, 0 as u64), |(and_mask, or_mask), c| {
                    (
                        (and_mask << 1) | if *c == '0' { 1 } else { 0 },
                        (or_mask << 1) | if *c == '1' { 1 } else { 0 },
                    )
                });
        (value & (u64::MAX - and_mask)) | or_mask
    }

    fn apply_floating(&self, address: u64) -> Vec<u64> {
        // 0001
        // 000X
        // ----
        // 0001
        // 0000

        // X 1 -> [0, 0]
        // 0X 11 -> [0]

        self.0
            .iter()
            .enumerate()
            .fold(vec![0 as u64], |addresses, (index, c)| match c {
                'X' => addresses
                    .iter()
                    .cloned()
                    .map(|addr| addr << 1)
                    .chain(addresses.iter().cloned().map(|addr| (addr << 1) + 1))
                    .collect(), // duplicate addresses with a 1 and a 0 at the end
                '1' => addresses.into_iter().map(|addr| (addr << 1) + 1).collect(),
                '0' => addresses
                    .into_iter()
                    .map(|addr| (addr << 1) + ((address >> (35 - index)) % 2))
                    .collect(),
                _ => addresses,
            })

        // addresses
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    MaskDef(Mask),
    MemAssign(u64, u64),
}

fn main() {
    let input = io::stdin();
    let part: String = env::args().nth(1).unwrap();

    let instructions: Vec<_> = input
        .lock()
        .lines()
        .filter_map(Result::ok)
        .filter_map(|l| {
            lazy_static! {
                static ref MASK_DEF_RE: Regex = Regex::new(r"mask\s*=\s*([X01]+)").unwrap();
                static ref MEM_ASSIGN_RE: Regex = Regex::new(r"mem\[(\d+)\]\s*=\s*(\d+)").unwrap();
            }
            MASK_DEF_RE
                .captures(l.as_str())
                .and_then(|capts| {
                    capts.get(1).and_then(|m| {
                        m.as_str()
                            .chars()
                            .collect::<Vec<char>>()
                            .try_into()
                            .ok()
                            .map(Mask)
                            .map(Instruction::MaskDef)
                    })
                })
                .or_else(|| {
                    MEM_ASSIGN_RE.captures(l.as_str()).and_then(|capts| {
                        capts
                            .get(1)
                            .and_then(|m| m.as_str().parse::<u64>().ok())
                            .zip(capts.get(2).and_then(|m| m.as_str().parse::<u64>().ok()))
                            .map(|(address, value)| Instruction::MemAssign(address, value))
                    })
                })
        })
        .collect();

    let result = match part.as_str() {
        "part_1" => part_1(instructions),
        "part_2" => part_2(instructions),
        _ => panic!("Part not supported"),
    };

    println!("{:?}", result);
}

fn part_1(instructions: Vec<Instruction>) -> u64 {
    let memory = instructions
        .into_iter()
        .fold(
            (
                Mask(
                    "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX"
                        .chars()
                        .collect::<Vec<char>>()
                        .try_into()
                        .ok()
                        .unwrap(),
                ),
                HashMap::new(),
            ),
            |(mask, mut hash), instruction| match instruction {
                Instruction::MaskDef(new_mask) => (new_mask, hash),
                Instruction::MemAssign(address, value) => {
                    hash.insert(address, mask.apply(value));
                    (mask, hash)
                }
            },
        )
        .1;
    memory.values().sum::<u64>()
}

fn part_2(instructions: Vec<Instruction>) -> u64 {
    let memory = instructions
        .into_iter()
        .fold(
            (
                Mask(
                    "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX"
                        .chars()
                        .collect::<Vec<char>>()
                        .try_into()
                        .ok()
                        .unwrap(),
                ),
                HashMap::new(),
            ),
            |(mask, mut hash), instruction| match instruction {
                Instruction::MaskDef(new_mask) => (new_mask, hash),
                Instruction::MemAssign(address, value) => {
                    mask.apply_floating(address)
                        .into_iter()
                        .for_each(|address| {
                            hash.insert(address, value);
                        });
                    (mask, hash)
                }
            },
        )
        .1;
    memory.values().sum::<u64>()
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn example_1() {
        let mask: Mask = Mask(
            "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"
                .chars()
                .collect::<Vec<char>>()
                .try_into()
                .ok()
                .unwrap(),
        );
        assert_eq!(mask.apply(11), 73);
    }

    #[test]
    fn example_2() {
        let mask: Mask = Mask(
            "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"
                .chars()
                .collect::<Vec<char>>()
                .try_into()
                .ok()
                .unwrap(),
        );
        assert_eq!(mask.apply(101), 101);
    }

    #[test]
    fn example_3() {
        let mask: Mask = Mask(
            "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"
                .chars()
                .collect::<Vec<char>>()
                .try_into()
                .ok()
                .unwrap(),
        );
        assert_eq!(mask.apply(0), 64);
    }

    #[test]
    fn example_4() {
        let mask: Mask = Mask(
            "00000000000000000000000000000000000X"
                .chars()
                .collect::<Vec<char>>()
                .try_into()
                .ok()
                .unwrap(),
        );
        assert_eq!(mask.apply_floating(0), vec![0, 1]);
    }

    #[test]
    fn example_5() {
        let mask: Mask = Mask(
            "000000000000000000000000000000X1001X"
                .chars()
                .collect::<Vec<char>>()
                .try_into()
                .ok()
                .unwrap(),
        );
        assert_eq!(
            HashSet::<u64>::from_iter(mask.apply_floating(42)),
            HashSet::from_iter(vec![26, 27, 58, 59])
        );
    }

    #[test]
    fn example_6() {
        let mask: Mask = Mask(
            "00000000000000000000000000000000X0XX"
                .chars()
                .collect::<Vec<char>>()
                .try_into()
                .ok()
                .unwrap(),
        );
        assert_eq!(
            HashSet::<u64>::from_iter(mask.apply_floating(26)),
            HashSet::from_iter(vec![16, 17, 18, 19, 24, 25, 26, 27])
        );
    }
}
