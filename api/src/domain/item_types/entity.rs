
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ItemType{
    pub id: i32,
    pub name: String,
}

impl ItemType {
    pub fn new(id: i32, name: String) -> Self {
        ItemType { id, name }
    }
}