use crate::problem::Problem;
use std::io::{BufRead, BufReader};
use std::ops::RangeInclusive;
use std::path::Path;

pub struct CampCleanup;

impl Problem for CampCleanup {
    type InputData = Vec<(RangeInclusive<u64>, RangeInclusive<u64>)>;
    type OutputData = u64;

    fn read_file(filename: impl AsRef<Path>) -> Self::InputData {
        let file = std::fs::File::open(filename).unwrap();
        let reader = BufReader::new(file);
        let lines = reader.lines().map(Result::unwrap).collect::<Vec<_>>();
        lines
            .iter()
            .map(|line| line.split_once(',').unwrap())
            .map(|(l, r)| (l.split_once('-').unwrap(), r.split_once('-').unwrap()))
            .map(|((ll, lr), (rl, rr))| {
                (
                    ll.parse::<u64>().unwrap()..=lr.parse::<u64>().unwrap(),
                    rl.parse::<u64>().unwrap()..=rr.parse::<u64>().unwrap(),
                )
            })
            .collect()
    }

    fn first_part(input: Self::InputData) -> Self::OutputData {
        input
            .into_iter()
            .filter(|(l, r)| l.is_overlap_fully(r) || r.is_overlap_fully(l))
            .count() as u64
    }

    fn second_part(input: Self::InputData) -> Option<Self::OutputData> {
        Some(input.into_iter().filter(|(l, r)| l.is_overlap(r)).count() as u64)
    }
}

trait OverlapRange<T: Ord + PartialOrd> {
    fn is_overlap_fully(&self, other: &Self) -> bool;
    fn is_overlap(&self, other: &Self) -> bool;
}

impl<T: Ord + PartialOrd> OverlapRange<T> for RangeInclusive<T> {
    fn is_overlap_fully(&self, other: &Self) -> bool {
        self.start() <= other.start() && other.end() <= self.end()
    }

    fn is_overlap(&self, other: &Self) -> bool {
        self.contains(other.start()) || other.contains(self.start())
    }
}
