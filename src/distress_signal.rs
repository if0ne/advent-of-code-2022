use crate::problem::Problem;
use std::cmp::Ordering;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Packet {
    Array(Vec<Packet>),
    Single(u8),
}

impl Packet {
    fn to_array(&self) -> Self {
        if let Packet::Single(_) = self {
            Packet::Array(vec![self.clone()])
        } else {
            self.clone()
        }
    }

    fn parse(tokens: &mut Vec<&str>) -> Packet {
        let mut inner = vec![];

        while let Some(token) = tokens.pop() {
            match token {
                "[" => inner.push(Self::parse(tokens)),
                "]" => break,
                "," => (/*skip*/),
                x => inner.push(Packet::Single(x.parse().unwrap())),
            }
        }

        Self::Array(inner)
    }
}

impl FromStr for Packet {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s
            .replace('[', " [ ")
            .replace(']', " ] ")
            .replace(',', " , ");
        let mut tokens = s.split_whitespace().rev().collect::<Vec<_>>();
        if tokens.pop() != Some("[") {
            return Err("Packet should start with '['".to_string());
        }

        Ok(Packet::parse(&mut tokens))
    }
}

impl PartialOrd<Self> for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Array(l), Packet::Array(r)) => {
                for pair in l.iter().zip(r) {
                    match pair.0.cmp(pair.1) {
                        Ordering::Less => return Ordering::Less,
                        Ordering::Greater => return Ordering::Greater,
                        Ordering::Equal => {}
                    }
                }

                l.len().cmp(&r.len())
            }
            (Packet::Array(_), Packet::Single(_)) => self.cmp(&other.to_array()),
            (Packet::Single(_), Packet::Array(_)) => self.to_array().cmp(other),
            (Packet::Single(l), Packet::Single(r)) => l.cmp(r),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Pair {
    left: Packet,
    right: Packet,
}

pub struct DistressSignal;

impl Problem for DistressSignal {
    type InputData = Vec<Pair>;
    type OutputDataFirstPart = usize;
    type OutputDataSecondPart = usize;

    fn read_file(filename: impl AsRef<Path>) -> Self::InputData {
        let file = std::fs::File::open(filename).unwrap();
        let reader = BufReader::new(file);
        reader
            .lines()
            .map(Result::unwrap)
            .collect::<Vec<_>>()
            .chunks(3)
            .into_iter()
            .map(|chunk| {
                let left = &chunk[0];
                let right = &chunk[1];
                let left = Packet::from_str(left).unwrap();
                let right = Packet::from_str(right).unwrap();

                Pair { left, right }
            })
            .collect()
    }

    fn first_part(input: Self::InputData) -> Self::OutputDataFirstPart {
        input
            .into_iter()
            .enumerate()
            .filter(|(_, el)| el.left.cmp(&el.right) == Ordering::Less)
            .map(|(i, _)| i + 1)
            .sum()
    }

    fn second_part(input: Self::InputData) -> Option<Self::OutputDataSecondPart> {
        let two = Packet::Array(vec![Packet::Array(vec![Packet::Single(2)])]);
        let six = Packet::Array(vec![Packet::Array(vec![Packet::Single(6)])]);

        let mut input = input
            .into_iter()
            .flat_map(|pair| [pair.left, pair.right])
            .collect::<Vec<_>>();
        input.push(two.clone());
        input.push(six.clone());

        input.sort();

        let (Ok(two) | Err(two)) = input.binary_search(&two);
        let (Ok(six) | Err(six)) = input.binary_search(&six);

        Some((two + 1) * (six + 1))
    }
}
