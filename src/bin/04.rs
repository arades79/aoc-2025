advent_of_code::solution!(4);

use pathfinding::grid::Grid;

fn parse_roll_coordinates(input: &str) -> Vec<(usize, usize)> {
    let mut coords = Vec::with_capacity(input.len());
    for (j, line) in input.lines().enumerate() {
        for (i, char) in line.as_bytes().iter().copied().enumerate() {
            if char == b'@' {
                coords.push((i, j));
            }
        }
    }
    coords
}

pub fn part_one(input: &str) -> Option<u64> {
    let coords = parse_roll_coordinates(input);
    let mut grid = coords.into_iter().collect::<Grid>();
    grid.enable_diagonal_mode();
    // println!("{grid:?}");
    let accessible_rolls = grid
        .iter()
        .filter(|roll| grid.neighbours(*roll).len() < 4)
        .count();
    Some(accessible_rolls as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let coords = parse_roll_coordinates(input);
    let mut grid = coords.into_iter().collect::<Grid>();
    let mut removed_rolls = 0;
    grid.enable_diagonal_mode();
    loop {
        let accessible_rolls: Vec<_> = grid
            .iter()
            .filter(|roll| grid.neighbours(*roll).len() < 4)
            .collect();
        let rolls_to_remove = accessible_rolls.len() as u64;
        if rolls_to_remove == 0 {
            break;
        }
        removed_rolls += rolls_to_remove;
        for roll in accessible_rolls {
            grid.remove_vertex(roll);
        }
    }
    Some(removed_rolls)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
