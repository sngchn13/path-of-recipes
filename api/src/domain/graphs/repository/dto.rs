use std::collections::HashSet;

use crate::domain::{edges::EdgeAggregate, graphs::{Graph, Verification}, nodes::NodeAggregate};

#[derive(Debug)]
pub struct GraphAggregate {
    pub id: i32,
    pub description: Option<String>,
    pub verification: Verification,
    pub nodes: HashSet<NodeAggregate>,
    pub edges: HashSet<EdgeAggregate>
}

impl GraphAggregate {
    pub fn assemble_graph(graph: Graph, nodes: HashSet<NodeAggregate>, edges: HashSet<EdgeAggregate>) -> Self {
        Self { id: graph.id, description: graph.description, verification: graph.verification, nodes, edges }
    }
}