use std::fs;

pub fn import(name: &str) -> Vec<String> {
    fs::read_to_string(name)
        .unwrap()
        .split_terminator('\n')
        .map(|s| s.to_string())
        .collect()
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
