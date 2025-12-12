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

fn parse_piece(input: &mut &str) -> winnow::Result<Piece> {
    preceded(
        opt((digit1, ':', newline)),
        repeat(
            PIECE_SIZE,
            terminated(
                repeat(PIECE_SIZE, one_of(['.', '#']).map(|c| c == '#'))
                    .map(|v: Vec<bool>| [v[0], v[1], v[3]]),
                newline,
            ),
        )
        .map(|v: Vec<_>| [v[0], v[1], v[3]]),
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
}

fn parse_region(input: &mut &str) -> winnow::Result<Region> {
    (
        terminated(dec_uint, 'x'),
        terminated(dec_uint, ':'),
        repeat(PIECES, preceded(' ', dec_uint::<_, usize, _>))
            .map(|v: Vec<usize>| [v[0], v[1], v[2], v[3], v[4], v[5]]),
    )
        .map(|(length, width, required)| Region::new(length, width, required))
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
    None
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
