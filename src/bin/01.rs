advent_of_code::solution!(1);

const SAFE_NUMBERS: isize = 100;
const START_POSITION: isize = 50;

enum Turn {
    Left(isize),
    Right(isize),
}

impl From<Turn> for isize {
    fn from(value: Turn) -> Self {
        match value {
            Turn::Left(distance) => -distance,
            Turn::Right(distance) => distance,
        }
    }
}

fn parse_turns(input: &str) -> Vec<Turn> {
    input
        .lines()
        .filter_map(|line| {
            let (direction_char, number_str) = line.trim().split_at(1);
            let number = number_str.parse().ok()?;
            match direction_char {
                "L" => Some(Turn::Left(number)),
                "R" => Some(Turn::Right(number)),
                _ => None,
            }
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let turns = parse_turns(input);
    let mut position = START_POSITION;
    let mut zero_positions = 0;
    for turn in turns {
        match turn {
            Turn::Left(amount) => position = (position - amount).rem_euclid(SAFE_NUMBERS),
            Turn::Right(amount) => position = (position + amount).rem_euclid(SAFE_NUMBERS),
        }
        zero_positions += (position == 0) as u64;
    }
    Some(zero_positions)
}

pub fn part_two(input: &str) -> Option<u64> {
    let turns = parse_turns(input);
    let mut position = START_POSITION;
    let mut zero_passes = 0;
    for turn in turns {
        let turn: isize = turn.into();
        let direction = if turn < 0 {-1} else {1};
        for _ in 0..turn.abs() {
            position = (position + direction).rem_euclid(SAFE_NUMBERS);
            if position == 0 {
                zero_passes += 1;
            }
        };
    }
    Some(zero_passes as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
