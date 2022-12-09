use std::collections::HashSet;

type Pos = (i32, i32);

fn get_new_tail_pos(tail_pos: Pos, _head_from: Pos, head_to: Pos) -> Pos {
    let dir_vec = (head_to.0 - tail_pos.0, head_to.1 - tail_pos.1);
    if dir_vec.0.abs() > 1 || dir_vec.1.abs() > 1 {
        let x = if dir_vec.0.abs() > 0 {
            tail_pos.0 + (dir_vec.0 / dir_vec.0.abs())
        } else {
            tail_pos.0
        };
        let y = if dir_vec.1.abs() > 0 {
            tail_pos.1 + (dir_vec.1 / dir_vec.1.abs())
        } else {
            tail_pos.1
        };
        // println!("old tail: {tail_pos:?}, new head: {head_to:?}, new tail: ({x}, {y})");
        (x, y)
    } else {
        tail_pos
    }
}

fn get_new_head_pos(head_pos: Pos, direction: &str, distance: i32) -> Pos {
    match direction {
        "R" => (head_pos.0 + distance, head_pos.1),
        "L" => (head_pos.0 - distance, head_pos.1),
        "U" => (head_pos.0, head_pos.1 + distance),
        "D" => (head_pos.0, head_pos.1 - distance),
        _ => head_pos,
    }
}

fn display_rope(rope: &[Pos], width: usize, height: usize, x_offset: i32, y_offset: i32) {
    let mut grid = vec![];
    for _ in 0..width {
        grid.push(vec![-1; height]);
    }

    for (i, &(x, y)) in rope.iter().enumerate() {
        if grid[(x + x_offset) as usize][(y + y_offset) as usize] == -1 {
            grid[(x + x_offset) as usize][(y + y_offset) as usize] = i as i32;
        }
    }

    if grid[x_offset as usize][y_offset as usize] == -1 {
        grid[x_offset as usize][y_offset as usize] = -2;
    }

    for y in (0..grid[0].len()).rev() {
        for x in 0..grid.len() {
            if grid[x][y] == -1 {
                print!(".");
            } else if grid[x][y] == 0 {
                print!("H");
            } else if grid[x][y] == -2 {
                print!("s");
            } else {
                print!("{}", grid[x][y]);
            }
        }
        println!();
    }
}

pub fn part1(input: &[String]) -> usize {
    let mut visited: HashSet<Pos> = HashSet::new();
    let mut head: Pos = (0, 0);
    let mut tail: Pos = (0, 0);
    visited.insert(tail);
    for (dir, dist) in input.iter().map(|x| {
        let temp = x.split(' ').collect::<Vec<_>>();
        (temp[0], temp[1].parse::<i32>().unwrap())
    }) {
        for _ in 0..dist {
            let new_head = get_new_head_pos(head, dir, 1);
            let new_tail = get_new_tail_pos(tail, head, new_head);
            visited.insert(new_tail);
            tail = new_tail;
            head = new_head;
        }
    }

    visited.len()
}

pub fn part2(input: &[String]) -> usize {
    // const XOFFSET: i32 = 15;
    // const YOFFSET: i32 = 8;
    // const WIDTH: usize = 32;
    // const HEIGHT: usize = 25;
    // const XOFFSET: i32 = 0;
    // const YOFFSET: i32 = 0;
    // const WIDTH: usize = 6;
    // const HEIGHT: usize = 6;
    let mut visited: HashSet<Pos> = HashSet::new();
    let mut rope: Vec<Pos> = vec![(0, 0); 10];
    visited.insert(*rope.last().unwrap());
    // println!("rope: {rope:?}");
    // display_rope(&rope, WIDTH, HEIGHT, XOFFSET, YOFFSET);
    println!();
    for (dir, dist) in input.iter().map(|x| {
        let temp = x.split(' ').collect::<Vec<_>>();
        (temp[0], temp[1].parse::<i32>().unwrap())
    }) {
        // println!("{} {}", dir, dist);
        for _ in 0..dist {
            let mut new_rope: Vec<Pos> = vec![get_new_head_pos(rope[0], dir, 1)];
            for i in 1..rope.len() {
                new_rope.push(get_new_tail_pos(
                    rope[i],
                    rope[i - 1],
                    *new_rope.last().unwrap(),
                ))
            }
            rope = new_rope;
            // println!("rope: {rope:?}");
            visited.insert(*rope.last().unwrap());
        }
        // display_rope(&rope, WIDTH, HEIGHT, XOFFSET, YOFFSET);
        // println!();
    }

    visited.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_input;

    #[test]
    fn test_part1() {
        let input = read_input("./input/9.txt");
        let result = part1(&input);
        println!("Result: {result}");
    }

    #[test]
    fn test_part2() {
        let input = read_input("./input/9.txt");
        let result = part2(&input);
        println!("Result: {result}");
    }
}
