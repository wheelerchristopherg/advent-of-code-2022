use std::collections::{HashMap, HashSet};

use regex::Regex;

#[derive(Debug)]
struct File {
    name: String,
    size: u32,
}
#[derive(Debug)]
struct Directory {
    name: String,
    directories: HashSet<String>,
    files: HashSet<String>,
    size: Option<u32>,
}

#[derive(Debug)]
enum Entry {
    D(Directory),
    F(File),
}

impl Entry {
    fn new_dir(name: &str) -> Self {
        Entry::D(Directory {
            name: name.to_string(),
            directories: HashSet::new(),
            files: HashSet::new(),
            size: None,
        })
    }

    fn new_file(name: &str, size: u32) -> Self {
        Entry::F(File {
            name: name.to_string(),
            size,
        })
    }
}

impl Directory {
    fn add_file(&mut self, entry: &str) {
        self.files.insert(entry.to_string());
    }

    fn add_dir(&mut self, entry: &str) {
        self.directories.insert(entry.to_string());
    }

    fn add_size(&mut self, value: u32) {
        if let Some(size) = self.size {
            self.size = Some(size + value);
        } else {
            self.size = Some(value);
        }
    }
}

fn get_full_name(path: &[String], name: &str) -> String {
    let p = get_full_path(path);
    format!("{p};{name}")
}

fn get_full_path(path: &[String]) -> String {
    path.iter()
        .cloned()
        .reduce(|acc, x| format!("{acc};{x}"))
        .unwrap()
}

pub fn part1(input: &[String]) -> u32 {
    let re_command = Regex::new(r"\$ (\S+)(\s(\S+))?").unwrap();
    let re_file = Regex::new(r"(\d+) (\S+)").unwrap();
    let re_dir = Regex::new(r"dir (\S+)").unwrap();

    let mut entries: HashMap<String, Entry> = HashMap::new();
    let mut path: Vec<String> = vec![String::from("/")];
    entries.insert("/".to_string(), Entry::new_dir("/"));

    for line in input {
        if let Some(command) = re_command.captures(line.as_str()) {
            if command.get(1).unwrap().as_str() == "cd" {
                let d = command.get(3).unwrap().as_str().to_string();
                if d == ".." {
                    path.pop();
                } else if d == "/" {
                    path.clear();
                    path.push(d);
                } else {
                    path.push(d);
                }
            }
        } else if let Some(file) = re_file.captures(line.as_str()) {
            let size = file[1].to_string().parse::<u32>().unwrap();
            let name = file[2].to_string();
            let full_name = get_full_name(&path, name.as_str());
            if !entries.contains_key(&full_name) {
                entries.insert(full_name.clone(), Entry::new_file(name.as_str(), size));
            }
            let p = get_full_path(&path);
            let Entry::D(p) = entries.get_mut(&p).unwrap() else {
                println!("panic'd here trying to parse {name}");
                panic!()
            };
            p.add_file(name.as_str());
            for i in 0..path.len() {
                let p = get_full_path(&path[0..i + 1]);
                if let Entry::D(p) = entries.get_mut(&p).unwrap() {
                    p.add_size(size);
                }
            }
        } else if let Some(dir) = re_dir.captures(line.as_str()) {
            let name = dir[1].to_string();
            let full_name = get_full_name(&path, &name);
            if !entries.contains_key(&full_name) {
                entries.insert(full_name.clone(), Entry::new_dir(&name));
            }
            let path_str = get_full_path(&path);
            println!("path: {path_str:?}");
            if let Entry::D(p) = entries.get_mut(&path_str).unwrap() {
                p.add_dir(name.as_str());
            }
        }
    }

    let mut sum = 0;
    for (_name, entry) in entries.iter() {
        if let Entry::D(dir) = entry {
            if let Some(s) = dir.size {
                if s <= 100_000 {
                    sum += s;
                }
            }
        }
    }

    sum
}

pub fn part2(input: &[String]) -> u32 {
    let re_command = Regex::new(r"\$ (\S+)(\s(\S+))?").unwrap();
    let re_file = Regex::new(r"(\d+) (\S+)").unwrap();
    let re_dir = Regex::new(r"dir (\S+)").unwrap();

    let mut entries: HashMap<String, Entry> = HashMap::new();
    let mut path: Vec<String> = vec![String::from("/")];
    entries.insert("/".to_string(), Entry::new_dir("/"));

    for line in input {
        if let Some(command) = re_command.captures(line.as_str()) {
            if command.get(1).unwrap().as_str() == "cd" {
                let d = command.get(3).unwrap().as_str().to_string();
                if d == ".." {
                    path.pop();
                } else if d == "/" {
                    path.clear();
                    path.push(d);
                } else {
                    path.push(d);
                }
            }
        } else if let Some(file) = re_file.captures(line.as_str()) {
            let size = file[1].to_string().parse::<u32>().unwrap();
            let name = file[2].to_string();
            let full_name = get_full_name(&path, name.as_str());
            if !entries.contains_key(&full_name) {
                entries.insert(full_name.clone(), Entry::new_file(name.as_str(), size));
            }
            let p = get_full_path(&path);
            let Entry::D(p) = entries.get_mut(&p).unwrap() else {
                println!("panic'd here trying to parse {name}");
                panic!()
            };
            p.add_file(name.as_str());
            for i in 0..path.len() {
                let p = get_full_path(&path[0..i + 1]);
                if let Entry::D(p) = entries.get_mut(&p).unwrap() {
                    p.add_size(size);
                }
            }
        } else if let Some(dir) = re_dir.captures(line.as_str()) {
            let name = dir[1].to_string();
            let full_name = get_full_name(&path, &name);
            if !entries.contains_key(&full_name) {
                entries.insert(full_name.clone(), Entry::new_dir(&name));
            }
            let path_str = get_full_path(&path);
            println!("path: {path_str:?}");
            if let Entry::D(p) = entries.get_mut(&path_str).unwrap() {
                p.add_dir(name.as_str());
            }
        }
    }

    let total_used = if let Entry::D(d) = entries.get("/").unwrap() {
        d.size.unwrap()
    } else {
        0
    };

    let needed_unused: u32 = 30_000_000;
    let total_disk_space: u32 = 70_000_000;
    let actual_unused: u32 = total_disk_space - total_used;
    let need_to_delete: u32 = needed_unused - actual_unused;

    let mut dirs = vec![];
    for (_name, entry) in entries.iter() {
        if let Entry::D(dir) = entry {
            if let Some(s) = dir.size {
                if s >= need_to_delete {
                    dirs.push((dir.name.clone(), s));
                }
            }
        }
    }

    dirs.sort_by(|x, y| x.1.cmp(&y.1));
    println!("dirs: {dirs:?}");

    dirs[0].1
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_input;

    #[test]
    fn test_part1() {
        let input = read_input("./input/7.txt");
        let result = part1(&input);
        println!("Result: {result}");
    }

    #[test]
    fn test_part2() {
        let input = read_input("./input/7.txt");
        let result = part2(&input);
        println!("Result: {result}");
    }
}
