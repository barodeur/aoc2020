use std::collections::HashSet;
use std::env;
use std::io;
use std::io::prelude::*;

struct GroupAnswersStream<R: BufRead> {
    lines: std::io::Lines<R>,
    done: bool,
}
impl<R: BufRead> GroupAnswersStream<R> {
    fn new(reader: R) -> GroupAnswersStream<R> {
        GroupAnswersStream {
            lines: reader.lines(),
            done: false,
        }
    }
}
impl<R: BufRead> Iterator for GroupAnswersStream<R> {
    type Item = Vec<Vec<char>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        let mut group: Self::Item = vec![];

        loop {
            let line = match self.lines.next() {
                None => {
                    self.done = true;
                    break;
                }
                Some(Err(_)) => panic!("error"),
                Some(Ok(l)) => l,
            };

            match line.as_str() {
                "" => break,
                _ => {
                    group.push(line.chars().collect());
                }
            }
        }

        Some(group)
    }
}

fn main() {
    let stdin = io::stdin();

    let part: String = env::args().nth(1).unwrap();
    match part.as_str() {
        "part_1" => {
            let stream = GroupAnswersStream::new(stdin.lock());
            let res = stream
                .map(|g| {
                    let t: HashSet<char> = g.into_iter().flatten().collect();
                    t.len()
                })
                .fold(0, |acc, count| acc + count);
            println!("{}", res);
        }
        "part_2" => {
            let stream = GroupAnswersStream::new(stdin.lock());
            let res = stream
                .map(|g| {
                    g.iter()
                        .fold(
                            g.first()
                                .unwrap()
                                .iter()
                                .cloned()
                                .collect::<HashSet<char>>(),
                            |memo, answers| {
                                let answers_set =
                                    answers.iter().cloned().collect::<HashSet<char>>();
                                memo.intersection(&answers_set).cloned().collect()
                            },
                        )
                        .len()
                })
                .fold(0, |acc, count| acc + count);
            println!("{}", res);
        }
        _ => (),
    }
}
