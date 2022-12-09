mod calorie_counting;
mod camp_cleanup;
mod no_space_left_on_device;
mod problem;
mod rock_paper_scissors;
mod rope_bridge;
mod rucksack_reorganization;
mod supply_stacks;
mod treetop_tree_house;
mod tuning_trouble;

use crate::problem::Problem;
use crate::rope_bridge::RopeBridge;

use std::path::Path;

pub fn solver<T: Problem>(filename: impl AsRef<Path>) -> (T::OutputData, Option<T::OutputData>) {
    let data = T::read_file(filename);
    let first_answer = T::first_part(data.clone());
    let second_answer = T::second_part(data);

    (first_answer, second_answer)
}

fn main() {
    println!("{:?}", solver::<RopeBridge>("rope_bridge.txt"));
}
