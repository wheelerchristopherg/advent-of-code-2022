use std::collections::HashSet;

fn find_marker(input: &str, length: usize) -> usize {
    let mut marker = length;
    let mut char_set: HashSet<char> = HashSet::with_capacity(length);
    for window in input.as_bytes().windows(length) {
        for &c in window {
            char_set.insert(c as char);
        }

        if char_set.len() == length {
            return marker;
        }
        char_set.clear();
        marker += 1;
    }

    0
}

pub fn part1(input: &[String]) -> usize {
    find_marker(input[0].as_str(), 4)
}

pub fn part2(input: &[String]) -> usize {
    find_marker(input[0].as_str(), 14)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_input;

    #[test]
    fn test_part1() {
        let input = read_input("./input/6.txt");
        let result = part1(&input);
        println!("Result: {result}");
    }

    #[test]
    fn test_part2() {
        let input = read_input("./input/6.txt");
        let result = part2(&input);
        println!("Result: {result}");
    }
}
