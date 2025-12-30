pub struct ItemBase {
    pub id: i32,
    pub name: String,
    pub display_en: Option<String>,
    pub display_kor: Option<String>,
    pub item_type_id: i32,
}

impl ItemBase {
    pub fn new(id: i32, name: String, display_en: Option<String>, display_kor: Option<String>, item_type_id: i32)
        -> Self
    {
        Self{id, name, display_en, display_kor, item_type_id}
    }
}