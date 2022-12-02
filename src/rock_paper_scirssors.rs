use crate::problem::Problem;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub struct RockPaperScissors;

fn translate(c: &str) -> u64 {
    match c {
        "A" => 0,
        "B" => 1,
        "C" => 2,
        "X" => 0,
        "Y" => 1,
        "Z" => 2,
        _ => panic!("UNKNOWN VARIANT"),
    }
}

#[allow(clippy::identity_op)]
fn score_first_part(l: u64, r: u64) -> u64 {
    match (l, r) {
        (0, 0) => 3 + 1,
        (0, 1) => 6 + 2,
        (0, 2) => 0 + 3,
        (1, 0) => 0 + 1,
        (1, 1) => 3 + 2,
        (1, 2) => 6 + 3,
        (2, 0) => 6 + 1,
        (2, 1) => 0 + 2,
        (2, 2) => 3 + 3,
        _ => panic!("BROKEN INVARIANT"),
    }
}

#[allow(clippy::identity_op)]
fn score_second_part(l: u64, r: u64) -> u64 {
    match (l, r) {
        (0, 0) => 0 + 3,
        (0, 1) => 3 + 1,
        (0, 2) => 6 + 2,

        (1, 0) => 0 + 1,
        (1, 1) => 3 + 2,
        (1, 2) => 6 + 3,

        (2, 0) => 0 + 2,
        (2, 1) => 3 + 3,
        (2, 2) => 6 + 1,
        _ => panic!("BROKEN INVARIANT"),
    }
}

impl Problem for RockPaperScissors {
    type InputData = Vec<(u64, u64)>;
    type OutputData = u64;

    fn read_file(filename: impl AsRef<Path>) -> Self::InputData {
        let mut pairs = vec![];

        let mut buffer = String::new();
        let file = std::fs::File::open(filename).unwrap();
        let mut reader = BufReader::new(file);

        while let Ok(line) = reader.read_line(&mut buffer) {
            if line == 0 {
                break;
            }
            let immutable_buffer = buffer.clone();
            let str = immutable_buffer.trim().split(' ').collect::<Vec<_>>();
            pairs.push((translate(str[0]), translate(str[1])));

            buffer.clear();
        }

        pairs
    }

    fn first_part(input: Self::InputData) -> Self::OutputData {
        input
            .into_iter()
            .fold(0, |acc, (l, r)| acc + score_first_part(l, r))
    }

    fn second_part(input: Self::InputData) -> Option<Self::OutputData> {
        dbg!(&input);
        Some(
            input
                .into_iter()
                .fold(0, |acc, (l, r)| acc + score_second_part(l, r)),
        )
    }
}
