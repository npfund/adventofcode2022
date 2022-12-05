use regex::Regex;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    part1();
}

fn part1() {
    let file = BufReader::new(File::open("input.txt").unwrap());

    let mut stacks = vec![VecDeque::new(); 9];
    let crates_regex = Regex::new(r#"(?:(?:\[([A-Z])\] ?)|(?:    ))(?:(?:\[([A-Z])\] ?)|(?:    ))(?:(?:\[([A-Z])\] ?)|(?:    ))(?:(?:\[([A-Z])\] ?)|(?:    ))(?:(?:\[([A-Z])\] ?)|(?:    ))(?:(?:\[([A-Z])\] ?)|(?:    ))(?:(?:\[([A-Z])\] ?)|(?:    ))(?:(?:\[([A-Z])\] ?)|(?:    ))(?:(?:\[([A-Z])\] ?)|(?:    ))"#).unwrap();
    let commands_regex = Regex::new(r#"move (\d+) from (\d) to (\d)"#).unwrap();
    for line in file.lines().map(|l| l.unwrap()) {
        if let Some(captures) = crates_regex.captures(&line) {
            for (pos, cap) in captures.iter().enumerate().skip(1) {
                if let Some(m) = cap {
                    stacks[pos - 1].push_front(m.as_str().to_owned());
                }
            }
        } else if let Some(captures) = commands_regex.captures(&line) {
            let count = captures.get(1).unwrap().as_str().parse().unwrap();
            let source = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();
            let destination = captures.get(3).unwrap().as_str().parse::<usize>().unwrap();

            for _ in 0..count {
                if let Some(c) = stacks[source - 1].pop_back() {
                    stacks[destination - 1].push_back(c);
                }
            }
        }
    }

    for stack in stacks {
        print!("{}", stack.back().unwrap());
    }
    println!();
}
