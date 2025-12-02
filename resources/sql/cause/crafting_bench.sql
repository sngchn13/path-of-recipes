
INSERT INTO causes (name, cause_type_id)
SELECT v.name, ct.id
FROM (
    VALUES
    ('crafting_bench')
) AS v(name)
JOIN cause_types ct ON ct.name = 'crafting_bench';