advent_of_code::solution!(3);

use std::{thread::sleep, time::Duration};

use itertools::Itertools;

// fn max_battery(bank: &str) -> (usize, u32) {
//     let mut max = 0;
//     let mut max_index = 0;
//     for (i, battery) in bank.chars().filter_map(|c| c.to_digit(10)).enumerate() {
//         if battery > max {
//             max = battery;
//             max_index = i;
//             if max == 9 {
//                 break;
//             }
//         }
//     };
//     (max_index, max)
// }

// fn add_joltages(batteries: &[u32]) -> u64 {
//     let mut sum = 0;
//     for battery in batteries {
//         sum = (sum * 10) + *battery as u64;
//     }
//     sum
// }

// fn max_joltage_ext(bank: &str, n_batteries: usize) -> Vec<u32> {
//     let mut max_batteries = Vec::new();
//     let mut index = 0;
//     while max_batteries.len() < n_batteries {
//         let (new_index, battery) = max_battery(&bank[index..]);
//         index = index + new_index + 1;
//         let bank_remaining = bank.len() - index;
//         let batteries_remaining = n_batteries - (max_batteries.len() + 1);
//         println!("found max battery {battery} at index {index}, need to find {batteries_remaining} more batteries with {bank_remaining} left to check");
//         if bank_remaining < batteries_remaining {
//             let sub_bank = &bank[..index - 1];
//             println!("recursing with sub bank {sub_bank}");
//             max_batteries = max_joltage_ext(sub_bank, batteries_remaining);
//         }
//         max_batteries.push(battery);
//     }
//     sleep(Duration::from_millis(10));
//     max_batteries
// }

pub fn part_one(input: &str) -> Option<u64> {
    let banks = input.lines();
    let mut total_joltage = 0;
    for bank in banks {
        // let max_batteries = max_joltage_ext(bank, 2);
        // let max_joltage = add_joltages(&max_batteries);
        let max_joltage = bank.chars().filter_map(|c|c.to_digit(10)).combinations(2).map(|batteries| add_joltages(&batteries)).max()?;
        total_joltage += max_joltage;
        println!("got {max_joltage} max joltage on bank {bank}");
    }
    Some(total_joltage)
}

pub fn part_two(input: &str) -> Option<u64> {
    let banks = input.lines();
    let mut total_joltage = 0;
    for bank in banks {
        // let max_batteries = max_joltage_ext(bank, 12);
        // let max_joltage = add_joltages(&max_batteries);
        let max_joltage = bank.chars().filter_map(|c|c.to_digit(10)).combinations(12).map(|batteries| add_joltages(&batteries)).max()?;

        total_joltage += max_joltage;
        println!("got {max_joltage} max joltage on bank {bank}");
    }
    Some(total_joltage)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
