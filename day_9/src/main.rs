use io::Result;
use std::io::prelude::*;
use std::{env, io};

fn main() {
    let input = io::stdin();

    let list: Vec<usize> = input
        .lock()
        .lines()
        .filter_map(Result::ok)
        .map(|l| l.parse().unwrap())
        .collect();

    let mut n: usize = 0;
    for (index, number) in list.iter().enumerate().skip(25) {
        let last_25 = &list[index - 25..index];
        let mut found = false;

        'n1: for n1 in last_25 {
            for n2 in last_25 {
                if n1 + n2 == *number {
                    found = true;
                    break 'n1;
                }
            }
        }

        if !found {
            n = number.clone();
            break;
        }
    }

    println!("{:#?}", n);

    let (min, max) = find_part_2(&list, &n);
    println!("{:?}", min + max);
}

fn find_part_2(list: &Vec<usize>, n: &usize) -> (usize, usize) {
    for (i2, n2) in list.iter().enumerate() {
        println!("{:?}", (i2, n2));
        let mut i1 = i2;
        let mut sum = 0;
        loop {
            let n1 = list[i1];
            sum = sum + n1;

            if sum == *n {
                return (n1, *n2);
            }

            if i1 == 0 {
                break;
            } else {
                i1 = i1 - 1;
            }
        }
    }
    panic!("didn't found")
}
