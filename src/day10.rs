pub fn part1(input: &[String]) -> i32 {
    let mut x = 1;
    let mut cycle = 0;
    let mut inst_cycle_counter = 0;
    let mut signal = 0;
    for instruction in input.iter() {
        let mut next_value: Option<i32> = None;
        if instruction == "noop" {
            println!("processing noop");
            next_value = None;
            inst_cycle_counter = 1;
            println!("inst_counter = {inst_cycle_counter}\nnext value = {next_value:?}");
        } else if !instruction.is_empty() {
            println!("processing addx");
            let value_vec: Vec<i32> = instruction
                .split(' ')
                .filter_map(|x| {
                    if x != "addx" {
                        Some(x.parse::<i32>().unwrap())
                    } else {
                        None
                    }
                })
                .collect();
            let value: i32 = value_vec[0];
            next_value = Some(value);
            inst_cycle_counter = 2;
            println!("inst_counter = {inst_cycle_counter}\nnext value = {next_value:?}");
        }
        for _ in 0..inst_cycle_counter {
            cycle += 1;
            if cycle == 20 || (cycle > 20 && (cycle - 20) % 40 == 0) {
                signal += cycle * x;
                println!("cycle: {cycle}, value = {}", cycle * x);
            }
        }
        if let Some(value) = next_value {
            x += value;
        }
    }
    signal
}

pub fn part2(input: &[String]) {
    let mut x = 1;
    let mut cycle = 0;
    let mut inst_cycle_counter = 0;
    for instruction in input.iter() {
        let mut next_value: Option<i32> = None;
        if instruction == "noop" {
            next_value = None;
            inst_cycle_counter = 1;
        } else if !instruction.is_empty() {
            let value_vec: Vec<i32> = instruction
                .split(' ')
                .filter_map(|x| {
                    if x != "addx" {
                        Some(x.parse::<i32>().unwrap())
                    } else {
                        None
                    }
                })
                .collect();
            let value: i32 = value_vec[0];
            next_value = Some(value);
            inst_cycle_counter = 2;
        }
        for _ in 0..inst_cycle_counter {
            cycle += 1;
            let pos = (cycle - 1) % 40;
            /*if cycle == 10 {
                println!("\ncycle: {cycle}, pos: {pos}, x: {x}");
                return;
            }*/
            if (pos == x - 1) || (pos == x) || (pos == x + 1) {
                print!("#");
            } else {
                print!(" ");
            }

            if pos == 39 {
                println!();
            }
        }
        if let Some(value) = next_value {
            x += value;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_input;

    #[test]
    fn test_part1() {
        let input = read_input("./input/10.txt");
        let result = part1(&input);
        println!("Result: {result}");
    }

    #[test]
    fn test_part2() {
        let input = read_input("./input/10.txt");
        part2(&input);
    }
}
