use std::collections::HashSet;
type NID = usize;

struct Graph<T> {
    nodes: Vec<Node<T>>,
    node_edges: Vec<Vec<NID>>,
}

struct Node<T> {
    id: NID,
    value: T,
}

impl<T> Graph<T> {
    pub fn new() -> Self {
        Graph {
            nodes: Vec::new(),
            node_edges: Vec::new(),
        }
    }

    pub fn add_node(&mut self, value: T) -> NID {
        let id = self.nodes.len() as NID;
        let node = Node::new(id, value);
        self.nodes.push(node);
        self.node_edges.push(Vec::new());

        id
    }

    pub fn add_edge(&mut self, from: NID, to: NID) {
        self.node_edges[from].push(to);
    }

    pub fn node_value(&mut self, node: NID) -> &mut T {
        &mut self.nodes[node].value
    }

    pub fn node_ids<'a> (&'a self) -> Box<dyn Iterator<Item=NID> + 'a> {
        Box::new(self.nodes.iter().map(|n| n.id))
    }

    pub fn topological_ordering(&self) -> Vec<NID> {
        let mut ordering = Vec::new();
        let mut nodes: Vec<_> = 
            self.node_edges.iter()
            .enumerate()
            .filter(|(_, n)| !n.is_empty())
            .map(|(i, _)| i)
            .collect();

        let mut edges = self.node_edges.clone();

        while let Some(n) = nodes.pop() {
            ordering.push(n);

        }

        Vec::new()
    }
}

impl<T> Node<T> {
    fn new(id: NID, value: T) -> Self {
        Node { id, value }
    }
}
