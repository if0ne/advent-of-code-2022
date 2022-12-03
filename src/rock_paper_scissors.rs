use crate::problem::Problem;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl Choice {
    fn translate(c: &str) -> Choice {
        match c {
            "A" => Choice::Rock,
            "B" => Choice::Paper,
            "C" => Choice::Scissors,
            "X" => Choice::Rock,
            "Y" => Choice::Paper,
            "Z" => Choice::Scissors,
            _ => panic!("UNKNOWN VARIANT"),
        }
    }

    fn get_extra_bonus(self) -> u64 {
        match self {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        }
    }

    fn get_weak(self) -> Choice {
        match self {
            Choice::Rock => Choice::Scissors,
            Choice::Paper => Choice::Rock,
            Choice::Scissors => Choice::Paper,
        }
    }

    fn get_strong(self) -> Choice {
        match self {
            Choice::Rock => Choice::Paper,
            Choice::Paper => Choice::Scissors,
            Choice::Scissors => Choice::Rock,
        }
    }

    fn get_bonus_for_beat(self, other: Choice) -> u64 {
        let other_weak = other.get_weak();
        if self == other_weak {
            6
        } else if self == other {
            3
        } else {
            0
        }
    }

    fn get_bonus_for_decision(self) -> u64 {
        match self {
            Choice::Rock => 0,
            Choice::Paper => 3,
            Choice::Scissors => 6,
        }
    }
}

pub struct RockPaperScissors;

#[allow(clippy::identity_op)]
fn score_first_part(l: Choice, r: Choice) -> u64 {
    l.get_bonus_for_beat(r) + r.get_extra_bonus()
}

#[allow(clippy::identity_op)]
fn score_second_part(l: Choice, r: Choice) -> u64 {
    let result = r.get_bonus_for_decision();
    result
        + match r {
            Choice::Rock => l.get_weak().get_extra_bonus(),
            Choice::Paper => l.get_extra_bonus(),
            Choice::Scissors => l.get_strong().get_extra_bonus(),
        }
}

impl Problem for RockPaperScissors {
    type InputData = Vec<(Choice, Choice)>;
    type OutputData = u64;

    fn read_file(filename: impl AsRef<Path>) -> Self::InputData {
        let file = std::fs::File::open(filename).unwrap();
        let reader = BufReader::new(file);

        reader
            .lines()
            .map(Result::unwrap)
            .map(|line| {
                let (l, r) = line.split_once(' ').unwrap();
                (Choice::translate(l), Choice::translate(r))
            })
            .collect()
    }

    fn first_part(input: Self::InputData) -> Self::OutputData {
        input
            .into_iter()
            .fold(0, |acc, (l, r)| acc + score_first_part(l, r))
    }

    fn second_part(input: Self::InputData) -> Option<Self::OutputData> {
        Some(
            input
                .into_iter()
                .fold(0, |acc, (l, r)| acc + score_second_part(l, r)),
        )
    }
}
