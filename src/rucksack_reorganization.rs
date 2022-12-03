use crate::Problem;
use std::collections::HashSet;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub struct RucksackReorganization;

fn score(c: char) -> u64 {
    let uppercase_bonus = if c.is_uppercase() { 26 } else { 0 };

    let score = (c.to_lowercase().next().unwrap() as u32 - 'a' as u32 + 1) as u64;

    score + uppercase_bonus
}

impl Problem for RucksackReorganization {
    type InputData = Vec<String>;
    type OutputData = u64;

    fn read_file(filename: impl AsRef<Path>) -> Self::InputData {
        let mut inventory = vec![];

        let mut buffer = String::new();
        let file = std::fs::File::open(filename).unwrap();
        let mut reader = BufReader::new(file);

        while let Ok(line) = reader.read_line(&mut buffer) {
            if line == 0 {
                break;
            }

            //inventory.push((l, r));

            inventory.push(buffer.trim().to_string());
            buffer.clear();
        }

        inventory
    }

    fn first_part(input: Self::InputData) -> Self::OutputData {
        input
            .into_iter()
            .map(|items| {
                let half_len = items.len() / 2;
                let l = items.chars().take(half_len).collect::<HashSet<char>>();
                let r = items
                    .chars()
                    .skip(half_len)
                    .take(half_len + 1)
                    .collect::<HashSet<char>>();

                (l, r)
            })
            .fold(0, |acc, (l, r)| {
                let intersection = l.intersection(&r).collect::<Vec<&char>>();
                let score = score(*intersection[0]);

                acc + score
            })
    }

    fn second_part(input: Self::InputData) -> Option<Self::OutputData> {
        Some(
            input
                .into_iter()
                .map(|item| item.chars().collect::<HashSet<char>>())
                .collect::<Vec<_>>()
                .chunks(3)
                .map(|arr| {
                    let intersection = arr[0]
                        .intersection(&arr[1])
                        .copied()
                        .collect::<HashSet<char>>();
                    let intersection = intersection.intersection(&arr[2]).collect::<Vec<&char>>();
                    score(*intersection[0])
                })
                .sum(),
        )
    }
}
