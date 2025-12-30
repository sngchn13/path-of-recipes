use std::hash::Hash;

use crate::domain::{causes::{Cause, SimpleCause}, edges::Edge};

#[derive(Debug, Clone)]
pub struct EdgeAggregate {
    pub id: i32,
    pub note: Option<String>,
    pub source_id: Option<i32>,
    pub target_id: Option<i32>,
    pub causes: Vec<SimpleCause>
}

impl EdgeAggregate {
    pub fn attach_causes(edge: Edge, causes: Vec<Cause>) -> Self {
        Self { 
            id: edge.id, 
            note: edge.note, 
            source_id: edge.source_id, 
            target_id: edge.targe_id, 
            causes:  causes.into_iter().map(|c| SimpleCause::from(c)).collect(),
        }
    }
}

impl Eq for EdgeAggregate { }
impl PartialEq for EdgeAggregate {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Hash for EdgeAggregate {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}