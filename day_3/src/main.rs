use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

enum Cell {
    Tree,
    Nothing,
}

fn main() -> std::io::Result<()> {
    let f = File::open("input.txt")?;
    let f = BufReader::new(f);

    let terrain: Vec<Vec<Cell>> = f
        .lines()
        .filter_map(Result::ok)
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '#' => Cell::Tree,
                    _ => Cell::Nothing,
                })
                .collect()
        })
        .collect();

    let result_part_1 = terrain
        .iter()
        .enumerate()
        .filter(|(i, l)| match l.iter().nth(i * 3 % l.len()).unwrap() {
            Cell::Nothing => false,
            Cell::Tree => true,
        })
        .count();
    println!("{}", result_part_1);

    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let result_part_2 = slopes
        .iter()
        .map(|slope| {
            let (right, down) = slope;
            let tree_count = terrain
                .iter()
                .enumerate()
                .filter(|(i, l)| {
                    (i % down == 0)
                        && match l.iter().nth((i / down) * right % l.len()).unwrap() {
                            Cell::Nothing => false,
                            Cell::Tree => true,
                        }
                })
                .count();

            println!("{:?} {}", slope, tree_count);

            tree_count
        })
        .fold(1, |acc, trees_count| acc * trees_count);
    println!("{}", result_part_2);

    Ok(())
}
