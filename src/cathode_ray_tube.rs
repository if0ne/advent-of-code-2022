use crate::problem::Problem;
use std::fmt::Debug;
use std::io::{BufRead, BufReader};
use std::ops::Add;
use std::path::Path;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub enum CpuCommand<T: Add> {
    Add(T),
    Noop,
}

impl<T: Add> CpuCommand<T> {
    fn cycles(&self) -> i64 {
        match self {
            CpuCommand::Add(_) => 2,
            CpuCommand::Noop => 1,
        }
    }
}

impl<T: Add + FromStr, S: AsRef<str>> From<S> for CpuCommand<T>
where
    <T as FromStr>::Err: Debug,
{
    fn from(value: S) -> Self {
        let (l, r) = value
            .as_ref()
            .split_once(' ')
            .unwrap_or((value.as_ref(), ""));
        match l {
            "addx" => Self::Add(r.parse().unwrap()),
            "noop" => Self::Noop,
            _ => panic!("Unknown command"),
        }
    }
}

pub struct CathodeRayTube;

impl Problem for CathodeRayTube {
    type InputData = Vec<CpuCommand<i64>>;
    type OutputDataFirstPart = i64;
    type OutputDataSecondPart = String;

    fn read_file(filename: impl AsRef<Path>) -> Self::InputData {
        let file = std::fs::File::open(filename).unwrap();
        let reader = BufReader::new(file);

        reader
            .lines()
            .map(Result::unwrap)
            .map(|line| line.into())
            .collect()
    }

    fn first_part(input: Self::InputData) -> Self::OutputDataFirstPart {
        let mut limits = vec![20, 60, 100, 140, 180, 220];
        input
            .into_iter()
            .fold((0, 1, 0), |(mut signal, mut x, mut cycles), command| {
                cycles += command.cycles();

                if let Some(first) = limits.first() {
                    if cycles >= *first {
                        signal += first * x;
                        limits.remove(0);
                    }
                }

                if let CpuCommand::Add(operand) = command {
                    x += operand;
                };

                (signal, x, cycles)
            })
            .0
    }

    fn second_part(input: Self::InputData) -> Option<Self::OutputDataSecondPart> {
        let mut crt = [['.'; 40]; 6];
        let mut sprite_pos = 1i64;
        let mut cycles = 0;

        for command in input {
            for _ in 0..command.cycles() {
                let cursor_x = cycles % 40;
                let cursor_y = cycles / 40;

                if ((sprite_pos - 1)..=(sprite_pos + 1)).contains(&cursor_x) {
                    crt[cursor_y as usize][cursor_x as usize] = '#';
                }

                cycles += 1;
            }

            if let CpuCommand::Add(operand) = command {
                sprite_pos += operand;
            };
        }

        Some(
            crt.into_iter()
                .map(|chunk| chunk.into_iter().collect::<String>())
                .collect::<Vec<_>>()
                .join("\n"),
        )
    }
}
