use std::collections::{HashMap, VecDeque};
use crate::custom_error::AocError;
use crate::{Coordinate, Pipe};

pub fn process(
    input: &str,
) -> Result<String, AocError> {
    let mut ground: HashMap<Coordinate, Pipe> = HashMap::new();
    let rows = input.split("\n").collect::<Vec<&str>>();
    let mut start: Coordinate = (0, 0);
    for (x , row) in rows.iter().enumerate() {
        for (y,char) in row.chars().enumerate() {
            match char  {
                'S' => { start = (x as u32,y as u32)}
                '.' => {}
                pipe_symbol => {
                    if let Some(pipe) = Pipe::parse(pipe_symbol, (x as u32,y as u32)) {
                        ground.insert((x as u32,y as u32), pipe);
                    }
                }
            }

        }
    }

    let mut todo: VecDeque<Coordinate> = VecDeque::from([]);
    fill_start_positions(&mut todo, &mut ground, start);

    assert!(todo.len() > 0, "Nothing added");

    while let Some(position) = todo.pop_front() {
        let mut current_dist = 0;
        let mut left: Coordinate = (0,0);
        let mut right: Coordinate = (0,0);
        if let Some(pipe) = ground.get(&position) {
            // println!("Checking out {:?}", pipe);
            current_dist = pipe.dist;
            left = pipe.connections.0.clone();
            right = pipe.connections.1.clone();
        }

        if current_dist > 0 {
            if let Some(left_pipe) = ground.get_mut(&left) {
                if left_pipe.is_connected(position) {
                    if left_pipe.dist == 0 {
                        left_pipe.dist = current_dist + 1;
                        todo.push_back(left_pipe.location)
                    }
                } else {
                    // println!("{:?} isn't connected to {:?}", left_pipe, position)
                }
            }
            if let Some(right_pipe) = ground.get_mut(&right) {
                if right_pipe.is_connected(position) {
                    // println!("found connected pipe to the right: {:?}", right_pipe);
                    if right_pipe.dist == 0 {
                        right_pipe.dist = current_dist + 1;
                        todo.push_back(right_pipe.location)
                    }
                } else {
                    // println!("{:?} isn't connected to {:?}", right_pipe, position)
                }
            }
        }
    }

    assert_eq!(0, todo.len());

    let winner = ground
        .values()
        .max_by( |a,b | a.dist.cmp(&b.dist) )
        .expect("there should be pipes");

    println!("a winner was found: {:?}", winner);

    Ok(winner.dist.to_string())
}

fn fill_start_positions(todo: &mut VecDeque<Coordinate>, ground: &mut HashMap<Coordinate, Pipe>, start: Coordinate ) {
    #[cfg(test)]
    println!("Filling positions around {:?}", start);
    push_if_connected(todo, ground, start, (start.0 + 1, start.1));
    push_if_connected(todo, ground, start, (start.0, start.1 + 1));

    if start.0 > 0 {
        push_if_connected(todo, ground, start, (start.0 - 1, start.1));
    }

    if start.1 > 0 {
        push_if_connected(todo, ground, start, (start.0, start.1 - 1));
    }
}

fn push_if_connected(todo: &mut VecDeque<Coordinate>, ground: &mut HashMap<Coordinate, Pipe>,start: Coordinate, location: Coordinate) {
    #[cfg(test)]
    println!("filling location: {:?}", location);
    if let Some(pipe) = ground.get_mut(&location) {
        #[cfg(test)]
        println!("found pipe: {:?}", pipe);
        if pipe.is_connected(start) {
            #[cfg(test)]
            println!("Pipe connected!");
            pipe.dist = 1;
            todo.push_back(pipe.location)
        }
    };
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fill_start_positions() {
        // let full_expectation: VecDeque<Coordinate> = VecDeque::from([(6,5),(5,6),(4,5),(5,4)]);
        let empty_expectation: VecDeque<Coordinate> = VecDeque::from([]);
        let mut todo: VecDeque<Coordinate> = VecDeque::from([]);
        let mut ground: HashMap<Coordinate, Pipe> = HashMap::new();

        fill_start_positions(&mut todo, &mut ground, (5,5));
        assert_eq!(empty_expectation, todo);
    }

    #[test]
    fn test_process() -> Result<(), AocError> {
        let input = include_str!("../example1.txt");
        assert_eq!("4", process(input)?);
        let input = include_str!("../example2.txt");
        assert_eq!("8", process(input)?);
        Ok(())
    }
}
