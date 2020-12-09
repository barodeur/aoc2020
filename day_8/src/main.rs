use std::io;
use std::io::prelude::*;
use std::{collections::HashSet, env};

#[derive(Debug)]
enum Instruction {
    Acc(isize),
    Jmp(isize),
    Nop(isize),
}

#[derive(Debug)]
struct State {
    position: usize,
    acc: isize,
    replaced: bool,
}

fn main() {
    let input = io::stdin();
    let part: String = env::args().nth(1).unwrap();

    let instructions: Vec<Instruction> = input
        .lock()
        .lines()
        .filter_map(Result::ok)
        .map(|l| {
            let split: Vec<&str> = l.split(' ').collect();
            let code = split[0];
            let value: isize = split[1].parse().unwrap();

            match code {
                "acc" => Instruction::Acc(value),
                "jmp" => Instruction::Jmp(value),
                "nop" => Instruction::Nop(value),
                _ => panic!("oops"),
            }
        })
        .collect();

    // println!("{:#?}", instructions);

    match part.as_str() {
        "part_1" => {
            let mut position: usize = 0;
            let mut acc = 0;

            let mut visited_positions: HashSet<usize> = HashSet::new();
            while !visited_positions.contains(&position) {
                let instruction = instructions.get(position).unwrap();
                if let Instruction::Acc(value) = instruction {
                    acc = acc + value;
                }

                visited_positions.insert(position);

                if let Instruction::Jmp(value) = instruction {
                    position = if *value < 0 {
                        position - (-(*value) as usize)
                    } else {
                        position + (*value as usize)
                    }
                } else {
                    position = position + 1
                }
            }

            let res = acc;
            println!("{}", res);
        }
        "part_2" => {
            let final_position = instructions.iter().count();

            let mut queue = vec![State {
                position: 0,
                acc: 0,
                replaced: false,
            }];

            let mut res = 0;

            let mut visited_positions: HashSet<(bool, usize)> = HashSet::new();
            while let Some(state) = queue.pop() {
                println!("{:?}", state);

                let position = state.position;
                let mut acc = state.acc;
                let replaced = state.replaced;

                if visited_positions.contains(&(replaced, position)) {
                    continue;
                }
                visited_positions.insert((replaced, position));

                if replaced && position == final_position {
                    res = state.acc;
                    break;
                }

                let instruction = instructions.get(position).unwrap();
                if let Instruction::Acc(value) = instruction {
                    acc = acc + value;
                }
                match instruction {
                    Instruction::Jmp(value) => queue.push(State {
                        acc: acc.clone(),
                        position: if *value < 0 {
                            position - (-(*value) as usize)
                        } else {
                            position + (*value as usize)
                        },
                        replaced: replaced,
                    }),
                    _ => queue.push(State {
                        acc: acc.clone(),
                        position: position + 1,
                        replaced: replaced,
                    }),
                }
                if !replaced {
                    match instruction {
                        Instruction::Nop(value) => queue.push(State {
                            acc: acc.clone(),
                            position: if *value < 0 {
                                position - (-(*value) as usize)
                            } else {
                                position + (*value as usize)
                            },
                            replaced: true,
                        }),
                        Instruction::Jmp(_) => queue.push(State {
                            acc: acc.clone(),
                            position: position + 1,
                            replaced: true,
                        }),
                        _ => {}
                    }
                }
            }

            println!("{}", res);
        }
        _ => (),
    }
}
