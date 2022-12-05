use crate::Problem;
use std::collections::VecDeque;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub struct SupplyStacks;

#[derive(Clone)]
pub struct SupplyData(Vec<Vec<char>>, Vec<SupplyCommand>);
#[derive(Debug, Clone)]
pub struct SupplyCommand(u64, usize, usize);

impl Problem for SupplyStacks {
    type InputData = SupplyData;
    type OutputData = String;

    fn read_file(filename: impl AsRef<Path>) -> Self::InputData {
        let file = std::fs::File::open(filename).unwrap();
        let reader = BufReader::new(file);

        let mut lines: Vec<String> = reader.lines().map(Result::unwrap).collect();
        let mut splitter = lines.split_mut(|s| s.is_empty());

        let stacks = splitter.next().unwrap();
        stacks.reverse();
        let text_commands = splitter.next().unwrap();

        let mut crates = vec![vec![]; stacks[0].split(' ').filter(|el| !el.is_empty()).count()];
        for stack in stacks.iter().skip(1) {
            stack
                .chars()
                .collect::<Vec<char>>()
                .chunks(4)
                .into_iter()
                .map(|t| t[1])
                .enumerate()
                .for_each(|(i, t)| {
                    if !t.is_whitespace() {
                        crates[i].push(t);
                    }
                });
        }

        let mut commands = vec![];
        for command in text_commands {
            let tokens = command.split(' ').collect::<Vec<_>>();
            commands.push(SupplyCommand(
                tokens[1].parse().unwrap(),
                tokens[3].parse::<usize>().unwrap() - 1,
                tokens[5].parse::<usize>().unwrap() - 1,
            ));
        }

        SupplyData(crates, commands)
    }

    fn first_part(input: Self::InputData) -> Self::OutputData {
        let mut crates = input.0;
        let commands = input.1;

        for command in commands {
            for _ in 0..command.0 {
                let pop = crates[command.1].pop().unwrap();
                crates[command.2].push(pop);
            }
        }

        crates.into_iter().fold(String::new(), |mut acc, crt| {
            acc.push(*crt.last().unwrap());
            acc
        })
    }

    fn second_part(input: Self::InputData) -> Option<Self::OutputData> {
        let mut crates = input.0;
        let commands = input.1;

        for command in commands {
            let mut queue = VecDeque::new();
            for _ in 0..command.0 {
                queue.push_back(crates[command.1].pop().unwrap());
            }
            while !queue.is_empty() {
                crates[command.2].push(queue.pop_back().unwrap());
            }
        }

        Some(crates.into_iter().fold(String::new(), |mut acc, crt| {
            acc.push(*crt.last().unwrap());
            acc
        }))
    }
}
