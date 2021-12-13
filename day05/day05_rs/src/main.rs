use std::{collections::HashMap, fs, iter::repeat};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    pub fn from_string(s: &str) -> Self {
        let parts: Vec<&str> = s.split(',').collect();
        Self {
            x: parts[0].trim().parse().unwrap(),
            y: parts[1].trim().parse().unwrap(),
        }
    }
}
#[derive(Debug)]
pub struct Line {
    start: Point,
    end: Point,
    points: Vec<Point>,
}

impl Line {
    pub fn new(start: Point, end: Point, points: Vec<Point>) -> Self {
        Self { start, end, points }
    }
    pub fn is_straight(&self) -> bool {
        self.start.x == self.end.x || self.start.y == self.end.y
    }
    pub fn range(a: i32, b: i32) -> Vec<i32> {
        match a.cmp(&b) {
            std::cmp::Ordering::Less => (a..=b).collect(),
            std::cmp::Ordering::Equal => repeat(a).take(1024).collect(),
            std::cmp::Ordering::Greater => (b..=a.abs()).rev().collect(),
        }
    }
    pub fn from_string(s: &str) -> Self {
        let parts = s.split(" -> ").collect::<Vec<_>>();
        let start = Point::from_string(parts[0]);
        let end = Point::from_string(parts[1]);
        let points = Line::range(start.x, end.x)
            .iter()
            .zip(Line::range(start.y, end.y))
            .map(|(&x, y)| Point::new(x, y))
            .collect();

        Self { start, end, points }
    }
}
fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let part1 = solve1(&input);
    println!("Part 1: {}", &part1);
    let part2 = solve2(&input);
    println!("Part 2: {}", &part2);
}
pub fn overlaps(lines: &[Line]) -> usize {
    let mut map = HashMap::with_capacity(lines.len());

    for point in lines.iter().clone().flat_map(|l| l.points.clone()) {
        let counter = map.entry(point).or_insert(0);
        *counter += 1;
    }
    map.iter().filter(|(_, &am)| am >= 2).count()
}

pub fn solve1(input: &str) -> usize {
    let straight_lines = input
        .lines()
        .map(Line::from_string)
        .filter(|l| l.is_straight())
        .collect::<Vec<_>>();
    overlaps(&straight_lines)
}
pub fn solve2(input: &str) -> usize {
    let lines = input.lines().map(Line::from_string).collect::<Vec<_>>();
    overlaps(&lines)
}
#[cfg(test)]
mod tests {
    use super::*;
    static EXAMPLE_DATA: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn test_point_fromstring() {
        let input = "100,500";
        assert_eq!(Point::from_string(input), Point::new(100, 500));
        assert_eq!(Point::from_string("100 , 500"), Point::new(100, 500));
    }
    #[test]
    fn test_solve1() {
        assert_eq!(solve1(EXAMPLE_DATA), 5);
    }

    #[test]
    fn test_solve2() {
        assert_eq!(solve2(EXAMPLE_DATA), 12);
    }
}
