fn part1(input: &[String]) -> i32 {
    let mut sums = vec![];
    let mut current_sum = 0;
    for line in input.iter() {
        if line.is_empty() {
            sums.push(current_sum);
            current_sum = 0;
        } else {
            current_sum += line.parse::<i32>().unwrap_or(0);
        }
    }
    if current_sum > 0 {
        sums.push(current_sum);
    }

    *sums.iter().max().unwrap_or(&0)
}

fn part2(input: &[String]) -> i32 {
    let mut sums = vec![];
    let mut current_sum = 0;
    for line in input.iter() {
        if line.is_empty() {
            sums.push(current_sum);
            current_sum = 0;
        } else {
            current_sum += line.parse::<i32>().unwrap_or(0);
        }
    }
    if current_sum > 0 {
        sums.push(current_sum);
    }

    sums.sort();
    sums.iter().rev().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_input;

    #[test]
    fn test_part1() {
        let input = read_input("./input/1.txt");
        let result = part1(&input[..]);
        println!("Result: {result}");
    }

    #[test]
    fn test_part2() {
        let input = read_input("./input/1.txt");
        let result = part2(&input[..]);
        println!("Result: {result}");
    }
}
