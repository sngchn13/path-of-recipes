INSERT INTO item_bases (name, display_en, display_kor, item_type_id)
SELECT v.name, v.display_en, v.display_kor, t.id
FROM (
    VALUES
    ('iron_gauntlets', 'Iron Gauntlets', '철제 건틀릿'),
    ('plated_gauntlets', 'Plated Gauntlets', '판금 건틀릿'),
    ('bronze_gauntlets', 'Bronze Gauntlets', '청동 건틀릿'),
    ('steel_gauntlets', 'Steel Gauntlets', '강철 건틀릿'),
    ('antique_gauntlets', 'Antique Gauntlets', '앤티크 건틀릿'),
    ('ancient_gauntlets', 'Ancient Gauntlets', '고대의 건틀릿'),
    ('goliath_gauntlets', 'Goliath Gauntlets', '거인 건틀릿'),
    ('vaal_gauntlets', 'Vaal Gauntlets', '바알 건틀릿'),
    ('titan_gauntlets', 'Titan Gauntlets', '거신 건틀릿'),
    ('precursor_gauntlets', 'Precursor Gauntlets', '선도자 건틀릿'),
    ('leviathan_gauntlets', 'Leviathan Gauntlets', '레비아탄 건틀릿'),
    ('preserving_gauntlets', 'Preserving Gauntlets', '보전의 건틀릿'),
    ('guarding_gauntlets', 'Guarding Gauntlets', '수호의 건틀릿'),
    ('thwarting_gauntlets', 'Thwarting Gauntlets', '좌절의 건틀릿'),
    ('spiked_gloves', 'Spiked Gloves', '가시 장갑')
) AS v(name, display_en, display_kor)
JOIN item_types t ON t.name = 'gloves_str';

INSERT INTO item_bases (name, display_en, display_kor, item_type_id)
SELECT v.name, v.display_en, v.display_kor, t.id
FROM (
    VALUES
    ('rawhide_gloves', 'Rawhide Gloves', '생가죽 장갑'),
    ('goathide_gloves', 'Goathide Gloves', '염소가죽 장갑'),
    ('deerskin_gloves', 'Deerskin Gloves', '사슴가죽 장갑'),
    ('nubuck_gloves', 'Nubuck Gloves', '소가죽 장갑'),
    ('eelskin_gloves', 'Eelskin Gloves', '장어가죽 장갑'),
    ('sharkskin_gloves', 'Sharkskin Gloves', '상어가죽 장갑'),
    ('shagreen_gloves', 'Shagreen Gloves', '거친 생가죽 장갑'),
    ('stealth_gloves', 'Stealth Gloves', '은신 장갑'),
    ('slink_gloves', 'Slink Gloves', '송아지 가죽 장갑'),
    ('harpyskin_gloves', 'Harpyskin Gloves', '하피가죽 장갑'),
    ('velour_gloves', 'Velour Gloves', '벨루어 장갑'),
    ('tinker_gloves', 'Tinker Gloves', '땜장이 장갑'),
    ('apprentice_gloves', 'Apprentice Gloves', '수습생 장갑'),
    ('trapsetter_gloves', 'Trapsetter Gloves', '덫꾼 장갑'),
    ('gripped_gloves', 'Gripped Gloves', '움켜쥔 장갑')
) AS v(name, display_en, display_kor)
JOIN item_types t ON t.name = 'gloves_dex';

INSERT INTO item_bases (name, display_en, display_kor, item_type_id)
SELECT v.name, v.display_en, v.display_kor, t.id
FROM (
    VALUES
    ('wool_gloves', 'Wool Gloves', '양모 장갑'),
    ('velvet_gloves', 'Velvet Gloves', '벨벳 장갑'),
    ('silk_gloves', 'Silk Gloves', '실크 장갑'),
    ('embroidered_gloves', 'Embroidered Gloves', '수놓은 장갑'),
    ('satin_gloves', 'Satin Gloves', '새틴 장갑'),
    ('samite_gloves', 'Samite Gloves', '금실 장갑'),
    ('conjurer_gloves', 'Conjurer Gloves', '요술사 장갑'),
    ('arcanist_gloves', 'Arcanist Gloves', '신비학자의 장갑'),
    ('sorcerer_gloves', 'Sorcerer Gloves', '악령술사 장갑'),
    ('sage_gloves', 'Sage Gloves', '현자 장갑'),
    ('warlock_gloves', 'Warlock Gloves', '마법사 장갑'),
    ('leyline_gloves', 'Leyline Gloves', '지맥 장갑'),
    ('aetherwind_gloves', 'Aetherwind Gloves', '에테르 바람 장갑'),
    ('nexus_gloves', 'Nexus Gloves', '연결부 장갑'),
    ('fingerless_silk_gloves', 'Fingerless Silk Gloves', '손가락 없는 실크 장갑')
) AS v(name, display_en, display_kor)
JOIN item_types t ON t.name = 'gloves_int';

