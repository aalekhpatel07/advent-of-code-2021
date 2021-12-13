use petgraph::graph::{NodeIndex, UnGraph};
use std::collections::HashMap;
use std::io;
use std::io::Read;

fn main() {
    let mut buffer: String = String::new();
    io::stdin()
        .read_to_string(&mut buffer)
        .expect("Couldn't read from stdin.");
    let (node_map, graph) = parser::graph(&buffer).expect("Couldn't parse.");

    let result_1 = part_1::solve(&node_map, &graph);
    let result_2 = part_2::solve(&node_map, &graph);
    println!("Part 1: {:?}\nPart 2: {:?}", result_1, result_2);
}

type StringToNodeMap = HashMap<String, NodeIndex>;
type Graph = UnGraph<(), ()>;

mod parser {
    use super::{Graph, StringToNodeMap};
    use petgraph::graph::{NodeIndex, UnGraph};
    use std::collections::HashMap;
    use std::io::ErrorKind;

    pub fn graph(input: &str) -> Result<(StringToNodeMap, Graph), ErrorKind> {
        let mut node_map: HashMap<String, NodeIndex> = HashMap::new();
        let mut un_graph: UnGraph<(), ()> = UnGraph::new_undirected();

        let node_pairs: Vec<(String, String)> = input
            .split_whitespace()
            .map(|x| {
                let nodes: Vec<String> = x.split('-').map(String::from).collect();
                (
                    nodes.first().unwrap().clone(),
                    nodes.last().unwrap().clone(),
                )
            })
            .collect();

        for (start, end) in node_pairs {
            if node_map.get_mut(&start).is_none() {
                let added = un_graph.add_node(());
                node_map.insert(start.clone(), added);
            }
            if node_map.get_mut(&end).is_none() {
                let added = un_graph.add_node(());
                node_map.insert(end.clone(), added);
            }
            let (start_node, end_node) =
                (*node_map.get(&start).unwrap(), *node_map.get(&end).unwrap());
            un_graph.add_edge(start_node, end_node, ());
        }

        Ok((node_map, un_graph))
    }
}

mod part_1 {
    use super::*;
    fn dfs(
        graph: &UnGraph<(), ()>,
        start: NodeIndex,
        end: NodeIndex,
        results: &mut Vec<Vec<NodeIndex>>,
        visited: &mut HashMap<NodeIndex, bool>,
        current_walk: &mut Vec<NodeIndex>,
        node_id_to_name_map: &HashMap<NodeIndex, String>,
    ) {
        if *visited.get(&start).unwrap() && is_small(node_id_to_name_map, start) {
            return;
        }
        visited.insert(start, true);
        current_walk.push(start);

        if start == end {
            results.push(current_walk.clone());
            visited.insert(start, false);
            current_walk.pop().unwrap();
            return;
        }

        for neighbor in graph.neighbors(start) {
            dfs(
                graph,
                neighbor,
                end,
                results,
                visited,
                current_walk,
                node_id_to_name_map,
            );
        }

        current_walk.pop().unwrap();
        visited.insert(start, false);
    }

    fn is_small(node_id_to_name_map: &HashMap<NodeIndex, String>, x: NodeIndex) -> bool {
        node_id_to_name_map.get(&x).unwrap().to_lowercase()
            == *(node_id_to_name_map).get(&x).unwrap()
    }

    pub fn solve(node_map: &HashMap<String, NodeIndex>, graph: &UnGraph<(), ()>) -> usize {
        let mut node_id_name_map: HashMap<NodeIndex, String> = HashMap::new();
        for (k, v) in node_map {
            node_id_name_map.insert(*v, k.clone());
        }

        let mut current_walk: Vec<NodeIndex> = vec![];
        let mut visited: HashMap<NodeIndex, bool> = HashMap::new();
        let mut results: Vec<Vec<NodeIndex>> = vec![];

        for node in graph.node_indices() {
            visited.insert(node, false);
        }

        dfs(
            graph,
            *node_map.get(&String::from("start")).unwrap(),
            *node_map.get(&String::from("end")).unwrap(),
            &mut results,
            &mut visited,
            &mut current_walk,
            &node_id_name_map,
        );
        results.len()
    }
}

mod part_2 {
    use super::*;

