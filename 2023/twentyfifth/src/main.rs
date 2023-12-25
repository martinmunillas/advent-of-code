use std::collections::{HashMap, VecDeque};

fn main() {
    run_tests();

    let input = include_str!("./input/input.txt");

    println!(
        "Result A: {}",
        cut_cables_and_multiply_group_sizes(&build_graph(input))
    );
}

type Graph = HashMap<String, HashMap<String, i32>>;

fn cut_cables_and_multiply_group_sizes(graph: &Graph) -> i32 {
    let first = graph.keys().next().unwrap().to_owned();
    let others = graph
        .keys()
        .skip(1)
        .map(|x| x.to_owned())
        .collect::<Vec<String>>();

    for node in others {
        let mut g = graph.clone();
        if min_cut(&mut g, first.clone(), node.clone()) == 3 {
            let g1 = cluster_size(&g, first.clone());
            return g1 * (graph.len() as i32 - g1);
        }
    }
    0
}

fn build_graph(input: &str) -> Graph {
    let mut graph: Graph = HashMap::new();
    for line in input.lines() {
        let chunks = line.split(": ").collect::<Vec<&str>>();
        let a = chunks[0];
        let b = chunks[1].split(" ").collect::<Vec<&str>>();

        for node in b {
            match graph.get(node) {
                Some(edges) => {
                    let mut new_edges = edges.clone();
                    new_edges.insert(a.to_owned(), 1);
                    graph.insert(node.to_owned(), new_edges);
                }
                None => {
                    graph.insert(node.to_owned(), HashMap::from([(a.to_owned(), 1)]));
                }
            }
            match graph.get(a) {
                Some(edges) => {
                    let mut new_edges = edges.clone();
                    new_edges.insert(node.to_owned(), 1);
                    graph.insert(a.to_owned(), new_edges);
                }
                None => {
                    graph.insert(a.to_owned(), HashMap::from([(node.to_owned(), 1)]));
                }
            }
        }
    }

    graph
}

fn path(graph: &Graph, start: String, end: String) -> Option<HashMap<String, Option<String>>> {
    let mut parents = HashMap::new();
    let mut q = VecDeque::new();
    q.push_back(start);
    while q.len() > 0 {
        let current = q.pop_front().unwrap();
        if current == end {
            return Some(parents);
        }

        for (neighbor, capacity) in &graph[&current] {
            if *capacity > 0 && !parents.contains_key(neighbor) {
                parents.insert(neighbor.to_owned(), Some(current.to_owned()));
                q.push_back(neighbor.to_owned());
            }
        }
    }

    None
}

fn cluster_size(graph: &Graph, start: String) -> i32 {
    let mut visited = HashMap::new();
    let mut q = VecDeque::new();
    q.push_back(start);
    while q.len() > 0 {
        let current = q.pop_front().unwrap();
        visited.insert(current.to_owned(), true);
        for (neighbor, capacity) in &graph[&current] {
            if *capacity > 0 && !visited.contains_key(neighbor) {
                q.push_back(neighbor.to_owned());
            }
        }
    }

    visited.len() as i32
}

fn min_cut(graph: &mut Graph, s: String, t: String) -> i32 {
    let mut max_flow = 0;
    while let Some(p) = path(graph, s.clone(), t.clone()) {
        let mut flow = std::i32::MAX;
        let mut current = t.clone();
        while current != s {
            let parent = p[&current].clone().unwrap();
            flow = std::cmp::min(flow, graph[&parent][&current]);
            current = parent;
        }
        max_flow += flow;

        let mut current = t.clone();
        while current != s {
            let parent = p[&current].clone().unwrap();
            let pre_par_cur = graph[&parent][&current];
            let pre_cur_par = graph[&current][&parent];

            graph
                .get_mut(&parent)
                .unwrap()
                .insert(current.to_owned(), pre_par_cur - flow);
            graph
                .get_mut(&current)
                .unwrap()
                .insert(parent.to_owned(), pre_cur_par + flow);
            current = parent;
        }
    }
    max_flow
}

fn run_tests() {
    let example = include_str!("./input/example.txt");

    assert_eq!(
        cut_cables_and_multiply_group_sizes(&build_graph(example)),
        54
    );
    println!("Test passed!");

    println!("");
    println!("Tests passed!");
    println!("");
}
