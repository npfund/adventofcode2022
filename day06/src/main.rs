use std::fs::File;
use std::io::Read;

fn main() {
    part1();
}

fn part1() {
    let mut line = Vec::new();
    let mut file = File::open("input.txt").unwrap();
    file.read_to_end(&mut line).unwrap();

    for (window_num, window) in line.windows(4).enumerate() {
        let mut good = true;
        'outer: for (pos, byte) in window.iter().enumerate() {
            for (other_pos, other) in window.iter().enumerate() {
                if pos != other_pos && byte == other {
                    good = false;
                    break 'outer;
                }
            }
        }

        if good {
            println!("{}", window_num + 4);
            break;
        }
    }
}