    #[allow(clippy::too_many_arguments)]
    fn dfs(
        graph: &UnGraph<(), ()>,
        start: NodeIndex,
        end: NodeIndex,
        results: &mut Vec<Vec<NodeIndex>>,
        visited: &mut HashMap<NodeIndex, usize>,
        current_walk: &mut Vec<NodeIndex>,
        node_id_to_name_map: &HashMap<NodeIndex, String>,
        mut twice_visited: bool,
        mut index_twice_visited: Option<NodeIndex>,
    ) {
        let times_visited: usize = *visited.get(&start).unwrap();

        if is_start_or_end(node_id_to_name_map, start) {
            if times_visited > 0 {
                return;
            }
        } else if is_small(node_id_to_name_map, start) {
            if times_visited == 1 {
                if !twice_visited {
                    twice_visited = true;
                    index_twice_visited = Some(start);
                } else {
                    return;
                }
            } else if times_visited >= 2 {
                return;
            }
        }

        visited.insert(start, times_visited + 1);
        current_walk.push(start);

        if start == end {
            results.push(current_walk.clone());
            visited.insert(start, times_visited);

            current_walk.pop().unwrap();

            // if index_twice_visited.is_some() {
            //     if index_twice_visited.unwrap() == last {
            //     }
            // }
            return;
        }

        for neighbor in graph.neighbors(start) {
            dfs(
                graph,
                neighbor,
                end,
                results,
                visited,
                current_walk,
                node_id_to_name_map,
                twice_visited,
                index_twice_visited,
            );
        }

        visited.insert(start, times_visited);
        current_walk.pop().unwrap();
    }

    fn is_small(node_id_to_name_map: &HashMap<NodeIndex, String>, x: NodeIndex) -> bool {
        let val = node_id_to_name_map.get(&x).unwrap();
        val.to_lowercase() == val.clone()
    }

    fn is_start_or_end(node_id_to_name_map: &HashMap<NodeIndex, String>, x: NodeIndex) -> bool {
        let val = node_id_to_name_map.get(&x).unwrap().as_str();
        val == "start" || val == "end"
    }

    pub fn solve(node_map: &HashMap<String, NodeIndex>, graph: &UnGraph<(), ()>) -> usize {
        let mut node_id_name_map: HashMap<NodeIndex, String> = HashMap::new();
        for (k, v) in node_map {
            node_id_name_map.insert(*v, k.clone());
        }

        let mut current_walk: Vec<NodeIndex> = vec![];
        let mut visited: HashMap<NodeIndex, usize> = HashMap::new();
        let mut results: Vec<Vec<NodeIndex>> = vec![];

        for node in graph.node_indices() {
            visited.insert(node, 0);
        }

        let twice_visited: bool = false;
        let index_twice_visited: Option<NodeIndex> = None;

        dfs(
            graph,
            *node_map.get(&String::from("start")).unwrap(),
            *node_map.get(&String::from("end")).unwrap(),
            &mut results,
            &mut visited,
            &mut current_walk,
            &node_id_name_map,
            twice_visited,
            index_twice_visited,
        );
        results.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use petgraph::prelude::*;

    fn setup_1() -> (HashMap<String, NodeIndex>, UnGraph<(), ()>) {
        let input: &str = "
                start-A
                start-b
                A-c
                A-b
                b-d
                A-end
                b-end
            ";
        parser::graph(input).expect("Couldn't parse input 1.")
    }

    fn setup_2() -> (HashMap<String, NodeIndex>, UnGraph<(), ()>) {
        let input: &str = "
                dc-end
                HN-start
                start-kj
                dc-start
                dc-HN
                LN-dc
                HN-end
                kj-sa
                kj-HN
                kj-dc
            ";
        parser::graph(input).expect("Couldn't parse input 2.")
    }

    fn setup_3() -> (HashMap<String, NodeIndex>, UnGraph<(), ()>) {
        let input: &str = "
                fs-end
                he-DX
                fs-he
                start-DX
                pj-DX
                end-zg
                zg-sl
                zg-pj
                pj-he
                RW-he
                fs-DX
                pj-RW
                zg-RW
                start-pj
                he-WI
                zg-he
                pj-fs
                start-RW
            ";
        parser::graph(input).expect("Couldn't parse input 3.")
    }

    #[test]
    fn test_1_sample_1() {
        let (node_indices, mut graph) = setup_1();
        let expected: usize = 10;
        assert_eq!(part_1::solve(&node_indices, &mut graph), expected);
    }

    #[test]
    fn test_1_sample_2() {
        let (node_indices, mut graph) = setup_2();
        let expected: usize = 19;
        assert_eq!(part_1::solve(&node_indices, &mut graph), expected);
    }

    #[test]
    fn test_1_sample_3() {
        let (node_indices, mut graph) = setup_3();
        let expected: usize = 226;
        assert_eq!(part_1::solve(&node_indices, &mut graph), expected);
    }

    #[test]
    fn test_2_sample_1() {
        let (node_map, mut graph) = setup_1();
        let expected: usize = 36;
        assert_eq!(part_2::solve(&node_map, &mut graph), expected);
    }

    #[test]
    fn test_2_sample_2() {
        let (node_map, mut graph) = setup_2();
        let expected: usize = 103;
        assert_eq!(part_2::solve(&node_map, &mut graph), expected);
    }

    #[test]
    fn test_2_sample_3() {
        let (node_map, mut graph) = setup_3();
        let expected: usize = 3509;
        assert_eq!(part_2::solve(&node_map, &mut graph), expected);
    }
}
