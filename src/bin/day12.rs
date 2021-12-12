use std::{
    collections::{HashMap, HashSet},
    io::{self, BufRead},
};

fn main() {
    let graph = parse_input(io::stdin().lock());

    dbg!(graph.count_paths("start", "end", false));
    dbg!(graph.count_paths("start", "end", true));
}

#[derive(Default, Debug)]
struct Graph {
    nodes: HashMap<String, usize>,
    is_big: Vec<bool>,
    adj: Vec<HashSet<usize>>,
}

impl Graph {
    fn get_or_create_node(&mut self, key: &str) -> usize {
        let new_id = self.nodes.len();

        *self.nodes.entry(key.to_owned()).or_insert_with(|| {
            self.adj.push(HashSet::new()); // create adjacency vector too
            self.is_big.push(key.chars().next().unwrap().is_uppercase());
            new_id
        })
    }

    fn connect(&mut self, from: &str, to: &str) {
        let id_from = self.get_or_create_node(from);
        let id_to = self.get_or_create_node(to);

        self.adj[id_from].insert(id_to);
    }

    fn shrink_to_fit(&mut self) {
        self.nodes.shrink_to_fit();
        for adj_set in &mut self.adj {
            adj_set.shrink_to_fit();
        }
        self.adj.shrink_to_fit();
    }

    #[allow(dead_code)]
    fn key_of(&self, node_id: usize) -> &str {
        self.nodes
            .iter()
            .find(|(_k, v)| **v == node_id)
            .map(|(k, _)| k)
            .unwrap()
    }

    fn count_paths(&self, from: &str, to: &str, can_visit_one_twice: bool) -> usize {
        let from = self.nodes[from];
        let to = self.nodes[to];

        #[derive(Clone, Copy, PartialEq)]
        enum VisitTwice {
            NotYet,
            PendingSecondVisit(usize),
            AlreadyDidIt,
        }

        fn _count(
            graph: &Graph,
            node: usize,
            to: usize,
            mut path_count: usize,
            visited: &mut HashSet<usize>,
            mut visit_twice: VisitTwice,
        ) -> usize {
            let mut visiting_for_the_second_time = false;
            if visited.contains(&node) {
                if visit_twice == VisitTwice::PendingSecondVisit(node) {
                    visit_twice = VisitTwice::AlreadyDidIt;
                    visiting_for_the_second_time = true;
                } else {
                    return path_count;
                }
            }

            //println!("going through {}", graph.key_of(node));
            if !graph.is_big[node] {
                visited.insert(node);
            }

            if node == to && !matches!(visit_twice, VisitTwice::PendingSecondVisit(_)) {
                //println!("IT'S MY TARGET");
                path_count += 1;
            } else {
                for &adj in &graph.adj[node] {
                    path_count = _count(graph, adj, to, path_count, visited, visit_twice);

                    if visit_twice == VisitTwice::NotYet && !graph.is_big[node] {
                        // ok now try the same thing but visiting this node twice
                        path_count = _count(
                            graph,
                            adj,
                            to,
                            path_count,
                            visited,
                            VisitTwice::PendingSecondVisit(node),
                        );
                    }
                }
            }

            //println!("done visiting {}", graph.key_of(node));
            if !visiting_for_the_second_time {
                visited.remove(&node);
            }
            path_count
        }

        let mut visited = HashSet::new();
        let initial_visit_twice = if can_visit_one_twice {
            VisitTwice::NotYet
        } else {
            VisitTwice::AlreadyDidIt
        };
        _count(self, from, to, 0, &mut visited, initial_visit_twice)
    }
}

fn parse_input(reader: impl BufRead) -> Graph {
    let mut graph = Graph::default();

    for line in reader
        .lines()
        .map(Result::unwrap)
        .filter(|l| !l.trim().is_empty())
    {
        let (edge_from, edge_to) = line.trim().split_once('-').unwrap();

        if edge_from != "end" && edge_to != "start" {
            graph.connect(edge_from, edge_to);
        }
        if edge_to != "end" && edge_from != "start" {
            graph.connect(edge_to, edge_from);
        }
    }

    graph.shrink_to_fit();
    graph
}
