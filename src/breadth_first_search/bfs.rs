use std::collections::{HashSet, VecDeque};

pub fn bfs(graph: &Graph, root: Vertex, target: Vertex) -> Option<Vec<u32>> {
    let mut visited: HashSet<Vertex> = HashSet::new();
    let mut history: Vec<u32> = Vec::new();
    let mut queue: VecDeque<Vertex> = VecDeque::new();

    visited.insert(root);
    queue.push_back(root);
    while let Some(vertex) = queue.pop_front() {
        history.push(vertex.0);

        if vertex == target {
            return Some(history);
        }

        for v in vertex.neighbors(graph) {
            if !visited.contains(&v) {
                visited.insert(v);
                queue.push_back(v);
            }
        }
    }
    None
}

pub struct Graph {
    vertecis: Vec<Vertex>,
    edges: Vec<Edge>,
}

impl Graph {
    pub fn new(vertecis: Vec<Vertex>, edges: Vec<Edge>) -> Self {
        Graph { vertecis, edges }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Vertex(u32);

pub struct Edge(u32, u32);

impl Vertex {
    pub fn neighbors(&self, graph: &Graph) -> VecDeque<Vertex> {
        graph
            .edges
            .iter()
            .filter(|e| e.0 == self.0)
            .map(|e| e.1.into())
            .collect()
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    /* Example graph #1:
     *
     *            (1)   <--- Root
     *           /   \
     *         (2)   (3)
     *        / |     | \
     *     (4) (5)   (6) (7)
     *          |
     *         (8)
     */
    fn graph1() -> Graph {
        let vertecis = vec![1, 2, 3, 4, 5, 6, 7];
        let edges = vec![(1, 2), (1, 3), (2, 4), (2, 5), (3, 6), (3, 7), (5, 8)];

        Graph::new(
            vertecis.into_iter().map(|v| v.into()).collect(),
            edges.into_iter().map(|e| e.into()).collect(),
        )
    }

    #[test]
    fn breadth_first_search_graph1_when_vertex_not_found_returns_none() {
        let graph = graph1();
        let root = 1;
        let target = 10;

        assert_eq!(bfs(&graph, root.into(), target.into()), None);
    }

    #[test]
    fn breadth_first_search_graph1_when_target_8_should_evaluate_all_vertecis_first() {
        let graph = graph1();
        let root = 1;
        let target = 8;

        let expected_path = vec![1, 2, 3, 4, 5, 6, 7, 8];

        assert_eq!(bfs(&graph, root.into(), target.into()), Some(expected_path));
    }

    /* Example graph #2:
     *
     *     (1) --- (2)     (3) --- (4)
     *            / |     /       /
     *          /   |   /       /
     *        /     | /       /
     *     (5)     (6) --- (7)     (8)
     */
    fn graph2() -> Graph {
        let vertecis = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let undirected_edges = vec![
            (1, 2),
            (2, 1),
            (2, 5),
            (5, 2),
            (2, 6),
            (6, 2),
            (3, 4),
            (4, 3),
            (3, 6),
            (6, 3),
            (4, 7),
            (7, 4),
            (6, 7),
            (7, 6),
        ];

        Graph::new(
            vertecis.into_iter().map(|v| v.into()).collect(),
            undirected_edges.into_iter().map(|e| e.into()).collect(),
        )
    }

    #[test]
    fn breadth_first_search_graph2_when_no_path_to_vertex_returns_none() {
        let graph = graph2();
        let root = 8;
        let target = 4;

        assert_eq!(bfs(&graph, root.into(), target.into()), None);
    }

    #[test]
    fn breadth_first_search_graph2_should_find_path_from_4_to_1() {
        let graph = graph2();
        let root = 4;
        let target = 1;

        let expected_path = vec![4, 3, 7, 6, 2, 1];

        assert_eq!(bfs(&graph, root.into(), target.into()), Some(expected_path));
    }
}
