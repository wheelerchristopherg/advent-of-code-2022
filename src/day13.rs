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

fn stringify_items(line: Vec<Item>) -> String {
    let mut result = String::new();
    for item in line.iter() {
        match item {
            Item::Start => result.push('['),
            Item::End => result.push(']'),
            Item::Number(n) => result.push_str(&format!(" {} ", n).to_string()),
        }
    }
    result
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

pub fn part2(input: &[String]) -> usize {
    let mut packets: Vec<Vec<Item>> = vec![];
    for lines in input.windows(2).step_by(3) {
        // println!("lines: {lines:?}");
        packets.push(parse_line(&lines[0]));
        packets.push(parse_line(&lines[1]));
    }

    let mut sorted: Vec<Vec<Item>> = vec![];
    let divider_packet1 = vec![
        Item::Start,
        Item::Start,
        Item::Number(2),
        Item::End,
        Item::End,
    ];
    let divider_packet2 = vec![
        Item::Start,
        Item::Start,
        Item::Number(6),
        Item::End,
        Item::End,
    ];
    packets.push(divider_packet1.clone());
    packets.push(divider_packet2.clone());

    for packet in packets.into_iter() {
        let mut i = sorted.len();
        for (ind, item) in sorted.iter().enumerate() {
            // println!("comparing:\n\t{packet:?}\n\t{:?}", item);
            if compare(packet.clone(), item.clone()) && !compare(item.clone(), packet.clone()) {
                i = ind;
                break;
            }
        }
        // println!("inserted {packet:?} at {i}\n");
        sorted.insert(i, packet.clone());
    }

    let div1_str = stringify_items(divider_packet1);
    let div2_str = stringify_items(divider_packet2);
    let mut div1_index = 0;
    let mut div2_index = 0;
    for (i, packet) in sorted.iter().enumerate() {
        let string_repr = stringify_items(packet.clone());
        if string_repr == div1_str {
            div1_index = i + 1;
        } else if string_repr == div2_str {
            div2_index = i + 1;
        }
    }

    div1_index * div2_index
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
        let input = read_input("./input/13.txt");
        let result = part2(&input);
        println!("Result: {result}");
    }
}
