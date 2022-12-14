mod parse;
mod tree;

use aoc_core::Solution;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use tree::*;

#[derive(Debug, PartialEq, Eq)]
pub enum SnailFish {
    Pair(Box<Pair>),
    Literal(usize),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Pair {
    pub left: SnailFish,
    pub right: SnailFish,
}

#[derive(Default, Debug)]
pub struct Solver {}

pub fn build_trees(s: &str) -> Vec<Tree> {
    s.lines().map(Tree::from).collect()
}

/// Add the given list of snailfish numbers while reducing
/// them as many times as required. Return the magnitude of
/// the final snailfish number that stands.
pub fn solve_part1(s: &str) -> usize {
    let trees = build_trees(s);
    let mut acc_tree = trees.first().unwrap().clone();
    trees.iter().skip(1).for_each(|tree| {
        acc_tree.add(tree);
    });
    acc_tree.magnitude(0)
}

/// Find the maximum magnitude amongst any sum of two snailfish numbers in the given input.
pub fn solve_part2(s: &str) -> usize {
    let trees = build_trees(s);
    let mut tree_pair: Vec<(Tree, Tree)> = vec![];
    for x in 0..trees.len() {
        for y in 0..trees.len() {
            if x == y {
                continue;
            }
            tree_pair.push((trees.get(x).unwrap().clone(), trees.get(y).unwrap().clone()));
        }
    }

    tree_pair
        // rayon goes brrrr.
        .par_iter()
        .map(|(t1, t2)| {
            let mut acc_tree = t1.clone();
            acc_tree.add(t2);
            acc_tree.magnitude(0)
        })
        .max()
        .unwrap()
}

impl Solution for Solver {
    fn part1(&self) -> String {
        solve_part1(include_str!("input.txt")).to_string()
    }
    fn part2(&self) -> String {
        solve_part2(include_str!("input.txt")).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(
        "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]",
        4140
    )]
    #[test_case(
        "[[[[2,8],[4,6]],[[2,4],[9,4]]],[[[0,6],[4,6]],[1,6]]]
[7,[[5,7],1]]
[[[[8,8],7],5],[[[5,6],1],6]]
[[[8,5],[[0,0],[4,9]]],[2,8]]
[7,[[5,2],[[3,0],[7,7]]]]
[[6,[6,8]],[3,[5,2]]]
[6,[[[8,9],[9,9]],[3,8]]]
[[[1,[0,2]],[7,[3,0]]],8]
[[9,6],6]
[[[2,3],1],[9,[3,7]]]
[5,[[[5,8],3],9]]
[[[[8,8],3],[2,2]],[2,3]]
[[[4,9],3],[[[7,3],8],5]]
[[[3,5],[3,7]],[[[9,7],9],[9,[7,8]]]]
[[7,1],8]
[0,[[[6,8],[1,1]],[1,[5,8]]]]
[[[[2,2],[9,5]],[0,[1,0]]],[4,[[2,4],4]]]
[[[[2,5],[7,3]],[7,6]],[[6,[4,4]],[3,8]]]
[[3,[[7,9],2]],[[0,[4,4]],[[6,9],9]]]
[[[7,7],[[1,4],[1,6]]],[7,[[6,3],6]]]
[[0,8],[[[1,6],2],4]]
[[0,[[2,7],[0,4]]],[[[3,8],[7,7]],5]]
[[[[9,9],[1,3]],[9,[4,3]]],[[[3,4],[6,4]],1]]
[[[9,[0,9]],[2,[7,6]]],[2,[[1,9],[3,3]]]]
[[4,[5,6]],[[[1,5],6],[[1,5],[5,2]]]]
[1,[[3,[2,1]],5]]
[[4,[3,8]],[3,[6,3]]]
[[7,1],[[3,[6,0]],[5,[1,1]]]]
[[8,7],[[[0,1],[2,6]],[5,[4,7]]]]
[9,[[[1,6],[8,9]],[6,6]]]
[4,9]
[[[[0,8],[8,5]],9],[7,[1,3]]]
[[[[8,5],0],[[4,6],4]],[8,4]]
[[[[8,9],8],[[3,1],[7,6]]],2]
[[[[6,3],0],[2,[4,8]]],[[[0,3],[3,5]],4]]
[0,[[9,[0,6]],5]]
[[[[1,9],[2,7]],[[4,0],[9,9]]],[[8,[3,6]],[3,4]]]
[[[[0,7],[8,4]],1],[[8,3],[[3,5],[8,0]]]]
[[[[3,5],4],[0,9]],[[[1,7],5],[9,[8,0]]]]
[[[8,[6,8]],[[3,7],[0,8]]],[[[5,2],[1,7]],[9,5]]]
[[[[5,1],[0,7]],4],[0,4]]
[[[[9,8],[3,9]],[[0,6],3]],[[[9,1],[8,7]],2]]
[[9,[[0,3],6]],[[3,4],[[8,9],5]]]
[[1,[1,8]],[[6,[4,2]],1]]
[7,[[1,[5,2]],[[9,7],0]]]
[0,[8,6]]
[1,4]
[[8,[4,1]],[[[4,0],[0,0]],[7,[3,4]]]]
[2,[[1,[1,8]],[[3,4],1]]]
[[8,[[1,2],[3,1]]],[[[4,4],[7,9]],1]]
[[4,[0,[6,4]]],[9,[0,[1,2]]]]
[[6,[3,1]],[[7,8],[8,[2,5]]]]
[[[2,[3,3]],[[6,4],[9,4]]],[[[1,5],[7,4]],[0,6]]]
[[[[8,0],3],[[4,0],3]],[[7,5],4]]
[[[2,[4,3]],[[2,1],5]],1]
[[[8,1],[0,4]],[9,[[1,4],[9,0]]]]
[[[5,0],[[7,7],9]],[[6,[6,2]],7]]
[[[[5,9],0],[[4,6],[3,8]]],[6,[6,5]]]
[[[6,[7,8]],[5,3]],[[3,[6,5]],[[8,7],[4,7]]]]
[[9,[[8,7],4]],[[[6,3],0],[[2,3],[5,9]]]]
[[[[1,8],6],1],[[[7,8],4],[7,2]]]
[[[[7,1],[6,2]],[[7,8],2]],0]
[[[4,5],[0,3]],[[2,4],1]]
[[[9,1],7],[[[8,8],[0,7]],[8,0]]]
[[5,[[7,5],[7,5]]],[3,[4,8]]]
[[7,[1,0]],[[3,[1,5]],0]]
[[[5,1],[[5,2],[7,3]]],[[7,[3,9]],9]]
[5,[1,[[9,9],[3,0]]]]
[[2,0],[9,[6,[3,3]]]]
[[[[0,4],[4,8]],[[1,9],[5,8]]],[[[7,0],5],[5,1]]]
[[[[1,5],[9,2]],[6,[3,6]]],[4,[1,[1,5]]]]
[[[[1,4],[4,6]],[[5,5],[3,5]]],[[[7,1],4],[[0,7],4]]]
[[6,[3,5]],1]
[8,[[1,[0,7]],[[2,5],6]]]
[[[[1,6],3],[[9,7],9]],[[7,8],3]]
[[[[9,9],[2,0]],0],[1,4]]
[[[[1,3],[5,1]],[[0,4],2]],0]
[[3,2],[7,[[9,3],8]]]
[[9,0],[4,[[8,7],[5,5]]]]
[[[[7,4],8],[[4,4],1]],9]
[[9,[[7,9],1]],[[[6,5],7],[[2,5],2]]]
[7,2]
[[[6,6],[[9,4],4]],6]
[[1,[[5,0],3]],[5,[4,4]]]
[[[3,2],[[4,6],6]],[[3,[9,5]],[[0,2],[4,6]]]]
[5,[[0,[3,0]],[7,[7,9]]]]
[[[[0,4],[1,5]],4],[8,[[4,7],8]]]
[[[[9,1],0],0],4]
[[[[8,4],[4,2]],[9,[1,7]]],[6,3]]
[2,[[[8,3],2],[[3,1],8]]]
[[[[9,0],[7,8]],[[2,7],[0,3]]],[[[8,5],3],[9,[6,8]]]]
[[[[8,9],[9,1]],[4,[0,1]]],[[[7,8],2],2]]
[[[[2,2],[4,1]],[2,[2,8]]],[[[6,5],1],9]]
[[[[3,0],7],7],[[[9,3],7],4]]
[[[[7,5],1],3],[[[0,7],7],[[2,6],[9,9]]]]
[[[[5,2],8],[9,[8,8]]],[2,[[0,8],[5,6]]]]
[[[[7,7],[1,2]],[6,6]],[8,[5,8]]]
[[7,[4,[8,9]]],[[4,[7,2]],8]]
[[[6,4],[7,7]],[[[3,7],0],[0,1]]]
[[1,[5,9]],[8,[4,6]]]",
        4347
    )]
    fn test_part1(raw: &str, final_sum: usize) {
        let solution = solve_part1(raw);
        assert_eq!(solution, final_sum);
    }

    #[test_case(
        "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]",
        3993
    )]
    #[test_case(
        "[[[[2,8],[4,6]],[[2,4],[9,4]]],[[[0,6],[4,6]],[1,6]]]
