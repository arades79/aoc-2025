advent_of_code::solution!(10);

use winnow::Result;
use winnow::ascii::{dec_uint, space1};
use winnow::combinator::delimited;
use winnow::combinator::repeat;
use winnow::combinator::separated;
use winnow::prelude::*;

enum Light {
    Off,
    On,
}

impl Light {
    const fn from_char(c: char) -> Option<Self> {
        match c {
            '#' => Some(Light::On),
            '.' => Some(Light::Off),
            _ => None,
        }
    }
}

struct Button(Vec<usize>);

fn parse_button(input: &mut &str) -> Result<Button> {
    let indecies =
        delimited('(', separated(1.., dec_uint::<_, usize, _>, ','), ')').parse_next(input)?;
    Ok(Button(indecies))
}

struct Machine {
    indicators: Vec<Light>,
    buttons: Vec<Button>,
    joltages: Vec<u32>,
}

fn parse_machine(input: &mut &str) -> Result<Machine> {
    let indicators = delimited('[', repeat(1.., ['.', '#']), ']').parse_next(input)?;
    let buttons = separated(1.., parse_button, space1).parse_next(input)?;
    let joltages =
        delimited('{', separated(1.., dec_uint::<_, u32, _>, ','), '}').parse_next(input)?;
    Ok(Machine {
        indicators,
        buttons,
        joltages,
    })
}

pub fn part_one(input: &str) -> Option<u64> {
    None
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
