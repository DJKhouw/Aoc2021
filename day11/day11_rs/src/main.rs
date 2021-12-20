use std::fs;

#[derive(Debug, PartialEq, Eq)]
pub enum PositionKind {
    UpperLeft,
    UpperRight,
    LowerLeft,
    LowerRight,
    TopRow,
    BottomRow,
    LeftEdge,
    RightEdge,
    Middle,
}
fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let part1 = solve1(&input, 100);
    println!("Part 1: {}", &part1);
    let part2 = solve2(&input);
    println!("Part 2: {}", &part2);
}

pub fn solve1(input: &str, steps: u32) -> u32 {
    let mut map = get_map(input);
    let mut total = 0;
    for _ in 0..steps {
        total += step(&mut map);
    }
    total
}
pub fn solve2(input: &str) -> u32 {
    let mut map = get_map(input);
    let mut steps = 0;
    loop {
        steps += 1;
        step(&mut map);
        if map.iter().flatten().all(|n| *n == 0) {
            break;
        }
    }
    steps
}
pub fn get_map(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c as i32 - 0x30).collect::<Vec<i32>>())
        .collect::<Vec<_>>()
}
pub fn step(map: &mut [Vec<i32>]) -> u32 {
    let mut flashes = 0;
    let max_row_idx = map.len() - 1;
    let max_column_idx = map[0].len() - 1;
    // increase all energy levels by 1
    for row in 0..=max_row_idx {
        for column in 0..=max_column_idx {
            map[row][column] += 1;
        }
    }

    while map.as_ref().iter().flatten().any(|&n| n > 9) {
        for row in 0..=max_row_idx {
            for column in 0..=max_column_idx {
                if map[row][column] > 9 {
                    flashes += 1;
                    flash(map, (row, column));
                }
            }
        }
    }
    for n in map.iter_mut().flatten().filter(|n| **n == -1) {
        *n = 0;
    }
    flashes
}
pub fn print_map(map: &[Vec<i32>]) {
    for row in 0..map.len() {
        for column in 0..map[0].len() {
            print!("{}", &map[row][column]);
        }
        println!();
    }
}
pub fn flash(map: &mut [Vec<i32>], (row, column): (usize, usize)) {
    let max_row_idx = map.len() - 1;
    let max_column_idx = map[0].len() - 1;
    map[row][column] = -1;
    let kind = get_position_kind((row, column), max_row_idx, max_column_idx);
    // println!("Flashing at {:?},{},{}", &kind, &row, &column);
    match kind {
        PositionKind::UpperLeft => {
            if map[row][column + 1] != -1 {
                map[row][column + 1] += 1;
            }
            if map[row + 1][column] != -1 {
                map[row + 1][column] += 1;
            }
            if map[row + 1][column + 1] != -1 {
                map[row + 1][column + 1] += 1;
            }
        }
        PositionKind::UpperRight => {
            if map[row][column - 1] != -1 {
                map[row][column - 1] += 1;
            }
            if map[row + 1][column] != -1 {
                map[row + 1][column] += 1;
            }
            if map[row + 1][column - 1] != -1 {
                map[row + 1][column - 1] += 1;
            }
        }
        PositionKind::LowerLeft => {
            if map[row][column + 1] != -1 {
                map[row][column + 1] += 1;
            }
            if map[row - 1][column] != -1 {
                map[row - 1][column] += 1;
            }
            if map[row - 1][column + 1] != -1 {
                map[row - 1][column + 1] += 1;
            }
        }
        PositionKind::LowerRight => {
            if map[row][column - 1] != -1 {
                map[row][column - 1] += 1;
            }
            if map[row - 1][column] != -1 {
                map[row - 1][column] += 1;
            }
            if map[row - 1][column - 1] != -1 {
                map[row - 1][column - 1] += 1;
            }
        }
        PositionKind::TopRow => {
            if map[row][column - 1] != -1 {
                map[row][column - 1] += 1;
            }
            if map[row][column + 1] != -1 {
                map[row][column + 1] += 1;
            }
            if map[row + 1][column - 1] != -1 {
                map[row + 1][column - 1] += 1;
            }
            if map[row + 1][column] != -1 {
                map[row + 1][column] += 1;
            }
            if map[row + 1][column + 1] != -1 {
                map[row + 1][column + 1] += 1;
            }
        }
        PositionKind::BottomRow => {
            if map[row][column - 1] != -1 {
                map[row][column - 1] += 1;
            }
            if map[row][column + 1] != -1 {
                map[row][column + 1] += 1;
            }
            if map[row - 1][column - 1] != -1 {
                map[row - 1][column - 1] += 1;
            }
            if map[row - 1][column] != -1 {
                map[row - 1][column] += 1;
            }
            if map[row - 1][column + 1] != -1 {
                map[row - 1][column + 1] += 1;
            }
        }
        PositionKind::LeftEdge => {
            if map[row - 1][column] != -1 {
                map[row - 1][column] += 1;
            }
            if map[row - 1][column + 1] != -1 {
                map[row - 1][column + 1] += 1;
            }
            if map[row][column + 1] != -1 {
                map[row][column + 1] += 1;
            }
            if map[row + 1][column] != -1 {
                map[row + 1][column] += 1;
            }
            if map[row + 1][column + 1] != -1 {
                map[row + 1][column + 1] += 1;
            }
        }
        PositionKind::RightEdge => {
            if map[row - 1][column] != -1 {
                map[row - 1][column] += 1;
            }
            if map[row - 1][column - 1] != -1 {
                map[row - 1][column - 1] += 1;
            }
            if map[row][column - 1] != -1 {
                map[row][column - 1] += 1;
            }
            if map[row + 1][column] != -1 {
                map[row + 1][column] += 1;
            }
            if map[row + 1][column - 1] != -1 {
                map[row + 1][column - 1] += 1;
            }
        }
        PositionKind::Middle => {
            if map[row - 1][column - 1] != -1 {
                map[row - 1][column - 1] += 1;
            }
            if map[row - 1][column] != -1 {
                map[row - 1][column] += 1;
            }
            if map[row - 1][column + 1] != -1 {
                map[row - 1][column + 1] += 1;
            }
            if map[row][column - 1] != -1 {
                map[row][column - 1] += 1;
            }
            if map[row][column + 1] != -1 {
                map[row][column + 1] += 1;
            }
            if map[row + 1][column - 1] != -1 {
                map[row + 1][column - 1] += 1;
            }
            if map[row + 1][column] != -1 {
                map[row + 1][column] += 1;
            }
            if map[row + 1][column + 1] != -1 {
                map[row + 1][column + 1] += 1;
            }
        }
    }
}

