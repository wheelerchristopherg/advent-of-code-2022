use regex::Regex;
use std::collections::VecDeque;

#[derive(Debug)]
struct Monkey {
    items: VecDeque<Item>,
    test: u64,
    operation: Op,
    value: Option<u64>,
    passed: ID,
    failed: ID,
    inspect_count: u64,
}

type Item = u64;
type ID = usize;

#[derive(Debug)]
enum Op {
    Add,
    Mult,
}

impl Monkey {
    fn new() -> Self {
        Monkey {
            items: VecDeque::new(),
            test: 0,
            operation: Op::Add,
            value: None,
            passed: 0,
            failed: 0,
            inspect_count: 0,
        }
    }

    fn take_turn(monkeys: &mut [Monkey], id: ID) {
        while let Some(item) = monkeys[id].items.pop_front() {
            monkeys[id].inspect_count += 1;
            let new: Item = monkeys[id].perform_operation(item) / 3;
            let next_monkey: ID = monkeys[id].perform_test(new);
            monkeys[next_monkey].throw_to(new);
        }
    }

    fn take_turn2(monkeys: &mut [Monkey], id: ID) {
        while let Some(item) = monkeys[id].items.pop_front() {
            monkeys[id].inspect_count += 1;
            let new: Item = monkeys[id].perform_operation(item);
            let next_monkey: ID = monkeys[id].perform_test(new);
            monkeys[next_monkey].throw_to2(new);
        }
    }

    fn perform_operation(&self, item: Item) -> Item {
        let operand = self.value.unwrap_or(item);
        match self.operation {
            Op::Add => item + operand,
            Op::Mult => item * operand,
        }
    }

    fn perform_test(&self, item: Item) -> ID {
        if item % self.test == 0 {
            self.passed
        } else {
            self.failed
        }
    }

    fn throw_to(&mut self, item: Item) {
        self.items.push_back(item);
    }
    
    fn throw_to2(&mut self, item: Item) {
        let mut corrected_item = item % (self.test * 2);
        if corrected_item == 0 {
            corrected_item = self.test;
        }
        self.items.push_back(corrected_item);
    }

    fn parse_operation(&mut self, line: &str) {
        let re_op = Regex::new(r"Operation: new = old (\S) (\S+)$").unwrap();
        let matches = re_op.captures(line).unwrap();
        let op_str = &matches[1];
        let value = &matches[2];
        println!("op: {op_str}\nvalue: {value}");
        self.operation = match op_str {
            "*" => Op::Mult,
            "+" => Op::Add,
            _ => panic!(),
        };

        self.value = if value == "old" {
            None
        } else {
            Some(value.parse::<u64>().unwrap())
        };
    }

    fn parse_pass_fail(&mut self, line: &str) {
        let re = Regex::new(r"If (\S+): throw to monkey (\d)$").unwrap();
        let matches = re.captures(line).unwrap();
        let predicate = &matches[1];
        let id: ID = matches[2].parse::<ID>().unwrap();
        if predicate == "true" {
            self.passed = id;
        } else {
            self.failed = id;
        }
    }
}

pub fn part1(input: &[String]) -> u64 {
    let mut monkeys: Vec<Monkey> = vec![];
    let mut input_iter = input.iter();
    let re_test = Regex::new(r"Test: divisible by (\S+)").unwrap();
    while let Some(line) = input_iter.next() {
        if !line.contains("Monkey") {
            continue;
        }
        let mut monkey = Monkey::new();

        let items_raw: Vec<&str> = input_iter.next().unwrap().split(':').collect();
        if let Some(items) = items_raw.get(1) {
            monkey.items = items
                .split(',')
                .map(|x| x.trim().parse::<Item>().unwrap())
                .collect();
        }
        monkey.parse_operation(input_iter.next().unwrap());

        let test_str = input_iter.next().unwrap().trim();
        let test_captures = re_test.captures(test_str).unwrap();
        monkey.test = test_captures[1].parse::<u64>().unwrap();
        monkey.parse_pass_fail(input_iter.next().unwrap());
        monkey.parse_pass_fail(input_iter.next().unwrap());

        println!("{monkey:?}");
        monkeys.push(monkey);
    }

    println!("do sim");
    for _round in 0..20 {
        for id in 0..monkeys.len() {
            Monkey::take_turn(&mut monkeys, id);
        }
    }
    println!("sim done");

    let mut monkey_business: Vec<u64> = vec![];
    for (i, monkey) in monkeys.iter().enumerate() {
        println!("Monkey {i}: {}", monkey.inspect_count);
        monkey_business.push(monkey.inspect_count);
    }
    monkey_business.sort();
    monkey_business.iter().rev().take(2).copied().reduce(|a, b| a * b).unwrap()
}

pub fn part2(input: &[String]) -> u64 {
    let mut monkeys: Vec<Monkey> = vec![];
    let mut input_iter = input.iter();
    let re_test = Regex::new(r"Test: divisible by (\S+)").unwrap();
    while let Some(line) = input_iter.next() {
        if !line.contains("Monkey") {
            continue;
        }
        let mut monkey = Monkey::new();

        let items_raw: Vec<&str> = input_iter.next().unwrap().split(':').collect();
        if let Some(items) = items_raw.get(1) {
            monkey.items = items
                .split(',')
                .map(|x| x.trim().parse::<Item>().unwrap())
                .collect();
        }
        monkey.parse_operation(input_iter.next().unwrap());

        let test_str = input_iter.next().unwrap().trim();
        let test_captures = re_test.captures(test_str).unwrap();
        monkey.test = test_captures[1].parse::<u64>().unwrap();
        monkey.parse_pass_fail(input_iter.next().unwrap());
        monkey.parse_pass_fail(input_iter.next().unwrap());

        println!("{monkey:?}");
        monkeys.push(monkey);
    }

    println!("do sim");
    for _round in 0..10000 {
        for id in 0..monkeys.len() {
            Monkey::take_turn2(&mut monkeys, id);
        }
    }
    println!("sim done");

    let mut monkey_business: Vec<u64> = vec![];
    for (i, monkey) in monkeys.iter().enumerate() {
        println!("Monkey {i}: {}", monkey.inspect_count);
        monkey_business.push(monkey.inspect_count);
    }
    monkey_business.sort();
    monkey_business.iter().rev().take(2).copied().reduce(|a, b| a * b).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_input;

    #[test]
    fn test_part1() {
        let input = read_input("./input/11.txt");
        let result = part1(&input);
        println!("Result: {result}");
    }

    #[test]
    fn test_part2() {
        let input = read_input("./input/test.txt");
        let result = part2(&input);
        println!("Result: {result}");
    }
}
