use std::hash::Hash;

use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Eq, Hash, PartialEq, Debug, Deserialize, Serialize)]
pub struct Coordinate {
    x: i32,
    y: i32,
}

impl From<i32> for Coordinate {
    fn from(value: i32) -> Self {
        Self { x: value, y: value }
    }
}

impl From<(i32, i32)> for Coordinate {
    fn from((x, y): (i32, i32)) -> Self {
        Self { x, y }
    }
}

impl Coordinate {
    pub fn distance(&self, other: &Coordinate) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }
}

#[cfg(test)]
mod tests {
    mod splat {
        use crate::map::cells::Coordinate;

        #[test]
        fn it_creates_coordinate_with_same_values() {
            let coordinate = Coordinate::from(2);
            assert_eq!(coordinate.x, 2);
            assert_eq!(coordinate.y, 2);
        }
    }

    mod new {
        use crate::map::cells::Coordinate;

        #[test]
        fn it_creates_coordinate_properly() {
            let coordinate = Coordinate::from((2, 4));
            assert_eq!(coordinate.x, 2);
            assert_eq!(coordinate.y, 4)
        }
    }
}
