use itertools::Itertools;
use winnow::{
    ascii::{dec_uint, digit1, newline},
    combinator::{opt, preceded, repeat, separated, terminated},
    prelude::*,
    token::one_of,
};

advent_of_code::solution!(12);

const PIECES: usize = 6;
const PIECE_SIZE: usize = 3;

#[derive(Debug, Clone, Copy)]
struct Piece([[bool; PIECE_SIZE]; PIECE_SIZE]);

impl Piece {
    const fn rotate_l(self) -> Self {
        Self([
            [self.0[0][2], self.0[1][2], self.0[2][2]],
            [self.0[0][1], self.0[1][1], self.0[2][1]],
            [self.0[0][0], self.0[1][0], self.0[2][0]],
        ])
    }

    const fn rotate_r(self) -> Self {
        Self([
            [self.0[2][0], self.0[1][0], self.0[0][0]],
            [self.0[2][1], self.0[1][1], self.0[0][1]],
            [self.0[2][2], self.0[1][2], self.0[0][2]],
        ])
    }

    const fn flip_h(mut self) -> Self {
        self.0[0].reverse();
        self.0[1].reverse();
        self.0[2].reverse();
        self
    }

    const fn flip_v(mut self) -> Self {
        self.0.reverse();
        self
    }

    const fn variants(self) -> [Piece; 5] {
        [
            self,
            self.rotate_l(),
            self.rotate_r(),
            self.flip_h(),
            self.flip_v(),
        ]
    }

    const fn size(&self) -> usize {
        self.0[0][0] as usize
            + self.0[0][1] as usize
            + self.0[0][2] as usize
            + self.0[1][0] as usize
            + self.0[1][1] as usize
            + self.0[1][2] as usize
            + self.0[2][0] as usize
            + self.0[2][1] as usize
            + self.0[2][2] as usize
    }
}

fn parse_piece(input: &mut &str) -> winnow::Result<Piece> {
    preceded(
        opt((digit1, ':', newline)),
        repeat(
            PIECE_SIZE,
            terminated(
                repeat(PIECE_SIZE, one_of(['.', '#']).map(|c| c == '#'))
                    .map(|v: Vec<bool>| [v[0], v[1], v[2]]),
                newline,
            ),
        )
        .map(|v: Vec<_>| [v[0], v[1], v[2]]),
    )
    .map(|arr| Piece(arr))
    .parse_next(input)
}

type Pieces = [Piece; PIECES];

struct Region {
    grid: Vec<Vec<bool>>,
    required: [usize; PIECES],
}

impl Region {
    fn new(length: usize, width: usize, required: [usize; PIECES]) -> Self {
        Region {
            grid: vec![vec![false; width]; length],
            required,
        }
    }

    fn can_insert_piece(&self, piece: &Piece, row: usize, col: usize) -> bool {
        if row > (self.grid.len() - PIECE_SIZE) || col > (self.grid[0].len() - PIECE_SIZE) {
            return false;
        }
        for y in 0..PIECE_SIZE {
            for x in 0..PIECE_SIZE {
                if self.grid[y + row][x + col] && piece.0[y][x] {
                    return false;
                }
            }
        }
        true
    }

    fn toggle_piece(&mut self, piece: &Piece, row: usize, col: usize) {
        for y in 0..PIECE_SIZE {
            for x in 0..PIECE_SIZE {
                self.grid[y + row][x + col] ^= piece.0[y][x];
            }
        }
    }

    fn requirements_possible(&self, pieces: &Pieces) -> bool {
        let area_available: usize = self.grid[0].len() * self.grid.len();
        let piece_total: usize = pieces
            .iter()
            .enumerate()
            .map(|(idx, piece)| piece.size() * self.required[idx])
            .sum();
        if piece_total > area_available {
            return false;
        }
        true
    }
}

fn parse_region(input: &mut &str) -> winnow::Result<Region> {
    (
        terminated(dec_uint, 'x'),
        terminated(dec_uint, ':'),
        repeat(PIECES, preceded(' ', dec_uint::<_, usize, _>))
            .map(|v: Vec<usize>| [v[0], v[1], v[2], v[3], v[4], v[5]]),
    )
        .map(|(width, length, required)| Region::new(length, width, required))
        .parse_next(input)
}

fn parse_input(input: &mut &str) -> winnow::Result<(Pieces, Vec<Region>)> {
    (
        separated(PIECES, parse_piece, "\n\n")
            .map(|v: Vec<_>| [v[0], v[1], v[2], v[3], v[4], v[5]]),
        preceded("\n\n", repeat(1.., parse_region)),
    )
        .parse_next(input)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (pieces, regions) = parse_input.parse(input).ok()?;
    let fittable_regions = regions
        .into_iter()
        .filter(|region| region.requirements_possible(&pieces))
        .count();
    Some(fittable_regions as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
