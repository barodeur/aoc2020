use std::iter;

use im::hashmap;

fn main() {
    let res = spoken_numbers(vec![0, 1, 4, 13, 15, 12, 16])
        .nth(30000000 - 1)
        .unwrap();
    println!("{:?}", res);
}

fn spoken_numbers(init: Vec<u64>) -> impl Iterator<Item = u64> {
    init.into_iter()
        .map(|n| Some(n))
        .chain(iter::repeat(None))
        .enumerate()
        .scan(
            (
                hashmap! {} as hashmap::HashMap<u64, u64>,
                None as Option<u64>,
            ),
            |state, (turn, number)| {
                // let history = state.0;
                let new_number = match (state.1, number) {
                    (None, _) => 0,
                    (_, Some(number)) => number,
                    (Some(last_number), _) => match state.0.get(&last_number) {
                        Some(&last_index) => (turn as u64 - 1) - last_index,
                        None => 0,
                    },
                };
                if let Some(last_number) = state.1 {
                    state.0.insert(last_number, turn as u64 - 1);
                }
                *state = (state.0.clone(), Some(new_number));
                Some(new_number)
            },
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let expected = vec![0, 3, 6, 0, 3, 3, 1, 0, 4, 0, 2, 0];
        let numbers: Vec<_> = spoken_numbers(vec![0, 3, 6]).take(expected.len()).collect();
        assert_eq!(numbers, expected);
    }

    #[test]
    fn example_2() {
        assert_eq!(spoken_numbers(vec![0, 3, 6]).nth(2019).unwrap(), 436);
    }

    #[test]
    fn example_3() {
        assert_eq!(
            spoken_numbers(vec![0, 3, 6]).nth(30000000 - 1).unwrap(),
            175594
        );
    }
}

// fn read_input() {
// }
