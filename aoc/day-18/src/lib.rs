mod parse;
mod tree;

// use aoc_core::Solution;
use tree::*;


#[derive(Debug, PartialEq, Eq)]
pub enum SnailFish {
    Pair(Box<Pair>),
    Literal(usize)
}

#[derive(Debug, PartialEq, Eq)]
pub struct Pair {
    pub left: SnailFish,
    pub right: SnailFish,
}

#[derive(Default, Debug)]
pub struct Solver {}



// impl Solution for Solver {
//     fn part1(&self) -> String {
//         "".to_owned()   
//     }
//     fn part2(&self) -> String {
//         "".to_owned()
//     }
// }