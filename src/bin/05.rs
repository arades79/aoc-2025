use std::ops::RangeInclusive;

use itertools::Itertools;
use rangetools::Rangetools;

advent_of_code::solution!(5);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct MyRange(u64, u64);

impl MyRange {
    fn contains(&self, element: u64) -> bool {
        self.0 <= element && self.1 >= element
    }
    fn overlaps(&self, other: &Self) -> bool {
        self.contains(other.0) || self.contains(other.1)
    }

    fn join(&mut self, other: &Self) {
        if self.overlaps(&other) {
            self.0 = self.0.min(other.0);
            self.1 = self.1.max(other.1);
        }
    }

    fn len(self) -> u64 {
        self.1 - self.0 + 1
    }
}

type FreshRanges = Vec<MyRange>;
type Ingredients = Vec<u64>;

fn parse_db(input: &str) -> Option<(FreshRanges, Ingredients)> {
    let mut fresh = FreshRanges::new();
    let mut ingredients = Ingredients::new();
    for line in input.lines() {
        let line = line.trim();
        if line.contains('-') {
            let (start, end) = line.split_once('-')?;
            fresh.push(MyRange(start.parse().ok()?, end.parse().ok()?));
        } else if let Some(ingredient) = line.parse().ok() {
            ingredients.push(ingredient);
        }
    }
    fresh.sort();
    Some((fresh, ingredients))
}

pub fn part_one(input: &str) -> Option<u64> {
    let (fresh, ingredients) = parse_db(input)?;
    let fresh_ingredient_count = ingredients
        .into_iter()
        .filter(|ingredient| fresh.iter().any(|r| r.contains(*ingredient)))
        .count();
    Some(fresh_ingredient_count as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (fresh, _) = parse_db(input)?;
    let mut merged_ranged = Vec::new();
    'fresh: for range in fresh.iter().sorted() {
        let mut new_range = *range;
        for other in fresh.iter().sorted() {
            new_range.join(other);
        }
        for other in merged_ranged.iter_mut().sorted() {
            if new_range.overlaps(other) {
                other.join(&new_range);
                continue 'fresh;
            }
        }
        merged_ranged.push(new_range);
    }
    // I dn't understand why this needs to happen twice, but if it doesn't happen twice there's one range unmerged
    let mut more_merged_ranged: Vec<MyRange> = Vec::new();
    'fresh: for range in merged_ranged.iter().sorted() {
        let mut new_range = *range;
        for other in merged_ranged.iter().sorted() {
            new_range.join(other);
        }
        for other in more_merged_ranged.iter_mut().sorted() {
            if other.overlaps(&new_range) {
                other.join(&new_range);
                continue 'fresh;
            }
        }
        more_merged_ranged.push(new_range);
    }
    // for range in more_merged_ranged.iter() {
    //     let any_overlaps = more_merged_ranged
    //         .iter()
    //         .any(|r| range != r && range.overlaps(r));
    //     println!("range {range:?} overlaps: {any_overlaps}");
    // }
    let final_count = more_merged_ranged.iter().map(|r| r.len()).sum();
    Some(final_count)
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
        assert_eq!(result, Some(14));
    }
}
