use uuid::Uuid;

pub struct Image {
    pub id: Uuid,
    pub relative_path: String,
    pub name: String,
    pub width: i32,
    pub height: i32,
    pub size: i32,
    pub extension: String,
    pub desc: String
}