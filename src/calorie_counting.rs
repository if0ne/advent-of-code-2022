use crate::problem::Problem;
use std::collections::BinaryHeap;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub struct CalorieCounting;

impl CalorieCounting {
    fn sum_inventory(inventories: Vec<Vec<u64>>) -> BinaryHeap<u64> {
        inventories
            .into_iter()
            .fold(vec![], |mut arr, subarray| {
                arr.push(subarray.into_iter().sum::<u64>());
                arr
            })
            .into_iter()
            .collect::<BinaryHeap<_>>()
    }
}

impl Problem for CalorieCounting {
    type InputData = Vec<Vec<u64>>;
    type OutputDataFirstPart = u64;
    type OutputDataSecondPart = u64;

    fn read_file(filename: impl AsRef<Path>) -> Self::InputData {
        let file = std::fs::File::open(filename).unwrap();
        let reader = BufReader::new(file);

        reader
            .lines()
            .map(Result::unwrap)
            .fold(vec![vec![]], |mut elfs, line| {
                if line.is_empty() {
                    elfs.push(vec![]);
                } else {
                    elfs.last_mut().unwrap().push(line.parse::<u64>().unwrap())
                }
                elfs
            })
    }

    fn first_part(input: Self::InputData) -> Self::OutputDataFirstPart {
        Self::sum_inventory(input).pop().unwrap()
    }

    fn second_part(input: Self::InputData) -> Option<Self::OutputDataSecondPart> {
        Some(Self::sum_inventory(input).into_iter().take(3).sum())
    }
}
