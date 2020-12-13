use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();

    let seat_ids: Vec<u32> = stdin
        .lock()
        .lines()
        .filter_map(Result::ok)
        .map(|l| {
            let mut n: u32 = 0;
            l.chars().for_each(|c| {
                n = n << 1;
                match c {
                    'B' => n = n + 1,
                    'R' => n = n + 1,
                    _ => (),
                }
            });
            n
        })
        .collect();

    let max_seat_id = seat_ids.iter().max().unwrap();

    println!("max seat id: {}", max_seat_id);

    let min_seat_id = seat_ids.iter().min().unwrap();
    let from_min_to_max_sum = ((max_seat_id - min_seat_id + 1) * (max_seat_id + min_seat_id)) / 2;
    let seat_ids_sum = seat_ids.iter().fold(0, |acc, id| acc + id);

    let my_seat_id = from_min_to_max_sum - seat_ids_sum;

    println!("my seat id: {}", my_seat_id);
}
