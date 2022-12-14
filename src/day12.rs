type Cell = (u8, bool);
type Point = (usize, usize);
type Tracker = (Point, u32);

fn chars_to_cell(c: char) -> Cell {
    match c {
        'S' => (0, false),
        'E' => (27, false),
        x => ((x as u8) - 96, false),
    }
}

fn push_next(grid: &mut Vec<Vec<Cell>>, current: Tracker, next: &mut Vec<Tracker>) {
    let height: u8 = grid[current.0.1][current.0.0].0;
    let length: u32 = current.1;
    
    if current.0.1 > 0 {
        let mut next_cell = &mut grid[current.0.1 - 1][current.0.0];
        if !next_cell.1 && next_cell.0 <= height + 1 {
            // add point above current to next stack
            next_cell.1 = true;
            next.push(((current.0.0, current.0.1 - 1), length + 1));
        }
    }
    if current.0.0 < grid[0].len() - 1 {
        let mut next_cell = &mut grid[current.0.1][current.0.0 + 1];
        if !next_cell.1 && next_cell.0 <= height + 1 {
            // add point to the right of current to next stack
            next_cell.1 = true;
            next.push(((current.0.0 + 1, current.0.1), length + 1));
        }
    }
    if current.0.1 < grid.len() - 1 {
        let mut next_cell = &mut grid[current.0.1 + 1][current.0.0];
        if !next_cell.1 && next_cell.0 <= height + 1 {
            // add point bellow current to next stack
            next_cell.1 = true;
            next.push(((current.0.0, current.0.1 + 1), length + 1));
        }
    }
    if current.0.0 > 0 {
        let mut next_cell = &mut grid[current.0.1][current.0.0 - 1];
        if !next_cell.1 && next_cell.0 <= height + 1 {
            // add point to the left of current to next stack
            next_cell.1 = true;
            next.push(((current.0.0 - 1, current.0.1), length + 1));
        }
    }
}

fn push_next2(grid: &mut Vec<Vec<Cell>>, current: Tracker, next: &mut Vec<Tracker>) {
    let height: u8 = grid[current.0.1][current.0.0].0;
    let length: u32 = current.1;
    
    if current.0.1 > 0 {
        let mut next_cell = &mut grid[current.0.1 - 1][current.0.0];
        if !next_cell.1 && next_cell.0 >= height - 1 {
            // add point above current to next stack
            next_cell.1 = true;
            next.push(((current.0.0, current.0.1 - 1), length + 1));
        }
    }
    if current.0.0 < grid[0].len() - 1 {
        let mut next_cell = &mut grid[current.0.1][current.0.0 + 1];
        if !next_cell.1 && next_cell.0 >= height - 1 {
            // add point to the right of current to next stack
            next_cell.1 = true;
            next.push(((current.0.0 + 1, current.0.1), length + 1));
        }
    }
    if current.0.1 < grid.len() - 1 {
        let mut next_cell = &mut grid[current.0.1 + 1][current.0.0];
        if !next_cell.1 && next_cell.0 >= height - 1 {
            // add point bellow current to next stack
            next_cell.1 = true;
            next.push(((current.0.0, current.0.1 + 1), length + 1));
        }
    }
    if current.0.0 > 0 {
        let mut next_cell = &mut grid[current.0.1][current.0.0 - 1];
        if !next_cell.1 && next_cell.0 >= height - 1 {
            // add point to the left of current to next stack
            next_cell.1 = true;
            next.push(((current.0.0 - 1, current.0.1), length + 1));
        }
    }
}

pub fn part1(input: &[String]) -> u32 {
    let mut grid: Vec<Vec<Cell>> = vec![]; //column, row
    let mut start: Point = (0, 0);
    let mut end: Point = (0, 0);
    for (y, line) in input.iter().enumerate() {
        let mut row: Vec<Cell> = line.trim().chars().map(chars_to_cell).collect();
        for (x, cell) in row.iter_mut().enumerate() {
            if cell.0 == 0 {
                start = (x, y);
                cell.0 = 1;
            } else if cell.0 == 27 {
                end = (x, y);
                cell.0 = 26;
            }
        }
        grid.push(row);
    }

    let mut next_stack: Vec<Tracker> = vec![(start, 0)];
    while !next_stack.is_empty() {
        let mut next: Vec<Tracker> = vec![];
        for current in next_stack.drain(..) {
            if current.0 == end {
                return current.1;
            }
            push_next(&mut grid, current, &mut next);
        }
        next_stack = next;
    }
    

    0
}

pub fn part2(input: &[String]) -> u32 {
    let mut grid: Vec<Vec<Cell>> = vec![]; //column, row
    let mut end: Point = (0, 0);
    let mut start: Point = (0, 0);
    for (y, line) in input.iter().enumerate() {
        let mut row: Vec<Cell> = line.trim().chars().map(chars_to_cell).collect();
        for (x, cell) in row.iter_mut().enumerate() {
            if cell.0 == 0 {
                start = (x, y);
                cell.0 = 1;
            } else if cell.0 == 27 {
                end = (x, y);
                cell.0 = 26;
            }
        }
        grid.push(row);
    }

    let mut next_stack: Vec<Tracker> = vec![(end, 0)];
    while !next_stack.is_empty() {
        let mut next: Vec<Tracker> = vec![];
        for current in next_stack.drain(..) {
            if grid[current.0.1][current.0.0].0 == 1 {
                return current.1;
            }
            push_next2(&mut grid, current, &mut next);
        }
        next_stack = next;
    }
    

    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_input;

    #[test]
    fn test_part1() {
        let input = read_input("./input/12.txt");
        let result = part1(&input);
        println!("Result: {result}");
    }

    #[test]
    fn test_part2() {
        let input = read_input("./input/12.txt");
        let result = part2(&input);
        println!("Result: {result}");
    }
}
