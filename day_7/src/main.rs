use regex::Regex;
use std::io::prelude::*;
use std::{collections::HashMap, env};
use std::{collections::HashSet, io};
#[macro_use]
extern crate lazy_static;

#[derive(Debug)]
struct ContainsRule {
    number: usize,
    name: String,
}

#[derive(Debug)]
struct BagRule {
    name: String,
    contains_rules: Vec<ContainsRule>,
}

fn main() {
    let stdin = io::stdin();
    let part: String = env::args().nth(1).unwrap();

    let rules: Vec<_> = stdin
        .lock()
        .lines()
        .filter_map(Result::ok)
        .map(|l| {
            lazy_static! {
                static ref RE: Regex = Regex::new(r"^(\w+ \w+) bags contain (.*).$").unwrap();
            }
            match RE.captures(l.as_str()) {
                Some(caps) => {
                    let color = caps.get(1).unwrap().as_str();
                    let res = caps.get(2).unwrap().as_str();
                    BagRule {
                        name: String::from(color),
                        contains_rules: if res == "no other bags" {
                            vec![]
                        } else {
                            res.split(", ")
                                .map(|s| {
                                    lazy_static! {
                                        static ref RE2: Regex =
                                            Regex::new(r"^(\d+) (\w+ \w+) bags?$").unwrap();
                                    }
                                    let caps2 = RE2.captures(s).unwrap();
                                    let number: usize =
                                        caps2.get(1).unwrap().as_str().parse().unwrap();
                                    let name = caps2.get(2).unwrap().as_str();
                                    ContainsRule {
                                        name: String::from(name),
                                        number: number,
                                    }
                                })
                                .collect::<Vec<_>>()
                        },
                    }
                }
                None => panic!("'{}' doesn't match", l),
            }
        })
        .collect();

    match part.as_str() {
        "part_1" => {
            let mut contained_vertices: HashMap<String, Vec<String>> = HashMap::new();
            for rule in rules {
                for contains_rule in rule.contains_rules {
                    let colors = contained_vertices
                        .entry(contains_rule.name.clone())
                        .or_insert(vec![]);
                    colors.push(rule.name.clone());
                }
            }

            let mut colors: HashSet<&str> = HashSet::new();
            let mut queue = vec!["shiny gold"];

            while let Some(color) = queue.pop() {
                colors.insert(color);
                if let Some(container_colors) = contained_vertices.get(color) {
                    for container_color in container_colors {
                        if !colors.contains(container_color.as_str()) {
                            queue.push(container_color.as_str())
                        }
                    }
                }
            }

            println!("{:?}", colors.iter().count() - 1);
        }
        "part_2" => {
            let mut contains_vertices: HashMap<String, Vec<ContainsRule>> = HashMap::new();
            for rule in rules {
                contains_vertices
                    .entry(rule.name)
                    .or_insert(rule.contains_rules);
            }

            println!("{:#?}", contains_vertices);

            println!("{:?}", bags_count(&contains_vertices, "shiny gold") - 1);
        }
        _ => (),
    }
}

fn bags_count(env: &HashMap<String, Vec<ContainsRule>>, color: &str) -> usize {
    let contains_rules = env.get(&String::from(color)).unwrap();
    contains_rules.iter().fold(1, |acc, rule| {
        acc + rule.number * bags_count(env, &rule.name)
    })
}

// bags_count("shiny gold")
// = 1 + 2 * bags_count("dark red")
// = 1 + 2 * (2 * bags_count("dark orange"))
