use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

struct PasswordPolicy {
    numbers: (usize, usize),
    char: char,
}

struct PasswordLine {
    policy: PasswordPolicy,
    password: String,
}

fn main() -> std::io::Result<()> {
    let f = File::open("input.txt")?;
    let f = BufReader::new(f);

    let re = Regex::new(r"(\d+)-(\d+)\s+([a-z]):\s+([a-z]+)").unwrap();

    let password_lines: Vec<PasswordLine> = f
        .lines()
        .filter_map(Result::ok)
        .map(|l| {
            let caps = re.captures(l.as_str()).unwrap();

            PasswordLine {
                policy: PasswordPolicy {
                    numbers: (
                        caps.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                        caps.get(2).unwrap().as_str().parse::<usize>().unwrap(),
                    ),
                    char: caps.get(3).unwrap().as_str().chars().next().unwrap(),
                },
                password: String::from(caps.get(4).unwrap().as_str()),
            }
        })
        .collect();

    let result_part_1 = password_lines
        .iter()
        .filter(|pl| {
            let char_occurences = pl.password.chars().filter(|&c| c == pl.policy.char).count();
            let (min, max) = pl.policy.numbers;
            min <= char_occurences && char_occurences <= max
        })
        .count();
    println!("{}", result_part_1);

    let result_part_2 = password_lines
        .iter()
        .filter(|pl| {
            let (p1, p2) = pl.policy.numbers;
            [p1, p2]
                .iter()
                .filter(|&p| pl.password.chars().nth(p - 1).unwrap() == pl.policy.char)
                .count()
                == 1
        })
        .count();
    println!("{}", result_part_2);

    Ok(())
}
