pub mod custom_error;

pub mod part1;
pub mod part2;

#[derive(Debug)]
pub struct Pipe {
    location: Coordinate,
    connections: (Coordinate, Coordinate),
    dist: u32,
    shape: char
}

type Coordinate = (u32, u32);

impl Pipe {

    pub fn next_location_from(&self, location: Coordinate) -> Coordinate {
        if self.connections.0 == location {
            self.connections.1
        } else {
            self.connections.0
        }
    }

    pub fn try_walk(&self, from: Coordinate) -> Option<Coordinate> {
        if self.connections.0 == from {
            Some(self.connections.1)
        } else if self.connections.1 == from {
            Some(self.connections.0)
        } else {
            None
        }
    }

    pub fn is_connected(&self, location: Coordinate) -> bool {
        location == self.connections.0 || location == self.connections.1
    }

    pub fn new(location: Coordinate, connections: (Coordinate,Coordinate), shape: char) -> Self {
        Self {
            location,
            connections,
            dist: 0,
            shape
        }
    }

    pub fn parse(input: char, location: Coordinate) -> Option<Self> {
        match parse_shape(input, location) {
            Some(connections) => { Some(Self::new(location, connections, input)) }
            None => None
        }
    }


}

fn parse_shape(input: char, reference: Coordinate) -> Option<(Coordinate,Coordinate)> {
    match input {
        '|' => {
            if reference.0 == 0 { return None }
            Some(((reference.0 - 1, reference.1), (reference.0 + 1, reference.1)))
        }
        'L' => {
            if reference.0 == 0 { return None }
            Some(((reference.0 - 1, reference.1), (reference.0, reference.1 + 1)))
        }
        'J' => {
            if reference.0 == 0 || reference.1 == 0 { return None }
            Some(((reference.0 - 1, reference.1), (reference.0, reference.1 - 1)))
        }
        '-' => {
            if reference.1 == 0 { return None }
            Some(((reference.0, reference.1 - 1), (reference.0, reference.1 + 1)))
        }
        '7' => {
            if reference.1 == 0 { return None }
            Some(((reference.0, reference.1 - 1), (reference.0 + 1, reference.1)))
        }
        'F' => { Some(((reference.0, reference.1 + 1), (reference.0 + 1, reference.1))) }
        rest => { panic!("I don't know what to do with a '{}'", rest)}
    }
}

// fn less_one(n: u32) -> u32 {
//     n.checked_sub(1).unwrap_or(n)
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_shape() {
        assert_eq!(((5,5),(6,6)), parse_shape('L', (6,5)))
    }
    #[test]
    fn test_next_location() {
        let pipe = Pipe { location: (5,6), connections: ((6,6),(5,5)), dist: 0, shape: '7'};
        assert_eq!((5,5), pipe.next_location_from((6,6)));
        assert_eq!((6,6), pipe.next_location_from((5,5)));
    }
}