advent_of_code::solution!(10);

use winnow::Result;
use winnow::ascii::{dec_uint, space1};
use winnow::combinator::separated;
use winnow::combinator::{delimited, preceded, repeat};
use winnow::prelude::*;
use winnow::token::one_of;

use pathfinding::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

    const fn toggle(self) -> Self {
        match self {
            Light::Off => Light::On,
            Light::On => Light::Off,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Button(Vec<usize>);

impl Button {
    fn apply_to_lights(&self, lights: &[Light]) -> Vec<Light> {
        let mut new_lights = lights.to_vec();
        for index in self.0.iter().copied() {
            new_lights[index] = lights[index].toggle();
        }
        new_lights
    }

    fn apply_to_jolts(&self, jolts: &[u32]) -> Vec<u32> {
        let mut new_jolts = jolts.to_vec();
        for index in self.0.iter().copied() {
            new_jolts[index] += 1;
        }
        new_jolts
    }

    const fn len(&self) -> usize {
        self.0.len()
    }
}

fn parse_button(input: &mut &str) -> Result<Button> {
    let indecies =
        delimited('(', separated(1.., dec_uint::<_, usize, _>, ','), ')').parse_next(input)?;
    Ok(Button(indecies))
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Machine {
    indicators: Vec<Light>,
    buttons: Vec<Button>,
    joltages: Vec<u32>,
}

impl Machine {
    fn starting_lights(&self) -> Vec<Light> {
        vec![Light::Off; self.indicators.len()]
    }

    fn next_lights(&self, lights: &[Light]) -> Vec<(Vec<Light>, usize)> {
        self.buttons
            .iter()
            .map(|button| (button.apply_to_lights(lights), 1))
            .collect()
    }

    fn lights_good(&self, lights: &[Light]) -> bool {
        self.indicators == lights
    }

    fn starting_jolts(&self) -> Vec<u32> {
        vec![0; self.joltages.len()]
    }

    fn next_jolts(&self, jolts: &[u32]) -> Vec<(Vec<u32>, usize)> {
        self.buttons
            .iter()
            .map(|button| (button.apply_to_jolts(jolts), 1))
            .filter(|(next_jolts, _)| {
                self.joltages
                    .iter()
                    .zip(next_jolts.iter())
                    .all(|(needed, next)| needed >= next)
            })
            .collect()
    }

    fn jolts_good(&self, jolts: &[u32]) -> bool {
        self.joltages == jolts
    }

    fn plausible_cost(&self, jolts: &[u32]) -> usize {
        let remaining_jolts: u32 = self
            .joltages
            .iter()
            .zip(jolts.iter())
            .map(|(needed, current)| needed.checked_sub(*current).unwrap_or(9999999))
            .sum();
        let best_button = self.buttons.iter().map(Button::len).max().unwrap();
        remaining_jolts as usize / best_button
    }
}

fn parse_machine(input: &mut &str) -> Result<Machine> {
    let indicators = delimited(
        '[',
        repeat(
            1..,
            one_of(['.', '#']).map(|c| Light::from_char(c).unwrap()),
        ),
        ']',
    )
    .parse_next(input)?;
    let buttons = preceded(space1, separated(1.., parse_button, space1)).parse_next(input)?;
    let joltages = preceded(
        space1,
        delimited('{', separated(1.., dec_uint::<_, u32, _>, ','), '}'),
    )
    .parse_next(input)?;
    Ok(Machine {
        indicators,
        buttons,
        joltages,
    })
}

pub fn part_one(input: &str) -> Option<u64> {
    let machines: Vec<_> = input
        .lines()
        .filter_map(|line| parse_machine.parse(line).ok())
        .collect();
    let mut total_buttons = 0;
    for machine in machines {
        let (_buttons, presses) = dijkstra(
            &machine.starting_lights(),
            |lights| machine.next_lights(lights),
            |lights| machine.lights_good(lights),
        )?;
        total_buttons += presses as u64;
    }
    Some(total_buttons)
}

pub fn part_two(input: &str) -> Option<u64> {
    let machines: Vec<_> = input
        .lines()
        .filter_map(|line| parse_machine.parse(line).ok())
        .collect();
    let mut total_buttons = 0;
    for machine in machines {
        let (_buttons, presses) = astar(
            &machine.starting_jolts(),
            |jolts| machine.next_jolts(jolts),
            |jolts| machine.plausible_cost(jolts),
            |jolts| machine.jolts_good(jolts),
        )?;
        println!(
            "Got machine to required joltages {:?} with {} presses",
            machine.joltages, presses
        );
        total_buttons += presses as u64;
    }
    Some(total_buttons)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(33));
    }
}
