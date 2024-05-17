use rand::Rng;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};

#[derive(Eq, PartialEq, Clone, Debug)]
struct Node {
    id: usize,
    exits: Vec<usize>,
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Edge {
    node_1: usize,
    node_2: usize,
}

impl Edge {
    fn new(node_1: usize, node_2: usize) -> Edge {
        let mut node_1 = node_1;
        let mut node_2 = node_2;
        if node_1 > node_2 {
            std::mem::swap(&mut node_1, &mut node_2);
        }
        Edge { node_1, node_2 }
    }
}

impl Display for Edge {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "edge [dir=both] {} -> {}", self.node_1, self.node_2)
    }
}

#[derive(Debug)]
struct Graph {
    edges: HashSet<Edge>,
}

impl Graph {
    fn new() -> Graph {
        Graph {
            edges: HashSet::new(),
        }
    }

    fn insert(&mut self, edge: Edge) -> Option<()> {
        self.edges.insert(edge);
        Some(())
    }

    fn len(&self) -> usize {
        self.edges.len()
    }
}

impl Display for Graph {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        for edge in &self.edges {
            result.push_str(&edge.to_string());
        }
        write!(f, "{}", result)
    }
}

fn remove_random_pair<T: PartialEq + Clone>(v: &mut Vec<T>) -> (T, T) {
    assert!(v.len() > 1);

    let mut rng = rand::thread_rng();
    let i = rng.gen_range(0..v.len());
    let n1 = v[i].clone();

    v.retain(|n| *n != n1);
    let j = rng.gen_range(0..v.len());
    let n2 = v[j].clone();
    v.retain(|n| *n != n2);
    (n1, n2)
}

type Nodes = Vec<Node>;

fn generate_nodes(size: usize) -> Result<Nodes, ()> {
    let mut valence_2_nodes: Vec<usize> = vec![];

    // Create a set of numbered nodes
    let mut nodes: Vec<Node> = vec![];
    for i in 0..size {
        let node = Node {
            id: i,
            exits: vec![],
        };
        nodes.push(node);
    }

    // create bidirectional connections of all nodes in a simple loop
    for (i, node) in nodes.iter_mut().enumerate() {
        node.exits.push((i + 1) % size);
        node.exits.push((i + 20 - 1) % size);
        valence_2_nodes.push(i);
    }

    while !valence_2_nodes.is_empty() {
        // chose a random pair of noes to connect
        let (n1, n2) = remove_random_pair(&mut valence_2_nodes);

        // connect the chosen nodes bidirectionally.
        nodes[n1].exits.push(n2);
        nodes[n2].exits.push(n1);
    }
    Ok(nodes)
}

fn search(
    nodes: &Nodes,
    start_node: usize,
    current_node: usize,
    path: Vec<usize>,
    depth: usize,
) -> usize {
    // bail out if recursion depth is crazy
    if (depth == 30usize) {
        panic!();
    }

    let mut node = nodes[current_node].clone();

    // Have we returned to the node we started from having made a loop?
    if (node.id == start_node) && (path.len() > 2) {
        if path.len() == 5 {
            println!("Pentagon found:");
        }
        println!("Path {:?}", path);
        return depth;
    }

    // Have we already been here
    if path.contains(&node.id) {
        return depth;
    }

    let mut path = path.clone();
    path.push(node.id);

    let mut path_length = 0usize;
    for (i, connection) in node.exits.iter().enumerate() {
        let l = search(nodes, start_node, *connection, path.clone(), depth + 1);
        path_length = std::cmp::max(path_length, 1);
    }
    path_length
}

#[cfg(test)]
mod test {
    use crate::data_structures::graph::{generate_nodes, search, Edge, Graph};

    #[test]
    fn ex1() {
        let mut graph;
        let mut retries = 0;
        loop {
            let mut nodes = generate_nodes(20).unwrap();

            graph = Graph::new();
            for node in &nodes {
                for connection in &node.exits {
                    graph.insert(Edge::new(node.id, *connection));
                }
            }

            if graph.len() == 30usize || retries == 16 {
                let path: Vec<usize> = vec![];
                let path_length = search(&nodes, 0, 0, path, 0);
                println!("longest path = {}", path_length);
                break;
            }
            retries += 1;
        }

        println!("Edges: {}", graph.len());
    }
}
