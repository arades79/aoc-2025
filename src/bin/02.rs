use std::{ops::RangeInclusive};

advent_of_code::solution!(2);

fn parse_ranges(input: &str) -> Vec<RangeInclusive<u64>> {
    input
        .trim()
        .split(',')
        .filter_map(|range| {
            let (start, end) = range.split_once('-')?;
            let (start, end) = (start.parse().ok()?, end.parse().ok()?);
            Some(start..=end)
        })
        .collect()
}

fn check_silly(num: u64) -> bool {
    // print!("Checking {num}... ");
    let digits = num.ilog10() + 1;
    if digits % 2 == 1 {
        // println!("Impossible for number with {digits} digits to be invalid");
        return false;
    }
    let half_digits = digits / 2;
    let half_magnitude = 10u64.pow(half_digits);
    let upper_digits = num / half_magnitude;
    let lower_digits = num % half_magnitude;
    let is_silly = upper_digits == lower_digits;
    // println!("is silly: {is_silly}");
    is_silly
}

// fn check_really_silly(num: u64, cache: &mut BTreeMap<u64,bool>) -> bool {
    fn check_really_silly(num: u64) -> bool {
    // if let Some(value) = cache.get(&num) {
    //     return *value
    // }

    let strnum = num.to_string();
    for digits in 1..=(strnum.len() / 2) {
        let mut subnums = strnum.as_bytes().chunks_exact(digits);
        let Some(first_chunk) = subnums.next() else {continue};
        if subnums.remainder().is_empty() && subnums.all(|chunk| chunk == first_chunk) {
            // cache.insert(num, true);
            return true;
        }
    }
    // cache.insert(num, false);
    false
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut invalid_sum = 0;
    let ranges = parse_ranges(input);
    for range in ranges {
        for i in range {
            if check_silly(i) {
                invalid_sum += i;
            }
        }
    }
    Some(invalid_sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut invalid_sum = 0;
    //let mut silly_cache = BTreeMap::new();
    let ranges = parse_ranges(input);
    for range in ranges {
        for i in range {
      //      if check_really_silly(i, &mut silly_cache) {
      if check_really_silly(i) {
                invalid_sum += i;
            }
        }
    }
    Some(invalid_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
