use std::{collections::{HashMap, HashSet}, sync::LazyLock};

use pathfinding::prelude::*;

advent_of_code::solution!(11);

type Graph = HashMap<String, HashSet<String>>;

fn parse_graph(input: &str) -> Graph {
    let mut graph = Graph::new();
    for line in input.lines() {
        let (key, values) = line.split_once(':').unwrap();
        graph.insert(
            key.trim().to_owned(),
            values
                .split_ascii_whitespace()
                .map(ToOwned::to_owned)
                .collect(),
        );
    }
    graph
}

pub fn part_one(input: &str) -> Option<u64> {
    let graph = parse_graph(input);
    let thing = count_paths(
        &"you".to_string(),
        |n| graph.get(*n).unwrap(),
        |v| *v == "out",
    );
    Some(thing as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    static EMPTY_SET: LazyLock<HashSet<String>> = std::sync::LazyLock::new(|| HashSet::new());
    let graph = parse_graph(input);
    let succ = |n: &&String| graph.get(*n).unwrap_or(&EMPTY_SET);
    let done = |done: &'static str| move |v: &&String| *v == done;

    let svr = "svr".to_string();
    let svr_dac_paths = count_paths(&svr, succ, done("dac"));
    let svr_fft_paths = count_paths(&svr, succ, done("fft"));
    let svr_out_paths = count_paths(&svr, succ, done("out"));

    let dac = "dac".to_string();
    let dac_fft_paths = count_paths(&dac, succ, done("fft"));
    let dac_out_paths = count_paths(&dac, succ, done("out"));

    let fft = "fft".to_string();
    let fft_dac_paths = count_paths(&fft, succ, done("dac"));
    let fft_out_paths = count_paths(&fft, succ, done("out"));

    dbg!(
        svr_dac_paths,
        svr_fft_paths,
        svr_out_paths,
        dac_fft_paths,
        dac_out_paths,
        fft_dac_paths,
        fft_out_paths,
    );
    let svr_fft_dac_out_paths = svr_fft_paths * fft_dac_paths * dac_out_paths;
    let svr_dac_fft_out_paths = svr_dac_paths * dac_fft_paths * fft_out_paths;
    let res_paths = (svr_dac_fft_out_paths + svr_fft_dac_out_paths) as u64;
    Some(res_paths)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(2));
    }
}
