use io::Result;
use std::io::prelude::*;
use std::{env, io};

fn main() {
    let input = io::stdin();

    let mut adapters: Vec<_> = input
        .lock()
        .lines()
        .filter_map(Result::ok)
        .map(|l| l.parse::<i32>().unwrap())
        .collect();

    adapters.sort();

    println!("{:#?}", adapters);

    let device = adapters.last().unwrap() + 3;
    let end = vec![device];

    let full_adapters: Vec<_> = [0]
        .iter()
        .chain(adapters.iter())
        .chain(end.iter())
        .collect();

    let vec: Vec<_> = full_adapters
        .iter()
        .zip(full_adapters.iter().skip(1).by_ref())
        .inspect(|l| println!("{:?}", l))
        .map(|(a, b)| *b - *a)
        .collect();

    println!("{:?}", vec);
}
