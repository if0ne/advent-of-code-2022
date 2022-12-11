use crate::problem::Problem;

use std::cell::RefCell;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Clone, Debug)]
pub enum Operation {
    Mul(u128),
    Add(u128),
    Sqr,
}

impl Operation {
    fn execute(&self, old: u128) -> u128 {
        match self {
            Operation::Mul(op) => old * (*op),
            Operation::Add(op) => old + (*op),
            Operation::Sqr => old * old,
        }
    }
}

#[derive(Clone, Debug)]
pub struct WorryTest {
    divider: u128,
    true_monkey: usize,
    false_monkey: usize,
}

#[derive(Clone, Debug)]
pub struct Monkey {
    items: RefCell<Vec<u128>>,
    operation: Operation,
    test: WorryTest,
}

pub struct MonkeyInTheMiddle;

impl MonkeyInTheMiddle {
    fn get_monkey_business(input: Vec<Monkey>, rounds: usize, divider: u128) -> u128 {
        let mut activity = vec![0u128; input.len()];
        let overflow_reducer = input.iter().fold(1, |acc, el| acc * el.test.divider);
        for _ in 0..rounds {
            for (i, monkey) in input.iter().enumerate() {
                let mut items = monkey.items.borrow_mut();
                while items.len() > 0 {
                    let item = items.remove(0);
                    let new_level = monkey.operation.execute(item) / divider;
                    let new_level = new_level % overflow_reducer;
                    if new_level % monkey.test.divider == 0 {
                        input[monkey.test.true_monkey]
                            .items
                            .borrow_mut()
                            .push(new_level);
                    } else {
                        input[monkey.test.false_monkey]
                            .items
                            .borrow_mut()
                            .push(new_level);
                    }

                    activity[i] += 1;
                }
            }
        }
        activity.sort();
        activity.reverse();

        activity[0] * activity[1]
    }
}

impl Problem for MonkeyInTheMiddle {
    type InputData = Vec<Monkey>;
    type OutputDataFirstPart = u128;
    type OutputDataSecondPart = u128;

    fn read_file(filename: impl AsRef<Path>) -> Self::InputData {
        let file = std::fs::File::open(filename).unwrap();
        let reader = BufReader::new(file);
        //TODO: Split parse into function. I'm too lazy
        reader
            .lines()
            .map(Result::unwrap)
            .collect::<Vec<_>>()
            .chunks(7)
            .fold(vec![], |mut monkeys, chunks| {
                let (_, items) = chunks[1].split_once(':').unwrap();
                let items = items
                    .trim()
                    .split(", ")
                    .map(|el| el.parse::<u128>().unwrap())
                    .collect::<Vec<_>>();

                let (_, operation) = chunks[2].split_once('=').unwrap();
                let tokens = operation.trim().split(' ').collect::<Vec<_>>();
                let operation = match tokens[1] {
                    "+" => {
                        let operand = tokens[2].parse().unwrap();
                        Operation::Add(operand)
                    }
                    "*" => {
                        let operand = tokens[2].parse();
                        match operand {
                            Ok(operand) => Operation::Mul(operand),
                            Err(_) => Operation::Sqr,
                        }
                    }
                    _ => panic!("Wrong operation"),
                };

                let divider = chunks[3]
                    .split(' ')
                    .last()
                    .unwrap()
                    .parse::<u128>()
                    .unwrap();
                let true_monkey = chunks[4]
                    .split(' ')
                    .last()
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();
                let false_monkey = chunks[5]
                    .split(' ')
                    .last()
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();

                let monkey = Monkey {
                    items: RefCell::new(items),
                    operation,
                    test: WorryTest {
                        divider,
                        true_monkey,
                        false_monkey,
                    },
                };
                monkeys.push(monkey);

                monkeys
            })
    }

    fn first_part(input: Self::InputData) -> Self::OutputDataFirstPart {
        const ROUNDS: usize = 20;
        MonkeyInTheMiddle::get_monkey_business(input, ROUNDS, 3)
    }

    fn second_part(input: Self::InputData) -> Option<Self::OutputDataSecondPart> {
        const ROUNDS: usize = 10000;
        Some(MonkeyInTheMiddle::get_monkey_business(input, ROUNDS, 1))
    }
}
