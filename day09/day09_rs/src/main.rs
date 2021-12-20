use std::{
    collections::{BinaryHeap, VecDeque},
    fs,
};
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PositionKind {
    Middle,
    UpperLeft,
    UpperRight,
    LowerLeft,
    LowerRight,
    Top,
    Bottom,
    LeftEdge,
    RightEdge,
}
fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let part1 = solve1(&input);
    println!("Part 1: {}", &part1);
    let part2: u64 = solve2(&input);
    println!("Part 2: {}", &part2);
}

pub fn solve1(input: &str) -> u32 {
    let map = get_ocean_map(input);
    let low_points = get_low_points(&map);
    low_points.iter().fold(0, |acc, n| acc + (*n + 1))
}
pub fn solve2(input: &str) -> u64 {
    let map = get_ocean_map(input);
    let basins = get_largest_basins(&map);
    basins.iter().product()
}

pub fn get_ocean_map(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|s| {
            s.chars()
                .map(|c| {
                    c.to_digit(10)
                        .unwrap_or_else(|| panic!("{} is not a number.", c))
                })
                .collect()
        })
        .collect()
}
pub fn get_low_points(input: &[Vec<u32>]) -> Vec<u32> {
    let max_rows = input.len() - 1;
    let max_columns = input[0].len() - 1;

    let mut kind;
    let mut low_points = Vec::new();

    for (row, values) in input.iter().enumerate() {
        for (column, &v) in values.iter().enumerate() {
            kind = get_position_kind((row, column), max_rows, max_columns);
            if is_low_point(input, (row, column), kind) {
                low_points.push(v)
            }
        }
    }
    low_points
}
pub fn get_position_kind(
    (row, column): (usize, usize),
    max_rows: usize,
    max_columns: usize,
) -> PositionKind {
    match (row, column) {
        (0_usize, 0_usize) => PositionKind::UpperLeft,
        (0, b) if b == max_columns => PositionKind::UpperRight,
        (a, 0) if a == max_rows => PositionKind::LowerLeft,
        (a, b) if a == max_rows && b == max_columns => PositionKind::LowerRight,
        (0, _) => PositionKind::Top,
        (a, _) if a == max_rows => PositionKind::Bottom,
        (_, 0) => PositionKind::LeftEdge,
        (_, a) if a == max_columns => PositionKind::RightEdge,
        _ => PositionKind::Middle,
    }
}

pub fn is_low_point(map: &[Vec<u32>], point: (usize, usize), kind: PositionKind) -> bool {
    let (row, column) = point;
    match kind {
        PositionKind::UpperLeft => {
            let control: u32 = map[row][column];
            control < map[row + 1][column] && control < map[row][column + 1]
        }
        PositionKind::UpperRight => {
            let control: u32 = map[row][column];
            control < map[row + 1][column] && control < map[row][column - 1]
        }
        PositionKind::LowerLeft => {
            let control: u32 = map[row][column];
            control < map[row - 1][column] && control < map[row][column + 1]
        }
        PositionKind::LowerRight => {
            let control: u32 = map[row][column];
            control < map[row - 1][column] && control < map[row][column - 1]
        }
        PositionKind::Middle => {
            let control: u32 = map[row][column];
            control < map[row - 1][column]
                && control < map[row + 1][column]
                && control < map[row][column - 1]
                && control < map[row][column + 1]
        }
        PositionKind::Top => {
            let control: u32 = map[row][column];
            control < map[row + 1][column]
                && control < map[row][column - 1]
                && control < map[row][column + 1]
        }
        PositionKind::Bottom => {
            let control: u32 = map[row][column];
            control < map[row - 1][column]
                && control < map[row][column - 1]
                && control < map[row][column + 1]
        }
        PositionKind::LeftEdge => {
            let control: u32 = map[row][column];
            control < map[row - 1][column]
                && control < map[row + 1][column]
                && control < map[row][column + 1]
        }
        PositionKind::RightEdge => {
            let control: u32 = map[row][column];
            control < map[row - 1][column]
                && control < map[row + 1][column]
                && control < map[row][column - 1]
        }
    }
}
pub fn print_map(map: &[Vec<u32>]) {
    for row in 0..map.len() {
        for column in 0..map[0].len() {
            print!("{}", &map[row][column]);
        }
        println!();
    }
}
pub fn get_largest_basins(map: &[Vec<u32>]) -> Vec<u64> {
    let mut map = map.iter().map(|v| v.to_vec()).collect::<Vec<_>>();
    let mut basin_areas = BinaryHeap::new();
    let mut fill_queue = VecDeque::new();
    let mut area: u64;
    let mut next: (usize, usize);
    let mut kind: PositionKind;
    let max_rows = map.len() - 1;
    let max_columns = map[0].len() - 1;
    for row_idx in 0..=max_rows {
        for column_idx in 0..=max_columns {
            if map[row_idx][column_idx] > 8 {
                continue;
            }
            area = 0;
            fill_queue.push_back((row_idx, column_idx));
            map[row_idx][column_idx] = 9;
            while !fill_queue.is_empty() {
                next = fill_queue.pop_front().unwrap();
                kind = get_position_kind(next, max_rows, max_columns);
                for (i, j) in get_adjacent_areas(next, kind) {
                    if map[i][j] < 9 {
                        fill_queue.push_back((i, j));
                        map[i][j] = 9;
                    }
                }
                area += 1;
            }
            basin_areas.push(area);
        }
    }

    if basin_areas.len() < 3 {
        panic!("less than 3 basins found!");
    } else {
        (0..3).map(|_| basin_areas.pop().unwrap()).collect()
    }
}