[7,[[5,7],1]]
[[[[8,8],7],5],[[[5,6],1],6]]
[[[8,5],[[0,0],[4,9]]],[2,8]]
[7,[[5,2],[[3,0],[7,7]]]]
[[6,[6,8]],[3,[5,2]]]
[6,[[[8,9],[9,9]],[3,8]]]
[[[1,[0,2]],[7,[3,0]]],8]
[[9,6],6]
[[[2,3],1],[9,[3,7]]]
[5,[[[5,8],3],9]]
[[[[8,8],3],[2,2]],[2,3]]
[[[4,9],3],[[[7,3],8],5]]
[[[3,5],[3,7]],[[[9,7],9],[9,[7,8]]]]
[[7,1],8]
[0,[[[6,8],[1,1]],[1,[5,8]]]]
[[[[2,2],[9,5]],[0,[1,0]]],[4,[[2,4],4]]]
[[[[2,5],[7,3]],[7,6]],[[6,[4,4]],[3,8]]]
[[3,[[7,9],2]],[[0,[4,4]],[[6,9],9]]]
[[[7,7],[[1,4],[1,6]]],[7,[[6,3],6]]]
[[0,8],[[[1,6],2],4]]
[[0,[[2,7],[0,4]]],[[[3,8],[7,7]],5]]
[[[[9,9],[1,3]],[9,[4,3]]],[[[3,4],[6,4]],1]]
[[[9,[0,9]],[2,[7,6]]],[2,[[1,9],[3,3]]]]
[[4,[5,6]],[[[1,5],6],[[1,5],[5,2]]]]
[1,[[3,[2,1]],5]]
[[4,[3,8]],[3,[6,3]]]
[[7,1],[[3,[6,0]],[5,[1,1]]]]
[[8,7],[[[0,1],[2,6]],[5,[4,7]]]]
[9,[[[1,6],[8,9]],[6,6]]]
[4,9]
[[[[0,8],[8,5]],9],[7,[1,3]]]
[[[[8,5],0],[[4,6],4]],[8,4]]
[[[[8,9],8],[[3,1],[7,6]]],2]
[[[[6,3],0],[2,[4,8]]],[[[0,3],[3,5]],4]]
[0,[[9,[0,6]],5]]
[[[[1,9],[2,7]],[[4,0],[9,9]]],[[8,[3,6]],[3,4]]]
[[[[0,7],[8,4]],1],[[8,3],[[3,5],[8,0]]]]
[[[[3,5],4],[0,9]],[[[1,7],5],[9,[8,0]]]]
[[[8,[6,8]],[[3,7],[0,8]]],[[[5,2],[1,7]],[9,5]]]
[[[[5,1],[0,7]],4],[0,4]]
[[[[9,8],[3,9]],[[0,6],3]],[[[9,1],[8,7]],2]]
[[9,[[0,3],6]],[[3,4],[[8,9],5]]]
[[1,[1,8]],[[6,[4,2]],1]]
[7,[[1,[5,2]],[[9,7],0]]]
[0,[8,6]]
[1,4]
[[8,[4,1]],[[[4,0],[0,0]],[7,[3,4]]]]
[2,[[1,[1,8]],[[3,4],1]]]
[[8,[[1,2],[3,1]]],[[[4,4],[7,9]],1]]
[[4,[0,[6,4]]],[9,[0,[1,2]]]]
[[6,[3,1]],[[7,8],[8,[2,5]]]]
[[[2,[3,3]],[[6,4],[9,4]]],[[[1,5],[7,4]],[0,6]]]
[[[[8,0],3],[[4,0],3]],[[7,5],4]]
[[[2,[4,3]],[[2,1],5]],1]
[[[8,1],[0,4]],[9,[[1,4],[9,0]]]]
[[[5,0],[[7,7],9]],[[6,[6,2]],7]]
[[[[5,9],0],[[4,6],[3,8]]],[6,[6,5]]]
[[[6,[7,8]],[5,3]],[[3,[6,5]],[[8,7],[4,7]]]]
[[9,[[8,7],4]],[[[6,3],0],[[2,3],[5,9]]]]
[[[[1,8],6],1],[[[7,8],4],[7,2]]]
[[[[7,1],[6,2]],[[7,8],2]],0]
[[[4,5],[0,3]],[[2,4],1]]
[[[9,1],7],[[[8,8],[0,7]],[8,0]]]
[[5,[[7,5],[7,5]]],[3,[4,8]]]
[[7,[1,0]],[[3,[1,5]],0]]
[[[5,1],[[5,2],[7,3]]],[[7,[3,9]],9]]
[5,[1,[[9,9],[3,0]]]]
[[2,0],[9,[6,[3,3]]]]
[[[[0,4],[4,8]],[[1,9],[5,8]]],[[[7,0],5],[5,1]]]
[[[[1,5],[9,2]],[6,[3,6]]],[4,[1,[1,5]]]]
[[[[1,4],[4,6]],[[5,5],[3,5]]],[[[7,1],4],[[0,7],4]]]
[[6,[3,5]],1]
[8,[[1,[0,7]],[[2,5],6]]]
[[[[1,6],3],[[9,7],9]],[[7,8],3]]
[[[[9,9],[2,0]],0],[1,4]]
[[[[1,3],[5,1]],[[0,4],2]],0]
[[3,2],[7,[[9,3],8]]]
[[9,0],[4,[[8,7],[5,5]]]]
[[[[7,4],8],[[4,4],1]],9]
[[9,[[7,9],1]],[[[6,5],7],[[2,5],2]]]
[7,2]
[[[6,6],[[9,4],4]],6]
[[1,[[5,0],3]],[5,[4,4]]]
[[[3,2],[[4,6],6]],[[3,[9,5]],[[0,2],[4,6]]]]
[5,[[0,[3,0]],[7,[7,9]]]]
[[[[0,4],[1,5]],4],[8,[[4,7],8]]]
[[[[9,1],0],0],4]
[[[[8,4],[4,2]],[9,[1,7]]],[6,3]]
[2,[[[8,3],2],[[3,1],8]]]
[[[[9,0],[7,8]],[[2,7],[0,3]]],[[[8,5],3],[9,[6,8]]]]
[[[[8,9],[9,1]],[4,[0,1]]],[[[7,8],2],2]]
[[[[2,2],[4,1]],[2,[2,8]]],[[[6,5],1],9]]
[[[[3,0],7],7],[[[9,3],7],4]]
[[[[7,5],1],3],[[[0,7],7],[[2,6],[9,9]]]]
[[[[5,2],8],[9,[8,8]]],[2,[[0,8],[5,6]]]]
[[[[7,7],[1,2]],[6,6]],[8,[5,8]]]
[[7,[4,[8,9]]],[[4,[7,2]],8]]
[[[6,4],[7,7]],[[[3,7],0],[0,1]]]
[[1,[5,9]],[8,[4,6]]]",
        4721
    )]
    fn test_part2(raw: &str, highest_magnitude: usize) {
        let solution = solve_part2(raw);
        assert_eq!(solution, highest_magnitude);
    }

    #[test_case(
        "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]",
        "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]
