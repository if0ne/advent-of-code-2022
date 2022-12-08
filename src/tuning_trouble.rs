use crate::problem::Problem;
use std::collections::{HashSet, VecDeque};
use std::io::{BufRead, BufReader};
use std::path::Path;

pub struct TuningTrouble;

impl TuningTrouble {
    fn find_marker(input: String, marker_size: usize) -> usize {
        let mut queue = VecDeque::new();
        for (i, c) in input.chars().enumerate() {
            if queue.len() < marker_size {
                queue.push_back(c);
            }
            if queue.iter().collect::<HashSet<_>>().len() == marker_size {
                return i + 1;
            } else if queue.len() == marker_size {
                queue.pop_front();
            }
        }

        panic!("WRONG INPUT")
    }
}

impl Problem for TuningTrouble {
    type InputData = String;
    type OutputData = usize;

    fn read_file(filename: impl AsRef<Path>) -> Self::InputData {
        let file = std::fs::File::open(filename).unwrap();
        let reader = BufReader::new(file);
        reader.lines().map(Result::unwrap).next().unwrap()
    }

    fn first_part(input: Self::InputData) -> Self::OutputData {
        const MARKER_SIZE: usize = 4;
        TuningTrouble::find_marker(input, MARKER_SIZE)
    }

    fn second_part(input: Self::InputData) -> Option<Self::OutputData> {
        const MARKER_SIZE: usize = 14;
        Some(TuningTrouble::find_marker(input, MARKER_SIZE))
    }
}
