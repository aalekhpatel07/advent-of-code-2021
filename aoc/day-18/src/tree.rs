use std::{fmt::Display, str::FromStr};
use crate::{SnailFish, parse::Parse};


#[derive(Clone, Eq)]
pub struct Tree {
    pub(crate) inner: Vec<Option<usize>>,
}

pub type Node = (usize, Option<usize>);

impl From<&str> for Tree {
    fn from(s: &str) -> Self {
        Tree::from_str(s).unwrap()
    }
}

impl FromStr for Tree {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        SnailFish::parse(s)
        .map(|(_, t)| t.into())
        .map_err(|e| e.to_string())
    }
}

/// Call two trees equal if the regular numbers they store at the leaves
/// are all equal.
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

impl Default for Tree {
    fn default() -> Self {
        Self::new()
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

    /// Set the left child of the `parent` node at the given index to `node`.
    pub fn set_left(&mut self, parent: usize, node: usize) {
        let i = parent * 2 + 1;
        if i >= self.inner.len() {
            self.inner.resize(i + 1, Default::default());
        }
        self.inner[i] = Some(node);
    }

    /// Set the right child of the `parent` node at the given index to `node`.
    pub fn set_right(&mut self, parent: usize, node: usize) {
        let i = parent * 2 + 2;
        if i >= self.inner.len() {
            self.inner.resize(i + 1, Default::default());
        }
        self.inner[i] = Some(node);
    }

    /// Given an index for a node, and a regular number,
    /// set the node at the index to the regular number.
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

    /// Mark the node at the index as having no regular number.
    pub fn clear_data(&mut self, index: usize) {
        self.inner[index] = None;
    }

    /// Get the node at the given index.
    pub fn at(&self, index: usize) -> Node {
        (index, *self.inner.get(index).unwrap())
    }

    /// Get the left child of the node at the given index.
    pub fn left(&self, index: usize) -> Node {
        (2 * index + 1, *self.inner.get(2 * index + 1).unwrap())
    }
    /// Returns true if the node at the given index is root, false otherwise.
    pub fn is_root(&self, index: usize) -> bool {
        index == 0
    }
    /// Returns true if the node at the given index has a left child, false otherwise.
    pub fn has_left(&self, index: usize) -> bool {
        (2 * index + 1) < self.len()
    }
    /// Returns true if the node at the given index has a right child, false otherwise.
    pub fn has_right(&self, index: usize) -> bool {
        (2 * index + 2) < self.len()
    }
    /// Get the right child of the node at the given index.
    pub fn right(&self, index: usize) -> Node {
        (2 * index + 2, *self.inner.get(2 * index + 2).unwrap())
    }
    /// Get the parent of the node at the given index.
    pub fn parent(&self, index: usize) -> Node {
        if index <= 1 { 
            return (0, None);
        }
        ((index - 1) / 2, *self.inner.get((index - 1) / 2).unwrap())
    }

    /// Get an iterator over the nodes living at the given depth.
    pub fn nodes_at_depth(&self, depth: usize) -> impl Iterator<Item=Node> + '_ {
        let start = 2usize.pow(depth as u32) - 1;
        let end = 2usize.pow((depth + 1) as u32) - 1;
        self
        .inner
        .iter()
        .skip(start)
        .take(end - start + 1)
        .enumerate()
        .map(
            move |(idx, x)|(start + idx, *x)
        )
    }


    /// Given a node's position in the tree, find the first regular number to the left of it.
    /// This is done by traversing up the tree while we're a left sibling and then finally if
    /// and when we become a right sibling, we ask the parent to give the right-most regular number in the sub-tree
    /// rooted at our left sibling.
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

    /// Given a node's position in the tree, find the first regular number to the right of it.
    /// This is done by traversing up the tree while we're a right sibling and then finally if
    /// and when we become a left sibling, we ask the parent to give the left-most regular number in the sub-tree
    /// rooted at our right sibling.
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

    /// Returns true if the node at `index` is a left child of its parent node, false otherwise.
    pub fn is_left_child(&self, index: usize) -> bool {
        let (parent_index, _) = self.parent(index);
        self.left(parent_index).0 == index
    }

    /// Returns true if the node at `index` is a right child of its parent node, false otherwise.
    pub fn is_right_child(&self, index: usize) -> bool {
        let (parent_index, _) = self.parent(index);
        self.right(parent_index).0 == index
    }

