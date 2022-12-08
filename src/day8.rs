pub fn part1(input: &[String]) -> i32 {
    let mut grid = vec![];
    for row in input {
        let r = row
            .chars()
            .filter_map(|x| {
                if x.is_numeric() {
                    Some(((x as i32) - 48, false))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        grid.push(r);
    }

    for row in grid.iter_mut() {
        let mut tallest_from_left = -1;
        for col in row.iter_mut() {
            if tallest_from_left < col.0 {
                tallest_from_left = col.0;
                col.1 = true;
            }
        }
        let mut tallest_from_right = -1;
        for col in row.iter_mut().rev() {
            if tallest_from_right < col.0 {
                tallest_from_right = col.0;
                col.1 = true;
            }
        }
    }

    for col in 0..grid.len() {
        let mut tallest_from_top = -1;
        for row in grid.iter_mut() {
            if tallest_from_top < row[col].0 {
                tallest_from_top = row[col].0;
                row[col].1 = true;
            }
        }
        let mut tallest_from_bottom = -1;
        for row in grid.iter_mut().rev() {
            if tallest_from_bottom < row[col].0 {
                tallest_from_bottom = row[col].0;
                row[col].1 = true;
            }
        }
    }

    let mut sum = 0;
    for row in grid.iter() {
        for &(_height, visible) in row.iter() {
            if visible {
                sum += 1;
            }
        }
    }

    sum
}

fn calculate_score(grid: &[Vec<i32>], x: usize, y: usize) -> i32 {
    // if x == 0 || x == grid[0].len() - 1 || y == 0 || y == grid.len() - 1 {
    //     return 0;
    // }

    let mut left_score = 0;
    for l in (0..x).rev() {
        left_score += 1;
        if grid[y][l] >= grid[y][x] {
            break;
        }
    }

    let mut right_score = 0;
    for r in (x + 1)..grid[0].len() {
        right_score += 1;
        if grid[y][r] >= grid[y][x] {
            break;
        }
    }

    let mut up_score = 0;
    for u in (0..y).rev() {
        up_score += 1;
        if grid[u][x] >= grid[y][x] {
            break;
        }
    }

    let mut down_score = 0;
    for d in (y + 1)..grid.len() {
        down_score += 1;
        if grid[d][x] >= grid[y][x] {
            break;
        }
    }

    left_score * right_score * down_score * up_score
}

pub fn part2(input: &[String]) -> i32 {
    let mut grid = vec![];
    for row in input {
        let r = row
            .chars()
            .filter_map(|x| {
                if x.is_numeric() {
                    Some((x as i32) - 48)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        grid.push(r);
    }

    let mut highest_score = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            let score = calculate_score(&grid, x, y);
            if score > highest_score {
                highest_score = score;
            }
        }
    }

    highest_score
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_input;

    #[test]
    fn test_part1() {
        let input = read_input("./input/8.txt");
        let result = part1(&input);
        println!("Result: {result}");
    }

    #[test]
    fn test_part2() {
        let input = read_input("./input/8.txt");
        let result = part2(&input);
        println!("Result: {result}");
    }
}
