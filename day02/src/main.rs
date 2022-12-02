use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    part1();
    part2();
}

fn part1() {
    let file = BufReader::new(File::open("input.txt").unwrap());

    let mut score = 0;
    for line in file.lines().map(|l| l.unwrap()) {
        let (them, me) = line.split_once(' ').unwrap();

        match me {
            "X" => score += 1,
            "Y" => score += 2,
            "Z" => score += 3,
            _ => unreachable!(),
        }

        match (me, them) {
            ("X", "C") | ("Y", "A") | ("Z", "B") => score += 6,
            ("X", "A") | ("Y", "B") | ("Z", "C") => score += 3,
            ("X", "B") | ("Y", "C") | ("Z", "A") => score += 0,
            _ => unreachable!(),
        }
    }

    println!("{score}");
}

fn part2() {
    let file = BufReader::new(File::open("input.txt").unwrap());

    let mut score = 0;
    for line in file.lines().map(|l| l.unwrap()) {
        let (them, me) = line.split_once(' ').unwrap();

        match me {
            "X" => score += 0,
            "Y" => score += 3,
            "Z" => score += 6,
            _ => unreachable!(),
        }

        match (me, them) {
            ("X", "B") | ("Y", "A") | ("Z", "C") => score += 1,
            ("X", "C") | ("Y", "B") | ("Z", "A") => score += 2,
            ("X", "A") | ("Y", "C") | ("Z", "B") => score += 3,
            _ => unreachable!(),
        }
    }

    println!("{score}");
}
