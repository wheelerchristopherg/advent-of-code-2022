fn get_priority(c: char) -> i32 {
    if ('A'..='Z').contains(&c) {
        c as i32 - 38
    } else {
        c as i32 - 96
    }
}

fn part1(input: &[String]) -> i32 {
    let mut sum = 0;
    for line in input.iter() {
        let middle = line.len() / 2;
        let first = &line[0..middle];
        let second = &line[middle..];

        println!("first : {first}");
        println!("second: {second}");

        let mut common = ' ';
        for c in first.chars() {
            if second.contains(c) {
                println!("same: {c}");
                common = c;
                break;
            }
        }

        let priority = get_priority(common);
        println!("priority: {priority}\n");
        sum += priority;
    }

    sum
}

fn part2(input: &[String]) -> i32 {
    let mut sum = 0;
    let mut input_iter = input.windows(3);
    while let Some(w) = input_iter.next() {
        let first = &w[0];
        let second = &w[1];
        let third = &w[2];

        println!("first : {first}");
        println!("second: {second}");
        println!("third : {third}");

        let mut common = ' ';
        for c in first.chars() {
            if second.contains(c) && third.contains(c) {
                println!("same: {c}");
                common = c;
                break;
            }
        }

        let priority = get_priority(common);
        println!("priority: {priority}\n");
        sum += priority;

        // skip windows that contain any lines that have already been processed
        input_iter.next();
        input_iter.next();
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_input;

    #[test]
    fn test_part1() {
        let input = read_input("./input/3.txt");
        let result = part1(&input[..]);
        println!("Result: {result}");
    }

    #[test]
    fn test_part2() {
        let input = read_input("./input/3.txt");
        let result = part2(&input[..]);
        println!("Result: {result}");
    }
}
