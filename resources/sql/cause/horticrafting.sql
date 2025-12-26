
INSERT INTO causes (name, cause_type_id)
SELECT v.name, ct.id
FROM (
    VALUES
    ('reforge_physical_mod'),
    ('reforge_lightning_mod'),
    ('reforge_cold_mod'),
    ('reforge_fire_mod'),
    ('reforge_defense_mod'),
    ('reforge_chaos_mod'),
    ('reforge_attack_mod'),
    ('reforge_caster_mod'),
    ('reforge_speed_mod'),
    ('reforge_critical_mod'),

    ('change_res_fire_to_cold'),
    ('change_res_fire_to_lightning'),
    ('change_res_cold_to_fire'),
    ('change_res_cold_to_lightning'),
    ('change_res_lightning_to_fire'),
    ('change_res_lightning_to_cold'),

    ('add_physical_mod_and_remove_random_mod'),
    ('add_lightning_mod_and_remove_random_mod'),
    ('add_cold_mod_and_remove_random_mod'),
    ('add_fire_mod_and_remove_random_mod'),
    ('add_chaos_mod_and_remove_random_mod'),
    ('add_life_mod_and_remove_random_mod'),
    ('add_defence_mod_and_remove_random_mod'),
    ('add_attack_mod_and_remove_random_mod'),
    ('add_caster_mod_and_remove_random_mod'),
    ('add_speed_mod_and_remove_random_mod'),
    ('add_critical_mod_and_remove_random_mod')
) AS v(name)
JOIN cause_types ct ON ct.name = 'horticrafting';