use std::{ops::Add, fmt::Display};

use crate::{SnailFish, parse::Parse};




#[derive(Debug, Clone, Eq)]
pub struct Tree {
    pub(crate) inner: Vec<Option<usize>>,
}

pub type Node = (usize, Option<usize>);

impl From<&str> for Tree {
    fn from(s: &str) -> Self {
        SnailFish::parse(s).unwrap().1.into()
    }
}


impl PartialEq for Tree {
    fn eq(&self, other: &Tree) -> bool {
        self
        .inner
        .iter()
        .zip(
            other
            .inner
            .iter()
        )
        .all(|(&x, &y)| {
            match (x, y) {
                (Some(x), Some(y)) => x == y,
                (None, None) => true,
                _ => false
            }
        })
    }
}


impl Tree {
    pub fn new() -> Self {
        Tree {
            inner: vec![]
        }
    }
    pub fn len(&self) -> usize {
        self.inner.len()
    }
    pub fn get_depth(&self, index: usize) -> usize {
        let mut i = index;
        let mut depth = 0;
        while i > 0 {
            i = (i - 1) / 2;
            depth += 1;
        }
        depth
    }

    pub fn root(&self) -> Node {
        (0, *self.inner.get(0).unwrap())
    }

    pub fn set_left(&mut self, parent: usize, node: usize) {
        let i = parent * 2 + 1;
        if i >= self.inner.len() {
            self.inner.resize(i + 1, Default::default());
        }
        self.inner[i] = Some(node);
    }

    pub fn set_right(&mut self, parent: usize, node: usize) {
        let i = parent * 2 + 2;
        if i >= self.inner.len() {
            self.inner.resize(i + 1, Default::default());
        }
        self.inner[i] = Some(node);
    }

    pub fn set_data(&mut self, index: usize, data: usize) {
        if index >= self.inner.len() {
            self.inner.resize(index + 1, Default::default());
        }

        if let Some(node) = &mut self.inner[index] {
            *node = data;
        } else {
            self.inner[index] = Some(data);
        }
    }

    pub fn clear_data(&mut self, index: usize) {
        self.inner[index] = None;
    }

    pub fn at(&self, index: usize) -> Node {
        (index, *self.inner.get(index).unwrap())
    }

    pub fn left(&self, index: usize) -> Node {
        (2 * index + 1, *self.inner.get(2 * index + 1).unwrap())
    }
    pub fn is_root(&self, index: usize) -> bool {
        index == 0
    }
    pub fn has_left(&self, index: usize) -> bool {
        (2 * index + 1) < self.len()
    }
    pub fn has_right(&self, index: usize) -> bool {
        (2 * index + 2) < self.len()
    }
    pub fn right(&self, index: usize) -> Node {
        (2 * index + 2, *self.inner.get(2 * index + 2).unwrap())
    }
    pub fn parent(&self, index: usize) -> Node {
        if index <= 1 { 
            return (0, None);
        }
        ((index - 1) / 2, *self.inner.get((index - 1) / 2).unwrap())
    }

