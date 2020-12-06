use std::default::*;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Default, Debug)]
struct Passport {
    byr: bool,
    iyr: bool,
    eyr: bool,
    hgt: bool,
    hcl: bool,
    ecl: bool,
    pid: bool,
    cid: bool,
}

impl Passport {
    fn is_valid(&self) -> bool {
        self.byr && self.iyr && self.eyr && self.hgt && self.hcl && self.ecl && self.pid
    }
}

struct PassportStream<R: BufRead> {
    lines: std::io::Lines<R>,
    done: bool,
}

impl<R: BufRead> PassportStream<R> {
    fn new(reader: R) -> PassportStream<R> {
        PassportStream {
            lines: reader.lines(),
            done: false,
        }
    }
}

impl<R: BufRead> Iterator for PassportStream<R> {
    type Item = Passport;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        let mut passport: Passport = Default::default();

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
                    for property in line.split_whitespace() {
                        match property.split(":").next().unwrap() {
                            "byr" => passport.byr = true,
                            "iyr" => passport.iyr = true,
                            "eyr" => passport.eyr = true,
                            "hgt" => passport.hgt = true,
                            "hcl" => passport.hcl = true,
                            "ecl" => passport.ecl = true,
                            "pid" => passport.pid = true,
                            "cid" => passport.cid = true,
                            _ => (),
                        }
                    }
                }
            }
        }

        Some(passport)
    }
}

fn main() -> std::io::Result<()> {
    let f = File::open("input.txt")?;
    let f = BufReader::new(f);

    let result = PassportStream::new(f).filter(Passport::is_valid).count();

    println!("{}", result);

    Ok(())
}
