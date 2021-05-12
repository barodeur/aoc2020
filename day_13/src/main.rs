use io::Result;
use std::io::prelude::*;
use std::{env, io};

fn main() {
    let input = io::stdin();
    let part: String = env::args().nth(1).unwrap();

    let params: Vec<_> = input
        .lock()
        .lines()
        .filter_map(Result::ok)
        .take(2)
        .collect();
    let timestamp: i64 = params.get(0).unwrap().parse().unwrap();
    let lines: Vec<(i64, i64)> = params
        .get(1)
        .unwrap()
        .split(',')
        .enumerate()
        .filter_map(|(i, l)| l.parse::<i64>().ok().map(|l| (l, i as i64)))
        .collect();

    let res = match part.as_str() {
        "part_1" => part_1(timestamp, lines.iter().map(|l| l.0).collect()),
        "part_2" => part_2(lines),
        _ => panic!("unknown part"),
    };

    println!("{}", res);
}

fn part_1(timestamp: i64, lines: Vec<i64>) -> i64 {
    let (wait_time, line) = lines
        .iter()
        .map(|l| {
            let rest = timestamp % l;
            if rest == 0 {
                return (0, l);
            }

            let last = ((timestamp / l) * l) as i64;
            let next = last + l;
            (next - timestamp, l)
        })
        .min_by_key(|(wait, _l)| *wait)
        .unwrap();

    wait_time * line
}

fn part_2(lines: Vec<(i64, i64)>) -> i64 {
    let residues: Vec<_> = lines.iter().map(|(n, a)| n - a).collect();
    let modulii: Vec<_> = lines.iter().map(|(n, _)| *n).collect();
    chinese_remainder(&residues[..], &modulii[..]).unwrap()
}

fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
    let prod = modulii.iter().product::<i64>();

    let mut sum = 0;

    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }

    Some(sum % prod)
}