    pub fn nodes_at_depth(&self, depth: usize) -> impl Iterator<Item=Node> + '_ {
        let start = 2usize.pow(depth as u32) - 1;
        let end = 2usize.pow((depth + 1) as u32) - 1;
        self
        .inner
        .iter()
        .skip(start)
        .take(end - start)
        .enumerate()
        .map(
            move |(idx, x)|(start + idx, *x)
        )
    }

    pub fn position(&self, value: usize) -> Option<usize> {
        self.inner.iter().position(|x| x == &Some(value))
    }

    pub fn get_first_left_number_in_subtree_rooted_at(&self, root: usize) -> Option<usize> {
        let mut result = None;
        self.find_leftmost_regular_number_ge(root, 0, &mut result);
        result
    }


    pub fn get_first_right_number_in_subtree_rooted_at(&self, root: usize) -> Option<usize> {
        let mut result = None;
        self.find_rightmost_regular_number_ge(root, 0, &mut result);
        result
    }

    pub fn find_first_regular_number_to_the_left(&self, index: usize) -> Option<Node> {
        if self.is_root(index) {
            return None;
        }

        if self.is_left_child(index) {
            let (parent, _) = self.parent(index);
            return self.find_first_regular_number_to_the_left(parent)
        }

        // Now we know we're a right child. So we may have a left sibling.

        let (parent, _parent_value) = self.parent(index);
        let (left_sibling, _sibling_value) = self.left(parent);

        let mut result = None;
        self.find_rightmost_regular_number_ge(left_sibling, 0, &mut result);

        result.map(|idx| self.at(idx))
    }
    pub fn find_first_regular_number_to_the_right(&self, index: usize) -> Option<Node> {

        if self.is_root(index) {
            return None;
        }

        if self.is_right_child(index) {
            let (parent, _) = self.parent(index);
            return self.find_first_regular_number_to_the_right(parent)
        }

        // Now we know we're a left child. So we may have a right sibling.

        let (parent, _parent_value) = self.parent(index);
        let (right_sibling, _sibling_value) = self.right(parent);

        let mut result = None;
        self.find_leftmost_regular_number_ge(right_sibling, 0, &mut result);

        result.map(|idx| self.at(idx))
    }

    pub fn is_left_child(&self, index: usize) -> bool {
        let (parent_index, _) = self.parent(index);
        self.left(parent_index).0 == index
    }

    pub fn is_right_child(&self, index: usize) -> bool {
        let (parent_index, _) = self.parent(index);
        self.right(parent_index).0 == index
    }

    pub fn explode_parent(&mut self, parent: usize) {

        let (left_index, left_value) = self.left(parent);
        let (right_index, right_value) = self.right(parent);

        let first_left = self.find_first_regular_number_to_the_left(left_index);
        let first_right = self.find_first_regular_number_to_the_right(right_index);

        if let Some((first_left_index, first_left_value)) = first_left {
            self.set_data(first_left_index, first_left_value.unwrap() + left_value.unwrap());
        }
        if let Some((first_right_index, first_right_value)) = first_right {
            self.set_data(first_right_index, first_right_value.unwrap() + right_value.unwrap());
        }

        self.clear_data(left_index);
        self.clear_data(right_index);

        if self.is_left_child(parent) {
            self.set_left(self.parent(parent).0, 0);
        } else {
            self.set_right(self.parent(parent).0, 0);
        }

    }

    pub fn split(&mut self, index: usize) {
        let (_, node) = self.at(index);
        if node.is_none() {
            panic!("Cannot split a non-literal node.");
        }
        // Since its a literal, its not gonna have any children.
        // So we can just set the left and right to the new values.
        self.clear_data(index);

        self.set_left(index, node.unwrap() / 2);
        self.set_right(index, (node.unwrap() + 1) / 2);

    }

    pub fn inorder_traverse(&self, index: usize, result: &mut Vec<String>) {

        if self.has_left(index) && self.has_right(index) {
            if let Some(last) = result.last() {
                if last == "]" {
                    result.push(",".to_owned());
                }
            }
            result.push("[".to_owned());
        }

        if self.has_left(index) {
            self.inorder_traverse(self.left(index).0, result);
            if self.at(self.left(index).0).1.is_some() {
                if let Some(last) = result.last() {
                    if last == "]" {
                        result.push(",".to_owned());
                    }
                }
                result.push(self.at(self.left(index).0).1.unwrap().to_string());
                result.push(",".to_owned());
            }
        }

        if self.has_right(index) {
            self.inorder_traverse(self.right(index).0, result);
            if self.at(self.right(index).0).1.is_some() {
                // If just closed a pair, add a comma.
                if let Some(last) = result.last() {
                    if last == "]" {
                        result.push(",".to_owned());
                    }
                }
                result.push(self.at(self.right(index).0).1.unwrap().to_string());
            }
        }
        
        if self.has_left(index) && self.has_right(index) {
            // Empty parenthesis can be removed.
            if result.last().unwrap() == "[" {
                result.pop();
            } else {
                result.push("]".to_owned());
            }
        }
    }

    pub fn as_string(&self) -> String {
        let mut result = vec![];
        self.inorder_traverse(0, &mut result);
        result.join("").to_owned()
    }

    pub fn find_index_of_child_whose_parent_to_explode(&self) -> Option<usize> {
        // We'll search from the left by default so the first
        // node we find at at the 5th depth is the node whose
        // parent we want to explode.
        self
        .nodes_at_depth(5)
        .filter(|(_, node_value)| {
            node_value.is_some()
        })
        .map(|(node_index, _)| node_index)
        .take(1)
        .next()
    }



    fn find_leftmost_regular_number_ge(&self, root: usize, geq: usize, result: &mut Option<usize>) -> Option<usize> {
        
        if result.is_some() {
            return *result
        }

        if self.has_left(root) {
            let (index, value) = self.left(root);
            if value.is_some() && value.unwrap() >= geq {
                *result = Some(index);
                return *result;
            }
            self.find_leftmost_regular_number_ge(index, geq, result);
        }

        // Check ourselves, in case we're a literal.
        let (index, value) = self.at(root);
        if value.is_some() && value.unwrap() >= geq {
            *result = Some(index);
            return *result;
        }

        if self.has_right(root) {
            let (index, value) = self.right(root);
            if value.is_some() && value.unwrap() >= geq {
                *result = Some(index);
                return *result;
            }
            self.find_leftmost_regular_number_ge(index, geq, result);
        }

        return None
    }


    fn find_rightmost_regular_number_ge(&self, root: usize, geq: usize, result: &mut Option<usize>) -> Option<usize> {
        
        if result.is_some() {
            return *result
        }

        if self.has_right(root) {
            let (index, value) = self.right(root);
            if value.is_some() && value.unwrap() >= geq {
                *result = Some(index);
                return *result;
            }
            self.find_rightmost_regular_number_ge(index, geq, result);
        }

        // Check ourselves, in case we're a literal.
        let (index, value) = self.at(root);
        if value.is_some() && value.unwrap() >= geq {
            *result = Some(index);
            return *result;
        }

        if self.has_left(root) {
            let (index, value) = self.left(root);
            if value.is_some() && value.unwrap() >= geq {
                *result = Some(index);
                return *result;
            }
            self.find_rightmost_regular_number_ge(index, geq, result);
        }


        return None
    }


    pub fn reduce(&mut self) -> bool {
        if let Some(index) = self.find_index_of_child_whose_parent_to_explode() {
            self.explode_parent(self.parent(index).0);
            true
        } else {
            let mut first_large_number = None;
            self.find_leftmost_regular_number_ge(0, 10, &mut first_large_number);
            if let Some(index) = first_large_number {
                self.split(index);
                true
            } else {
                false
            }
        }
    }

    pub fn reduce_all_the_way(&mut self) {
        // let mut current = self.clone();
        let mut previous = None;
        loop {
            self.reduce();
            if let Some(prev) = previous {
                if prev == *self {
                    break;
                } else {
                    previous = Some(self.clone());
                }
            } else {
                previous = Some(self.clone());
            }
        }
    }


    pub fn magnitude(&self, root: usize) -> usize {
        let mut result: usize = 0;
        
        let (_, current_value) = self.at(root);
        if let Some(value) = current_value {
            result += value;
            return result;
        }

        if self.has_left(root) {
            let (index, _) = self.left(root);
            result += 3 * self.magnitude(index);
        }

        if self.has_right(root) {
            let (index, _) = self.right(root);
            result += 2 * self.magnitude(index);
        }

        result

    }

}


