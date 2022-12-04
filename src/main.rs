mod calorie_counting;
mod camp_cleanup;
mod problem;
mod rock_paper_scissors;
mod rucksack_reorganization;

use crate::camp_cleanup::CampCleanup;
use crate::problem::Problem;
use std::path::Path;

pub fn solver<T: Problem>(filename: impl AsRef<Path>) -> (T::OutputData, Option<T::OutputData>) {
    let data = T::read_file(filename);
    let first_answer = T::first_part(data.clone());
    let second_answer = T::second_part(data);

    (first_answer, second_answer)
}

fn main() {
    println!("{:?}", solver::<CampCleanup>("camp_cleanup.txt"));
}