pub fn get_position_kind(
    (row, column): (usize, usize),
    max_row_idx: usize,
    max_column_idx: usize,
) -> PositionKind {
    match (row, column) {
        (0, 0) => PositionKind::UpperLeft,
        (0, a) if a == max_column_idx => PositionKind::UpperRight,
        (a, 0) if a == max_row_idx => PositionKind::LowerLeft,
        (a, b) if a == max_row_idx && b == max_column_idx => PositionKind::LowerRight,
        (0, _) => PositionKind::TopRow,
        (a, _) if a == max_row_idx => PositionKind::BottomRow,
        (_, 0) => PositionKind::LeftEdge,
        (_, a) if a == max_column_idx => PositionKind::RightEdge,
        _ => PositionKind::Middle,
    }
}
#[cfg(test)]
mod tests {

    use super::*;

    static EXAMPLE_DATA: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    #[test]
    fn test_get_map() {
        let map = get_map(EXAMPLE_DATA);
        assert_eq!(map.len(), 10);
        assert_eq!(map[0].len(), 10);
        let mut iter = map.into_iter();
        assert_eq!(iter.next(), Some(vec![5, 4, 8, 3, 1, 4, 3, 2, 2, 3]));
        assert_eq!(iter.next(), Some(vec![2, 7, 4, 5, 8, 5, 4, 7, 1, 1]));
        assert_eq!(iter.next(), Some(vec![5, 2, 6, 4, 5, 5, 6, 1, 7, 3]));
        assert_eq!(iter.next(), Some(vec![6, 1, 4, 1, 3, 3, 6, 1, 4, 6]));
        assert_eq!(iter.next(), Some(vec![6, 3, 5, 7, 3, 8, 5, 4, 7, 8]));
        assert_eq!(iter.next(), Some(vec![4, 1, 6, 7, 5, 2, 4, 6, 4, 5]));
        assert_eq!(iter.next(), Some(vec![2, 1, 7, 6, 8, 4, 1, 7, 2, 1]));
        assert_eq!(iter.next(), Some(vec![6, 8, 8, 2, 8, 8, 1, 1, 3, 4]));
        assert_eq!(iter.next(), Some(vec![4, 8, 4, 6, 8, 4, 8, 5, 5, 4]));
        assert_eq!(iter.next(), Some(vec![5, 2, 8, 3, 7, 5, 1, 5, 2, 6]));
    }
    #[test]
    fn test_get_position_kind() {
        assert_eq!(get_position_kind((0, 0), 9, 9), PositionKind::UpperLeft);
        assert_eq!(get_position_kind((0, 9), 9, 9), PositionKind::UpperRight);
        assert_eq!(get_position_kind((9, 0), 9, 9), PositionKind::LowerLeft);
        assert_eq!(get_position_kind((9, 9), 9, 9), PositionKind::LowerRight);
        assert_eq!(get_position_kind((0, 1), 9, 9), PositionKind::TopRow);
        assert_eq!(get_position_kind((9, 1), 9, 9), PositionKind::BottomRow);
        assert_eq!(get_position_kind((1, 0), 9, 9), PositionKind::LeftEdge);
        assert_eq!(get_position_kind((1, 9), 9, 9), PositionKind::RightEdge);
        assert_eq!(get_position_kind((1, 1), 9, 9), PositionKind::Middle);
    }
    #[test]
    fn test_flash() {
        let mut map = get_map(EXAMPLE_DATA);
        flash(&mut map, (0, 0));
        assert_eq!(map[0][0], -1);
        assert_eq!(map[1][0], 3);
        assert_eq!(map[0][1], 5);
    }
    #[test]
    fn test_step() {
        let mut map: Vec<Vec<i32>> = get_map(EXAMPLE_DATA);
        let step1 = step(&mut map);
        assert_eq!(step1, 0);
        let mut iter = map.clone().into_iter();
        assert_eq!(iter.next(), Some(vec![6, 5, 9, 4, 2, 5, 4, 3, 3, 4]));
        assert_eq!(iter.next(), Some(vec![3, 8, 5, 6, 9, 6, 5, 8, 2, 2]));
        assert_eq!(iter.next(), Some(vec![6, 3, 7, 5, 6, 6, 7, 2, 8, 4]));
        assert_eq!(iter.next(), Some(vec![7, 2, 5, 2, 4, 4, 7, 2, 5, 7]));
        assert_eq!(iter.next(), Some(vec![7, 4, 6, 8, 4, 9, 6, 5, 8, 9]));
        assert_eq!(iter.next(), Some(vec![5, 2, 7, 8, 6, 3, 5, 7, 5, 6]));
        assert_eq!(iter.next(), Some(vec![3, 2, 8, 7, 9, 5, 2, 8, 3, 2]));
        assert_eq!(iter.next(), Some(vec![7, 9, 9, 3, 9, 9, 2, 2, 4, 5]));
        assert_eq!(iter.next(), Some(vec![5, 9, 5, 7, 9, 5, 9, 6, 6, 5]));
        assert_eq!(iter.next(), Some(vec![6, 3, 9, 4, 8, 6, 2, 6, 3, 7]));
    }
    #[test]
    fn test_step2() {
        let input = "11111
19991
19191
19991
11111";
        let mut map = get_map(input);
        let step = step(&mut map);
        assert_eq!(step, 9);
        let mut iter = map.into_iter();
        assert_eq!(iter.next(), Some(vec![3, 4, 5, 4, 3]));
        assert_eq!(iter.next(), Some(vec![4, 0, 0, 0, 4]));
        assert_eq!(iter.next(), Some(vec![5, 0, 0, 0, 5]));
        assert_eq!(iter.next(), Some(vec![4, 0, 0, 0, 4]));
        assert_eq!(iter.next(), Some(vec![3, 4, 5, 4, 3]));
    }
    #[test]
    fn test_solve1() {
        assert_eq!(solve1(EXAMPLE_DATA, 2), 35);
        assert_eq!(solve1(EXAMPLE_DATA, 3), 80);
        assert_eq!(solve1(EXAMPLE_DATA, 4), 96);
        assert_eq!(solve1(EXAMPLE_DATA, 5), 104);
        assert_eq!(solve1(EXAMPLE_DATA, 6), 105);
        assert_eq!(solve1(EXAMPLE_DATA, 7), 112);
        assert_eq!(solve1(EXAMPLE_DATA, 8), 136);

        assert_eq!(solve1(EXAMPLE_DATA, 10), 204);
        assert_eq!(solve1(EXAMPLE_DATA, 100), 1656);
    }
    #[test]
    fn test_solve2() {
        assert_eq!(solve2(EXAMPLE_DATA), 195);
    }
}