impl Add for Tree {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        // It's kinda expensive (i.e. O(n) space+time)
        // because of the linear representation of the binary tree
        // so might as well bring in the existing parser setup.
        let as_str = format!("[{},{}]", self.as_string(), rhs.as_string());
        let mut initial: Tree = SnailFish::parse(&as_str).unwrap().1.into();
        initial.reduce_all_the_way();
        initial
    }
}

impl Display for Tree {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.as_string())
    }
}


impl From<SnailFish> for Tree {
    fn from(snailfish: SnailFish) -> Self {
        let mut tree = Tree::new();
        let mut stack = vec![(0, snailfish)];
        while let Some((i, snailfish)) = stack.pop() {
            match snailfish {
                SnailFish::Literal(l) => {
                    tree.set_data(i, l);
                },
                SnailFish::Pair(p) => {
                    stack.push((i * 2 + 1, p.left));
                    stack.push((i * 2 + 2, p.right));
                }
            }
        }
        tree
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;


    #[test]
    fn it_works() {
        let raw = "[[[[[9,8],1],2],3],4]";
        let (_, p) = SnailFish::parse(raw).unwrap();
        let mut tree: Tree = p.into();
        println!("{:#?}", tree.nodes_at_depth(5).collect::<Vec<_>>());
        tree.explode_parent((32 - 1) / 2);
        println!("{:#?}", tree.nodes_at_depth(4).collect::<Vec<_>>());
    }

    #[test_case("[[[[[9,8],1],2],3],4]", (32 - 1) / 2, "[[[[0,9],2],3],4]")]
    #[test_case("[[[[[9,8],1],2],3],4]", (32 - 2) / 2, "[[[[0,9],2],3],4]")]
    #[test_case("[7,[6,[5,[4,[3,2]]]]]", (62 - 1) / 2, "[7,[6,[5,[7,0]]]]")]
    #[test_case("[7,[6,[5,[4,[3,2]]]]]", (62 - 2) / 2, "[7,[6,[5,[7,0]]]]")]
    fn test_explode(starting: &str, parent_index: usize, expected: &str) {
        let mut tree: Tree = SnailFish::parse(starting).unwrap().1.into();
        tree.explode_parent(parent_index);
        let result = tree.as_string();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_explode1() {
        let raw = "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]";
        let mut tree: Tree = SnailFish::parse(raw).unwrap().1.into();
        tree.explode_parent(22);
        // tree.reduce();
        // println!("{}", tree);
        assert_eq!(tree.as_string(), "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]".to_owned());
    }
        

    #[test]
    fn test_string() {
        let raw = "[[6,[5,[7,0]]],3]";
        let tree: Tree = SnailFish::parse(raw).unwrap().1.into();
        assert_eq!(&tree.as_string(), raw);
    }

    #[test_case("[[[[0,7],4],[15,[0,13]]],[1,1]]", 15, "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]")]
    #[test_case("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]", 13, "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]")]
    fn test_split(initial: &str, value: usize, expected: &str) {
        let mut tree: Tree = SnailFish::parse(initial).unwrap().1.into();
        let index = tree.position(value).unwrap();
        tree.split(index);
        assert_eq!(tree.as_string(), expected);
    }

    #[test]
    fn test_reduce_all_the_way() {
        let raw: &str = "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]";
        let mut tree: Tree = raw.into();
        tree.reduce_all_the_way();
        let expected: Tree = "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]".into();
        assert_eq!(tree, expected);
    }

    #[test_case("[1,5]", 3 * 1 + 2 * 5)]
    #[test_case("[1,1]", 3 * 1 + 2 * 1)]
    #[test_case("[1,[1,2]]", 3 * 1 + 2 * (3 * 1 + 2 * 2))]
    #[test_case("[[1,2],[[3,4],5]]", 143)]
    #[test_case("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", 1384)]
    #[test_case("[[[[1,1],[2,2]],[3,3]],[4,4]]", 445)]
    #[test_case("[[[[3,0],[5,3]],[4,4]],[5,5]]", 791)]
    #[test_case("[[[[5,0],[7,4]],[5,5]],[6,6]]", 1137)]
    #[test_case("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]", 3488)]
    fn test_magnitude(raw: &str, expected: usize) {
        let tree = Tree::from(raw);
        assert_eq!(tree.magnitude(0), expected);
    }
}