INSERT INTO item_bases (name, display_en, display_kor, item_type_id)
SELECT v.name, v.display_en, v.display_kor, t.id
FROM (
    VALUES
    ('fishscale_gauntlets', 'Fishscale Gauntlets', '생선 비늘 건틀릿'),
    ('ironscale_gauntlets', 'Ironscale Gauntlets', '철제 비늘 건틀릿'),
    ('bronzescale_gauntlets', 'Bronzescale Gauntlets', '청동 비늘 건틀릿'),
    ('steelscale_gauntlets', 'Steelscale Gauntlets', '강철 비늘 건틀릿'),
    ('serpentscale_gauntlets', 'Serpentscale Gauntlets', '뱀 비늘 건틀릿'),
    ('wyrmscale_gauntlets', 'Wyrmscale Gauntlets', '아룡 비늘 건틀릿'),
    ('hydrascale_gauntlets', 'Hydrascale Gauntlets', '히드라 비늘 건틀릿'),
    ('dragonscale_gauntlets', 'Dragonscale Gauntlets', '용 비늘 건틀릿'),
    ('chimerascale_gauntlets', 'Chimerascale Gauntlets', '키메라비늘 건틀릿'),
    ('wyvernscale_gauntlets', 'Wyvernscale Gauntlets', '비룡비늘 건틀릿')
) AS v(name, display_en, display_kor)
JOIN item_types t ON t.name = 'gloves_str_dex';

INSERT INTO item_bases (name, display_en, display_kor, item_type_id)
SELECT v.name, v.display_en, v.display_kor, t.id
FROM (
    VALUES
    ('chain_gloves', 'Chain Gloves', '사슬 장갑'),
    ('ringmail_gloves', 'Ringmail Gloves', '고리 장갑'),
    ('mesh_gloves', 'Mesh Gloves', '망사 장갑'),
    ('riveted_gloves', 'Riveted Gloves', '고정못 장갑'),
    ('zealot_gloves', 'Zealot Gloves', '맹신자 장갑'),
    ('soldier_gloves', 'Soldier Gloves', '병사 장갑'),
    ('legion_gloves', 'Legion Gloves', '군단 장갑'),
    ('crusader_gloves', 'Crusader Gloves', '십자군 장갑'),
    ('martyr_gloves', 'Martyr Gloves', '순교자 장갑'),
    ('paladin_gloves', 'Paladin Gloves', '성기사 장갑'),
    ('apothecarys_gloves', 'Apothecary''s Gloves', '약제상의 장갑')
) AS v(name, display_en, display_kor)
JOIN item_types t ON t.name = 'gloves_str_int';

INSERT INTO item_bases (name, display_en, display_kor, item_type_id)
SELECT v.name, v.display_en, v.display_kor, t.id
FROM (
    VALUES
    ('wrapped_mitts', 'Wrapped Mitts', '스트랩형 미트'),
    ('strapped_mitts', 'Strapped Mitts', '끈 미트'),
    ('clasped_mitts', 'Clasped Mitts', '버클 미트'),
    ('trapper_mitts', 'Trapper Mitts', '덫사냥꾼 미트'),
    ('ambush_mitts', 'Ambush Mitts', '매복 미트'),
    ('carnal_mitts', 'Carnal Mitts', '육욕의 미트'),
    ('assassins_mitts', 'Assassin''s Mitts', '암살자의 미트'),
    ('murder_mitts', 'Murder Mitts', '살인자의 미트'),
    ('infiltrator_mitts', 'Infiltrator Mitts', '잠입자 미트'),
    ('phantom_mitts', 'Phantom Mitts', '혼령 미트')
) AS v(name, display_en, display_kor)
JOIN item_types t ON t.name = 'gloves_dex_int';