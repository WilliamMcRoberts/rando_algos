use std::collections::VecDeque;

// Return an Option<Vec<_>> with history of verteces visited
// or None if the element does not exist in the graph
pub fn depth_first_search(graph: &Graph, root: Vertex, target: Vertex) -> Option<Vec<u32>> {
    let mut history: Vec<u32> = Vec::new();
    let mut queue = VecDeque::new();

    queue.push_back(root);

    // While the queue is not empty
    // get the first element
    while let Some(current_vertex) = queue.pop_front() {
        // Add the current vertex in the history
        history.push(current_vertex.value());

        // check if vertex equals the target if so return the history
        if current_vertex == target {
            // Return the history
            return Some(history);
        }

        // iterate over neighbors of current vertex
        for neighbor in current_vertex.neighbors(graph).into_iter().rev() {
            if !history.contains(&neighbor.value()) {
                queue.push_front(neighbor);
            }
        }
    }

    // Return None if all verteces are visited and the target is not found
    None
}

// Data Structures

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Vertex(u32);

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Edge(u32, u32);

#[derive(Clone)]
pub struct Graph {
    #[allow(dead_code)]
    vertices: Vec<Vertex>,
    edges: Vec<Edge>,
}

// Graph constructor
impl Graph {
    pub fn new(vertices: Vec<Vertex>, edges: Vec<Edge>) -> Self {
        Graph { vertices, edges }
    }
}

impl Vertex {
    pub fn value(&self) -> u32 {
        self.0
    }

    pub fn neighbors(&self, graph: &Graph) -> VecDeque<Vertex> {
        graph
            .edges
            .iter()
            .filter(|e| e.0 == self.0)
            .map(|e| e.1.into())
            .collect()
    }
}

// impl From trait for Vertex and Edge
impl From<u32> for Vertex {
    fn from(item: u32) -> Self {
        Vertex(item)
    }
}

impl From<(u32, u32)> for Edge {
    fn from(item: (u32, u32)) -> Self {
        Edge(item.0, item.1)
    }
}
