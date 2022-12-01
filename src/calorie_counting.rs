use crate::problem::Problem;
use std::collections::BinaryHeap;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub struct CalorieCounting;

impl Problem for CalorieCounting {
    type InputData = Vec<Vec<u64>>;
    type OutputData = u64;

    fn read_file(filename: impl AsRef<Path>) -> Self::InputData {
        let mut elfs = vec![vec![]];
        let mut elf_id = 0;

        let mut buffer = String::new();
        let file = std::fs::File::open(filename).unwrap();
        let mut reader = BufReader::new(file);

        while let Ok(line) = reader.read_line(&mut buffer) {
            if line == 0 {
                break;
            }
            let calorie = buffer.trim().parse::<u64>();
            match calorie {
                Ok(num) => elfs[elf_id].push(num),
                Err(_) => {
                    elf_id += 1;
                    elfs.push(vec![]);
                }
            }

            buffer.clear();
        }

        elfs
    }

    fn first_part(input: Self::InputData) -> Self::OutputData {
        input
            .into_iter()
            .reduce(|arr, subarray| [arr, vec![subarray.into_iter().sum()]].concat())
            .unwrap()
            .into_iter()
            .collect::<BinaryHeap<_>>()
            .pop()
            .unwrap()
    }

    fn second_part(input: Self::InputData) -> Option<Self::OutputData> {
        Some(
            input
                .into_iter()
                .reduce(|arr, subarray| [arr, vec![subarray.into_iter().sum()]].concat())
                .unwrap()
                .into_iter()
                .collect::<BinaryHeap<_>>()
                .into_iter()
                .take(3)
                .sum(),
        )
    }
}
