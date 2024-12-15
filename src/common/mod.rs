use std::fs;

pub fn import(name: &str) -> Vec<String> {
    fs::read_to_string(name)
        .unwrap()
        .split_terminator('\n')
        .map(|s| s.to_string())
        .collect()
}

#[derive(Clone, PartialEq)]
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

pub enum Rotation {
    CW,
    CCW,
}

impl Direction {
    pub fn r#rotate(&self, rotation: &Rotation) -> Direction {
        match self {
            Direction::North => match rotation {
                Rotation::CW => Direction::NorthEast,
                Rotation::CCW => Direction::NorthWest,
            },
            Direction::NorthEast => match rotation {
                Rotation::CW => Direction::East,
                Rotation::CCW => Direction::North,
            },

            Direction::East => match rotation {
                Rotation::CW => Direction::SouthEast,
                Rotation::CCW => Direction::NorthEast,
            },
            Direction::SouthEast => match rotation {
                Rotation::CW => Direction::South,
                Rotation::CCW => Direction::East,
            },

            Direction::South => match rotation {
                Rotation::CW => Direction::SouthWest,
                Rotation::CCW => Direction::SouthEast,
            },
            Direction::SouthWest => match rotation {
                Rotation::CW => Direction::West,
                Rotation::CCW => Direction::South,
            },
            Direction::West => match rotation {
                Rotation::CW => Direction::NorthWest,
                Rotation::CCW => Direction::SouthWest,
            },
            Direction::NorthWest => match rotation {
                Rotation::CW => Direction::North,
                Rotation::CCW => Direction::West,
            },
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    // pub fn manhattan(&self, rhs: &Self) -> i64 {
    //     (rhs.x - self.x).abs() + (rhs.y - self.y).abs()
    // }

    pub fn offset(&self, o: &(i64, i64)) -> Point {
        Point {
            x: self.x + o.0,
            y: self.y + o.1,
        }
    }

    pub fn r#move(self, dir: &Direction) -> Point {
        match dir {
            Direction::North => self.offset(&(0, -1)),
            Direction::NorthEast => self.offset(&(1, -1)),
            Direction::East => self.offset(&(1, 0)),
            Direction::SouthEast => self.offset(&(1, 1)),
            Direction::South => self.offset(&(0, 1)),
            Direction::SouthWest => self.offset(&(-1, 1)),
            Direction::West => self.offset(&(-1, 0)),
            Direction::NorthWest => self.offset(&(-1, -1)),
        }
    }
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Neg for Point {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Point {
            x: -self.x,
            y: -self.y,
        }
    }
}
