use std::str::FromStr;

use itertools::Itertools;

use crate::common::import;

pub fn part1() {
    let rules = parse_rules(&import("/workspaces/Advent-2024/src/day5/input_rules.txt"));
    let updates = parse_updates(&import(
        "/workspaces/Advent-2024/src/day5/input_updates.txt",
    ));

    let valid_updates = valid_updates(&updates, &rules);

    println!(
        "Day 5 part 1 : {}",
        valid_updates
            .iter()
            .fold(0i64, |acc, e| acc + e.middle_page())
    );
}

pub fn part2() {
    let rules = parse_rules(&import("/workspaces/Advent-2024/src/day5/input_rules.txt"));
    let updates = parse_updates(&import(
        "/workspaces/Advent-2024/src/day5/input_updates.txt",
    ));

    let valid_updates = valid_updates(&updates, &rules);

    let invalid_updates = updates
        .iter()
        .filter(|u| !valid_updates.contains(u))
        .collect::<Vec<_>>();

    let corrected_updates = corrected_updates(&invalid_updates, &rules);

    let sum_of_middle_pages = corrected_updates
        .iter()
        .fold(0i64, |acc, e| acc + e.middle_page());

    println!("Day 5 part 2 : {}", sum_of_middle_pages);
}

#[derive(Debug)]
struct Rule {
    before: i64,
    after: i64,
}

#[derive(Debug, PartialEq)]
struct Update {
    pages: Vec<i64>,
}

impl Update {
    fn middle_page(&self) -> i64 {
        self.pages[self.pages.len() / 2]
    }
}

#[derive(Debug)]
struct OrderParseError;

impl FromStr for Rule {
    type Err = OrderParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fields = s.split_once('|').unwrap();
        Ok(Rule {
            before: fields.0.parse().unwrap(),
            after: fields.1.parse().unwrap(),
        })
    }
}

#[derive(Debug)]
struct UpdateParseError;

impl FromStr for Update {
    type Err = UpdateParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Update {
            pages: s
                .split(',')
                .map(|token| token.parse::<i64>().unwrap())
                .collect::<Vec<_>>(),
        })
    }
}

impl Rule {
    fn applies_to(&self, update: &Update) -> bool {
        update.pages.contains(&self.before) && update.pages.contains(&self.after)
    }

    fn is_valid(&self, update: &Update) -> bool {
        let before_index = update
            .pages
            .iter()
            .position(|page| *page == self.before)
            .unwrap();

        let after_index = update
            .pages
            .iter()
            .position(|page| *page == self.after)
            .unwrap();

        before_index < after_index
    }
}

fn parse_rules(input: &Vec<String>) -> Vec<Rule> {
    input
        .iter()
        .map(|s| s.parse::<Rule>().unwrap())
        .collect::<Vec<_>>()
}

fn parse_updates(input: &Vec<String>) -> Vec<Update> {
    input
        .iter()
        .map(|s| s.parse::<Update>().unwrap())
        .collect::<Vec<_>>()
}

/// Returns if the update is valid with given rules
fn is_valid_update(update: &Update, rules: &Vec<Rule>) -> bool {
    rules
        .iter()
        .filter(|rule| rule.applies_to(&update))
        .all(|rule| rule.is_valid(update))
}

/// For each update in updates:
/// - Find all rules that apply to this update
/// - And then check if all those rules apply.
/// Collect all updates for which these conditions are true
fn valid_updates<'a>(updates: &'a Vec<Update>, rules: &'a Vec<Rule>) -> Vec<&'a Update> {
    updates
        .iter()
        .filter(|update| is_valid_update(&update, &rules))
        .collect::<Vec<_>>()
}

fn corrected_updates(invalid_updates: &Vec<&Update>, rules: &Vec<Rule>) -> Vec<Update> {
    let mut corrected_updates = vec![];

    for update in invalid_updates {
        for corrected in update
            .pages
            .iter()
            .permutations(update.pages.len())
            .unique()
        {
            let corrected_update = Update {
                pages: corrected.iter().map(|&n| *n).collect::<Vec<_>>(),
            };

            if is_valid_update(&corrected_update, &rules) {
                corrected_updates.push(corrected_update);
            }
        }
    }
    corrected_updates
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn part1() {
        let rules_input = vec![
            "47|53", "97|13", "97|61", "97|47", "75|29", "61|13", "75|53", "29|13", "97|29",
            "53|29", "61|53", "97|53", "61|29", "47|13", "75|47", "97|75", "47|61", "75|61",
            "47|29", "75|13", "53|13",
        ];

        let rules = parse_rules(&rules_input.iter().map(|s| s.to_string()).collect());

        assert!(rules.len() == 21);

        let updates_input = vec![
            "75,47,61,53,29",
            "97,61,53,29,13",
            "75,29,13",
            "75,97,47,61,53",
            "61,13,29",
            "97,13,75,29,47",
        ];

        let updates = parse_updates(&updates_input.iter().map(|s| s.to_string()).collect());

        assert!(updates.len() == 6);

        let valid_updates = valid_updates(&updates, &rules);

        assert_eq!(valid_updates.len(), 3);
        assert_eq!(*valid_updates[0], updates[0]);
        assert_eq!(*valid_updates[1], updates[1]);
        assert_eq!(*valid_updates[2], updates[2]);

        assert_eq!(valid_updates[0].middle_page(), 61);
        assert_eq!(valid_updates[1].middle_page(), 53);
        assert_eq!(valid_updates[2].middle_page(), 29);

        let sum_of_middle_pages = valid_updates
            .iter()
            .fold(0i64, |acc, e| acc + e.middle_page());

        assert_eq!(sum_of_middle_pages, 143);
    }

    #[test]
    fn part2() {
        let rules_input = vec![
            "47|53", "97|13", "97|61", "97|47", "75|29", "61|13", "75|53", "29|13", "97|29",
            "53|29", "61|53", "97|53", "61|29", "47|13", "75|47", "97|75", "47|61", "75|61",
            "47|29", "75|13", "53|13",
        ];

        let rules = parse_rules(&rules_input.iter().map(|s| s.to_string()).collect());

        assert!(rules.len() == 21);

        let updates_input = vec![
            "75,47,61,53,29",
            "97,61,53,29,13",
            "75,29,13",
            "75,97,47,61,53",
            "61,13,29",
            "97,13,75,29,47",
        ];

        let updates = parse_updates(&updates_input.iter().map(|s| s.to_string()).collect());

        assert!(updates.len() == 6);

        let valid_updates = valid_updates(&updates, &rules);

        let invalid_updates = updates
            .iter()
            .filter(|u| !valid_updates.contains(u))
            .collect::<Vec<_>>();

        let corrected_updates = corrected_updates(&invalid_updates, &rules);

        assert_eq!(corrected_updates.len(), 3);
        let sum_of_middle_pages = corrected_updates
            .iter()
            .fold(0i64, |acc, e| acc + e.middle_page());

        assert_eq!(sum_of_middle_pages, 123);
    }
}
