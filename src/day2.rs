use aoc_runner_derive::{aoc, aoc_generator};

struct Report {
    report: Vec<u32>,
}

impl Report {
    fn new(report: Vec<u32>) -> Report {
        Report { report }
    }

    fn is_safe(&self) -> bool {
        let mut is_increasing = None;

        for i in 0..self.report.len() {
            if i > 0 && self.report[i] < self.report[i - 1] {
                if is_increasing == Some(true) {
                    return false;
                } else if is_increasing == None {
                    is_increasing = Some(false);
                }
            } else if i > 0 && self.report[i] > self.report[i - 1] {
                if is_increasing == Some(false) {
                    return false;
                } else if is_increasing == None {
                    is_increasing = Some(true);
                }
            }

            if i > 0 {
                let diff = self.report[i].abs_diff(self.report[i - 1]);
                if diff < 1 || diff > 3 {
                    return false;
                }
            }
        }

        true
    }
}

pub struct Reports {
    reports: Vec<Report>,
}

impl Reports {
    fn new(reports: Vec<Vec<u32>>) -> Reports {
        Reports {
            reports: reports.iter().map(|r| Report::new(r.clone())).collect(),
        }
    }

    fn count_safe(&self) -> u32 {
        let mut count = 0;
        for report in &self.reports {
            if report.is_safe() {
                count += 1;
            }
        }

        count
    }
}

#[aoc_generator(day2)]
pub fn day2_generator(input: &str) -> Reports {
    let reports = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|l| l.parse::<u32>().unwrap())
                .collect()
        })
        .collect();

    Reports::new(reports)
}

#[aoc(day2, part1)]
pub fn part1(input: &Reports) -> u32 {
    input.count_safe()
}
