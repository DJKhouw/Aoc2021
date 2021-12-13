use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt")
        .unwrap()
        .split(',')
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let part1 = solve1(&input);
    println!("Part 1: {:?}", &part1);
    let part2 = solve2(&input);
    println!("Part 2: {:?}", &part2);
}
pub fn solve1(positions: &[usize]) -> (usize, usize) {
    let max = positions.iter().max().unwrap();
    let mut move_to = 0;
    let mut fuel_used = 0;
    let mut temp = 0;
    for n in 1..*max {
        for p in positions.iter() {
            match p.cmp(&n) {
                std::cmp::Ordering::Less => temp += n - *p,
                std::cmp::Ordering::Equal => continue,
                std::cmp::Ordering::Greater => temp += *p - n,
            }
        }
        if fuel_used == 0 {
            fuel_used = temp;
            temp = 0;
            continue;
        }
        if temp < fuel_used {
            fuel_used = temp;
            move_to = n;
            temp = 0
        }
    }
    (move_to, fuel_used)
}

pub fn solve2(positions: &[usize]) -> (usize, usize) {
    let max = positions.iter().max().unwrap();
    let mut move_to = 0;
    let mut fuel_used = 0;
    let mut temp: usize = 0;
    for n in 1..*max {
        for p in positions {
            temp += calculate_fuel_use(*p, n);
        }
        if fuel_used == 0 {
            fuel_used = temp;
            move_to = n;
            temp = 0;
            continue;
        }
        if temp < fuel_used {
            fuel_used = temp;
            move_to = n;
            temp = 0;
        }
    }
    (move_to, fuel_used)
}

pub fn calculate_fuel_use(from: usize, to: usize) -> usize {
    let n = match from.cmp(&to) {
        std::cmp::Ordering::Less => to - from,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => from - to,
    };
    n * (n + 1) / 2
}
#[cfg(test)]
mod tests {
    use super::*;
    static EXAMPLE_DATA: [usize; 10] = [16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
    #[test]
    fn test_solve1() {
        assert_eq!(solve1(&EXAMPLE_DATA), (2_usize, 37_usize));
    }

    #[test]
    fn test_caluculate_fuel_use() {
        assert_eq!(calculate_fuel_use(10, 2), 36);
        assert_eq!(calculate_fuel_use(2, 10), 36);
    }

    #[test]
    fn test_solve2() {
        assert_eq!(solve2(&EXAMPLE_DATA), (5_usize, 168_usize));
    }
}
