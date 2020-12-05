use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut input_file = File::open("input.txt")?;
    let mut contents = String::new();
    input_file.read_to_string(&mut contents)?;

    let numbers: Vec<i32> = contents
        .split('\n')
        .filter_map(|s| s.parse().ok())
        .collect();

    let result_part_1 = numbers.iter().enumerate().find_map(|(i, &n1)| {
        numbers[i..]
            .iter()
            .find_map(|n2| if n1 + n2 == 2020 { Some(n1 * n2) } else { None })
    });
    println!("{:#?}", result_part_1.unwrap());

    let result_part_2 = numbers.iter().enumerate().find_map(|(i, &n1)| {
        numbers[i..].iter().enumerate().find_map(|(j, &n2)| {
            numbers[j..].iter().find_map(|&n3| {
                if n1 + n2 + n3 == 2020 {
                    Some(n1 * n2 * n3)
                } else {
                    None
                }
            })
        })
    });
    println!("{:#?}", result_part_2.unwrap());

    Ok(())
}
