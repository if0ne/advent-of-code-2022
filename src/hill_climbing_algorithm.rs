use crate::problem::Problem;
use std::collections::VecDeque;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Graph {
    map: Vec<Vec<u64>>,
    start: (usize, usize),
    end: (usize, usize),
}

impl Graph {
    fn width(&self) -> usize {
        self.map[0].len()
    }

    fn height(&self) -> usize {
        self.map.len()
    }
}

pub struct HillClimbingAlgorith;

impl Problem for HillClimbingAlgorith {
    type InputData = Graph;
    type OutputDataFirstPart = u64;
    type OutputDataSecondPart = u64;

    fn read_file(filename: impl AsRef<Path>) -> Self::InputData {
        let file = std::fs::File::open(filename).unwrap();
        let reader = BufReader::new(file);
        reader.lines().map(Result::unwrap).enumerate().fold(
            Graph {
                map: vec![],
                start: (0, 0),
                end: (0, 0),
            },
            |mut graph, (y, line)| {
                let start = line.find('S');
                let end = line.find('E');

                if let Some(start) = start {
                    graph.start = (start, y);
                }

                if let Some(end) = end {
                    graph.end = (end, y);
                }

                let row = line
                    .chars()
                    .map(|el| {
                        if el == 'S' {
                            0
                        } else if el == 'E' {
                            ('z' as u32 - 'a' as u32) as u64
                        } else {
                            (el as u32 - 'a' as u32) as u64
                        }
                    })
                    .collect();
                graph.map.push(row);

                graph
            },
        )
    }

    fn first_part(input: Self::InputData) -> Self::OutputDataFirstPart {
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut step_map: Vec<Vec<Option<i32>>> = (0..input.height())
            .map(|_| (0..input.width()).map(|_| None).collect())
            .collect();
        let mut stack = VecDeque::from_iter([(input.end, 0)]);

        while let Some((pos, steps)) = stack.pop_front() {
            let height = input.map[pos.1][pos.0];

            for direction in &directions {
                let next_pos = (pos.0 as i64 + direction.0, pos.1 as i64 + direction.1);
                if next_pos.0 < 0
                    || next_pos.0 >= input.width() as i64
                    || next_pos.1 < 0
                    || next_pos.1 >= input.height() as i64
                    || step_map[next_pos.1 as usize][next_pos.0 as usize].is_some()
                {
                    continue;
                }

                let next_height = input.map[next_pos.1 as usize][next_pos.0 as usize];
                if height as i64 - next_height as i64 <= 1 {
                    stack.push_back(((next_pos.0 as usize, next_pos.1 as usize), steps + 1));
                    step_map[next_pos.1 as usize][next_pos.0 as usize] = Some(steps + 1);
                }
            }
        }

        step_map[input.start.1][input.start.0].unwrap() as u64
    }

    fn second_part(input: Self::InputData) -> Option<Self::OutputDataSecondPart> {
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut step_map: Vec<Vec<Option<i32>>> = (0..input.height())
            .map(|_| (0..input.width()).map(|_| None).collect())
            .collect();
        let mut stack = VecDeque::from_iter([(input.end, 0)]);
        let mut min_steps = None;

        while let Some((pos, steps)) = stack.pop_front() {
            let height = input.map[pos.1][pos.0];

            if height == 0 && steps < min_steps.unwrap_or(i64::MAX) {
                min_steps = Some(steps);
            }

            for direction in &directions {
                let next_pos = (pos.0 as i64 + direction.0, pos.1 as i64 + direction.1);
                if next_pos.0 < 0
                    || next_pos.0 >= input.width() as i64
                    || next_pos.1 < 0
                    || next_pos.1 >= input.height() as i64
                    || step_map[next_pos.1 as usize][next_pos.0 as usize].is_some()
                {
                    continue;
                }

                let next_height = input.map[next_pos.1 as usize][next_pos.0 as usize];
                if height as i64 - next_height as i64 <= 1 {
                    stack.push_back(((next_pos.0 as usize, next_pos.1 as usize), steps + 1));
                    step_map[next_pos.1 as usize][next_pos.0 as usize] = Some((steps + 1) as i32);
                }
            }
        }

        min_steps.map(|el| el as u64)
    }
}
