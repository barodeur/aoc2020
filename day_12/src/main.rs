use io::Result;
use regex::Regex;
use std::io;
use std::io::prelude::*;
#[macro_use]
extern crate lazy_static;

#[derive(Debug, Copy, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}
impl Direction {
    fn from_deg(deg: usize) -> Option<Direction> {
        match (deg / 90) % 4 {
            0 => Some(Direction::East),
            1 => Some(Direction::South),
            2 => Some(Direction::West),
            3 => Some(Direction::North),
            _ => None,
        }
    }

    fn to_deg(&self) -> usize {
        match self {
            Direction::East => 0,
            Direction::South => 90,
            Direction::West => 180,
            Direction::North => 270,
        }
    }

    fn turn(&self, turn: &Turn) -> Option<Direction> {
        Self::from_deg(
            self.to_deg()
                + match turn {
                    Turn::Right(val) => *val,
                    Turn::Left(val) => 360 - *val,
                },
        )
    }
}

#[derive(Debug, Copy, Clone)]
enum Turn {
    Right(usize),
    Left(usize),
}

#[derive(Debug, Copy, Clone)]
enum Move {
    North(usize),
    South(usize),
    East(usize),
    West(usize),
    Forward(usize),
}

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Move(Move),
    Turn(Turn),
}

#[derive(Debug, Copy, Clone)]
struct Position(isize, isize);
impl Position {
    fn manhattan_distance(&self) -> isize {
        self.0.abs() + self.1.abs()
    }

    fn turn(&self, trn: &Turn) -> Position {
        match trn {
            Turn::Left(0) => *self,
            Turn::Right(0) => *self,
            Turn::Left(v) => Position(0 - self.1, self.0).turn(&Turn::Left(v - 90)),
            Turn::Right(v) => Position(self.1, 0 - self.0).turn(&Turn::Right(v - 90)),
            _ => self.clone(),
        }
    }
}

#[derive(Debug)]
struct BoatState {
    pos: Position,
    dir: Direction,
    wpt_pos: Position,
}
impl BoatState {
    fn apply_part_1(&self, instruction: &Instruction) -> BoatState {
        BoatState {
            pos: match instruction {
                Instruction::Move(mv) => match mv {
                    Move::North(val) => Position(self.pos.0, self.pos.1 + (*val as isize)),
                    Move::South(val) => Position(self.pos.0, self.pos.1 - (*val as isize)),
                    Move::East(val) => Position(self.pos.0 + (*val as isize), self.pos.1),
                    Move::West(val) => Position(self.pos.0 - (*val as isize), self.pos.1),
                    Move::Forward(val) => Position(
                        self.pos.0
                            + (match self.dir {
                                Direction::East => 1,
                                Direction::West => -1,
                                _ => 0,
                            }) * (*val as isize),
                        self.pos.1
                            + (match self.dir {
                                Direction::North => 1,
                                Direction::South => -1,
                                _ => 0,
                            }) * (*val as isize),
                    ),
                },
                _ => self.pos,
            },
            dir: match instruction {
                Instruction::Turn(turn) => self.dir.turn(turn).unwrap(),
                _ => self.dir,
            },
            wpt_pos: self.wpt_pos,
        }
    }

    fn apply_part_2(&self, instruction: &Instruction) -> BoatState {
        BoatState {
            pos: match instruction {
                Instruction::Move(Move::Forward(value)) => Position(
                    self.pos.0 + (*value as isize) * self.wpt_pos.0,
                    self.pos.1 + (*value as isize) * self.wpt_pos.1,
                ),
                _ => self.pos,
            },
            dir: self.dir,
            wpt_pos: match instruction {
                Instruction::Move(mv) => match mv {
                    Move::North(val) => Position(self.wpt_pos.0, self.wpt_pos.1 + (*val as isize)),
                    Move::South(val) => Position(self.wpt_pos.0, self.wpt_pos.1 - (*val as isize)),
                    Move::East(val) => Position(self.wpt_pos.0 + (*val as isize), self.wpt_pos.1),
                    Move::West(val) => Position(self.wpt_pos.0 - (*val as isize), self.wpt_pos.1),
                    _ => self.wpt_pos,
                },
                Instruction::Turn(trn) => self.wpt_pos.turn(trn),
                _ => self.wpt_pos,
            },
        }
    }

    fn manhattan_distance(&self) -> isize {
        self.pos.manhattan_distance()
    }
}

fn main() {
    let input = io::stdin();

    let state = instructions_iter(input.lock()).fold(
        BoatState {
            pos: Position(0, 0),
            dir: Direction::East,
            wpt_pos: Position(10, 1),
        },
        |state, instruction| state.apply_part_2(&instruction),
    );

    println!("{:?}", state);

    println!("{:?}", state.manhattan_distance());
}

fn instructions_iter<R: BufRead>(input: R) -> impl Iterator<Item = Instruction> {
    input.lines().filter_map(Result::ok).filter_map(|l| {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^([NSEWLRF])(\d+)$").unwrap();
        }
        let caps = RE.captures(l.as_str()).unwrap();
        let letter = caps.get(1).unwrap().as_str().chars().nth(0).unwrap_or('_');
        let value = caps.get(2).unwrap().as_str().parse().unwrap_or(0);
        match letter {
            'N' => Some(Instruction::Move(Move::North(value))),
            'E' => Some(Instruction::Move(Move::East(value))),
            'S' => Some(Instruction::Move(Move::South(value))),
            'W' => Some(Instruction::Move(Move::West(value))),
            'L' => Some(Instruction::Turn(Turn::Left(value))),
            'R' => Some(Instruction::Turn(Turn::Right(value))),
            'F' => Some(Instruction::Move(Move::Forward(value))),
            _ => None,
        }
    })
}
