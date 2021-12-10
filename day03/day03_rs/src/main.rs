use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|l| u32::from_str_radix(l, 2).unwrap())
        .collect::<Vec<_>>();
    let part1 = solve1(&input, 12);
    println!("Part 1: {}", part1);
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

pub fn most_common_bits(input: &[u32], bits: usize) -> Vec<bool> {
    let halfway = input.len() / 2;
    let mut ones = vec![0; bits.into()];
    for val in input {
        for pos in 1..=bits {
            if get_bit_at(*val, pos) {
                ones[pos - 1] += 1;
            }
        }
    }
    let mut result = Vec::with_capacity(bits);
    for n in ones.iter().take(bits) {
        if *n >= halfway {
            result.push(true);
        } else {
            result.push(false);
        }
    }
    result
}
pub fn solve1(input: &[u32], bits: usize) -> u32 {
    let most_common = most_common_bits(input, bits);
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
    println!("Values: {:?}", &result);
    let mut mcb = most_common_bits(&result, bits);
    let mut temp = Vec::new();
    for pos in (1..=bits).rev() {
        println!("[reduce] position {}", &pos);
        println!(
            "MCB: {}",
            mcb.iter()
                .rev()
                .map(|b| if *b { '1' } else { '0' })
                .collect::<String>()
        );
        for n in result.iter() {
            if most_common {
                if get_bit_at(*n, pos) == mcb[pos - 1] {
                    println!(
                        "Position {}, value: {}, bit: {}, mcb: {}",
                        &pos,
                        *n,
                        get_bit_at(*n, pos),
                        &mcb[pos - 1]
                    );
                    temp.push(*n);
                    continue;
                }
            } else if get_bit_at(*n, pos) != mcb[pos - 1] {
                temp.push(*n);
                continue;
            }
        }
        if temp.len() == 1 {
            return temp[0];
        }
        println!("Found {} numbers: {:?}", temp.len(), &temp);
        result.clear();
        result.append(&mut temp);
        mcb = most_common_bits(&result, bits);
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
    fn test_inverse() {
        assert_eq!(get_inverse(1, 5), 30);
        assert_eq!(get_inverse(3, 5), 28);
    }

    #[test]
    fn test_get_bit_at() {
        let most_common = vec![false, true, true, false, true];
        assert_eq!(get_bit_at(4, 1), false);
        assert_eq!(get_bit_at(4, 2), false);
        assert_eq!(get_bit_at(4, 3), true);
        assert_eq!(get_bit_at(4, 4), false);
        assert_eq!(get_bit_at(4, 5), false);
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
    fn test_oxygen_generator_rating() {
        let input = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ]
        .iter()
        .map(|l| u32::from_str_radix(l, 2).unwrap())
        .collect::<Vec<_>>();
        assert_eq!(solve2(&input, 5), 230);
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
        let most_common = most_common_bits(&input, 5);
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
        let most_common = most_common_bits(&input, 5);
        assert_eq!(reduce(&input, 5, true), 23);
        // fails, because mcb from 2nd iteration is 11111 instead of 10100
    }

    #[test]
    fn divide() {
        println!("{}", (7_usize / 2_usize));
    }
}
