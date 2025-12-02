
INSERT INTO causes (name, cause_type_id)
SELECT v.name, ct.id
FROM (
    VALUES
    ('deafening_essence_of_hatred'),
    ('deafening_essence_of_woe'),
    ('deafening_essence_of_greed'),
    ('deafening_essence_of_contempt'),
    ('deafening_essence_of_sorrow'),
    ('deafening_essence_of_anger'),
    ('deafening_essence_of_torment'),
    ('deafening_essence_of_rage'),
    ('deafening_essence_of_wrath'),
    ('deafening_essence_of_doubt'),
    ('deafening_essence_of_anguish'),
    ('deafening_essence_of_loathing'),
    ('deafening_essence_of_spite'),
    ('deafening_essence_of_zeal'),
    ('deafening_essence_of_misery'),
    ('deafening_essence_of_dread'),
    ('deafening_essence_of_scorn'),
    ('deafening_essence_of_envy'),
    ('deafening_essence_of_hysteria'),
    ('deafening_essence_of_horror'),
    ('deafening_essence_of_delirium')
) AS v(name)
JOIN cause_types ct ON ct.name = 'essence';