use crate::custom_error::AocError;

pub fn process(
    input: &str,
) -> Result<String, AocError> {
    println!("lines: {}", input.len());
    let result = input.split("\n").map(
        |line| {
            if line.len() > 0 {
                complete_series(decode_line(line))
            } else { 0isize }
        }).sum::<isize>();

    return Ok(result.to_string())
}

fn decode_line(line: &str) -> Vec<isize> {
    line
        .split(" ")
        .map( |number| {
            number
                .parse::<isize>()
                .unwrap_or_else(|_| panic!("could not parse {}", number))
        }).collect()
}

fn complete_series(series: Vec<isize>) -> isize {

    let last = series.last().expect("This should really exist").clone();
    let diffs= find_diffs(series);
    if diffs.iter().all(|n| n == &0isize ) {
        // println!("Zeroes found");
        return last
    }
    let underlying_diff = complete_series(diffs);
    // println!("returning {} + {}", last, underlying_diff);
    return last + underlying_diff
}

fn find_diffs(series: Vec<isize>) -> Vec<isize> {
    // println!("looking at series {:?}", series);
    assert!(series.len() > 1 , "Bad series");

    series
        .windows(2)
        .map(|pair| pair.last().expect("pairs are pairs") - pair.first().expect("pairs are pairs") )
        .collect::<Vec<isize>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_diff() {
        assert_eq!(complete_series(decode_line("1 3 6 10 15 21")), 28isize);
        assert_eq!(complete_series(decode_line("10 13 16 21 30 45")), 68isize);
        assert_eq!(complete_series(decode_line("16 22 45 102 210 392 701 1271 2420 4857 10085 21163 44142 90823 184171 369045 733382 1449608 2854917 5606422 10981452")), 68isize);
    }
    #[test]
    fn test_process() -> Result<(), AocError> {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!("114", process(input)?);
        Ok(())
    }
}
