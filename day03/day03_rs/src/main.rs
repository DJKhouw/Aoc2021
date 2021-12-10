use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|l| u32::from_str_radix(l, 2).unwrap())
        .collect::<Vec<_>>();
    let part1 = solve1(&input, 12);
    println!("Part 1: {}", part1);
    let part2 = solve2(&input, 12);
    println!("Part 2: {}", part2);
}
pub fn get_inverse(n: u32, bits: usize) -> u32 {
    n ^ (2_u32.pow(bits.try_into().unwrap()) - 1)
}

pub fn get_bit_at(n: u32, pos: usize) -> bool {
    if pos < 32 {
        n & (1 << (pos - 1)) != 0
    } else {
        false
    }
}

pub fn common_bits(input: &[u32], bits: usize, most: bool) -> Vec<bool> {
    let halfway = input.len() / 2;
    let mut ones = vec![0; bits];
    for val in input {
        for pos in 1..=bits {
            if get_bit_at(*val, pos) {
                ones[pos - 1] += 1;
            }
        }
    }
    let mut result = Vec::with_capacity(bits);
    for n in ones.iter().take(bits) {
        if *n > halfway {
            if most {
                result.push(true);
            } else {
                result.push(false);
            }
        } else if *n == halfway && input.len() % 2 == 0 {
            if most {
                result.push(true);
            } else {
                result.push(false)
            }
        } else if most {
            result.push(false);
        } else {
            result.push(true);
        }
    }
    result
}
pub fn solve1(input: &[u32], bits: usize) -> u32 {
    let most_common = common_bits(input, bits, true);
    let gamma_string = most_common
        .iter()
        .map(|b| if *b { '1' } else { '0' })
        .rev()
        .collect::<String>();
    let gamma_rate = u32::from_str_radix(&gamma_string, 2).unwrap();
    let epsilon_rate = get_inverse(gamma_rate, bits);

    gamma_rate * epsilon_rate
}
pub fn reduce(input: &[u32], bits: usize, most_common: bool) -> u32 {
    let mut result = input.to_vec();
    let mut cb = common_bits(&result, bits, most_common);
    let mut temp = Vec::new();
    for pos in (1..=bits).rev() {
        for n in result.iter() {
            if get_bit_at(*n, pos) == cb[pos - 1] {
                temp.push(*n);
                continue;
            }
        }

        if temp.len() == 1 {
            return temp[0];
        }
        result.clear();
        result.append(&mut temp);
        cb = common_bits(&result, bits, most_common);
    }
    0
}
pub fn solve2(input: &[u32], bits: usize) -> u32 {
    let oxygen_rating = reduce(input, bits, true);
    let co2_rating = reduce(input, bits, false);

    oxygen_rating * co2_rating
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_most_common() {
        let sequence = [30, 22, 23, 21, 28, 16, 25];
        let mcb = common_bits(&sequence, 5, true);
        println!("most common: {:?}", &mcb);
        assert!(!mcb[0]);
        assert!(!mcb[1]);
        assert!(mcb[2]);
        assert!(!mcb[3]);
        assert!(mcb[4]);
    }
    #[test]
    fn test_least_common() {
        let sequence = [30, 22, 23, 21, 28, 16, 25];
        let lcb = common_bits(&sequence, 5, false);
        assert!(lcb[0]);
        assert!(lcb[1]);
        assert!(!lcb[2]);
        assert!(lcb[3]);
        assert!(!lcb[4]);
    }
    #[test]
    fn test_inverse() {
        assert_eq!(get_inverse(1, 5), 30);
        assert_eq!(get_inverse(3, 5), 28);
    }

    #[test]
    fn test_get_bit_at() {
        assert!(!get_bit_at(4, 1));
        assert!(!get_bit_at(4, 2));
        assert!(get_bit_at(4, 3));
        assert!(!get_bit_at(4, 4));
        assert!(!get_bit_at(4, 5));
    }
    #[test]
    fn test_solve1() {
        let input = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ]
        .iter()
        .map(|l| u32::from_str_radix(l, 2).unwrap())
        .collect::<Vec<_>>();

        assert_eq!(solve1(&input, 5), 198);
    }

    #[test]
    fn test_most_common_bits() {
        let input = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ]
        .iter()
        .map(|l| u32::from_str_radix(l, 2).unwrap())
        .collect::<Vec<_>>();
        println!("input: {:?}", &input);
        let most_common = common_bits(&input, 5, true);
        println!("most common: {:?}", &most_common);
        assert!(!most_common[0]);
        assert!(most_common[1]);
        assert!(most_common[2]);
        assert!(!most_common[3]);
        assert!(most_common[4]);
    }
    #[test]
    fn test_reduce() {
        let input = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ]
        .iter()
        .map(|l| u32::from_str_radix(l, 2).unwrap())
        .collect::<Vec<_>>();
        assert_eq!(reduce(&input, 5, true), 23);
        assert_eq!(reduce(&input, 5, false), 10);
    }

    #[test]
    fn test_solve2() {
        let input = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ]
        .iter()
        .map(|l| u32::from_str_radix(l, 2).unwrap())
        .collect::<Vec<_>>();
        assert_eq!(solve2(&input, 5), 230);
    }
}
