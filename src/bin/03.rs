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

fn max_joltage(bank: &str) -> u64 {
    let second_max;
    let (first_max_index, mut first_max) = max_battery(bank);
    let (prior_bank, post_bank) = (&bank[0..first_max_index],&bank[first_max_index+1..bank.len()]);
    if post_bank.is_empty() {
        (_, second_max) = (first_max_index, first_max);
        (_,first_max) = max_battery(prior_bank);
    } else {
        (_, second_max) = max_battery(post_bank);
    }
    add_joltages(&[first_max,second_max])
}

fn add_joltages(batteries: &[u32]) -> u64 {
    let mut sum = 0;
    for battery in batteries {
        sum = (sum * 10) + *battery as u64;
    }
    sum
}

fn max_joltage_ext(bank: &str, n_batteries: usize) -> Vec<u32> {
    let mut max_batteries = Vec::new();
    let mut index = 0;
    while max_batteries.len() < n_batteries {
        let (new_index, battery) = max_battery(&bank[index..]);
        if (bank.len() - index) < n_batteries {
            max_batteries = max_joltage_ext(&bank[..index], bank.len() - index);
        }
        max_batteries.push(battery);
        index = new_index;
    }

    max_batteries
}

pub fn part_one(input: &str) -> Option<u64> {
    let banks = input.lines();
    let mut total_joltage = 0;
    for bank in banks {
        total_joltage += max_joltage(bank);
    }
    Some(total_joltage)
}

pub fn part_two(input: &str) -> Option<u64> {
    let banks = input.lines();
    let mut total_joltage = 0;
    for bank in banks {
        let max_batteries = max_joltage_ext(bank, 12);
        total_joltage += add_joltages(&max_batteries);
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
