
INSERT INTO causes (name, cause_type_id)
SELECT v.name, ct.id
FROM (
    VALUES
    ('chaos_orb'),
    ('orb_of_alchemy'),
    ('orb_of_transmutation'),
    ('orb_of_alteration'),
    ('orb_of_scouring'),
    ('exalted_orb'),
    ('regal_orb'),
    ('orb_of_augmentation'),
    ('vaal_orb'),
    ('orb_of_annulment'),

    ('veiled_exalted_orb'),
    ('veiled_chaos_orb'),

    ('shapers_exalted_orb'),
    ('elders_exalted_orb'),
    ('crusaders_exalted_orb'),
    ('redeemers_exalted_orb'),
    ('hunters_exalted_orb'),
    ('warloads_exalted_orb'),
    ('awakeners_orb'),
    ('orb_of_dominance'),

    ('lesser_eldritch_ember'),
    ('greater_eldritch_ember'),
    ('grand_eldritch_ember'),
    ('exceptional_eldritch_ember'),
    ('lesser_eldritch_ichor'),
    ('greater_eldritch_ichor'),
    ('grand_eldritch_ichor'),
    ('exceptional_eldritch_ichor'),
    ('orb_of_conflict'),
    
    ('eldritch_chaos_orb'),
    ('eldritch_exalted_orb'),
    ('eldritch_orb_of_annulment')
) AS v(name)
JOIN cause_types ct ON ct.name = 'orb';