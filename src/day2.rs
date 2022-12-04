fn get_score(value: char) -> i32 {
    match value {
        'A' => 1,
        'B' => 2,
        'C' => 3,
        _ => 0,
    }
}

fn winner(opponent: char, you: char) -> i32 {
    if opponent == you {
        3
    } else if (opponent == 'A' && you == 'B')
        || (opponent == 'B' && you == 'C')
        || (opponent == 'C' && you == 'A')
    {
        6
    } else {
        0
    }
}

fn part1(input: &[String]) -> i32 {
    let mut score = 0;
    for round in input.iter() {
        let (a, b) = (round.chars().nth(0).unwrap(), round.chars().nth(2).unwrap());
        let c = match b {
            'X' => 'A',
            'Y' => 'B',
            'Z' => 'C',
            _ => ' ',
        };

        let win = winner(a, c);
        score += win + get_score(c);
    }

    score
}

fn get_shape_score(opponent: char, result: char) -> i32 {
    let rock = 1;
    let paper = 2;
    let scissors = 3;
    match result {
        //lose
        'X' => match opponent {
            'A' => scissors,
            'B' => rock,
            'C' => paper,
            _ => 0,
        },
        //draw
        'Y' => {
            (match opponent {
                'A' => rock,
                'B' => paper,
                'C' => scissors,
                _ => 0,
            }) + 3
        }
        //win
        'Z' => {
            (match opponent {
                'A' => paper,
                'B' => scissors,
                'C' => rock,
                _ => 0,
            }) + 6
        }
        _ => 0,
    }
}

fn part2(input: &[String]) -> i32 {
    let mut score = 0;
    for round in input.iter() {
        let (a, b) = (round.chars().next().unwrap(), round.chars().nth(2).unwrap());

        score += get_shape_score(a, b);
    }

    score
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_input;

    #[test]
    fn test_part1() {
        let input = read_input("./input/2.txt");
        let result = part1(&input[..]);
        println!("Result: {result}");
    }

    #[test]
    fn test_part2() {
        let input = read_input("./input/2.txt");
        let result = part2(&input[..]);
        println!("Result: {result}");
    }
}
