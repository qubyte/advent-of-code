use std::io::{stdin, BufRead};
use std::collections::{HashSet, HashMap};

fn node_is_revisitable(node: &String) -> bool {
    node.to_uppercase() == *node
}

fn search_from_node(path_so_far: &Vec<String>, edges: &HashMap<String, HashSet<String>>) -> Vec<Vec<String>> {
    let mut paths = vec![];
    let node = path_so_far[path_so_far.len() - 1].clone();

    if let Some(edges_from_node) = edges.get(&node) {
        for to in edges_from_node {
            if to == "end" {
                let mut path = path_so_far.to_vec();
                path.push(to.clone());
                paths.push(path)
            } else if node_is_revisitable(to) || !path_so_far.contains(to) {
                let mut path_with_this_edge = path_so_far.to_vec();
                path_with_this_edge.push(to.clone());

                for path in search_from_node(&path_with_this_edge, edges).iter() {
                    paths.push(path.clone());
                }
            }
        }
    }

    paths
}

fn main() {
    let edges = stdin()
        .lock()
        .lines()
        .filter_map(|line_result| line_result.ok())
        .filter_map(|line| {
            let split: Vec<_> = line.split('-').collect();

            if split.len() == 2 {
                Some((split[0].to_string(), split[1].to_string()))
            } else {
                None
            }
        })
        .fold(HashMap::<String, HashSet<String>>::new(), |mut edges, (left, right)| {
            if let Some(to) = edges.get_mut(&left) {
                to.insert(right.clone());
            } else {
                edges.insert(left.clone(), HashSet::from([right.clone()]));
            }

            if let Some(from) = edges.get_mut(&right) {
                from.insert(left.clone());
            } else {
                edges.insert(right.clone(), HashSet::from([left.clone()]));
            }

            edges
        });

    let paths = search_from_node(&vec![String::from("start")], &edges);

    println!("{}", paths.len());
}
