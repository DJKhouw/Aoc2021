use std::{collections::VecDeque, fs};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Fish {
    timer: u8,
}

impl Fish {
    pub fn new(timer: u8) -> Self {
        Self { timer }
    }
    pub fn spawn() -> Self {
        Self { timer: 8 }
    }
    pub fn next_day(&mut self) -> Option<Fish> {
        if self.timer == 0 {
            self.timer = 6;
            Some(Self::spawn())
        } else {
            self.timer -= 1;
            None
        }
    }
}
#[derive(Debug)]
pub struct School {
    fish: Vec<Fish>,
}

impl School {
    pub fn new(timers: &[u8]) -> Self {
        let fish = timers.iter().map(|&n| Fish::new(n)).collect();
        Self { fish }
    }
    pub fn add(&mut self, fish: Fish) {
        self.fish.push(fish)
    }
    pub fn default() -> Self {
        Self { fish: Vec::new() }
    }
    pub fn next_day(&mut self) {
        let mut spawned = Vec::new();
        for f in self.fish.iter_mut() {
            if let Some(f) = f.next_day() {
                spawned.push(f);
            }
        }
        self.fish.append(&mut spawned);
    }
    pub fn count(&self) -> usize {
        self.fish.len()
    }
}

fn main() {
    let input = fs::read_to_string("input.txt")
        .unwrap()
        .split(',')
        .map(|n| n.parse::<u8>().unwrap())
        .collect::<Vec<_>>();
    let part1 = solve1(&input, 80);
    println!("Part 1: {}", &part1);
    let part2 = solve2(&input, 256);
    println!("Part 2: {}", &part2);
}
pub fn solve1(input: &[u8], days: u16) -> usize {
    // This is brute force:
    let mut school = School::new(input);
    for _ in 0..days {
        school.next_day();
    }
    school.count()
}

pub fn solve2(input: &[u8], days: u16) -> u128 {
    // count birthdays
    let mut buckets: VecDeque<u128> = [0_u128; 9].into_iter().collect();
    for n in input {
        buckets[*n as usize] += 1;
    }
    for _ in 0..days {
        if let Some(s) = buckets.pop_front() {
            buckets.push_back(s);
            buckets[6] += s;
        } else {
            panic!("empty queue!");
        }
    }
    buckets.iter().sum()
}
#[cfg(test)]
mod tests {
    use super::*;
    static EXAMPLE_DATA: [u8; 5] = [3, 4, 3, 1, 2];
    #[test]
    fn test_fish_next_day() {
        let mut fish = Fish::new(0);
        let mut fish2 = Fish::new(1);
        let spawned = fish.next_day();
        let spawned2 = fish2.next_day();
        assert_eq!(fish.timer, 6);
        assert_eq!(spawned, Some(Fish { timer: 8 }));
        assert_eq!(fish2.timer, 0);
        assert_eq!(spawned2, None);
    }
    #[test]
    fn test_new_school() {
        let school = School::new(&EXAMPLE_DATA);
        let mut control = School::default();
        for f in EXAMPLE_DATA.iter().map(|&n| Fish::new(n)) {
            control.add(f)
        }
        assert_eq!(school.fish, control.fish);
    }
    #[test]
    fn test_school_next_day() {
        let mut school = School::new(&EXAMPLE_DATA);
        let day2 = vec![2, 3, 2, 0, 1];
        assert_eq!(school.count(), 5);
        school.next_day();
        assert_eq!(school.count(), 5);
        let control = School::new(&day2);
        assert_eq!(school.fish, control.fish);
        school.next_day();
        assert_eq!(school.count(), 6);
    }

    #[test]
    fn test_solve1() {
        assert_eq!(solve1(&EXAMPLE_DATA, 18), 26);
        assert_eq!(solve1(&EXAMPLE_DATA, 80), 5934);
    }

    #[test]
    fn test_solve2() {
        assert_eq!(solve2(&EXAMPLE_DATA, 18), 26);
        assert_eq!(solve2(&EXAMPLE_DATA, 80), 5934);
        assert_eq!(solve2(&EXAMPLE_DATA, 256), 26984457539);
    }
}
