use aoc_runner_derive::{aoc, aoc_generator};

pub struct HistorianLists {
    left: Vec<u32>,
    right: Vec<u32>,
}

impl HistorianLists {
    fn new(left: Vec<u32>, right: Vec<u32>) -> HistorianLists {
        HistorianLists { left, right }
    }

    fn total_distance(&self) -> u32 {
        let mut left_sorted = self.left.clone();
        left_sorted.sort();
        let mut right_sorted = self.right.clone();
        right_sorted.sort();

        let mut total_distance = 0;
        for i in 0..left_sorted.len() {
            total_distance += left_sorted[i].abs_diff(right_sorted[i]);
        }

        total_distance
    }

    fn similarity_score(&self) -> u32 {
        let mut similarity_score = 0;

        for x in &self.left {
            similarity_score += x * (self.right.iter().filter(|&y| x == y).count() as u32);
        }

        similarity_score
    }
}

#[aoc_generator(day1)]
pub fn day1_generator(input: &str) -> HistorianLists {
    let mut v = Vec::<(u32, u32)>::new();
    for line in input.lines() {
        let nums: Vec<u32> = line.split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect();

        v.push((nums[0], nums[1]));
    }
    let (left, right) = v.iter().cloned().unzip();

    HistorianLists::new(left, right)
}

#[aoc(day1, part1)]
pub fn part1(input: &HistorianLists) -> u32 {
    input.total_distance()
}

#[aoc(day1, part2)]
pub fn part2(input: &HistorianLists) -> u32 {
    input.similarity_score()
}