[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]
[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]
[[[[7,7],[7,8]],[[9,5],[8,7]]],[[[6,8],[0,8]],[[9,9],[9,0]]]]
[[[[6,6],[6,6]],[[6,0],[6,7]]],[[[7,7],[8,9]],[8,[8,1]]]]
[[[[6,6],[7,7]],[[0,7],[7,7]]],[[[5,5],[5,6]],9]]
[[[[7,8],[6,7]],[[6,8],[0,8]]],[[[7,7],[5,0]],[[5,5],[5,6]]]]
[[[[7,7],[7,7]],[[8,7],[8,7]]],[[[7,0],[7,7]],9]]
[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
        "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
        3488
    )]
    fn test_list_sum_reduction(
        homework_assignment: &str,
        intermediate_sums: &str,
        final_sum: &str,
        magnitude: usize,
    ) {
        let trees = build_trees(homework_assignment);
        let mut acc_tree = trees.first().unwrap().clone();
        trees.iter().skip(1).enumerate().for_each(|(index, tree)| {
            acc_tree.add(tree);
            let expected_tree: Tree = intermediate_sums.lines().nth(index).unwrap().into();
            assert_eq!(acc_tree.as_string(), expected_tree.as_string());
        });
        assert_eq!(acc_tree.as_string(), final_sum);
        assert_eq!(acc_tree.magnitude(0), magnitude);
    }
}
