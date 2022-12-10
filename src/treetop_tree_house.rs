use std::io::{BufRead, BufReader};
use std::ops::ControlFlow::{Break, Continue};
use std::path::Path;

use crate::problem::Problem;

pub struct TreetopTreeHouse;

impl TreetopTreeHouse {
    fn is_hide_tree((i, j): (usize, usize), field: &[Vec<u8>]) -> bool {
        let current = field[i][j];

        //Left
        if *field[i][..j].iter().max().unwrap() < current {
            return false;
        }

        //Right
        if *field[i][(j + 1)..].iter().max().unwrap() < current {
            return false;
        }

        //Top
        if field[..i].iter().map(|el| el[j]).max().unwrap() < current {
            return false;
        }

        //Bottom
        if field[(i + 1)..].iter().map(|el| el[j]).max().unwrap() < current {
            return false;
        }

        true
    }

    //TODO: Helper function to reduce duplicate
    fn count_visible_tree_in_row(
        current: u8,
        index: usize,
        mut range: impl Iterator<Item = usize>,
        field: &[Vec<u8>],
    ) -> usize {
        let (Continue(count) | Break(count)) = range.try_fold(0, |acc, k| {
            if field[index][k] < current {
                Continue(acc + 1)
            } else {
                Break(acc + 1)
            }
        });

        count
    }

    fn count_visible_tree_in_col(
        current: u8,
        index: usize,
        mut range: impl Iterator<Item = usize>,
        field: &[Vec<u8>],
    ) -> usize {
        let (Continue(count) | Break(count)) = range.try_fold(0, |acc, k| {
            if field[k][index] < current {
                Continue(acc + 1)
            } else {
                Break(acc + 1)
            }
        });

        count
    }
}

impl Problem for TreetopTreeHouse {
    type InputData = Vec<Vec<u8>>;
    type OutputDataFirstPart = usize;
    type OutputDataSecondPart = usize;

    fn read_file(filename: impl AsRef<Path>) -> Self::InputData {
        let file = std::fs::File::open(filename).unwrap();
        let reader = BufReader::new(file);

        reader
            .lines()
            .map(Result::unwrap)
            .fold(vec![], |mut acc, el| {
                acc.push(
                    el.chars()
                        .map(|el| el.to_digit(10).unwrap())
                        .map(|el| el as u8)
                        .collect::<Vec<_>>(),
                );

                acc
            })
    }

    fn first_part(input: Self::InputData) -> Self::OutputDataFirstPart {
        let mut count = 0;
        //Borders
        count += input.len() * 2;
        count += (input[0].len() - 2) * 2;

        for i in 1..(input.len() - 1) {
            for j in 1..(input[i].len() - 1) {
                if !Self::is_hide_tree((i, j), &input) {
                    count += 1;
                }
            }
        }

        count
    }

    fn second_part(input: Self::InputData) -> Option<Self::OutputDataSecondPart> {
        let mut max = 0;

        for i in 0..input.len() {
            for j in 0..input[i].len() {
                let current = input[i][j];

                let left_count = Self::count_visible_tree_in_row(current, i, (0..j).rev(), &input);
                let right_count =
                    Self::count_visible_tree_in_row(current, i, (j + 1)..input[i].len(), &input);
                let top_count = Self::count_visible_tree_in_col(current, j, (0..i).rev(), &input);
                let bottom_count =
                    Self::count_visible_tree_in_col(current, j, (i + 1)..input.len(), &input);

                max = max.max(right_count * left_count * top_count * bottom_count);
            }
        }

        Some(max)
    }
}
