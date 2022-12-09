use std::collections::HashSet;
use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::problem::Problem;

pub struct RopeBridge;

impl RopeBridge {
    fn get_direction(sym: &str) -> (i64, i64) {
        match sym {
            "U" => (0, 1),
            "D" => (0, -1),
            "R" => (1, 0),
            "L" => (-1, 0),
            _ => panic!("Unexpected direction"),
        }
    }

    fn get_distance(l: (i64, i64), r: (i64, i64)) -> (i64, i64) {
        (r.0 - l.0, r.1 - l.1)
    }
}

impl Problem for RopeBridge {
    type InputData = Vec<((i64, i64), u64)>;
    type OutputData = usize;

    fn read_file(filename: impl AsRef<Path>) -> Self::InputData {
        let file = std::fs::File::open(filename).unwrap();
        let reader = BufReader::new(file);

        reader
            .lines()
            .map(Result::unwrap)
            .map(|line| {
                let (l, r) = line.split_once(' ').unwrap();
                (RopeBridge::get_direction(l), r.parse::<u64>().unwrap())
            })
            .collect()
    }

    fn first_part(input: Self::InputData) -> Self::OutputData {
        let mut unique_position = HashSet::new();
        let mut h = (1, 1);
        let mut t = (1, 1);

        unique_position.insert(t);
        for command in input {
            for _ in 0..command.1 {
                h.0 += command.0 .0;
                h.1 += command.0 .1;
                let (x, y) = RopeBridge::get_distance(t, h);

                if x.abs() == 2 || y.abs() == 2 {
                    t.0 += x.normalize();
                    t.1 += y.normalize();

                    unique_position.insert(t);
                }
            }
        }

        unique_position.len()
    }

    fn second_part(input: Self::InputData) -> Option<Self::OutputData> {
        const LAST: usize = 9;
        const FIRST: usize = 0;

        let mut unique_position = HashSet::new();
        let mut rope = vec![(1, 1); 10];

        unique_position.insert(rope[LAST]);
        for command in input {
            for _ in 0..command.1 {
                rope[FIRST].0 += command.0 .0;
                rope[FIRST].1 += command.0 .1;

                for i in 1..10 {
                    let (x, y) = RopeBridge::get_distance(rope[i], rope[i - 1]);

                    if x.abs() == 2 || y.abs() == 2 {
                        rope[i].0 += x.normalize();
                        rope[i].1 += y.normalize();

                        if i == LAST {
                            unique_position.insert(rope[i]);
                        }
                    }
                }
            }
        }

        Some(unique_position.len())
    }
}

trait Normalizer {
    fn normalize(self) -> Self;
}

impl Normalizer for i64 {
    fn normalize(self) -> Self {
        if self != 0 {
            self / self.abs()
        } else {
            0
        }
    }
}
