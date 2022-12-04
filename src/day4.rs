fn convert_ranges(input: &str) -> (i32, i32) {
    let r = input
        .split('-')
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    (r[0], r[1])
}

fn part1(input: &[String]) -> i32 {
    let mut sum = 0;
    for line in input {
        let ranges = line.split(',').map(convert_ranges).collect::<Vec<_>>();
        if (ranges[0].0 <= ranges[1].0 && ranges[0].1 >= ranges[1].1)
            || (ranges[0].0 >= ranges[1].0 && ranges[0].1 <= ranges[1].1)
        {
            println!("{line}");
            sum += 1;
        }
    }

    sum
}

fn part2(input: &[String]) -> i32 {
    let mut sum = 0;
    for line in input {
        let ranges = line.split(',').map(convert_ranges).collect::<Vec<_>>();
        if (ranges[0].0 <= ranges[1].0 && ranges[0].1 >= ranges[1].1)
            || (ranges[0].0 >= ranges[1].0 && ranges[0].1 <= ranges[1].1)
            || (ranges[0].0 <= ranges[1].1 && ranges[0].0 >= ranges[1].0)
            || (ranges[0].1 >= ranges[1].0 && ranges[0].1 <= ranges[1].1)
        {
            println!("{line}");
            sum += 1;
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_input;

    #[test]
    fn test_part1() {
        let input = read_input("./input/4.txt");
        let result = part1(&input[..]);
        println!("Result: {result}");
    }

    #[test]
    fn test_part2() {
        let input = read_input("./input/4.txt");
        let result = part2(&input[..]);
        println!("Result: {result}");
    }
}
