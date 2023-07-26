pub fn dfs_set(graph: &Graph, root: Vertex, target: Vertex) {
    let mut visited = HashSet<Vertex>::new();
    let mut history = Vec<u32>::new();
    let mut queue = VecDeque<Vertex>::new();

    queue.push_back(root);

    while let Some(v) = queue.pop_front() {
        history.push(v.value());

        if v == target {
            return Some(history);
        }

        for neighbor in v.neighbors(graph) {
            if visited.insert(neighbor) {
                queue.push_front(neighbor);
            }
        }
    }
}

struct Vertex(u32);

struct Edge(u32, u32);

struct Graph {
    vertices: Vec<Vertex>,
    edges: Vec<Edge>,
}

impl Vertex {
    pub fn value(&self) -> u32 {
        self.0
    }
}

impl Graph {
    pub fn new(vertices: Vec<Vertex>, edges: Vec<Edge>) -> Self {
        Graph { vertices, edges }
    }
}

impl From<(u32, u32)> for Edge {
    fn from(item: u32) -> Self {
        Edge(item, item)
    }
}

impl From<u32> for Vertex {
    fn from(item: u32) -> Self {
        Vertex(item)
    }
}

impl Vertex {
    pub fn neighbors(&self, graph: &Graph) -> VecDeque<Vertex> {
        graph
            .edges
            .iter()
            .filter(|e| e.0 == self.0)
            .map(|e| e..0.into())
            .collect()
    }
}
