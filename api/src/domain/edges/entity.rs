

pub struct Edge{
    pub id: i32,
    pub note: Option<String>,
    pub source_id: Option<i32>,
    pub targe_id: Option<i32>,
    pub graph_id: i32,
}

impl Edge {
    pub fn new(id: i32, note: Option<String>, source_id: Option<i32>, targe_id: Option<i32>,graph_id: i32,) -> Self
    {
        Self { id, note, source_id, targe_id, graph_id }
    }
}