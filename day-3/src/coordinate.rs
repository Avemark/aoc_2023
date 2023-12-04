use std::cmp::Ordering;

#[derive(Debug, Clone, Copy)]
pub struct Coordinate {
    x: usize,
    y: usize,
}

impl PartialEq<Self> for Coordinate {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Coordinate {}

impl PartialOrd<Self> for Coordinate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Coordinate {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.x > other.x {
            return Ordering::Greater;
        }
        if self.x < other.x {
            return Ordering::Less;
        }
        if self.y > other.y {
            return Ordering::Greater;
        }
        if self.y < other.y {
            return Ordering::Less;
        }
        return Ordering::Equal;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort() {
        let top = Coordinate { x: 1, y: 2 };
        let middle = Coordinate { x: 1, y: 3 };
        let bottom = Coordinate { x: 2, y: 1 };

        let mut start = vec![bottom, top, middle];

        start.sort();

        assert_eq!(start, vec![top, middle, bottom])
    }
}
