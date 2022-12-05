use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    part1();
}

fn part1() {
    let file = BufReader::new(File::open("input.txt").unwrap());

    let mut sum = 0;
    for line in file.lines().map(|l| l.unwrap()) {
        let (left, right) = line.split_at(line.len() / 2);

        let l_chars = left.chars().collect::<HashSet<_>>();
        let r_chars = right.chars().collect::<HashSet<_>>();

        for &char in l_chars.intersection(&r_chars) {
            let priority = if char.is_ascii_lowercase() {
                char as u32 - 96
            } else {
                char as u32 - 64 + 26
            };

            sum += priority
        }
    }

    println!("{sum}");
}
