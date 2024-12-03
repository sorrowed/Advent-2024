use std::fs;

use regex::Regex;

struct Mul(i64, i64);

fn parse_muls(input: &str) -> Vec<Mul> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    re.captures_iter(input)
        .map(|f| {
            Mul(
                f.get(1).unwrap().as_str().parse::<i64>().unwrap(),
                f.get(2).unwrap().as_str().parse::<i64>().unwrap(),
            )
        })
        .collect()
}

fn parse_muls_repaired(input: &str) -> Vec<Mul> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|don't|do").unwrap();

    let mut skip = false;

    re.captures_iter(input)
        .filter_map(|captures| {
            let m = captures.get(0).unwrap().as_str();

            let mut result = None;

            if m == "do" {
                skip = false;
            } else if m == "don't" {
                skip = true;
            } else if !skip {
                result = Some(Mul(
                    captures.get(1).unwrap().as_str().parse::<i64>().unwrap(),
                    captures.get(2).unwrap().as_str().parse::<i64>().unwrap(),
                ))
            }

            result
        })
        .collect()
}

pub fn part1() {
    let input = fs::read_to_string("/workspaces/Advent-2024/src/day3/input.txt").unwrap();

    let sum = parse_muls(&input)
        .iter()
        .fold(0i64, |acc, e| acc + e.0 * e.1);

    println!("Day 3 part 1 : {}", sum);
}

pub fn part2() {
    let input = fs::read_to_string("/workspaces/Advent-2024/src/day3/input.txt").unwrap();

    let sum = parse_muls_repaired(&input)
        .iter()
        .fold(0i64, |acc, e| acc + e.0 * e.1);

    println!("Day 3 part 2 : {}", sum);
}

#[cfg(test)]
mod tests {
    use super::{parse_muls, parse_muls_repaired};

    #[test]
    fn part1() {
        let input: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

        let muls = parse_muls(input);

        assert_eq!(muls.len(), 4);

        let sum = muls.iter().fold(0i64, |acc, e| acc + e.0 * e.1);

        assert_eq!(sum, 161);
    }

    #[test]
    fn part2() {
        let input: &str =
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

        let muls = parse_muls_repaired(input);

        let sum = muls.iter().fold(0i64, |acc, e| acc + e.0 * e.1);

        assert_eq!(sum, 48);
    }
}
