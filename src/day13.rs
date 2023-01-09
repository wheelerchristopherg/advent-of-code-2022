#[derive(Debug, Clone)]
enum Item {
    Start,
    End,
    Number(u32),
}

fn parse_line(line: &str) -> Vec<Item> {
    let mut num = String::new();
    let mut result: Vec<Item> = vec![];
    for c in line.chars() {
        if c == '[' {
            result.push(Item::Start);
        } else if c == ']' || c == ',' {
            if let Ok(n) = num.parse::<u32>() {
                num.clear();
                result.push(Item::Number(n));
            }
            if c == ']' {
                result.push(Item::End);
            }
        } else if c.is_numeric() {
            num.push(c);
        }
    }
    result
}

fn print_items(line: Vec<Item>) {
    for item in line.iter() {
        match item {
            Item::Start => print!("["),
            Item::End => print!("]"),
            Item::Number(n) => print!(" {n} "),
        }
    }
    println!();
}

fn compare(left: Vec<Item>, right: Vec<Item>) -> bool {
    let mut l = left;
    let mut r = right;
    let mut right_order = true;
    let mut i = 0;
    loop {
        let Some(li) = l.get(i) else {break};
        let Some(ri) = r.get(i) else {break};
        // println!("compare {li:?} to {ri:?}");

        match (li, ri) {
            (Item::Number(ln), Item::Number(rn)) => {
                if rn < ln {
                    right_order = false;
                    break;
                } else if ln < rn {
                    right_order = true;
                    break;
                }
            }
            (Item::Start, Item::Start) => {
                right_order = true;
            }
            (Item::End, Item::End) => {
                right_order = true;
            }
            (Item::Start, Item::Number(_)) => {
                r.insert(i, Item::Start);
                r.insert(i + 2, Item::End);
            }
            (Item::Number(_), Item::Start) => {
                l.insert(i, Item::Start);
                l.insert(i + 2, Item::End);
            }
            (Item::End, _) => {
                break;
            }
            (_, Item::End) => {
                right_order = false;
                break;
            }
        }
        i += 1;
    }
    right_order
}

pub fn part1(input: &[String]) -> i32 {
    let mut pairs: Vec<(Vec<Item>, Vec<Item>)> = vec![];
    for lines in input.windows(2).step_by(3) {
        // println!("lines: {lines:?}");
        let left: Vec<Item> = parse_line(&lines[0]);
        let right: Vec<Item> = parse_line(&lines[1]);
        pairs.push((left, right));
    }

    let mut sum = 0;
    for (n, (left, right)) in pairs.into_iter().enumerate() {
        // println!("pair {}", n + 1);

        let right_order = compare(left, right);
        if right_order {
            // println!("order is correct");
            sum += (n as i32) + 1;
        }
        // println!();
    }
    sum
}

pub fn part2(input: &[String]) -> i32 {
    let mut packets: Vec<Vec<Item>> = vec![];
    for lines in input.windows(2).step_by(3) {
        // println!("lines: {lines:?}");
        packets.push(parse_line(&lines[0]));
        packets.push(parse_line(&lines[1]));
    }

    let mut sorted: Vec<Vec<Item>> = vec![];

    packets.push(vec![
        Item::Start,
        Item::Start,
        Item::Number(6),
        Item::End,
        Item::End,
    ]);
    packets.push(vec![
        Item::Start,
        Item::Start,
        Item::Number(2),
        Item::End,
        Item::End,
    ]);

    for packet in packets.into_iter().skip(5).take(2) {
        let mut i = sorted.len();
        for ind in 0..sorted.len() {
            println!("comparing:\n\t{packet:?}\n\t{:?}", sorted[ind]);
            if compare(packet.clone(), sorted[ind].clone()) {
                i = ind;
            }
        }
        println!("inserted {packet:?} at {i}\n");
        sorted.insert(i, packet.clone());
    }

    for line in sorted {
        print_items(line);
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_input;

    #[test]
    fn test_part1() {
        let input = read_input("./input/13.txt");
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
