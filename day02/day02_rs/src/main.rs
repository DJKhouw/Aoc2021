use core::panic;
use std::fs;
pub enum Direction {
    Up,
    Down,
    Forward,
}
impl Direction {
    pub fn parse(s: &str) -> Self {
        match s {
            "up" => Direction::Up,
            "down" => Direction::Down,
            "forward" => Direction::Forward,
            _ => panic!("Unknown direction."),
        }
    }
}

pub struct SubCommand {
    dir: Direction,
    am: i32,
}

impl SubCommand {
    pub fn parse(s: &str) -> Self {
        let mut parts = s.split_whitespace();
        let dir = Direction::parse(parts.next().unwrap());
        let am = parts.next().unwrap().parse().unwrap();
        Self { dir, am }
    }
}
pub struct Position {
    x: i32,
    depth: i32,
    aim: i32,
}

impl Position {
    pub fn new(x: i32, depth: i32, aim: i32) -> Self {
        Self { x, depth, aim }
    }
    pub fn default() -> Self {
        Self {
            x: 0,
            depth: 0,
            aim: 0,
        }
    }
    pub fn process(&mut self, c: &SubCommand) {
        match c.dir {
            Direction::Up => self.depth -= c.am,
            Direction::Down => self.depth += c.am,
            Direction::Forward => self.x += c.am,
        }
    }
    pub fn process2(&mut self, c: &SubCommand) {
        match c.dir {
            Direction::Up => self.aim -= c.am,
            Direction::Down => self.aim += c.am,
            Direction::Forward => {
                self.x += c.am;
                self.depth += self.aim * c.am;
            }
        }
    }
}
pub fn get_commands(s: &str) -> Vec<SubCommand> {
    s.lines().map(SubCommand::parse).collect::<Vec<_>>()
}
pub fn solve1(commands: &[SubCommand]) -> i32 {
    let mut pos = Position::new(0, 0, 0);
    for c in commands {
        pos.process(c);
    }
    pos.x * pos.depth
}

pub fn solve2(commands: &[SubCommand]) -> i32 {
    let mut pos = Position::new(0, 0, 0);
    for c in commands {
        pos.process2(c);
    }
    pos.x * pos.depth
}
fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let commands = get_commands(&input);

    let part1 = solve1(&commands);
    println!("Part 1: {}", part1);
    let part2 = solve2(&commands);
    println!("Part 2: {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let input = "forward 5
        down 5
        forward 8
        up 3
        down 8
        forward 2";
        let commands = get_commands(input);
        assert_eq!(solve1(&commands), 150);
    }

    #[test]
    fn test_solve2() {
        let input = "forward 5
        down 5
        forward 8
        up 3
        down 8
        forward 2";
        let commands = get_commands(input);
        assert_eq!(solve2(&commands), 900);
    }
}
