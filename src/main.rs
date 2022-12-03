mod calorie_counting;
mod problem;
mod rock_paper_scissors;
mod rucksack_reorganization;

use std::path::Path;
use crate::problem::Problem;

pub fn solver<T: Problem>(filename: impl AsRef<Path>) -> (T::OutputData, Option<T::OutputData>) {
    let data = T::read_file(filename);
    let first_answer = T::first_part(data.clone());
    let second_answer = T::second_part(data);

    (first_answer, second_answer)
}

fn main() {
}
