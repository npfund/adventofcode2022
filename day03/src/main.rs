use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    part1();
    part2();
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

fn part2() {
    let file = BufReader::new(File::open("input.txt").unwrap());

    let lines = file.lines().map(|l| l.unwrap()).collect::<Vec<_>>();
    let mut sum = 0;
    for chunk in lines.chunks(3) {
        let one = chunk[0].chars().collect::<HashSet<_>>();
        let two = chunk[1].chars().collect::<HashSet<_>>();
        let three = chunk[2].chars().collect::<HashSet<_>>();

        let first = one.intersection(&two).copied().collect::<HashSet<_>>();

        for &char in first.intersection(&three) {
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
