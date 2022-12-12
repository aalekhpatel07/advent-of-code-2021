use aoc_core::Solution;
mod packet;
mod parser_utils;
pub use parser_utils::*;

use crate::packet::{Packet, Parse};


pub struct Solver {}

impl Default for Solver {
    fn default() -> Self {
        Self {}
    }
}

impl Solution for Solver {
    fn part1(&self) -> String {
        let input = include_str!("input.txt");
        let bin = input.chars().map(|c| format!("{:04b}", c.to_digit(16).unwrap())).collect::<String>();
        let (_, packet) = Packet::parse(&bin).unwrap();
        let result = packet.sum_version_numbers();
        println!("Part 1: {:#?}", result);
        format!("{}", result)
    }

    fn part2(&self) -> String {
        let input = include_str!("input.txt");
        let bin = input.chars().map(|c| format!("{:04b}", c.to_digit(16).unwrap())).collect::<String>();
        let (_, packet) = Packet::parse(&bin).unwrap();

        format!("{}", packet.value())
    }
}


#[cfg(test)]
mod test {

}