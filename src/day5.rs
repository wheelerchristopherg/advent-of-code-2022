use std::collections::VecDeque;

pub fn part1(input: &[String]) -> String {
    let mut len = 0;
    for line in input.iter() {
        let no_spaces = line.clone().trim().to_owned();
        if no_spaces.contains('1') {
            len = no_spaces.chars().last().unwrap() as usize - 48;
            break;
        }
    }

    let mut stacks = vec![];
    for _ in 0..len {
        let v: VecDeque<char> = VecDeque::new();
        stacks.push(v);
    }

    let mut read_moves = false;
    for line in input.iter() {
        if !read_moves {
            if let Some(c) = line.chars().nth(1) {
                if c == '1' {
                    read_moves = true;
                    continue;
                }
            }
            for (i, s) in stacks.iter_mut().enumerate() {
                if let Some(c) = line.chars().nth(i * 4 + 1) {
                    if c != ' ' {
                        s.push_back(c);
                    }
                }
            }
        } else if line.is_empty() {
            continue;
        } else {
            let words = line.split(' ').collect::<Vec<&str>>();
            let count = words[1].parse::<usize>().unwrap();
            let from = words[3].parse::<usize>().unwrap();
            let to = words[5].parse::<usize>().unwrap();
            for _ in 0..count {
                let c = stacks[from - 1].pop_front();
                if let Some(c) = c {
                    stacks[to - 1].push_front(c);
                }
            }
        }
    }

    let mut message = String::new();
    for s in stacks.iter_mut() {
        if let Some(c) = s.pop_front() {
            message.push(c);
        }
    }

    message
}

pub fn part2(input: &[String]) -> String {
    let mut len = 0;
    for line in input.iter() {
        let no_spaces = line.clone().trim().to_owned();
        if no_spaces.contains('1') {
            len = no_spaces.chars().last().unwrap() as usize - 48;
            break;
        }
    }

    let mut stacks = vec![];
    for _ in 0..len {
        let v: VecDeque<char> = VecDeque::new();
        stacks.push(v);
    }

    let mut temp_stack = vec![];
    let mut read_moves = false;
    for line in input.iter() {
        if !read_moves {
            if let Some(c) = line.chars().nth(1) {
                if c == '1' {
                    read_moves = true;
                    continue;
                }
            }
            for (i, s) in stacks.iter_mut().enumerate() {
                if let Some(c) = line.chars().nth(i * 4 + 1) {
                    if c != ' ' {
                        s.push_back(c);
                    }
                }
            }
        } else if line.is_empty() {
            continue;
        } else {
            let words = line.split(' ').collect::<Vec<&str>>();
            let count = words[1].parse::<usize>().unwrap();
            let from = words[3].parse::<usize>().unwrap();
            let to = words[5].parse::<usize>().unwrap();
            for _ in 0..count {
                let c = stacks[from - 1].pop_front();
                if let Some(c) = c {
                    temp_stack.push(c);
                }
            }
            for _ in 0..count {
                if let Some(c) = temp_stack.pop() {
                    stacks[to - 1].push_front(c);
                }
            }
        }
    }

    let mut message = String::new();
    for s in stacks.iter_mut() {
        if let Some(c) = s.pop_front() {
            message.push(c);
        }
    }

    message
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_input;

    #[test]
    fn test_part1() {
        let input = read_input("./input/5.txt");
        let result = part1(&input[..]);
        println!("Result: {result}");
    }

    #[test]
    fn test_part2() {
        let input = read_input("./input/5.txt");
        let result = part2(&input[..]);
        println!("Result: {result}");
    }
}
