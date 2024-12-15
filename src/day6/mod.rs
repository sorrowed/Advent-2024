use crate::common::{import, Direction, Point, Rotation};
use core::panic;
use std::collections::HashMap;

pub fn part1() {
    let mut map = Map::from(&import("/workspaces/Advent-2024/src/day6/input.txt"));
    while !map.r#move() {
    }
    //map.print();

    println!("Day 6 part 1 : {}", map.visited().len());
}

pub fn part2() {
    println!("Day 6 part 2 :");
}

#[derive(Clone, PartialEq)]
enum Status {
    Normal,
    Obstruction,
    Visited { direction: Direction },
}

impl From<char> for Status {
    fn from(value: char) -> Self {
        match value {
            '^' => Status::Visited {
                direction: Direction::North,
            },
            '>' => Status::Visited {
                direction: Direction::East,
            },
            'v' => Status::Visited {
                direction: Direction::South,
            },
            '<' => Status::Visited {
                direction: Direction::West,
            },
            '.' => Status::Normal,
            '#' => Status::Obstruction,
            _ => panic!(),
        }
    }
}

impl From<Status> for char {
    fn from(value: Status) -> Self {
        match value {
            Status::Visited {
                direction: Direction::North,
            } => '^',
            Status::Visited {
                direction: Direction::East,
            } => '>',
            Status::Visited {
                direction: Direction::South,
            } => 'v',
            Status::Visited {
                direction: Direction::West,
            } => '<',
            Status::Normal => '.',
            Status::Obstruction => '#',
            _ => panic!(),
        }
    }
}

struct Map {
    locations: HashMap<Point, Status>,
    current: (Point, Direction),
}

impl Map {
    fn x_range(&self) -> std::ops::Range<i64> {
        0..self
            .locations
            .keys()
            .max_by_key(|k| k.x)
            .map(|k| k.x + 1)
            .unwrap()
    }

    fn y_range(&self) -> std::ops::Range<i64> {
        0..self
            .locations
            .keys()
            .max_by_key(|k| k.y)
            .map(|k| k.y + 1)
            .unwrap()
    }

    fn print(&self) {
        for y in self.y_range() {
            for x in self.x_range() {
                if let Some(status) = self.locations.get(&Point { x, y }) {
                    print!("{}", char::from(status.clone()));
                } else {
                    panic!()
                }
            }
            println!();
        }
    }

    fn next_position(&self) -> Option<(&Point, &Status)> {
        self.locations
            .get_key_value(&self.current.0.clone().r#move(&self.current.1))
    }

    fn r#move(&mut self) -> bool {
        if let Some((p, s)) = self.next_position() {
            match s {
                Status::Obstruction => {
                    self.current.1 = self.current.1.rotate(&Rotation::CW).rotate(&Rotation::CW);
                }
                _ => {
                    self.current.0 = p.clone();
                }
            }

            if let Some(s) = self.locations.get_mut(&self.current.0) {
                *s = Status::Visited {
                    direction: self.current.1.clone(),
                }
            }

            false
        } else {
            true
        }
    }

    fn visited(&self) -> Vec<Point> {
        self.locations
            .iter()
            .filter_map(|(p, s)| match s {
                Status::Visited { .. } => Some(p.clone()),
                _ => None,
            })
            .collect()
    }
}

impl From<&Vec<String>> for Map {
    fn from(value: &Vec<String>) -> Self {
        let locations = value
            .iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().map(move |(x, c)| {
                    (
                        Point {
                            x: x as i64,
                            y: y as i64,
                        },
                        Status::from(c),
                    )
                })
            })
            .collect::<Vec<_>>();

        locations
            .iter()
            .find(|(_, s)| {
                s == &Status::Visited {
                    direction: Direction::North,
                }
            })
            .map(|f| Map {
                current: (f.0.clone(), Direction::North),
                locations: locations.clone().into_iter().collect(),
            })
            .expect("")
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        common::Point,
        day6::{Map, Status},
    };

    #[test]
    fn part1() {
        let input = vec![
            "....#.....".to_string(),
            ".........#".to_string(),
            "..........".to_string(),
            "..#.......".to_string(),
            ".......#..".to_string(),
            "..........".to_string(),
            ".#..^.....".to_string(),
            "........#.".to_string(),
            "#.........".to_string(),
            "......#...".to_string(),
        ];

        let mut map = Map::from(&input);

        map.print();
        println!();

        assert_eq!(map.current.0, Point { x: 4, y: 6 });

        while !map.r#move() {
            map.print();
            println!();
        }

        assert_eq!(map.current.0, Point { x: 7, y: 9 });

        assert_eq!(map.visited().len(), 41);
    }

    #[test]
    fn part2() {
        assert!(false)
    }
}