    /// Given the index of the parent to explode (i.e. the index of the pair node to explode),
    /// send the left number to the right-most regular number to the left it and send the right
    /// number to the left-most regular number to the right of it, if they exist. Finally, 
    /// replace the whole pair with a `0`.
    pub fn explode_parent(&mut self, parent: usize) {

        let (left_index, left_value) = self.left(parent);
        let (right_index, right_value) = self.right(parent);

        if left_value.is_none() || right_value.is_none() {
            panic!("Tried to explode a parent with non-regular entries.")
        }

        let first_left = self.find_first_regular_number_to_the_left(left_index);
        let first_right = self.find_first_regular_number_to_the_right(right_index);

        // If right-most regular number to the left exists, update its value.
        if let Some((first_left_index, first_left_value)) = first_left {
            self.set_data(first_left_index, first_left_value.unwrap() + left_value.unwrap());
        }
        // If left-most regular number to the right exists, update its value.
        if let Some((first_right_index, first_right_value)) = first_right {
            self.set_data(first_right_index, first_right_value.unwrap() + right_value.unwrap());
        }

        // Erase the nodes inside this pair.
        self.clear_data(left_index);
        self.clear_data(right_index);

        // Ask the pair's parent about which child to set to 0.
        if self.is_left_child(parent) {
            self.set_left(self.parent(parent).0, 0);
        } else {
            self.set_right(self.parent(parent).0, 0);
        }

    }

    /// Given the index of a regular node, split it into a pair of regular nodes
    /// with the numbers being half of the original number rounded down and up,
    /// respectively.
    pub fn split(&mut self, index: usize) {
        let (node_index, node) = self.at(index);
        if node.is_none() {
            panic!("Tried to split a irregular node at : {}", node_index);
        }

        // We're about to become a pair so clear our `Some`-ness.
        self.clear_data(index);

        // Since its a regular node, its not gonna have any children.
        // and we can just set the left and right to the new values.
        self.set_left(index, node.unwrap() / 2);
        self.set_right(index, (node.unwrap() + 1) / 2);

    }

   /// Iterate over the tree in-order and build a vector representing
   /// the bracket string representation of the tree.
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


    /// Build the bracket representation of the tree
    /// to help with debugging.
    pub fn as_string(&self) -> String {
        let mut result = vec![];
        self.inorder_traverse(0, &mut result);
        result.join("")
    }

    pub fn find_index_of_child_whose_parent_to_explode(&self) -> Option<usize> {
        // We'll search from the left by default so the first
        // node we find at at the 5th depth (i.e. inside 4 brackets) is the node whose
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

    /// Find the index of the left-most regular number in a sub-tree rooted at `root`
    /// that is greater than or equal to `geq`.
    /// 
    /// If no such number exists, set the result to `None`.
    pub fn find_leftmost_regular_number_ge(&self, root: usize, geq: usize, result: &mut Option<usize>) {
        
        if result.is_some() {
            return;
        }

        if self.has_left(root) {
            let (index, value) = self.left(root);
            if value.is_some() && value.unwrap() >= geq {
                *result = Some(index);
            } else {
                self.find_leftmost_regular_number_ge(index, geq, result);
            }
        }

        if result.is_some() {
            return;
        }

        // Check ourselves, in case we're a literal.
        let (index, value) = self.at(root);
        if value.is_some() && value.unwrap() >= geq {
            *result = Some(index);
        }

        if self.has_right(root) {
            let (index, value) = self.right(root);
            if value.is_some() && value.unwrap() >= geq {
                *result = Some(index);
            } else {
                self.find_leftmost_regular_number_ge(index, geq, result);
            }
        }
    }


    /// Find the index of the right-most regular number in a sub-tree rooted at `root`
    /// that is greater than or equal to `geq`.
    /// 
    /// If no such number exists, set the result to `None`.
    pub fn find_rightmost_regular_number_ge(&self, root: usize, geq: usize, result: &mut Option<usize>) {
        
        if result.is_some() {
            return
        }

        if self.has_right(root) {
            let (index, value) = self.right(root);
            if value.is_some() && value.unwrap() >= geq {
                *result = Some(index);
            }
            else {
                self.find_rightmost_regular_number_ge(index, geq, result);
            }
        }

        if result.is_some() {
            return
        }

        // Check ourselves, in case we're a literal.
        let (index, value) = self.at(root);
        if value.is_some() && value.unwrap() >= geq {
            *result = Some(index);
        }

        if self.has_left(root) {
            let (index, value) = self.left(root);
            if value.is_some() && value.unwrap() >= geq {
                *result = Some(index);
            } else {
                self.find_rightmost_regular_number_ge(index, geq, result);
            }
        }
    }

    /// If we can find an pair 4 levels deep, explode it. Otherwise if we can find a number we can split,
    /// split it. Otherwise, we're done. If we didn't do either, return False, otherwise return True.
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

    /// Repeatedly reduce until we can't any more.
    pub fn reduce_all_the_way(&mut self) {
        loop {
            if !self.reduce() {
                break
            }
        }
    }

    /// Get the magnitude of a snailfish number,
    /// i.e. for pairs it is the 3 * magnitude(left) + 2 * magnitude(right)
    /// and for literals it is the literal value.
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

    /// Given another tree, add that to us, mutating
    /// ourselves in the process. Then reduce ourselves all the way.
    pub fn add(&mut self, rhs: &Self) {

        // It's kinda expensive (i.e. O(n) time + roughly O(2 ** n) space) anyways
        // because of the linear representation of the binary tree
        // so might as well bring in the existing parser setup.

        // I'd guess a better way to implement this Tree would be a HashMap<usize, usize>
        // where we can treat absence of keys as the absence of nodes and nodes are encoded
        // the same way as this linear representation of binary tree (i.e. left = 2 * parent + 1, right = 2 * parent + 2).
        let as_str = format!("[{},{}]", self.as_string(), rhs.as_string());
        let mut initial: Tree = Tree::from(as_str.as_str());

        initial.reduce_all_the_way();
        self.inner = initial.inner;
    }

}


impl Display for Tree {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.as_string())
    }
}


