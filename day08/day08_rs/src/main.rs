use core::panic;
use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let part1 = solve1(&input);
    println!("Part 1: {}", &part1);
    let part2 = solve2(&input);
    println!("Part 2: {}", &part2);
}

pub fn get_patterns_and_output(input: &str) -> (Vec<Vec<&str>>, Vec<Vec<&str>>) {
    let mut output = Vec::new();
    let mut patterns = Vec::new();
    for line in input.lines() {
        let parts = line.split(" | ").collect::<Vec<_>>();
        patterns.push(parts[0].trim().split(' ').collect());
        output.push(parts[1].trim().split(' ').collect());
    }
    (patterns, output)
}

pub fn solve1(input: &str) -> usize {
    let (_, output) = get_patterns_and_output(input);
    output
        .iter()
        .flatten()
        .filter(|&&w| [2_usize, 3_usize, 4_usize, 7_usize].contains(&w.len()))
        .count()
}
pub fn sort_chars(input: &str) -> Vec<char> {
    let mut output = input.chars().collect::<Vec<_>>();
    output.sort_unstable();
    output
}
pub fn sort_alphabetic(input: &str) -> String {
    let mut output = input.chars().collect::<Vec<_>>();
    output.sort_unstable();
    output.iter().collect()
}
pub fn decode_pattern(pattern: &[&str]) -> (Vec<Vec<char>>, HashMap<String, usize>) {
    let mut chars = vec![vec!['A']; 10];
    // let mut lines = Vec::with_capacity(7);

    let iterable = pattern.iter().map(|s| sort_chars(s)).collect::<Vec<_>>();
    chars[1] = if let Some(v) = iterable.iter().find(|s| s.len() == 2) {
        v.to_vec()
    } else {
        panic!("no '1' digit in pattern");
    };
    chars[4] = iterable
        .iter()
        .find(|s| s.len() == 4)
        .expect("no '4' digit in pattern")
        .to_vec();
    chars[7] = iterable
        .iter()
        .find(|s| s.len() == 3)
        .expect("no '7' digit in pattern")
        .to_vec();
    chars[8] = iterable
        .iter()
        .find(|s| s.len() == 7)
        .expect("no '8' digit in pattern")
        .to_vec();
    // 0,6,9 = 6, 2,3,5= 5
    let six_line_digits = iterable.iter().filter(|s| s.len() == 6).collect::<Vec<_>>();

    // 6 = length == 6, does not contain both letters of the '1' digit.
    chars[6] = if let Some(v) = six_line_digits
        .iter()
        .find(|s| chars[1].iter().any(|c| !s.contains(c)))
    {
        v.to_vec()
    } else {
        panic!("no '6' digit in pattern")
    };
    // 0 = length = 6, not digit '6' && missing letter in comparison to 8 is present in 4
    for d in six_line_digits.iter() {
        // skip if it's digit '6'
        if **d == chars[6] {
            continue;
        }
        // find the missing letter in the current digit:
        let missing = chars[8]
            .iter()
            .find(|&c| !d.contains(c))
            .expect("should be a missing letter");
        // if the missing letter is present in digit '4', then we have our '0' else, it's 9
        if chars[4].contains(missing) {
            chars[0] = d.to_vec();
        } else {
            chars[9] = d.to_vec();
        }
    }

    let five_line_digits = iterable.iter().filter(|s| s.len() == 5).collect::<Vec<_>>();

    // 3 = length == 5, contains both letters of the '1' digit.

    // 5 = length == 5, missing only 1 letter of the '9' digit.
    // 2 = length == 5, ??
    for &d in five_line_digits.iter() {
        if chars[1].iter().all(|c| d.contains(c)) {
            chars[3] = d.to_vec();
        } else if chars[9].iter().filter(|c| !d.contains(c)).count() == 1 {
            chars[5] = d.to_vec();
        } else {
            chars[2] = d.to_vec();
        }
    }
    let mut translation_table = HashMap::with_capacity(10);
    for (i, v) in chars.iter().enumerate() {
        translation_table.insert(v.iter().collect::<String>(), i);
    }
    (chars, translation_table)
}
pub fn get_value(input: &[&str], table: &HashMap<String, usize>) -> usize {
    let mut s = String::with_capacity(input.len());
    for &v in input.iter() {
        if let Some(&number) = table.get(&sort_alphabetic(v)) {
            s.push(char::from_digit(number.try_into().unwrap(), 10).unwrap());
        } else {
            panic!("couldn't find the number for {}", &v);
        }
    }
    s.parse().expect("not a number")
}
pub fn solve2(input: &str) -> usize {
    let (pattern, output) = get_patterns_and_output(input);

    pattern.iter().zip(output).fold(0, |acc, (p, o)| {
        let (_, table) = decode_pattern(p);
        acc + get_value(&o, &table)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    static EXAMPLE_DATA: &str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
    #[test]
    fn test_get_output() {
        let (_, output) = get_patterns_and_output(EXAMPLE_DATA);
        assert_eq!(output.len(), 10);
        assert_eq!(output[0].len(), 4);
        assert_eq!(output[0][0].len(), 7);
        assert_eq!(output[0][0], "fdgacbe");
    }
    #[test]
    fn test_get_patterns() {
        let (patterns, _) = get_patterns_and_output(EXAMPLE_DATA);
        assert_eq!(patterns.len(), 10);
        assert_eq!(patterns[0].len(), 10);
        assert_eq!(patterns[0][0].len(), 2);
        assert_eq!(patterns[0][0], "be");
    }
    #[test]
    fn test_sort_chars() {
        assert_eq!(sort_chars("edcba"), vec!['a', 'b', 'c', 'd', 'e']);
    }
    #[test]
    fn test_solve1() {
        assert_eq!(solve1(EXAMPLE_DATA), 26);
    }
    #[test]
    fn test_sort_alphabetic() {
        let input = "edcba";
        assert_eq!(sort_alphabetic(input), "abcde".to_string());
    }
    #[test]
    fn test_decode_pattern() {
        let (pattern, _) = get_patterns_and_output(
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf",
        );
        let (decoded, table) = decode_pattern(&pattern[0]);
        assert_eq!(decoded[0], vec!['a', 'b', 'c', 'd', 'e', 'g']);
        assert_eq!(table.len(), 10);
    }
    #[test]
    fn test_get_value() {
        let (pattern, output) = get_patterns_and_output(
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf",
        );
        let (_, table) = decode_pattern(&pattern[0]);
        assert_eq!(get_value(&output[0], &table), 5353);
    }
    #[test]
    fn test_solve2() {
        assert_eq!(solve2(EXAMPLE_DATA), 61229);
    }
}
