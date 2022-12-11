mod calorie_counting;
mod camp_cleanup;
mod cathode_ray_tube;
mod monkey_in_the_middle;
mod no_space_left_on_device;
mod problem;
mod rock_paper_scissors;
mod rope_bridge;
mod rucksack_reorganization;
mod supply_stacks;
mod treetop_tree_house;
mod tuning_trouble;

use crate::problem::Problem;

use crate::monkey_in_the_middle::MonkeyInTheMiddle;
use std::path::Path;

pub fn solver<T: Problem>(
    filename: impl AsRef<Path>,
) -> (T::OutputDataFirstPart, Option<T::OutputDataSecondPart>) {
    let data = T::read_file(filename);
    let first_answer = T::first_part(data.clone());
    let second_answer = T::second_part(data);

    (first_answer, second_answer)
}

fn main() {
    println!(
        "{:?}",
        solver::<MonkeyInTheMiddle>("monkey_in_the_middle.txt")
    )
}
