use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|l| l.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let part1 = solve1(&input);
    println!("Part 1: {}", part1);
    let part2 = solve2(&input);
    println!("Part 2: {}", part2);
}

pub fn solve1(input: &[i32]) -> i32 {
    let mut output = 0;
    for i in 0..input.len() {
        if i == 0 {
            continue;
        }
        if input[i] > input[i - 1] {
            output += 1;
        }
    }
    output
}

pub fn solve2(input: &[i32]) -> i32 {
    let mut output = 0;
    let max = input.len() - 2;
    let mut windows = Vec::new();
    for i in 0..max {
        windows.push(input[i]+input[i+1]+input[i+2]);
    }
    for i in 0..windows.len() {
        if i == 0 {
            continue;
        }
        if windows[i] > windows[i - 1] {
            output += 1;
        }
    }
    output
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(solve1(&input), 7);
    }

    #[test]
    fn test_solve2() {
        let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(solve2(&input),5);
        
    }
}
