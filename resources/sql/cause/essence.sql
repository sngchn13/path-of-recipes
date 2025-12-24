
INSERT INTO causes (name, cause_type_id)
SELECT v.name, ct.id
FROM (
    VALUES
    ('essence_of_hatred'),
    ('essence_of_woe'),
    ('essence_of_greed'),
    ('essence_of_contempt'),
    ('essence_of_sorrow'),
    ('essence_of_anger'),
    ('essence_of_torment'),
    ('essence_of_fear'),
    ('essence_of_suffering'),
    ('essence_of_rage'),
    ('essence_of_wrath'),
    ('essence_of_doubt'),
    ('essence_of_anguish'),
    ('essence_of_loathing'),
    ('essence_of_spite'),
    ('essence_of_zeal'),
    ('essence_of_misery'),
    ('essence_of_dread'),
    ('essence_of_scorn'),
    ('essence_of_envy'),
    ('essence_of_hysteria'),
    ('essence_of_horror'),
    ('essence_of_delirium'),
    ('essence_of_insanity')
) AS v(name)
JOIN cause_types ct ON ct.name = 'essence';