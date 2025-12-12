use std::collections::{BTreeSet, VecDeque};

use itertools::Itertools;

advent_of_code::solution!(9);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Point(u32, u32);

impl Point {
    const fn area(self, other: Point) -> u64 {
        let l = self.0.abs_diff(other.0) + 1;
        let w = self.1.abs_diff(other.1) + 1;

        let a = l as u64 * w as u64;
        //println!("{self:?} and {other:?} ({l}*{w}) = {a}");
        a
    }
    fn from_str(input: &str) -> Option<Self> {
        // dbg!(input);
        let (x, y) = input.trim().split_once(',')?;
        // dbg!(x, y);
        let x = x.parse().ok()?;
        let y = y.parse().ok()?;
        // dbg!(x,y);
        Some(Self(x, y))
    }

    fn neighbors(&self) -> [Point; 4] {
        [
            Point(self.0 - 1, self.1),
            Point(self.0 + 1, self.1),
            Point(self.0, self.1 - 1),
            Point(self.0, self.1 + 1),
        ]
    }
}

fn parse_tiles(input: &str) -> Vec<Point> {
    input.lines().filter_map(Point::from_str).collect()
}

type Limits = (u32, u32, u32, u32);

fn get_limits(red_tiles: &[Point]) -> Limits {
    red_tiles.iter().fold(
        (999999999, 999999999, 0, 0),
        |(min_x, min_y, max_x, max_y), tile| {
            (
                min_x.min(tile.0),
                min_y.min(tile.1),
                max_x.max(tile.0),
                max_y.max(tile.1),
            )
        },
    )
}

fn green_from_red(red_tiles: &[Point]) -> BTreeSet<Point> {
    let mut green_tiles = BTreeSet::new();
    let mut add_greens = |point1: &Point, point2: &Point| {
        if point1.0 == point2.0 {
            let x = point1.0;
            let min_y = point1.1.min(point2.1);
            let max_y = point1.1.max(point2.1);
            for y in min_y..max_y {
                green_tiles.insert(Point(x, y));
            }
        } else if point1.1 == point2.1 {
            let y = point1.1;
            let min_x = point1.0.min(point2.0);
            let max_x = point1.0.max(point2.0);
            for x in min_x..max_x {
                green_tiles.insert(Point(x, y));
            }
        }
    };
    for reds in red_tiles.windows(2) {
        let [red1, red2] = reds else {
            continue;
        };
        add_greens(red1, red2);
    }
    let (last_red, first_red) = (red_tiles.first().unwrap(), red_tiles.last().unwrap());
    add_greens(last_red, first_red);
    let limits = get_limits(red_tiles);
    let center_x = ((limits.2 - limits.0) / 2) + limits.0;
    let center_x = ((limits.3 - limits.1) / 2) + limits.1;
    let center = Point(center_x, center_y);
    let mut fill_greens: VecDeque<Point> = VecDeque::new();
    fill_greens.push_back(center);
    while let Some(tile) = fill_greens.pop_front() {
        green_tiles.insert(tile);
        for near in tile.neighbors() {
            if !green_tiles.contains(&near) {
                fill_greens.push_back(near);
            }
        }
    }
    green_tiles
}

pub fn part_one(input: &str) -> Option<u64> {
    let red_tiles = parse_tiles(input);
    // dbg!(&red_tiles);
    red_tiles
        .into_iter()
        .tuple_combinations()
        .map(|(p1, p2)| p1.area(p2))
        .max()
}

pub fn part_two(input: &str) -> Option<u64> {
    let red_tiles = parse_tiles(input);
    let green_tiles = green_from_red(&red_tiles);
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
