use std::collections::BTreeSet;

use itertools::Itertools;

advent_of_code::solution!(8);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
struct Point(i64, i64, i64);

impl Point {
    const fn new(x: i64, y: i64, z: i64) -> Self {
        Self(x, y, z)
    }

    fn try_from_iter(mut iter: impl Iterator<Item = i64>) -> Option<Self> {
        Some(Self::new(iter.next()?, iter.next()?, iter.next()?))
    }

    const fn sub(self, rhs: Self) -> Self {
        Self::new(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }

    const fn square(self) -> Self {
        Self::new(self.0.pow(2), self.1.pow(2), self.2.pow(2))
    }

    const fn sum(self) -> i64 {
        self.0 + self.1 + self.2
    }

    fn distance(self, other: Self) -> f64 {
        let sum = self.sub(other).square().sum() as f64;
        sum.sqrt()
    }
}

fn parse_box_positions(input: &str) -> Vec<Point> {
    input
        .lines()
        .filter_map(|line| {
            Point::try_from_iter(
                line.trim()
                    .split(',')
                    .filter_map(|num| num.parse::<i64>().ok()),
            )
        })
        .collect()
}

type Circuit = BTreeSet<Point>;

fn merge_circuits(circuits: &mut [Circuit]) -> Vec<Circuit> {
    let mut merged = BTreeSet::new();
    circuits.sort_by(|c1, c2| c2.len().cmp(&c1.len()));
    let circuits = &*circuits;
    for circuit in circuits {
        let mut new_circuit = circuit.clone();
        for other in circuits {
            if circuit.iter().any(|p| other.contains(p)) {
                new_circuit.append(&mut other.clone());
            }
        }
        merged.insert(new_circuit);
    }
    merged.into_iter().collect()
}

fn connect_boxes(boxes: &[Point], max_connections: usize) -> Vec<Circuit> {
    let mut circuits: Vec<BTreeSet<Point>> = Vec::new();
    for (_idx, (b1, b2)) in boxes
        .iter()
        .copied()
        .tuple_combinations()
        .sorted_by(|p1: &(Point, Point), p2: &(Point, Point)| {
            let d1: f64 = p1.0.distance(p1.1);
            let d2: f64 = p2.0.distance(p2.1);
            d1.total_cmp(&d2)
        })
        .take(max_connections)
        .enumerate()
    {
        if let Some(existing) = circuits
            .iter_mut()
            .filter(|circuit| circuit.contains(&b1) || circuit.contains(&b2))
            .next()
        {
            // println!("Connection {idx}: adding {b1:?} and {b2:?} to existing circuit {existing:?}");
            existing.insert(b1);
            existing.insert(b2);
        } else {
            let new_circuit = BTreeSet::from([b1, b2]);
            // println!("Connection {idx}: adding new circuit {new_circuit:?}");
            circuits.push(new_circuit);
        }
    }
    for _ in 0..circuits.len() {
        circuits = merge_circuits(&mut circuits);
    }
    circuits
}

fn connect_boxes_2(boxes: &[Point]) -> (Point, Point) {
    let mut circuits: Vec<BTreeSet<Point>> = boxes.iter().map(|p| BTreeSet::from([*p])).collect();
    let mut last_connection = Default::default();
    for (_idx, (b1, b2)) in boxes
        .iter()
        .copied()
        .tuple_combinations()
        .sorted_by(|p1: &(Point, Point), p2: &(Point, Point)| {
            let d1: f64 = p1.0.distance(p1.1);
            let d2: f64 = p2.0.distance(p2.1);
            d1.total_cmp(&d2)
        })
        .enumerate()
    {
        last_connection = (b1, b2);
        if let Some(existing) = circuits
            .iter_mut()
            .filter(|circuit| circuit.contains(&b1) || circuit.contains(&b2))
            .next()
        {
            // println!("Connection {idx}: adding {b1:?} and {b2:?} to existing circuit {existing:?}");
            existing.insert(b1);
            existing.insert(b2);
        } else {
            let new_circuit = BTreeSet::from([b1, b2]);
            // println!("Connection {idx}: adding new circuit {new_circuit:?}");
            circuits.push(new_circuit);
        }
        // for _ in 0..circuits.len() {
            circuits = merge_circuits(&mut circuits);
        // }
        let merged_len = circuits.len();
        // println!(
        //     "with {_idx} connections {_idx} there are {merged_len} circuits (last connection {last_connection:?})"
        // );
        if merged_len == 1 {
            break;
        }
    }

    last_connection
}

pub fn part_one(input: &str) -> Option<u64> {
    let boxes = parse_box_positions(input);
    let max_connections = if boxes.len() < 100 { 10 } else { 1000 };
    let circuits = connect_boxes(&boxes, max_connections);
    // println!("Final circuits:");
    // for circuit in circuits.iter() {
    //     println!("  {circuit:?}");
    // }
    let silly_number = circuits
        .into_iter()
        .map(|circuit| circuit.len() as u64)
        .sorted()
        .rev()
        .take(3)
        .product();
    Some(silly_number)
}

pub fn part_two(input: &str) -> Option<u64> {
    let boxes = parse_box_positions(input);
    let last_connection = connect_boxes_2(&boxes);
    Some((last_connection.0.0 * last_connection.1.0) as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
