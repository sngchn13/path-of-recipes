pub use sea_orm_migration::prelude::*;

mod m20251122_162130_create_item_type_table;
mod m20251122_162444_create_item_base_table;
mod m20251122_163531_create_section_table;
mod m20251122_163915_create_tag_table;
mod m20251122_164214_create_tier_group_table;
mod m20251122_164421_create_family_table;
mod m20251122_165034_create_mod_table;
mod m20251122_184709_create_mod_tag_table;
mod m20251123_114800_create_cause_type_table;
mod m20251123_114838_create_cause_table;
mod m20251123_115054_create_post_table;
mod m20251123_115211_create_graph_table;
mod m20251123_120144_create_node_table;
mod m20251123_122743_create_node_mod_table;
mod m20251123_123245_create_edge_table;
mod m20251123_190950_seed_item_type;
mod m20251124_102528_seed_item_base;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20251122_162130_create_item_type_table::Migration),
            Box::new(m20251122_162444_create_item_base_table::Migration),
            Box::new(m20251122_163531_create_section_table::Migration),
            Box::new(m20251122_163915_create_tag_table::Migration),
            Box::new(m20251122_164214_create_tier_group_table::Migration),
            Box::new(m20251122_164421_create_family_table::Migration),
            Box::new(m20251122_165034_create_mod_table::Migration),
            Box::new(m20251122_184709_create_mod_tag_table::Migration),
            Box::new(m20251123_114800_create_cause_type_table::Migration),
            Box::new(m20251123_114838_create_cause_table::Migration),
            Box::new(m20251123_115054_create_post_table::Migration),
            Box::new(m20251123_115211_create_graph_table::Migration),
            Box::new(m20251123_120144_create_node_table::Migration),
            Box::new(m20251123_122743_create_node_mod_table::Migration),
            Box::new(m20251123_123245_create_edge_table::Migration),
            Box::new(m20251123_190950_seed_item_type::Migration),
            Box::new(m20251124_102528_seed_item_base::Migration),
        ]
    }
}
