use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    part1();
    part2();
}

fn part1() {
    let file = BufReader::new(File::open("input.txt").unwrap());

    let mut count = 0;
    for line in file.lines().map(|l| l.unwrap()) {
        let (lhs, rhs) = line.split_once(',').unwrap();

        let (l_start, l_end): (u64, u64) = lhs
            .split_once('-')
            .map(|(s, e)| (s.parse().unwrap(), e.parse().unwrap()))
            .unwrap();

        let (r_start, r_end): (u64, u64) = rhs
            .split_once('-')
            .map(|(s, e)| (s.parse().unwrap(), e.parse().unwrap()))
            .unwrap();

        if (l_start <= r_start && r_end <= l_end) || (r_start <= l_start && l_end <= r_end) {
            count += 1;
        }
    }

    println!("{count}");
}

fn part2() {
    let file = BufReader::new(File::open("input.txt").unwrap());

    let mut count = 0;
    for line in file.lines().map(|l| l.unwrap()) {
        let (lhs, rhs) = line.split_once(',').unwrap();

        let (l_start, l_end): (u64, u64) = lhs
            .split_once('-')
            .map(|(s, e)| (s.parse().unwrap(), e.parse().unwrap()))
            .unwrap();

        let (r_start, r_end): (u64, u64) = rhs
            .split_once('-')
            .map(|(s, e)| (s.parse().unwrap(), e.parse().unwrap()))
            .unwrap();

        if (l_start <= r_start && l_end >= r_start)
            || (l_start <= r_end && r_end <= l_end)
            || (r_start <= l_start && r_end >= l_start)
            || (r_start <= l_end && l_end <= r_end)
        {
            count += 1;
        }
    }

    println!("{count}");
}
