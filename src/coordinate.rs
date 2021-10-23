use std::hash::Hash;

#[derive(Copy, Clone, Eq, Hash, PartialEq, Debug)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

impl Coordinate {
    pub fn new(x: i32, y: i32) -> Self {
        Coordinate { x, y }
    }

    pub fn splat(value: i32) -> Self {
        Coordinate { x: value, y: value }
    }
}

#[cfg(test)]
mod tests {
    mod splat {
        use crate::coordinate::Coordinate;

        #[test]
        fn it_creates_coordinate_with_same_values() {
            let coordinate = Coordinate::splat(2);
            assert_eq!(coordinate.x, 2);
            assert_eq!(coordinate.y, 2);
        }
    }

    mod new {
        use crate::coordinate::Coordinate;

        #[test]
        fn it_creates_coordinate_properly() {
            let coordinate = Coordinate::new(2, 4);
            assert_eq!(coordinate.x, 2);
            assert_eq!(coordinate.y, 4)
        }
    }
}
