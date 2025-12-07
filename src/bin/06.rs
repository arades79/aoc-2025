advent_of_code::solution!(6);

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add,
    Multiply,
}

type Operands = [u64; 4];

#[derive(Debug, Clone)]
struct Problem {
    operation: Operation,
    operands: Operands,
}

impl Problem {
    const fn new(operation: Operation, operands: Operands) -> Self {
        Self {
            operation,
            operands,
        }
    }

    fn calculate(self) -> u64 {
        match self.operation {
            Operation::Add => self.operands.into_iter().sum(),
            Operation::Multiply => self.operands.into_iter().product(),
        }
    }
}

fn parse_problems_part_1(input: &str) -> Option<Vec<Problem>> {
    let mut lines = input.lines();
    let operation_line = lines.next_back()?;
    let operations = operation_line
        .split_ascii_whitespace()
        .filter_map(|operation_char| match operation_char {
            "*" => Some(Operation::Multiply),
            "+" => Some(Operation::Add),
            _ => None,
        });

    let operand1: Vec<u64> = lines
        .next()?
        .split_ascii_whitespace()
        .filter_map(|num_str| num_str.parse().ok())
        .collect();
    let operand2: Vec<u64> = lines
        .next()?
        .split_ascii_whitespace()
        .filter_map(|num_str| num_str.parse().ok())
        .collect();
    let operand3: Vec<u64> = lines
        .next()?
        .split_ascii_whitespace()
        .filter_map(|num_str| num_str.parse().ok())
        .collect();
    let operand4: Option<Vec<u64>> = lines.next().map(|line| {
        line.split_ascii_whitespace()
            .filter_map(|num_str| num_str.parse().ok())
            .collect()
    });

    Some(
        operations
            .enumerate()
            .map(|(index, operation)| {
                let op4 = operand4.clone().map(|op| op[index]).unwrap_or(match operation {
                    Operation::Add => 0,
                    Operation::Multiply => 1,
                });
                Problem::new(
                    operation,
                    [
                        operand1[index],
                        operand2[index],
                        operand3[index],
                        op4,
                    ],
                )
            })
            .collect(),
    )
}

pub fn part_one(input: &str) -> Option<u64> {
    parse_problems_part_1(input).map(|problems| {
        problems
            .into_iter()
            .map(|problem| problem.calculate())
            .sum()
    })
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines: Vec<_> = input.lines().collect();
    // dbg!(&lines);
    let rows = lines.len();
    let mut total = 0;
    let mut operands = Vec::new();
    'outer: for i in (0..lines[0].len()).rev() {
        let mut operand: u64 = 0;
        for j in 0..rows {
            match lines[j].as_bytes()[i] {
                n @ b'0'..=b'9' => operand = (operand * 10) + (n - b'0') as u64,
                b'*' => {
                    operands.push(operand);
                    // print!("multiplying {:?} = ", &operands);
                    let problem_result = operands.clone().into_iter().fold(1u64, |acc, num| acc * num as u64);
                    // println!("{problem_result}");
                    total += problem_result;
                    operands.clear();
                    continue 'outer;
                }
                b'+' => {
                    operands.push(operand);
                    // print!("adding {:?} = ", &operands);
                    let problem_result = operands.clone().into_iter().fold(0u64, |acc, num| acc + num as u64);
                    // println!("{problem_result}");
                    total += problem_result;
                    operands.clear();
                    continue 'outer;
                }
                _ => continue,
            };
        }
        if operand > 0 {
            operands.push(operand);
        }
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
