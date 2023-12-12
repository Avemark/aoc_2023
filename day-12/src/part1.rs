use std::slice::Iter;
use crate::custom_error::AocError;
use crate::SpringStatus;



pub fn process(
    input: &str,
) -> Result<String, AocError> {
    let rows: Vec<(Vec<usize>, Vec<SpringStatus>)> = input
        .split("\n")
        .map( |row| {
            let mut parts = row.split(" ");

            let statuses = parts
                .next()
                .expect("what is this row?")
                .chars()
                .map( |character| {
                    match character {
                        '#' => SpringStatus::Broken,
                        '.' => SpringStatus::Whole,
                        '?' => SpringStatus::Unknown,
                        _ => panic!("unknown character")
                    }
                })
                .collect::<Vec<SpringStatus>>();

            let groups = parts
                .next()
                .expect("No groups?")
                .split(",")
                .map( |n| {
                    str::parse::<usize>(n).expect("not numbers?")
                })
                .collect::<Vec<usize>>();

            (groups, statuses)
        }).collect();

    Ok(rows.iter().map(|row| combinations_in_row(row.0.iter(), row.1.iter()) ).sum::<usize>().to_string())
}

#[cfg(test)]
fn combinations_in_row(mut groups: Iter<usize>, mut statuses: Iter<SpringStatus>) -> usize {
    if groups.len() == 0 {
        return 1
    }
    if statuses.len() < groups.clone().sum::<usize>() + groups.len() -1 {
        return 0
    }
    if let Some(status) = statuses.next() {
        match status {
            SpringStatus::Whole => { combinations_in_row(groups, statuses) }
            SpringStatus::Unknown => {
                // println!("Checking out iterators: {:?}, {:?}", groups, statuses);
                let other_full_combinations = combinations_in_row(groups.clone(), statuses.clone());
                if let Some(group_length) = groups.next() {
                    if statuses.clone().take(group_length - 1).all( |status| status != &SpringStatus::Whole ) {
                        for _ in 0..(u32::try_from(group_length.clone()).expect("Too big numbers involved")) { statuses.next(); }
                        // println!("Statuses {:?} did have room for {} broken springs", statuses, group_length);
                        // println!("passing on {:?} and {:?}, adding {} other combos", groups, statuses, other_full_combinations);
                        other_full_combinations +  combinations_in_row(groups, statuses)
                    } else {
                        other_full_combinations
                    }
                } else { 1 }
            }
            SpringStatus::Broken => {
                if let Some(group_length) = groups.next() {
                    let mut group= statuses.clone().take(group_length - 1);
                    if group.len() == group_length - 1 && group.all( |status| status != &SpringStatus::Whole ) {
                        println!("found {} possible broken springs in Broken + {:?}", group_length, statuses);
                        if statuses.len() == 0 { return if any_broken_left(statuses.clone()) { 0 } else { 1 } }
                            for _ in 1..(u32::try_from(group_length.clone()).expect("Too big numbers involved")) { statuses.next(); }
                    } else {
                        0
                    }
                } else { 0 }
            }
        }
    } else { 0 }
}
// TODO: use these functions instead, it's getting messy
fn fits_n_broken(n: u32, mut statuses: Iter<SpringStatus>) -> bool {
    for _ in 1..n {
        if let Some(status) = statuses.next() {
            if status == &SpringStatus::Whole { return false }
        } else { return false }
    }
    statuses.next().is_some_and(|status| status == &SpringStatus::Whole )
}

fn next_is_whole(mut statuses: Iter<SpringStatus>) -> bool {
    statuses.len() > 0 && statuses.next().is_some_and( |status| status == &SpringStatus::Whole)
}
fn any_broken_left(mut statuses: Iter<SpringStatus>) -> bool {
    !statuses.all(|status| status != &SpringStatus::Broken)
}

mod tests {
    use crate::SpringStatus::{Broken, Unknown, Whole};
    use super::*;

    #[test]
    fn test_combinations_in_row() {
        //assert_eq!(1usize, combinations_in_row(vec![1,1,3].iter(), vec![Unknown, Unknown, Unknown, Whole, Broken, Broken, Broken].iter()));
        //assert_eq!(4usize, combinations_in_row(vec![1,1,3].iter(), vec![Whole, Unknown, Unknown, Whole, Unknown, Unknown, Whole, Unknown, Broken, Broken].iter()));
        assert_eq!(10usize, combinations_in_row(vec![3,2,1].iter(), vec![Unknown, Broken, Broken, Broken, Unknown, Unknown, Unknown, Unknown, Unknown, Unknown, Unknown, Unknown].iter()));
    }

    #[test]
    fn test_process() -> Result<(), AocError> {
        todo!("haven't built test yet");
        let input = include_str!("../example.txt");
        assert_eq!("", process(input)?);
        Ok(())
    }
}
