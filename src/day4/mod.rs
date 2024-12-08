use std::collections::HashMap;

use crate::common::*;

pub fn part1() {
    let map = parse_map(&import("/workspaces/Advent-2024/src/day4/input.txt"));

    println!("Day 4 part 1 : {}", count_xmas_lines(&map));
}

pub fn part2() {
    let map = parse_map(&import("/workspaces/Advent-2024/src/day4/input.txt"));

    println!("Day 4 part 2 : {}", count_x_mas_lines(&map));
}

fn parse_map(input: &Vec<String>) -> HashMap<Point, char> {
    input
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, c)| {
                (
                    Point {
                        x: x as i64,
                        y: y as i64,
                    },
                    c,
                )
            })
        })
        .collect()
}

fn count_xmas_lines(points: &HashMap<Point, char>) -> usize {
    points
        .keys()
        .map(|location| {
            [
                xmas_line(points, location, &[(0, 0), (1, 0), (2, 0), (3, 0)]), // EAST
                xmas_line(points, location, &[(0, 0), (1, 1), (2, 2), (3, 3)]), // SOUTH-EAST
                xmas_line(points, location, &[(0, 0), (0, 1), (0, 2), (0, 3)]), // SOUTH
                xmas_line(points, location, &[(0, 0), (-1, 1), (-2, 2), (-3, 3)]), // SOUTH-WEST
                xmas_line(points, location, &[(0, 0), (-1, 0), (-2, 0), (-3, 0)]), // WEST
                xmas_line(points, location, &[(0, 0), (-1, -1), (-2, -2), (-3, -3)]), // NORTH-WEST
                xmas_line(points, location, &[(0, 0), (0, -1), (0, -2), (0, -3)]), // NORTH
                xmas_line(points, location, &[(0, 0), (1, -1), (2, -2), (3, -3)]), // NORTH-EAST
            ]
            .iter()
            .filter(|&s| s == "XMAS")
            .count()
        })
        .sum()
}

fn xmas_line(points: &HashMap<Point, char>, location: &Point, offsets: &[(i64, i64)]) -> String {
    offsets
        .iter()
        .filter_map(|o| {
            if let Some(p) = points.get(&location.offset(o)) {
                Some(p)
            } else {
                None
            }
        })
        .collect()
}

fn count_x_mas_lines(points: &HashMap<Point, char>) -> usize {
    points
        .keys()
        .filter_map(|location| {
            points
                .get(&location)
                .and_then(|c| if *c == 'A' { Some(location) } else { None })
        })
        .map(|location| {
            [
                xmas_line(points, location, &[(-1, -1), (0, 0), (1, 1)]),
                xmas_line(points, location, &[(-1, 1), (0, 0), (1, -1)]),
            ]
        })
        .filter(|lines| {
            (lines[0] == "MAS" || lines[0] == "SAM") && (lines[1] == "MAS" || lines[1] == "SAM")
        })
        .count()
}

#[cfg(test)]
mod tests {
    use crate::day4::{count_xmas_lines, import, parse_map};

    use super::count_x_mas_lines;

    #[test]
    fn part1() {
        let input = vec![
            "MMMSXXMASM",
            "MSAMXMSMSA",
            "AMXSXMAAMM",
            "MSAMASMSMX",
            "XMASAMXAMM",
            "XXAMMXXAMA",
            "SMSMSASXSS",
            "SAXAMASAAA",
            "MAMMMXMMMM",
            "MXMXAXMASX",
        ];

        let map = parse_map(&input.iter().map(|f| f.to_string()).collect());

        assert_eq!(map.len(), 100);

        let n = count_xmas_lines(&map);

        assert_eq!(n, 18);
    }

    #[test]
    fn part2() {
        let input = vec![
            "MMMSXXMASM",
            "MSAMXMSMSA",
            "AMXSXMAAMM",
            "MSAMASMSMX",
            "XMASAMXAMM",
            "XXAMMXXAMA",
            "SMSMSASXSS",
            "SAXAMASAAA",
            "MAMMMXMMMM",
            "MXMXAXMASX",
        ];

        let map = parse_map(&input.iter().map(|f| f.to_string()).collect());
        let n = count_x_mas_lines(&map);
        assert_eq!(n, 9);
    }
}
