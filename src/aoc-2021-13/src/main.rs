use std::collections::HashSet;
use std::io;
use std::io::Read;

type Paper = HashSet<Point>;
type Point = (usize, usize);

type Fold = (Axes, usize);

pub trait Foldable {
    fn fold_(&self, ins: Fold) -> Self;
}

impl Foldable for Paper {
    fn fold_(&self, ins: Fold) -> Self {
        let mut acc: HashSet<Point> = HashSet::new();

        match ins {
            (Axes::X, m) => {
                self.iter().for_each(|&x| match x.0 > m {
                    true => {
                        acc.insert((m - (x.0 - m), x.1));
                    }
                    false => {
                        if x.0 != m {
                            acc.insert((x.0, x.1));
                        }
                    }
                });
            }
            (Axes::Y, m) => {
                self.iter().for_each(|&x| match x.1 > m {
                    true => {
                        acc.insert((x.0, m - (x.1 - m)));
                    }
                    false => {
                        if x.1 != m {
                            acc.insert((x.0, x.1));
                        }
                    }
                });
            }
        };
        acc
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Axes {
    X,
    Y,
}

mod parser {
    use super::*;
    use nom::bytes::complete::tag;
    use nom::character::complete;
    use nom::character::complete::space0;
    use nom::sequence::{pair, separated_pair};
    use nom::IResult;
    use std::io::ErrorKind;

    fn point(input: &str) -> IResult<&str, Point> {
        match pair(
            space0,
            separated_pair(complete::u64, tag(","), complete::u64),
        )(input)
        {
            Ok((rem, point)) => Ok((rem, (point.1 .0 as usize, point.1 .1 as usize))),
            Err(e) => Err(e),
        }
    }

    fn fold_ins(input: &str) -> Result<Fold, ErrorKind> {
        let last = input.split("fold along ").last();
        if last.is_none() {
            return Err(ErrorKind::InvalidInput);
        }
        let last = last.unwrap();

        match last.trim().contains('=') {
            false => Err(ErrorKind::InvalidInput),
            true => Ok((
                match last.trim().chars().next().expect("No first char?") {
                    'x' => Axes::X,
                    'y' => Axes::Y,
                    _ => panic!("Valid axes are x and y only."),
                },
                last[last.trim().find('=').expect("Couldn't find '='.") + 1..]
                    .parse()
                    .expect("Couldn't parse to u64."),
            )),
        }
    }

    pub fn parse(input: &str) -> Result<(Vec<Point>, Vec<Fold>), ErrorKind> {
        let parsed_points: Vec<Point> = input
            .split('\n')
            .filter_map(|x| point(x).ok())
            .map(|(_, x)| x)
            .collect::<Vec<Point>>();

        let parsed_folds: Vec<Fold> = input
            .split('\n')
            .filter_map(|x| fold_ins(x).ok())
            .collect::<Vec<Fold>>();

        Ok((parsed_points, parsed_folds))
    }
}

mod part_1 {
    use super::*;

    pub fn solve(paper: &mut Paper, folds: &[Fold]) -> usize {
        let folded: Paper = paper.fold_(*folds.get(0).unwrap());
        folded.len()
    }
}

mod part_2 {
    use super::*;

    fn display(paper: &Paper) {
        let mut grid: [[char; 40]; 6] = [[' '; 40]; 6];

        for &(x, y) in paper {
            grid[y][x] = '#';
        }
        for row in grid {
            println!(
                "{}",
                row.iter()
                    .map(|&x| String::from(x))
                    .collect::<Vec<String>>()
                    .join(" ")
            );
        }
    }
    pub fn solve(paper: &Paper, folds: &[Fold]) {
        let mut result: Paper = paper.clone();
        for fold in folds {
            result = result.fold_(*fold);
        }
        display(&result);
    }
}

fn main() {
    let mut buffer: String = String::new();
    io::stdin()
        .read_to_string(&mut buffer)
        .expect("Could not read from stdin.");

    let (points, fold_ins) = parser::parse(&buffer).expect("Couldn't parse paper.");

    let mut paper: Paper = HashSet::new();

    points.iter().for_each(|x| {
        paper.insert(*x);
    });

    let result_part_1 = part_1::solve(&mut paper, &fold_ins);

    println!("Part 1: {:?}\nPart 2: \n", result_part_1);
    part_2::solve(&paper, &fold_ins);
}

#[cfg(test)]
mod tests {
    use super::{parser, part_1, part_2};
    use super::{Foldable, Paper, Point};
    use crate::{Axes, Fold};
    use std::collections::HashSet;

    fn setup() -> (Vec<Point>, Vec<Fold>) {
        let input: &str = "
            6,10
            0,14
            9,10
            0,3
            10,4
            4,11
            6,0
            6,12
            4,1
            0,13
            10,12
            3,4
            3,0
            8,4
            1,10
            2,14
            8,10
            9,0

            fold along y=7
            fold along x=5
        ";
        parser::parse(input).expect("Couldn't parse paper test.")
    }

    #[test]
    fn test_sample_1() {
        let (points, folds): (Vec<Point>, Vec<Fold>) = setup();

        let mut paper: Paper = HashSet::new();

        points.iter().for_each(|x| {
            paper.insert(*x);
        });

        let observed = part_1::solve(&mut paper, &folds);

        let expected: usize = 17;
        assert_eq!(observed, expected);
    }
}
