use regex::Regex;
use std::default::*;
use std::env;
use std::io;
use std::io::prelude::*;
#[macro_use]
extern crate lazy_static;

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

#[derive(Default, Debug)]
struct StrictPassport {
    byr: Option<BirthYear>,
    iyr: Option<IssueYear>,
    eyr: Option<ExpirationYear>,
    hgt: Option<Height>,
    hcl: Option<HairColor>,
    ecl: Option<EyeColor>,
    pid: Option<PassportID>,
    cid: Option<CountryID>,
}
impl StrictPassport {
    fn is_valid(&self) -> bool {
        self.byr.as_ref().map_or(false, BirthYear::is_valid)
            && self.iyr.as_ref().map_or(false, IssueYear::is_valid)
            && self.eyr.as_ref().map_or(false, ExpirationYear::is_valid)
            && self.hgt.as_ref().map_or(false, Height::is_valid)
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }
}

#[derive(Debug)]
struct BirthYear(usize);
impl BirthYear {
    fn is_valid(&self) -> bool {
        1920 <= self.0 && self.0 <= 2002
    }
}

#[derive(Debug)]
struct IssueYear(usize);
impl IssueYear {
    fn is_valid(&self) -> bool {
        2010 <= self.0 && self.0 <= 2020
    }
}

#[derive(Debug)]
struct ExpirationYear(usize);
impl ExpirationYear {
    fn is_valid(&self) -> bool {
        2020 <= self.0 && self.0 <= 2030
    }
}

#[derive(Debug)]
enum Height {
    Cm(usize),
    In(usize),
}
impl Height {
    fn parse(string: &str) -> Result<Height, &'static str> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(\d+)(cm|in)$").unwrap();
        }
        match RE.captures(string) {
            Some(caps) => {
                let value = caps.get(1).unwrap().as_str().parse().unwrap();
                let unit = caps.get(2).unwrap().as_str();
                match unit {
                    "cm" => Ok(Height::Cm(value)),
                    "in" => Ok(Height::In(value)),
                    _ => Err("unkown unit"),
                }
            }
            None => Err("Wrong format"),
        }
    }
    fn is_valid(&self) -> bool {
        match *self {
            Height::Cm(v) => 150 <= v && v <= 193,
            Height::In(v) => 59 <= v && v <= 76,
        }
    }
}
#[derive(Debug)]
struct HairColor(String);
impl HairColor {
    fn parse(input: &str) -> Result<HairColor, &'static str> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^#[0-9a-z]{6}$").unwrap();
        }
        if RE.is_match(input) {
            Ok(HairColor(String::from(input)))
        } else {
            Err("unkown hair color format")
        }
    }
}

#[derive(Debug)]
enum EyeColor {
    Amber,
    Blue,
    Brown,
    Grey,
    Green,
    Hazel,
    Other,
}
impl EyeColor {
    fn parse(input: &str) -> Result<EyeColor, &'static str> {
        match input {
            "amb" => Ok(EyeColor::Amber),
            "blu" => Ok(EyeColor::Blue),
            "brn" => Ok(EyeColor::Brown),
            "gry" => Ok(EyeColor::Grey),
            "grn" => Ok(EyeColor::Green),
            "hzl" => Ok(EyeColor::Hazel),
            "oth" => Ok(EyeColor::Other),
            _ => Err("Unkown eye color"),
        }
    }
}

#[derive(Debug)]
struct PassportID(String);
impl PassportID {
    fn parse(input: &str) -> Result<PassportID, &'static str> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^\d{9}$").unwrap();
        }
        if RE.is_match(input) {
            Ok(PassportID(String::from(input)))
        } else {
            Err("Unkown passport id format")
        }
    }
}

#[derive(Debug)]
struct CountryID(String);

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
                        let split: Vec<&str> = property.split(":").collect();
                        let key = split[0];
                        match key {
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

struct StrictPassportStream<R: BufRead> {
    lines: std::io::Lines<R>,
    done: bool,
}
impl<R: BufRead> StrictPassportStream<R> {
    fn new(reader: R) -> StrictPassportStream<R> {
        StrictPassportStream {
            lines: reader.lines(),
            done: false,
        }
    }
}
impl<R: BufRead> Iterator for StrictPassportStream<R> {
    type Item = StrictPassport;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        let mut passport: StrictPassport = Default::default();

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
                        let split: Vec<&str> = property.split(":").collect();
                        let key = split[0];
                        let value = split[1];
                        match key {
                            "byr" => passport.byr = Some(BirthYear(value.parse().unwrap())),
                            "iyr" => passport.iyr = Some(IssueYear(value.parse().unwrap())),
                            "eyr" => passport.eyr = Some(ExpirationYear(value.parse().unwrap())),
                            "hgt" => passport.hgt = Height::parse(value).ok(),
                            "hcl" => passport.hcl = HairColor::parse(value).ok(),
                            "ecl" => passport.ecl = EyeColor::parse(value).ok(),
                            "pid" => passport.pid = PassportID::parse(value).ok(),
                            "cid" => passport.cid = Some(CountryID(String::from(value))),
                            _ => (),
                        }
                    }
                }
            }
        }

        Some(passport)
    }
}

fn main() {
    let stdin = io::stdin();

    let part: String = env::args().nth(1).unwrap();
    match part.as_str() {
        "part_1" => {
            let result = PassportStream::new(stdin.lock())
                .filter(Passport::is_valid)
                .count();
            println!("{}", result);
        }
        "part_2" => {
            let result = StrictPassportStream::new(stdin.lock())
                .filter(StrictPassport::is_valid)
                .count();
            println!("{}", result);
        }
        _ => {
            println!("unkown part")
        }
    }
}
