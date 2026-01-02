INSERT INTO causes (name, cause_type_id)
SELECT v.name, ct.id
FROM (
    VALUES
    ('scorched_fossil'),
    ('frigid_fossil'),
    ('metallic_fossil'),
    ('jagged_fossil'),
    ('abberrant_fossil'),
    ('pristine_fossil'),
    ('dense_fossil'),
    ('corroded_fossil'),
    ('prismatic_fossil'),
    ('aetheric_fossil'),
    ('serrated_fossil'),
    ('lucent_fossil'),
    ('shuddering_fossil'),
    ('bound_fossil'),
    ('opulent_fossil'),
    ('deft_fossil'),
    ('fundamental_fossil'),
    ('faceted_fossil'),
    ('bloodstained_fossil'),
    ('hollow_fossil'),
    ('fractured_fossil'),
    ('glyphic_fossil'),
    ('tangled_fossil'),
    ('sanctified_fossil'),
    ('gilded_fossil')
) AS v(name)
JOIN cause_types ct ON ct.name = 'fossil';