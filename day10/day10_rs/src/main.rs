use std::{collections::HashMap, fs};

#[derive(Debug, PartialEq, Eq)]
pub enum LineState {
    Legal,
    Corrupt(char, char),
    Incomplete(Vec<char>),
}
fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let part1 = solve1(&input);
    println!("Part 1: {}", &part1);
    let part2 = solve2(&input);
    println!("Part 2: {}", &part2);
}

pub fn solve1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            if let LineState::Corrupt(_, f) = get_line_state(line) {
                get_syntax_error_score(f)
            } else {
                0
            }
        })
        .sum()
}
pub fn solve2(input: &str) -> usize {
    let mut scores = input
        .lines()
        .map(|line| {
            if let LineState::Incomplete(chars) = get_line_state(line) {
                get_autocomplete_score(&chars)
            } else {
                0
            }
        })
        .filter(|s| *s != 0)
        .collect::<Vec<_>>();
    scores.sort_unstable();
    let middle_idx = (scores.len() - 1) / 2;
    scores[middle_idx]
}
pub fn get_autocomplete_score(chars: &[char]) -> usize {
    let mut total = 0;
    let mut score: usize;
    for c in chars.iter() {
        score = match c {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => panic!("illegal character: {}", c),
        };
        total = (5 * total) + score;
    }
    total
}
pub fn get_syntax_error_score(c: char) -> usize {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("illegal character: {}", c),
    }
}
pub fn get_line_state(input: &str) -> LineState {
    let mut chunks = HashMap::new();
    let mut expected = Vec::new();
    for c in ['(', '[', '{', '<'] {
        chunks.insert(c, 0);
    }
    for c in input.chars() {
        match c {
            '(' => {
                *chunks.entry(c).or_insert(0) += 1;
                expected.push(')');
            }
            ')' => {
                *chunks.entry(c).or_insert(0) -= 1;
                if expected[expected.len() - 1] != c {
                    return LineState::Corrupt(expected.pop().unwrap(), c);
                } else {
                    expected.pop();
                }
            }
            '[' => {
                *chunks.entry(c).or_insert(0) += 1;
                expected.push(']');
            }
            ']' => {
                *chunks.entry(c).or_insert(0) -= 1;
                if expected[expected.len() - 1] != c {
                    return LineState::Corrupt(expected.pop().unwrap(), c);
                } else {
                    expected.pop();
                }
            }
            '{' => {
                *chunks.entry(c).or_insert(0) += 1;
                expected.push('}');
            }
            '}' => {
                *chunks.entry(c).or_insert(0) -= 1;
                if expected[expected.len() - 1] != c {
                    return LineState::Corrupt(expected.pop().unwrap(), c);
                } else {
                    expected.pop();
                }
            }
            '<' => {
                *chunks.entry(c).or_insert(0) += 1;
                expected.push('>');
            }
            '>' => {
                *chunks.entry(c).or_insert(0) -= 1;
                if expected[expected.len() - 1] != c {
                    return LineState::Corrupt(expected.pop().unwrap(), c);
                } else {
                    expected.pop();
                }
            }
            _ => panic!("Illegal character encountered: {}", c),
        }
    }
    let sum: i32 = chunks.iter().map(|(_, &v)| v).sum();
    match sum.cmp(&0) {
        std::cmp::Ordering::Less => panic!("sum < 0."),
        std::cmp::Ordering::Equal => LineState::Legal,
        std::cmp::Ordering::Greater => LineState::Incomplete(expected.into_iter().rev().collect()),
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_DATA: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn test_line_state() {
        let mut states = EXAMPLE_DATA.lines().map(get_line_state);

        assert_eq!(
            states.next(),
            Some(LineState::Incomplete(vec![
                '}', '}', ']', ']', ')', '}', ')', ']'
            ]))
        );
        assert_eq!(
            states.next(),
            Some(LineState::Incomplete(vec![')', '}', '>', ']', '}', ')']))
        );
        assert_eq!(states.next(), Some(LineState::Corrupt(']', '}')));
        assert_eq!(
            states.next(),
            Some(LineState::Incomplete(vec![
                '}', '}', '>', '}', '>', ')', ')', ')', ')'
            ]))
        );
        assert_eq!(states.next(), Some(LineState::Corrupt(']', ')')));
        assert_eq!(states.next(), Some(LineState::Corrupt(')', ']')));
        assert_eq!(
            states.next(),
            Some(LineState::Incomplete(vec![
                ']', ']', '}', '}', ']', '}', ']', '}', '>'
            ]))
        );
        assert_eq!(states.next(), Some(LineState::Corrupt('>', ')')));
        assert_eq!(states.next(), Some(LineState::Corrupt(']', '>')));
        assert_eq!(
            states.next(),
            Some(LineState::Incomplete(vec![']', ')', '}', '>']))
        );
    }
    #[test]
    fn test_solve1() {
        assert_eq!(solve1(EXAMPLE_DATA), 26397);
    }
    #[test]
    fn test_solve2() {
        assert_eq!(solve2(EXAMPLE_DATA), 288957);
    }
}
