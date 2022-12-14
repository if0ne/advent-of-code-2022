use crate::problem::Problem;
use std::collections::HashSet;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub struct RegolithReservoir;

impl Problem for RegolithReservoir {
    type InputData = HashSet<(usize, usize)>;
    type OutputDataFirstPart = u64;
    type OutputDataSecondPart = u64;

    fn read_file(filename: impl AsRef<Path>) -> Self::InputData {
        let file = std::fs::File::open(filename).unwrap();
        let reader = BufReader::new(file);
        reader
            .lines()
            .map(Result::unwrap)
            .fold(HashSet::new(), |mut rock_positions, line| {
                let mut positions = line.split(" -> ").map(|pos| {
                    let (l, r) = pos.split_once(',').unwrap();
                    (l.parse::<usize>().unwrap(), r.parse::<usize>().unwrap())
                });
                let mut current = positions.next().unwrap();
                for (x, y) in positions {
                    if x == current.0 {
                        rock_positions
                            .extend((y.min(current.1)..=y.max(current.1)).map(|y| (x, y)));
                    } else if y == current.1 {
                        rock_positions
                            .extend((x.min(current.0)..=x.max(current.0)).map(|x| (x, y)));
                    }
                    current = (x, y);
                }

                rock_positions
            })
    }

    fn first_part(input: Self::InputData) -> Self::OutputDataFirstPart {
        const START: (usize, usize) = (500, 0);
        let mut particles = input;
        let bottom_limit = particles.iter().map(|(_, y)| *y).max().unwrap();
        let mut sand_count = 0;

        'generator: loop {
            let mut sand_position = START;
            'moving: loop {
                let (x, y) = sand_position;
                if y > bottom_limit {
                    break 'generator sand_count;
                }

                if let Some(next_position) = [(x, y + 1), (x - 1, y + 1), (x + 1, y + 1)]
                    .into_iter()
                    .find(|point| !particles.contains(point))
                {
                    sand_position = next_position;
                    continue;
                }

                particles.insert(sand_position);
                sand_count += 1;
                break 'moving;
            }
        }
    }

    fn second_part(input: Self::InputData) -> Option<Self::OutputDataSecondPart> {
        const START: (usize, usize) = (500, 0);
        let mut particles = input;
        let floor = particles.iter().map(|(_, y)| *y).max().unwrap() + 2;
        let mut sand_count = 0;

        Some('generator: loop {
            let mut sand_position = START;
            'moving: loop {
                let (x, y) = sand_position;
                if y == floor {
                    particles.insert(sand_position);
                    break 'moving;
                }

                if let Some(next_position) = [(x, y + 1), (x - 1, y + 1), (x + 1, y + 1)]
                    .into_iter()
                    .find(|point| !particles.contains(point))
                {
                    sand_position = next_position;
                    continue;
                }

                if sand_position == START {
                    // + 1, cause (500, 0) also new sand particle
                    break 'generator sand_count + 1;
                }

                particles.insert(sand_position);
                sand_count += 1;
                break 'moving;
            }
        })
    }
}
