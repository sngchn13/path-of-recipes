
pub struct Cause{
    pub id: i32,
    pub name: String,
    pub cause_type_id: i32,
}

impl Cause {
    pub fn new( id: i32, name: String, cause_type_id: i32) -> Self{
        Self { id, name, cause_type_id }
    }
}