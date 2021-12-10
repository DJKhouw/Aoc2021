use lazy_static::lazy_static;
use regex::Regex;
use std::fs;

#[derive(Debug, PartialEq, Eq)]
pub struct BingoField {
    val: usize,
    state: bool,
}
impl BingoField {
    pub fn new(val: usize) -> Self {
        Self { val, state: false }
    }
    pub fn mark(&mut self) {
        self.state = true;
    }
}
pub struct BingoBoard {
    rows: Vec<Vec<BingoField>>,
}
impl BingoBoard {
    pub fn new(rows: Vec<Vec<BingoField>>) -> Self {
        Self { rows }
    }
    pub fn mark(&mut self, val: usize) {
        self.rows.iter_mut().for_each(|row| {
            row.iter_mut().filter(|f| f.val == val).for_each(|field| {
                field.mark();
            });
        });
    }
    pub fn calculate_score(&self, called: usize) -> usize {
        let mut score = 0;
        for row in self.rows.iter() {
            score += row
                .iter()
                .filter(|f| !f.state)
                .fold(0, |acc, field| acc + field.val);
        }
        score * called
    }
}

pub fn parse_row(s: &str) -> Vec<BingoField> {
    lazy_static! {
        static ref RE_BINGODIGIT: Regex = Regex::new(r" +").unwrap();
    }
    RE_BINGODIGIT
        .split(s.trim())
        .map(|n| n.parse::<usize>().unwrap())
        .map(BingoField::new)
        .collect()
}

pub fn parse_puzzle_data(input: &str) -> (Vec<usize>, Vec<BingoBoard>) {
    let mut lines = input.lines();
    let drawn_numbers = lines
        .next()
        .expect("couldn't read first line")
        .split(',')
        .map(|c| c.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let mut raw_boards = lines.filter(|l| !l.trim().is_empty()).peekable();
    let mut boards = Vec::new();
    while raw_boards.peek().is_some() {
        let mut rows = Vec::with_capacity(5);
        for _ in 0..5 {
            if let Some(s) = raw_boards.next() {
                rows.push(parse_row(s));
            } else {
                panic!("Unable to finish board.")
            }
        }
        boards.push(BingoBoard::new(rows));
    }
    (drawn_numbers, boards)
}
fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let (drawn_numbers, boards) = parse_puzzle_data(&input);
}
#[cfg(test)]
mod tests {
    use super::*;
    static EXAMPLE_DATA: &str =
        "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    #[test]
    fn test_parse_puzzle_data() {
        let (drawn_numbers, boards) = parse_puzzle_data(EXAMPLE_DATA);
        assert_eq!(
            drawn_numbers,
            vec![
                7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8,
                19, 3, 26, 1
            ]
        );
        assert_eq!(boards.len(), 3);
        assert_eq!(
            boards[1].rows[0],
            vec![
                BingoField::new(3),
                BingoField::new(15),
                BingoField::new(0),
                BingoField::new(2),
                BingoField::new(22)
            ]
        );
    }
    #[test]
    fn test_parse_row() {
        let row = "10 25  36 740 1";
        assert_eq!(
            parse_row(row),
            vec![
                BingoField::new(10),
                BingoField::new(25),
                BingoField::new(36),
                BingoField::new(740),
                BingoField::new(1)
            ]
        );
    }
    #[test]
    fn test_parse_row2() {
        let row = " 8  2 23  4 24";
        assert_eq!(
            parse_row(row),
            vec![
                BingoField::new(8),
                BingoField::new(2),
                BingoField::new(23),
                BingoField::new(4),
                BingoField::new(24)
            ]
        );
    }
}
