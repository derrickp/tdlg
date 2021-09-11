use std::hash::Hash;

#[derive(Copy, Clone, Eq, Hash, PartialEq, Debug)]
pub struct Coordinate<T: Copy + std::ops::Add<Output = T> + Eq + Hash> {
    pub x: T,
    pub y: T,
}

impl<T: Copy + std::ops::Add<Output = T> + Eq + Hash> Coordinate<T> {
    pub fn new(x: T, y: T) -> Self {
        return Coordinate { x, y };
    }

    pub fn splat(value: T) -> Self {
        return Coordinate { x: value, y: value };
    }
}

#[cfg(test)]
mod tests {
    mod splat {
        use crate::coordinate::Coordinate;

        #[test]
        fn it_creates_coordinate_with_same_values() {
            let coordinate = Coordinate::<i32>::splat(2);
            assert_eq!(coordinate.x, 2);
            assert_eq!(coordinate.y, 2);
        }
    }

    mod new {
        use crate::coordinate::Coordinate;

        #[test]
        fn it_creates_coordinate_properly() {
            let coordinate = Coordinate::<i32>::new(2, 4);
            assert_eq!(coordinate.x, 2);
            assert_eq!(coordinate.y, 4)
        }
    }
}
