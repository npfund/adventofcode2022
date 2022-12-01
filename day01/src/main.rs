use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    part1();
    part2();
}

fn part1() {
    let file = BufReader::new(File::open("input.txt").unwrap());

    let mut elves = Vec::new();
    let mut elf = Vec::new();
    for line in file.lines().map(|l| l.unwrap()) {
        if line.len() == 0 {
            elves.push(elf);
            elf = Vec::new();
        } else {
            let calories = line.parse::<u64>().unwrap();
            elf.push(calories);
        }
    }

    elves.sort_by(|l, r| r.iter().sum::<u64>().cmp(&l.iter().sum::<u64>()));

    let most = &elves[0];
    println!("{}", most.iter().sum::<u64>());
}

fn part2() {
    let file = BufReader::new(File::open("input.txt").unwrap());

    let mut elves = Vec::new();
    let mut elf = Vec::new();
    for line in file.lines().map(|l| l.unwrap()) {
        if line.len() == 0 {
            elves.push(elf);
            elf = Vec::new();
        } else {
            let calories = line.parse::<u64>().unwrap();
            elf.push(calories);
        }
    }

    elves.sort_by(|l, r| r.iter().sum::<u64>().cmp(&l.iter().sum::<u64>()));

    let most = &elves[0];
    let second = &elves[1];
    let third = &elves[2];
    let total = most.iter().sum::<u64>() + second.iter().sum::<u64>() + third.iter().sum::<u64>();
    println!("{}", total);
}
