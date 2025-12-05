advent_of_code::solution!(3);

fn max_battery(bank: &str) -> (usize, u32) {
    let mut max = 0;
    let mut max_index = 0;
    for (i, battery) in bank.chars().filter_map(|c| c.to_digit(10)).enumerate() {
        if battery > max {
            max = battery;
            max_index = i;
            if max == 9 {
                break;
            }
        }
    };
    (max_index, max)
}

fn add_joltages(batteries: &[u32]) -> u64 {
    let mut sum = 0;
    for battery in batteries {
        sum = (sum * 10) + *battery as u64;
    }
    sum
}

fn max_joltage_ext(bank: &str, n_batteries: usize) -> Vec<u32> {
    // println!("searching {bank} for {n_batteries} more...");
    let mut max_batteries = Vec::new();
    let mut index = 0;
    while max_batteries.len() < n_batteries {
        let (new_index, battery) = max_battery(&bank[index..]);
        index = index + new_index + 1;
        let bank_remaining = bank.len() - index;
        let batteries_remaining = n_batteries - (max_batteries.len() + 1);
        if bank_remaining < batteries_remaining {
            let sub_bank = &bank[..index - 1];
            let rest = &bank[(index - 1)..];
            let mut to_push: Vec<u32> = rest.chars().filter_map(|c| c.to_digit(10)).collect();
            let more = n_batteries - to_push.len();
            // println!("Ran out, pushing {rest}, looking for {more} more");
            max_batteries = max_joltage_ext(sub_bank, more);
            max_batteries.extend_from_slice(&mut to_push);
        } else {
            // println!("pushing {battery}, looking for {batteries_remaining} more");
            max_batteries.push(battery);
        }
    }
    max_batteries
}

pub fn part_one(input: &str) -> Option<u64> {
    let banks = input.lines();
    let mut total_joltage = 0;
    for bank in banks {
        let max_batteries = max_joltage_ext(bank, 2);
        let max_joltage = add_joltages(&max_batteries);
        total_joltage += max_joltage;
    }
    Some(total_joltage)
}

pub fn part_two(input: &str) -> Option<u64> {
    let banks = input.lines();
    let mut total_joltage = 0;
    for bank in banks {
        let max_batteries = max_joltage_ext(bank, 12);
        let max_joltage = add_joltages(&max_batteries);
        total_joltage += max_joltage;
        // println!("got {max_joltage} max joltage on bank {bank}");
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
    fn test_part_two_individual() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let mut lines = input.lines();
        assert_eq!(part_two(lines.next().unwrap()), Some(987654321111));
        assert_eq!(part_two(lines.next().unwrap()), Some(811111111119));
        let n = lines.next().unwrap();
        assert_eq!(part_two(n), Some(434234234278));
        assert_eq!(part_two(lines.next().unwrap()), Some(888911112111));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