pub fn get_adjacent_areas(
    (row, column): (usize, usize),
    kind: PositionKind,
) -> Vec<(usize, usize)> {
    match kind {
        PositionKind::Middle => vec![
            (row + 1, column),
            (row - 1, column),
            (row, column + 1),
            (row, column - 1),
        ],
        PositionKind::UpperLeft => vec![(row + 1, column), (row, column + 1)],
        PositionKind::UpperRight => vec![(row + 1, column), (row, column - 1)],
        PositionKind::LowerLeft => vec![(row - 1, column), (row, column + 1)],
        PositionKind::LowerRight => vec![(row - 1, column), (row, column - 1)],
        PositionKind::Top => vec![(row + 1, column), (row, column + 1), (row, column - 1)],
        PositionKind::Bottom => vec![(row - 1, column), (row, column + 1), (row, column - 1)],
        PositionKind::LeftEdge => vec![(row + 1, column), (row - 1, column), (row, column + 1)],
        PositionKind::RightEdge => vec![(row + 1, column), (row - 1, column), (row, column - 1)],
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    static EXAMPLE_DATA: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";
    #[test]
    fn test_get_ocean_map() {
        let control = vec![
            vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
            vec![3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
            vec![9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
            vec![8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
            vec![9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
        ];
        assert_eq!(get_ocean_map(EXAMPLE_DATA), control);
    }
    #[test]
    fn test_get_position_kind() {
        let map = get_ocean_map(EXAMPLE_DATA);
        let max_rows = map.len() - 1;
        assert_eq!(max_rows, 4);

        let max_columns = map[0].len() - 1;
        assert_eq!(max_columns, 9);
        let ul = get_position_kind((0, 0), max_rows, max_columns);
        let ur = get_position_kind((0, 9), max_rows, max_columns);
        let ll = get_position_kind((max_rows, 0), max_rows, max_columns);
        let lr = get_position_kind((max_rows, max_columns), max_rows, max_columns);
        let top = get_position_kind((0, 1), max_rows, max_columns);
        let bottom = get_position_kind((max_rows, 1), max_rows, max_columns);
        let left = get_position_kind((1, 0), max_rows, max_columns);
        let right = get_position_kind((1, max_columns), max_rows, max_columns);
        let normal = get_position_kind((2, 2), max_rows, max_columns);
        assert_eq!(ul, PositionKind::UpperLeft);
        assert_eq!(ur, PositionKind::UpperRight);
        assert_eq!(ll, PositionKind::LowerLeft);
        assert_eq!(lr, PositionKind::LowerRight);
        assert_eq!(normal, PositionKind::Middle);
        assert_eq!(top, PositionKind::Top);
        assert_eq!(bottom, PositionKind::Bottom);
        assert_eq!(left, PositionKind::LeftEdge);
        assert_eq!(right, PositionKind::RightEdge);
    }
    #[test]
    fn test_is_low_point() {
        let map = get_ocean_map(EXAMPLE_DATA);
        let max_rows = map.len() - 1;
        let max_columns = map[0].len() - 1;
        let kind = get_position_kind((0, 1), max_rows, max_columns);
        assert_eq!(kind, PositionKind::Top);
        assert!(is_low_point(&map, (0, 1), kind));
        assert!(is_low_point(
            &map,
            (2, 2),
            get_position_kind((2, 2), max_rows, max_columns)
        ));
    }
    #[test]
    fn test_solve1() {
        assert_eq!(solve1(EXAMPLE_DATA), 15);
    }

    #[test]
    fn test_get_adjacent_areas() {
        let map = get_ocean_map(EXAMPLE_DATA);
        let max_rows = map.len() - 1;
        let max_columns = map[0].len() - 1;

        for row in 0..=max_rows {
            for column in 0..=max_columns {
                let kind = get_position_kind((row, column), max_rows, max_columns);
                let adjacent = get_adjacent_areas((row, column), kind);
                match kind {
                    PositionKind::Middle => {
                        assert_eq!(adjacent.len(), 4);
                        assert_eq!(adjacent[0], (row + 1, column), "Middle length failure");
                        assert_eq!(adjacent[1], (row - 1, column), "Middle value failure 0");
                        assert_eq!(adjacent[2], (row, column + 1), "Middle value failure 1");
                        assert_eq!(adjacent[3], (row, column - 1), "Middle value failure 2");
                    }
                    PositionKind::UpperLeft => {
                        assert_eq!(adjacent.len(), 2, "UpperLeft length failure");
                        assert_eq!(adjacent[0], (row + 1, column), "UpperLeft value failure 0");
                        assert_eq!(adjacent[1], (row, column + 1), "UpperLeft value failure 1");
                    }
                    PositionKind::UpperRight => {
                        assert_eq!(adjacent.len(), 2, "UpperRight length failure");
                        assert_eq!(adjacent[0], (row + 1, column), "UpperRight value failure 0");
                        assert_eq!(adjacent[1], (row, column - 1), "UpperRight value failure 1");
                    }
                    PositionKind::LowerLeft => {
                        assert_eq!(adjacent.len(), 2, "LowerLeft length failure");
                        assert_eq!(adjacent[0], (row - 1, column), "LowerLeft value failure 0");
                        assert_eq!(adjacent[1], (row, column + 1), "LowerLeft value failure 1");
                    }
                    PositionKind::LowerRight => {
                        assert_eq!(adjacent.len(), 2, "LowerRight length failure");
                        assert_eq!(adjacent[0], (row - 1, column), "LowerRight value failure 0");
                        assert_eq!(adjacent[1], (row, column - 1), "LowerRight value failure 1");
                    }
                    PositionKind::Top => {
                        assert_eq!(adjacent.len(), 3, "Top length failure");
                        assert_eq!(adjacent[0], (row + 1, column), "Top value failure 0");
                        assert_eq!(adjacent[1], (row, column + 1), "Top value failure 1");
                        assert_eq!(adjacent[2], (row, column - 1), "Top value failure 2");
                    }
                    PositionKind::Bottom => {
                        assert_eq!(adjacent.len(), 3, "Bottom length failure");
                        assert_eq!(adjacent[0], (row - 1, column), "Bottom value failure 0");
                        assert_eq!(adjacent[1], (row, column + 1), "Bottom value failure 1");
                        assert_eq!(adjacent[2], (row, column - 1), "Bottom value failure 2");
                    }
                    PositionKind::LeftEdge => {
                        assert_eq!(adjacent.len(), 3, "LeftEdge length failure");
                        assert_eq!(adjacent[0], (row + 1, column), "LeftEdge value failure 0");
                        assert_eq!(adjacent[1], (row - 1, column), "LeftEdge value failure 1");
                        assert_eq!(adjacent[2], (row, column + 1), "LeftEdge value failure 2");
                    }
                    PositionKind::RightEdge => {
                        assert_eq!(adjacent.len(), 3, "RightEdge length failure");
                        assert_eq!(adjacent[0], (row + 1, column), "RightEdge value failure 0");
                        assert_eq!(adjacent[1], (row - 1, column), "RightEdge value failure 1");
                        assert_eq!(adjacent[2], (row, column - 1), "RightEdge value failure 2");
                    }
                }
            }
        }
    }
    #[test]
    fn test_get_basins() {
        let map = get_ocean_map(EXAMPLE_DATA);
        let basins = get_largest_basins(&map);
        assert_eq!(basins.len(), 3);
        assert_eq!(basins[0], 14);
        assert_eq!(basins[1], 9);
        assert_eq!(basins[2], 9);
    }
    #[test]
    fn test_solve2() {
        assert_eq!(solve2(EXAMPLE_DATA), 1134);
    }
}
