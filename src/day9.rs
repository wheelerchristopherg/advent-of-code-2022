use std::collections::HashSet;

type Pos = (i32, i32);

fn get_new_tail_pos(tail_pos: Pos, head_to: Pos) -> Pos {
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
        for column in grid.iter() {
            match column[y] {
                -2 => print!("s"),
                -1 => print!("."),
                0 => print!("H"),
                x => print!("{x}"),
            }
        }
        println!();
    }
}

fn variable_rope(input: &[String], length: usize) -> usize {
    // const XOFFSET: i32 = 25;
    // const YOFFSET: i32 = 25;
    // const WIDTH: usize = 50;
    // const HEIGHT: usize = 50;
    let mut visited: HashSet<Pos> = HashSet::new();
    let mut rope: Vec<Pos> = vec![(0, 0); length];
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
            rope[0] = get_new_head_pos(rope[0], dir, 1);
            for i in 1..rope.len() {
                rope[i] = get_new_tail_pos(rope[i], rope[i - 1]);
            }
            // display_rope(&rope, WIDTH, HEIGHT, XOFFSET, YOFFSET);
            // println!();
            visited.insert(*rope.last().unwrap());
        }
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
        let result = variable_rope(&input, 2);
        println!("Result: {result}");
    }

    #[test]
    fn test_part2() {
        let input = read_input("./input/9.txt");
        let result = variable_rope(&input, 10);
        println!("Result: {result}");
    }
}
