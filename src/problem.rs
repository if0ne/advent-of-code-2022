use std::path::Path;

pub trait Problem {
    type InputData: Clone;
    type OutputDataFirstPart;
    type OutputDataSecondPart;

    fn read_file(filename: impl AsRef<Path>) -> Self::InputData;
    fn first_part(input: Self::InputData) -> Self::OutputDataFirstPart;
    fn second_part(input: Self::InputData) -> Option<Self::OutputDataSecondPart>;
}
