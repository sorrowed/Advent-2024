use std::str::FromStr;

use crate::common::import;

#[derive(Debug)]
struct Report {
    levels: Vec<i64>,
}
impl Report {
    fn new(l: Vec<i64>) -> Self {
        Self { levels: l }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseReportError;

impl FromStr for Report {
    type Err = ParseReportError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Report {
            levels: s
                .split_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect(),
        })
    }
}

trait Safe {
    fn is_safe(&self) -> bool;
    fn is_safe_when_dampened(&self) -> bool;
}

impl Safe for Report {
    fn is_safe(&self) -> bool {
        let adjacent: bool = self.levels.windows(2).all(|window| {
            let distance = (window[1] - window[0]).abs();
            (1..=3).contains(&distance)
        });

        let increasing: bool = self.levels.windows(2).all(|window| window[1] > window[0]);

        let decreasing: bool = self.levels.windows(2).all(|window| window[1] < window[0]);

        adjacent && (increasing ^ decreasing)
    }

    fn is_safe_when_dampened(&self) -> bool {
        self.is_safe() || {
            (0..self.levels.len())
                .map(|i| [&self.levels[..i], &self.levels[i + 1..]].concat())
                .map(Report::new)
                .any(|r| r.is_safe())
        }
    }
}

pub fn part1() {
    let safe_reports = import("/workspaces/Advent-2024/src/day2/input.txt")
        .iter()
        .map(|line| Report::from_str(line).unwrap())
        .filter(|r| r.is_safe())
        .count();

    println!("Day 2 part 1 : {}", safe_reports);
}

pub fn part2() {
    let safe_reports = import("/workspaces/Advent-2024/src/day2/input.txt")
        .iter()
        .map(|line| Report::from_str(line).unwrap())
        .filter(|r| r.is_safe_when_dampened())
        .count();

    println!("Day 2 part 2 : {}", safe_reports);
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::day2::Safe;

    use super::Report;

    const INPUT: [&str; 6] = [
        "7 6 4 2 1",
        "1 2 7 8 9",
        "9 7 6 2 1",
        "1 3 2 4 5",
        "8 6 4 4 1",
        "1 3 6 7 9",
    ];

    fn parse_levels(input: &[&str]) -> Vec<Report> {
        input
            .iter()
            .map(|&s| Report::from_str(s).unwrap())
            .collect()
    }

    #[test]
    fn part1() {
        let safe = parse_levels(&INPUT).iter().filter(|&r| r.is_safe()).count();
        assert_eq!(safe, 2)
    }

    #[test]
    fn part2() {
        let safe = parse_levels(&INPUT)
            .iter()
            .filter(|&r| r.is_safe_when_dampened())
            .count();
        assert_eq!(safe, 4)
    }
}
