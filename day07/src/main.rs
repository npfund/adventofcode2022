use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    part1();
    part2();
}

fn part1() {
    let file = BufReader::new(File::open("input.txt").unwrap());

    let mut current_dir = "/".to_owned();
    let mut tree = HashMap::new();
    let cd_regex = Regex::new(r#"\$ cd (.*)"#).unwrap();
    let file_regex = Regex::new(r#"(\d+) (.*)"#).unwrap();
    let dir_regex = Regex::new(r#"dir (.*)"#).unwrap();
    for line in file.lines().map(|l| l.unwrap()) {
        if let Some(captures) = cd_regex.captures(&line) {
            let dir = captures.get(1).unwrap().as_str().to_owned();
            if dir == ".." {
                let current = tree.get(&current_dir).unwrap();
                if let Object::Dir {
                    contents: _contents,
                    parent,
                } = current
                {
                    current_dir = parent.clone();
                }
            } else {
                let dir = format!("{}/{}", current_dir, dir);
                tree.entry(dir.clone()).or_insert(Object::Dir {
                    contents: vec![],
                    parent: current_dir.clone(),
                });
                current_dir = dir;
            }
        } else if let Some(captures) = file_regex.captures(&line) {
            let size: i64 = captures.get(1).unwrap().as_str().parse().unwrap();
            let file_name = captures.get(2).unwrap().as_str().to_owned();
            let file_name = format!("{}!{}", current_dir, file_name);

            let dir = tree.get_mut(&current_dir).unwrap();
            if let Object::Dir {
                contents,
                parent: _parent,
            } = dir
            {
                contents.push(file_name.clone());
            }
            tree.insert(file_name.clone(), Object::File { size });
        } else if let Some(captures) = dir_regex.captures(&line) {
            let dir_name = captures.get(1).unwrap().as_str().to_owned();
            let dir = tree.get_mut(&current_dir).unwrap();
            if let Object::Dir {
                contents,
                parent: _parent,
            } = dir
            {
                let dir_name = format!("{}/{}", current_dir, dir_name);
                contents.push(dir_name.clone());
            }
        }
    }

    let mut sum = 0;
    for (name, object) in tree.iter() {
        if matches!(object, Object::Dir { .. }) {
            let size = get_dir_size(&tree, name);
            if size <= 100000 {
                sum += size;
            }
        }
    }

    println!("{}", sum);
}

fn part2() {
    let file = BufReader::new(File::open("input.txt").unwrap());

    let mut current_dir = "/".to_owned();
    let mut tree = HashMap::new();
    let cd_regex = Regex::new(r#"\$ cd (.*)"#).unwrap();
    let file_regex = Regex::new(r#"(\d+) (.*)"#).unwrap();
    let dir_regex = Regex::new(r#"dir (.*)"#).unwrap();
    for line in file.lines().map(|l| l.unwrap()) {
        if let Some(captures) = cd_regex.captures(&line) {
            let dir = captures.get(1).unwrap().as_str().to_owned();
            if dir == ".." {
                let current = tree.get(&current_dir).unwrap();
                if let Object::Dir {
                    contents: _contents,
                    parent,
                } = current
                {
                    current_dir = parent.clone();
                }
            } else {
                let dir = format!("{}/{}", current_dir, dir);
                tree.entry(dir.clone()).or_insert(Object::Dir {
                    contents: vec![],
                    parent: current_dir.clone(),
                });
                current_dir = dir;
            }
        } else if let Some(captures) = file_regex.captures(&line) {
            let size: i64 = captures.get(1).unwrap().as_str().parse().unwrap();
            let file_name = captures.get(2).unwrap().as_str().to_owned();
            let file_name = format!("{}!{}", current_dir, file_name);

            let dir = tree.get_mut(&current_dir).unwrap();
            if let Object::Dir {
                contents,
                parent: _parent,
            } = dir
            {
                contents.push(file_name.clone());
            }
            tree.insert(file_name.clone(), Object::File { size });
        } else if let Some(captures) = dir_regex.captures(&line) {
            let dir_name = captures.get(1).unwrap().as_str().to_owned();
            let dir = tree.get_mut(&current_dir).unwrap();
            if let Object::Dir {
                contents,
                parent: _parent,
            } = dir
            {
                let dir_name = format!("{}/{}", current_dir, dir_name);
                contents.push(dir_name.clone());
            }
        }
    }

    let disk_space = 70_000_000;
    let target = 30_000_000;
    let consumed = get_dir_size(&tree, "///");
    let free = disk_space - consumed;
    let to_delete = target - free;

    let mut delete_size = consumed;

    for (name, object) in tree.iter() {
        if matches!(object, Object::Dir { .. }) {
            let size = get_dir_size(&tree, name);
            if size > to_delete && size < delete_size {
                delete_size = size;
            }
        }
    }

    println!("{}", delete_size);
}

#[derive(Debug)]
enum Object {
    File {
        size: i64,
    },
    Dir {
        contents: Vec<String>,
        parent: String,
    },
}

fn get_dir_size(tree: &HashMap<String, Object>, dir_name: &str) -> i64 {
    let dir = tree.get(dir_name).unwrap();
    let mut total = 0;
    if let Object::Dir {
        contents,
        parent: _parent,
    } = dir
    {
        for name in contents {
            let thing = tree.get(name).unwrap();
            match thing {
                Object::File { size } => total += size,
                Object::Dir { .. } => {
                    total += get_dir_size(tree, name);
                }
            }
        }
    } else {
        panic!("{} is not a dir", dir_name);
    }

    total
}
