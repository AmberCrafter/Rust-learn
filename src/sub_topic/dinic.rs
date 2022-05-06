use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
struct Graph {
    nodes: HashMap<&'static str, Node>,
}

#[derive(Debug)]
struct Node {
    name: &'static str,
    next: Option<Vec<(&'static str, Flow)>>,
}

#[derive(Debug)]
struct Flow {
    cap: usize,
    back: usize,
}

trait Dinic {
    fn bfs(graph: &Graph, root: &'static str) -> Vec<Vec<&'static str>>;
    fn gen_level_graph(graph: &Graph, root: &'static str) -> Graph;
}

impl Graph {
    fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }
}

impl Node {
    pub fn new(name: &'static str) -> Self {
        Self { name, next: None }
    }

    pub fn add(&mut self, node_name: &'static str, cap: usize) {
        let path = (node_name, Flow { cap, back: 0 });
        if let Some(next) = self.next.as_mut() {
            next.push(path)
        } else {
            self.next = Some(vec![path]);
        }
    }

    // pub fn point_node_by_index(&mut self, index: usize) -> Result<&mut Node, String> {
    //     if let Some(next) = self.next.as_mut() {
    //         if index<next.len() {
    //             Ok(&mut next[index].0)
    //         } else {
    //             Err(format!("Index out of bound. This next nodes only has {} leaf.",next.len()))
    //         }
    //     } else {
    //         Err(format!("Index out of bound. This next nodes only has {} leaf.", 0))
    //     }
    // }
}

impl Dinic for Graph {
    fn bfs(graph: &Graph, root: &'static str) -> Vec<Vec<&'static str>> {
        let mut queue = VecDeque::new();
        let mut res: Vec<Vec<&'static str>> = Vec::new();

        if let Some(root_node) = graph.nodes.get(root) {
            queue.push_back(vec![root_node]);
        }

        while queue.len() > 0 {
            if let Some(level_elements) = queue.pop_front() {
                let next = level_elements.iter().map(|&node| {
                    if let Some(next) = node.next {
                        
                    }
                })
                queue.push();
                res.push(level_elements
                    .iter()
                    .filter_map(|&e| res.iter().flatten().find(|&&v| v != e.name))
                    .map(|&v| v)
                    .collect::<Vec<&str>>());
                
            }
        }

        res
    }

    fn gen_level_graph(graph: &Graph, root: &'static str) -> Graph {
        let mut level_graph = Graph::new();

        level_graph
    }
}

pub fn main() {
    /*
    S -> [v1 (10), v2 (10)]
        v1 -> [v2 (2), v3 (4), v4 (8)]
        v2 -> [v4 (9)]
            v3 -> [t (10)]
            v4 -> [v3 (6), t (10)]
                t -> []
    */
    let mut graph = Graph::new();
    graph.nodes.insert("S", Node::new("S"));
    graph.nodes.insert("v1", Node::new("v1"));
    graph.nodes.insert("v2", Node::new("v2"));
    graph.nodes.insert("v3", Node::new("v3"));
    graph.nodes.insert("v4", Node::new("v4"));
    graph.nodes.insert("t", Node::new("t"));

    graph.nodes.entry("S").and_modify(|v| v.add("v1", 10));
    graph.nodes.entry("S").and_modify(|v| v.add("v2", 10));
    graph.nodes.entry("v1").and_modify(|v| v.add("v2", 2));
    graph.nodes.entry("v1").and_modify(|v| v.add("v3", 3));
    graph.nodes.entry("v1").and_modify(|v| v.add("v4", 8));
    graph.nodes.entry("v2").and_modify(|v| v.add("v4", 9));
    graph.nodes.entry("v3").and_modify(|v| v.add("t", 10));
    graph.nodes.entry("v4").and_modify(|v| v.add("v3", 6));
    graph.nodes.entry("v4").and_modify(|v| v.add("t", 10));

    println!("Graph: {graph:#?}");
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        main();
    }
}
