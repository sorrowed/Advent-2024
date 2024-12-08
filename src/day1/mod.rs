use crate::common::import;

fn parse_into_sorted_vectors(pairs: &Vec<String>) -> (Vec<i64>, Vec<i64>) {
    let (mut f, mut s): (Vec<i64>, Vec<i64>) = pairs
        .into_iter()
        .filter_map(|line| {
            let mut parts = line.splitn(2, ' ');
            let first = parts.next()?.trim().parse::<i64>().unwrap();
            let second = parts.next()?.trim().parse::<i64>().unwrap();
            Some((first, second))
        })
        .unzip();

    f.sort();
    s.sort();

    (f, s)
}

fn sum_of_differences(f: &[i64], s: &[i64]) -> i64 {
    f.iter().zip(s.iter()).map(|(f, s)| (f - s).abs()).sum()
}

fn count_score(f: &[i64], s: &[i64]) -> i64 {
    let matches = f
        .iter()
        .map(|n| (n, s.iter().filter(|&&s| s == *n).count()));

    matches.map(|(n, c)| n * (c as i64)).sum()
}

fn parse_input() -> (Vec<i64>, Vec<i64>) {
    parse_into_sorted_vectors(&import("/workspaces/Advent-2024/src/day1/input.txt"))
}

pub fn part1() {
    let s = parse_input();

    println!("Day 1 part 1 : {}", sum_of_differences(&s.0, &s.1));
}

pub fn part2() {
    let s = parse_input();

    println!("Day 1 part 2 : {}", count_score(&s.0, &s.1));
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;
    use std::{fmt::Debug, str::FromStr};

    static LISTS: [&str; 2] = ["3 4 2 1 3 3", "4 3 5 3 9 3"];

    fn parse_and_sort<T>(input: &str) -> Vec<T>
    where
        T: FromStr,
        <T as FromStr>::Err: Debug,
        T: Ord,
    {
        input
            .split_whitespace()
            .map(|s| s.parse::<T>().unwrap())
            .sorted()
            .collect::<Vec<T>>()
    }

    #[test]
    fn part1() {
        let sum = sum_of_differences(&parse_and_sort(LISTS[0]), &parse_and_sort(LISTS[1]));

        assert_eq!(sum, 11);
    }

    #[test]
    fn part2() {
        let score = count_score(&parse_and_sort(LISTS[0]), &parse_and_sort(LISTS[1]));

        assert_eq!(score, 31);
    }
}
