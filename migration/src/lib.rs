pub use sea_orm_migration::prelude::*;

mod m20251122_162130_create_item_type_table;
mod m20251122_162135_create_image_table;
mod m20251122_162444_create_item_base_table;
mod m20251122_162449_create_mod_genration_type_table;
mod m20251122_163531_create_section_table;
mod m20251122_163915_create_tag_table;
mod m20251122_164214_create_family_table;
mod m20251122_164314_create_mod_group_table;
mod m20251122_165034_create_mod_table;
mod m20251122_184709_create_mod_group_tag_table;
mod m20251123_114800_create_cause_type_table;
mod m20251123_114838_create_cause_table;
mod m20251123_115054_create_post_table;
mod m20251123_115211_create_graph_table;
mod m20251123_120144_create_node_table;
mod m20251123_122743_create_node_mod_table;
mod m20251123_123245_create_edge_table;
mod m20251123_175358_create_edge_cause_table;
mod m20251123_180000_create_mod_group_family_table;
mod m20251123_180005_create_comment_table;
mod m20251123_180010_create_reaction_table;
mod m20251123_190950_seed_item_type;
mod m20251123_191000_seed_image;
mod m20251124_102528_seed_item_base;
mod m20251126_191134_seed_section;
mod m20251127_072541_seed_tag;
mod m20251127_074022_seed_cause_type;
mod m20251127_080024_seed_cause;
mod m20251127_093003_seed_family;
mod m20251205_113229_seed_mod_generation_type;
mod m20251205_123555_seed_mod;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20251122_162130_create_item_type_table::Migration),
            Box::new(m20251122_162135_create_image_table::Migration),
            Box::new(m20251122_162444_create_item_base_table::Migration),
            Box::new(m20251122_162449_create_mod_genration_type_table::Migration),
            Box::new(m20251122_163531_create_section_table::Migration),
            Box::new(m20251122_163915_create_tag_table::Migration),
            Box::new(m20251122_164214_create_family_table::Migration),
            Box::new(m20251122_164314_create_mod_group_table::Migration),
            Box::new(m20251122_165034_create_mod_table::Migration),
            Box::new(m20251122_184709_create_mod_group_tag_table::Migration),
            Box::new(m20251123_114800_create_cause_type_table::Migration),
            Box::new(m20251123_114838_create_cause_table::Migration),
            Box::new(m20251123_115054_create_post_table::Migration),
            Box::new(m20251123_115211_create_graph_table::Migration),
            Box::new(m20251123_120144_create_node_table::Migration),
            Box::new(m20251123_122743_create_node_mod_table::Migration),
            Box::new(m20251123_123245_create_edge_table::Migration),
            Box::new(m20251123_175358_create_edge_cause_table::Migration),
            Box::new(m20251123_180000_create_mod_group_family_table::Migration),
            Box::new(m20251123_180005_create_comment_table::Migration),
            Box::new(m20251123_180010_create_reaction_table::Migration),
            Box::new(m20251123_190950_seed_item_type::Migration),
            Box::new(m20251123_191000_seed_image::Migration),
            Box::new(m20251124_102528_seed_item_base::Migration),
            Box::new(m20251126_191134_seed_section::Migration),
            Box::new(m20251127_072541_seed_tag::Migration),
            Box::new(m20251127_074022_seed_cause_type::Migration),
            Box::new(m20251127_080024_seed_cause::Migration),
            Box::new(m20251127_093003_seed_family::Migration),
            Box::new(m20251205_113229_seed_mod_generation_type::Migration),
            Box::new(m20251205_123555_seed_mod::Migration),
        ]
    }
}