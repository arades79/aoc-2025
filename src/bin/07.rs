advent_of_code::solution!(7);

use pathfinding::prelude::*;

const DEBUG: bool = false;

type Pos = (usize, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Nothing,
    Beam,
    Reflector,
}

impl From<u8> for Tile {
    fn from(value: u8) -> Self {
        match value {
            b'.' => Tile::Nothing,
            b'S' | b'|' => Tile::Beam,
            b'^' => Tile::Reflector,
            _ => panic!(),
        }
    }
}

impl Tile {
    fn to_char(&self) -> char {
        match self {
            Tile::Nothing => '.',
            Tile::Beam => '|',
            Tile::Reflector => '^',
        }
    }
}

struct Manifold(Vec<Vec<Tile>>);

impl Manifold {
    fn from_str(input: &str) -> Manifold {
        Manifold(
            input
                .lines()
                .map(|line| line.bytes().map(Tile::from).collect())
                .collect(),
        )
    }

    fn get(&self, row: usize, col: usize) -> Tile {
        self.0[row][col]
    }

    fn set(&mut self, row: usize, col: usize, tile: Tile) {
        self.0[row][col] = tile;
    }

    fn row_len(&self) -> usize {
        self.0.len()
    }

    fn col_len(&self) -> usize {
        self.0[0].len()
    }

    fn update_row(&mut self, row: usize) -> u64 {
        let mut reflections = 0;
        for i in 0..self.col_len() {
            let above = self.get(row - 1, i);
            let here = self.get(row, i);
            match (above, here) {
                (Tile::Nothing, _) | (Tile::Reflector, _) | (Tile::Beam, Tile::Beam) => continue,
                (Tile::Beam, Tile::Nothing) => self.set(row, i, Tile::Beam),
                (Tile::Beam, Tile::Reflector) => {
                    if DEBUG {println!("Beam hit reflector at {row}, {i}")};
                    reflections += 1;
                    self.set(row, i - 1, Tile::Beam);
                    self.set(row, i + 1, Tile::Beam)
                }
            }
        }
        if DEBUG {self.print()};
        reflections
    }

    fn get_start(&self) -> Option<Pos> {
        for (i, tile) in self.0[0].iter().enumerate() {
            if *tile == Tile::Beam {
                return Some((0, i));
            }
        }
        None
    }

    fn beam_successors(&self, (row, col): Pos) -> Vec<Pos> {
        let below = if row < self.row_len() {
            self.get(row + 1, col)
        } else {
            return vec![];
        };
        match below {
            Tile::Nothing | Tile::Beam => vec![(row + 1, col)],
            Tile::Reflector => vec![(row + 1, col - 1), (row + 1, col + 1)],
        }
    }

    fn print(&self) {
        for row in 0..self.row_len() {
            for col in 0..self.col_len() {
                print!("{}", self.get(row, col).to_char())
            }
            println!("");
        }
        println!("");
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut manifold = Manifold::from_str(input);
    let mut reflections = 0;
    for row in 1..manifold.row_len() {
        reflections += manifold.update_row(row);
    }
    Some(reflections)
}

pub fn part_two(input: &str) -> Option<u64> {
    let manifold = Manifold::from_str(input);
    let start = manifold.get_start()?;
    let realities = count_paths(
        start,
        |pos| manifold.beam_successors(*pos),
        |&(row, _)| row == manifold.row_len() - 1,
    );
    Some(realities as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
