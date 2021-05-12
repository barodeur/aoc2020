use io::Result;
use std::io;
use std::io::prelude::*;

static DIRECTIONS: &[(isize, isize)] = &[
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
];

#[derive(Debug, Clone, PartialEq)]
enum Seat {
    Floor,
    Empty,
    Occupied,
}
impl Seat {
    fn from_char(c: char) -> Seat {
        match c {
            'L' => Seat::Empty,
            '#' => Seat::Occupied,
            _ => Seat::Floor,
        }
    }

    fn to_char(&self) -> char {
        match self {
            Seat::Floor => '.',
            Seat::Empty => 'L',
            Seat::Occupied => '#',
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct SeatLayout(Vec<Vec<Seat>>);
impl SeatLayout {
    fn print(&self) {
        for row in self.0.iter() {
            for seat in row {
                print!("{}", seat.to_char())
            }
            print!("\n");
        }
    }

    fn next(&self) -> SeatLayout {
        SeatLayout(
            self.0
                .iter()
                .enumerate()
                .map(|(i, row)| {
                    row.iter()
                        .enumerate()
                        .map(
                            |(j, s)| match (s, self.occupied_visible_seats_count((i, j))) {
                                (Seat::Empty, 0) => Seat::Occupied,
                                (Seat::Occupied, n) if n >= 5 => Seat::Empty,
                                _ => s.clone(),
                            },
                        )
                        .collect()
                })
                .collect(),
        )
    }

    fn occupied_adjacent_seats_count(&self, (i, j): (usize, usize)) -> usize {
        DIRECTIONS
            .iter()
            .filter_map(|(incr_i, incr_j)| {
                adj_pos((i, j), (*incr_i, *incr_j))
                    .and_then(|p| self.seat_at_pos(p))
                    .and_then(|s| match s {
                        Seat::Occupied => Some(1),
                        _ => None,
                    })
            })
            .sum()
    }

    fn occupied_visible_seats_count(&self, pos: (usize, usize)) -> usize {
        DIRECTIONS
            .iter()
            .filter_map(|dir| {
                self.first_visible_seat(pos, *dir).and_then(|s| match s {
                    Seat::Occupied => Some(1),
                    _ => None,
                })
            })
            .sum()
    }

    fn first_visible_seat(&self, seat_pos: (usize, usize), dir: (isize, isize)) -> Option<&Seat> {
        adj_pos(seat_pos, dir).and_then(|p| {
            self.seat_at_pos(p).and_then(|s| match s {
                Seat::Floor => self.first_visible_seat(p, dir),
                _ => Some(s),
            })
        })
    }

    fn seat_at_pos(&self, (i, j): (usize, usize)) -> Option<&Seat> {
        self.0.get(i).and_then(|row| row.get(j))
    }

    fn occupied_seats_count(&self) -> usize {
        self.0
            .iter()
            .flatten()
            .map(|s| match s {
                Seat::Occupied => 1,
                _ => 0,
            })
            .sum()
    }
}

fn adj_pos((i, j): (usize, usize), (incr_i, incr_j): (isize, isize)) -> Option<(usize, usize)> {
    let adj_i = if incr_i < 0 {
        i.checked_sub((-incr_i) as usize)
    } else {
        Some(i + incr_i as usize)
    };
    let adj_j = if incr_j < 0 {
        j.checked_sub((-incr_j) as usize)
    } else {
        Some(j + incr_j as usize)
    };
    match (adj_i, adj_j) {
        (Some(i), Some(j)) => Some((i, j)),
        _ => None,
    }
}

fn main() {
    let input = io::stdin();

    let initial_seat_layout = SeatLayout(
        input
            .lock()
            .lines()
            .filter_map(Result::ok)
            .map(|l| l.chars().map(Seat::from_char).collect())
            .collect(),
    );

    let mut layout = initial_seat_layout;
    loop {
        // layout.print();
        // println!();
        let new_layout = layout.next();
        if new_layout == layout {
            break;
        }
        layout = new_layout;
    }
    println!("{}", layout.occupied_seats_count());
}
