use std::path::Path;

pub trait Problem {
    type InputData: Clone;
    type OutputData;

    fn read_file(filename: impl AsRef<Path>) -> Self::InputData;
    fn first_part(input: Self::InputData) -> Self::OutputData;
    fn second_part(input: Self::InputData) -> Option<Self::OutputData>;
}