impl From<SnailFish> for Tree {
    /// Let's build a tree from the snailfish representation
    /// by doing a depth-first traversal of the snailfish representation.
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
        

    #[test_case("[[6,[5,[7,0]]],3]")]
    #[test_case("[[[[7,8],[6,7]],[[6,8],[0,8]]],[[[7,7],[5,0]],[[5,5],[5,6]]]]")]
    #[test_case("[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]")]
    #[test_case("[[[5,[2,8]],4],[5,[[9,9],0]]]")]
    #[test_case("[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]")]
    #[test_case("[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]")]
    #[test_case("[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]")]
    #[test_case("[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]")]
    #[test_case("[[[[5,4],[7,7]],8],[[8,3],8]]")]
    #[test_case("[[9,3],[[9,9],[6,[4,9]]]]")]
    #[test_case("[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]")]
    #[test_case("[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]")]
    #[test_case("[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]")]
    #[test_case("[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]")]
    fn test_string(raw: &str) {
        let tree: Tree = raw.into();
        assert_eq!(&tree.as_string(), raw);
    }

    #[test_case("[[[[0,7],4],[15,[0,13]]],[1,1]]", "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]")]
    #[test_case("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]", "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]")]
    fn test_split(initial: &str, expected: &str) {
        let mut tree: Tree = initial.into();
        tree.reduce();
        assert_eq!(tree.as_string(), expected);
    }

    #[test]
    fn test_reduce_all_the_way() {
        let raw: &str = "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]";
        let mut tree: Tree = raw.into();
        tree.reduce_all_the_way();
        let expected: Tree = "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]".into();
        assert_eq!(tree.as_string(), expected.as_string());
    }

    #[test_case("[1,5]", 3 + 2 * 5)]
    #[test_case("[1,1]", 3 + 2)]
    #[test_case("[1,[1,2]]", 3 + 2 * (3 + 2 * 2))]
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


    #[test_case("[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]", "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]", "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]")]
    #[test_case("[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]", "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]", "[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]")]
    #[test_case("[[[[4,3],4],4],[7,[[8,4],9]]]", "[1,1]", "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")]
    #[test_case("[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]", "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]", "[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]")]
    #[test_case("[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]", "[7,[5,[[3,8],[1,4]]]]", "[[[[7,7],[7,8]],[[9,5],[8,7]]],[[[6,8],[0,8]],[[9,9],[9,0]]]]")]
    #[test_case("[[[[7,7],[7,8]],[[9,5],[8,7]]],[[[6,8],[0,8]],[[9,9],[9,0]]]]", "[[2,[2,2]],[8,[8,1]]]", "[[[[6,6],[6,6]],[[6,0],[6,7]]],[[[7,7],[8,9]],[8,[8,1]]]]")]
    #[test_case("[[[[6,6],[6,6]],[[6,0],[6,7]]],[[[7,7],[8,9]],[8,[8,1]]]]", "[2,9]", "[[[[6,6],[7,7]],[[0,7],[7,7]]],[[[5,5],[5,6]],9]]")]
    #[test_case("[[[[6,6],[7,7]],[[0,7],[7,7]]],[[[5,5],[5,6]],9]]", "[1,[[[9,3],9],[[9,0],[0,7]]]]", "[[[[7,8],[6,7]],[[6,8],[0,8]]],[[[7,7],[5,0]],[[5,5],[5,6]]]]")]
    fn test_add(op1: &str, op2: &str, expected: &str) {
        let mut tree1: Tree = op1.into();
        let tree2: Tree = op2.into();
        tree1.add(&tree2);
        assert_eq!(tree1.as_string(), expected);
    }


    #[test_case("[[[[5,11],[13,0]],[[15,14],[14,0]]],[[2,[0,[11,4]]],[[[6,7],1],[7,[1,6]]]]]", "[[[[5,11],[13,0]],[[15,14],[14,0]]],[[2,[11,0]],[[[10,7],1],[7,[1,6]]]]]")]
    fn test_reduce(initial: &str, expected: &str) {
        let mut tree: Tree = initial.into();
        println!("{}", tree.as_string());
        tree.reduce();
        assert_eq!(tree.as_string(), expected);
    }

}