
INSERT INTO item_bases (name, display_en, display_kor, item_type_id)
SELECT v.name, v.display_en, v.display_kor, t.id
FROM (
    VALUES
    ('plate_vest', 'Plate Vest', '판금 조끼'),
    ('chestplate', 'Chestplate', '가슴 보호 갑옷'),
    ('copper_plate', 'Copper Plate', '구리 판금 갑옷'),
    ('war_plate', 'War Plate', '전쟁용 판금 갑옷'),
    ('full_plate', 'Full Plate', '전신 판금 갑옷'),
    ('arena_plate', 'Arena Plate', '투기장 판금 갑옷'),
    ('lordly_plate', 'Lordly Plate', '위풍당당한 판금 갑옷'),
    ('bronze_plate', 'Bronze Plate', '청동 판금 갑옷'),
    ('battle_plate', 'Battle Plate', '전투용 판금 갑옷'),
    ('sun_plate', 'Sun Plate', '태양의 판금 갑옷'),
    ('colosseum_plate', 'Colosseum Plate', '원형 경기장 판금 갑옷'),
    ('majestic_plate', 'Majestic Plate', '압도적인 판금 갑옷'),
    ('golden_plate', 'Golden Plate', '황금 판금 갑옷'),
    ('crusader_plate', 'Crusader Plate', '십자군 판금 갑옷'),
    ('astral_plate', 'Astral Plate', '우주의 판금 갑옷'),
    ('gladiator_plate', 'Gladiator Plate', '검투사 판금 갑옷'),
    ('glorious_plate', 'Glorious Plate', '영광의 판금 갑옷'),
    ('titan_plate', 'Titan Plate', '거신 판금 갑옷'),
    ('legion_plate', 'Legion Plate', '군단 판금 갑옷'),
    ('royal_plate', 'Royal Plate', '왕실 판금 갑옷'),
    ('kaoms_plate', 'Kaom''s Plate', '카옴의 판금 갑옷')
) AS v(name, display_en, display_kor)
JOIN item_types t ON t.name = 'body_armours_str';


INSERT INTO item_bases (name, display_en, display_kor, item_type_id)
SELECT v.name, v.display_en, v.display_kor, t.id
FROM (
    VALUES
    ('shabby_jerkin', 'Shabby Jerkin', '허름한 조끼'),
    ('strapped_leather', 'Strapped Leather', '끈을 멘 가죽 갑옷'),
    ('buckskin_tunic', 'Buckskin Tunic', '사슴가죽 튜닉'),
    ('wild_leather', 'Wild Leather', '거친 가죽 갑옷'),
    ('full_leather', 'Full Leather', '전신 가죽 갑옷'),
    ('sun_leather', 'Sun Leather', '태양의 가죽 갑옷'),
    ('thiefs_garb', 'Thief''s Garb', '도둑의 의복'),
    ('eelskin_tunic', 'Eelskin Tunic', '장어가죽 튜닉'),
    ('frontier_leather', 'Frontier Leather', '개척자 가죽 갑옷'),
    ('glorious_leather', 'Glorious Leather', '영광의 가죽 갑옷'),
    ('coronal_leather', 'Coronal Leather', '광환의 가죽 갑옷'),
    ('cutthroats_garb', 'Cutthroat''s Garb', '살인자의 의복'),
    ('sharkskin_tunic', 'Sharkskin Tunic', '상어가죽 튜닉'),
    ('destiny_leather', 'Destiny Leather', '운명의 가죽 갑옷'),
    ('exquisite_leather', 'Exquisite Leather', '정교한 가죽 갑옷'),
    ('zodiac_leather', 'Zodiac Leather', '성좌의 가죽 갑옷'),
    ('assassins_garb', 'Assassin''s Garb', '암살자의 의복'),
    ('supreme_leather', 'Supreme Leather', '최고급 가죽 갑옷'),
    ('astral_leather', 'Astral Leather', '우주의 가죽 갑옷'),
    ('syndicates_garb', 'Syndicate''s Garb', '연합의 의복')
) AS v(name, display_en, display_kor)
JOIN item_types t ON t.name = 'body_armours_dex';


INSERT INTO item_bases (name, display_en, display_kor, item_type_id)
SELECT v.name, v.display_en, v.display_kor, t.id
FROM (
    VALUES
    ('simple_robe', 'Simple Robe', '단순한 로브'),
    ('silken_vest', 'Silken Vest', '실크 조끼'),
    ('scholars_robe', 'Scholar''s Robe', '학자의 로브'),
    ('silken_garb', 'Silken Garb', '실크 의복'),
    ('mages_vestment', 'Mage''s Vestment', '마법사의 법의'),
    ('silk_robe', 'Silk Robe', '실크 로브'),
    ('cabalist_regalia', 'Cabalist Regalia', '비법가 예복'),
    ('sages_robe', 'Sage''s Robe', '현자의 로브'),
    ('silken_wrap', 'Silken Wrap', '실크 외투'),
    ('conjurers_vestment', 'Conjurer''s Vestment', '요술사의 법의'),
    ('spidersilk_robe', 'Spidersilk Robe', '거미줄 실크 로브'),
    ('destroyer_regalia', 'Destroyer Regalia', '파괴자 예복'),
    ('savants_robe', 'Savant''s Robe', '석학의 로브'),
    ('necromancer_silks', 'Necromancer Silks', '네크로맨서 실크 갑옷'),
    ('occultists_vestment', 'Occultist''s Vestment', '오컬티스트의 법의'),
    ('widowsilk_robe', 'Widowsilk Robe', '과부거미줄 실크 로브'),
    ('vaal_regalia', 'Vaal Regalia', '바알 예복'),
    ('arcane_vestment', 'Arcane Vestment', '비전의 법의'),
    ('nightweave_robe', 'Nightweave Robe', '밤의 직물 로브'),
    ('twilight_regalia', 'Twilight Regalia', '황혼의 예복')
) AS v(name, display_en, display_kor)
JOIN item_types t ON t.name = 'body_armours_int';


INSERT INTO item_bases (name, display_en, display_kor, item_type_id)
SELECT v.name, v.display_en, v.display_kor, t.id
FROM (
    VALUES
    ('scale_vest', 'Scale Vest', '미늘 조끼'),
    ('light_brigandine', 'Light Brigandine', '가벼운 브리간딘'),
    ('scale_doublet', 'Scale Doublet', '비늘 더블릿'),
    ('infantry_brigandine', 'Infantry Brigandine', '보병 브리간딘'),
    ('full_scale_armour', 'Full Scale Armour', '전신 미늘 갑옷'),
    ('soldiers_brigandine', 'Soldier''s Brigandine', '병사용 브리간딘'),
    ('field_lamellar', 'Field Lamellar', '야전 라멜라'),
    ('wyrmscale_doublet', 'Wyrmscale Doublet', '아룡 비늘 더블릿'),
    ('hussar_brigandine', 'Hussar Brigandine', '경기병 브리간딘'),
    ('full_wyrmscale', 'Full Wyrmscale', '전신 아룡 비늘 갑옷'),
    ('commanders_brigandine', 'Commander''s Brigandine', '지휘관의 브리간딘'),
    ('battle_lamellar', 'Battle Lamellar', '전투용 라멜라'),
    ('dragonscale_doublet', 'Dragonscale Doublet', '용 비늘 더블릿'),
    ('desert_brigandine', 'Desert Brigandine', '사막 브리간딘'),
    ('full_dragonscale', 'Full Dragonscale', '전신 용 비늘 갑옷'),
    ('generals_brigandine', 'General''s Brigandine', '장군의 브리간딘'),
    ('triumphant_lamellar', 'Triumphant Lamellar', '승리의 라멜라'),
    ('full_wyvernscale', 'Full Wyvernscale', '전체 비룡비늘 갑옷'),
    ('marshalls_brigandine', 'Marshall''s Brigandine', '원수의 브리간딘'),
    ('conquest_lamellar', 'Conquest Lamellar', '정복 라멜라')
) AS v(name, display_en, display_kor)
JOIN item_types t ON t.name = 'body_armours_str_dex';


INSERT INTO item_bases (name, display_en, display_kor, item_type_id)
SELECT v.name, v.display_en, v.display_kor, t.id
FROM (
    VALUES
    ('chainmail_vest', 'Chainmail Vest', '사슬 조끼'),
    ('chainmail_tunic', 'Chainmail Tunic', '사슬 튜닉'),
    ('ringmail_coat', 'Ringmail Coat', '외투 고리 갑옷'),
    ('chainmail_doublet', 'Chainmail Doublet', '사슬 더블릿'),
    ('full_ringmail', 'Full Ringmail', '전신 고리 갑옷'),
    ('full_chainmail', 'Full Chainmail', '전신 사슬 갑옷'),
    ('holy_chainmail', 'Holy Chainmail', '성스러운 사슬 갑옷'),
    ('latticed_ringmail', 'Latticed Ringmail', '격자형 고리 갑옷'),
    ('crusader_chainmail', 'Crusader Chainmail', '십자군 사슬 갑옷'),
    ('ornate_ringmail', 'Ornate Ringmail', '화려한 고리 갑옷'),
    ('chain_hauberk', 'Chain Hauberk', '쇠사슬 갑옷'),
    ('devout_chainmail', 'Devout Chainmail', '신도 사슬 갑옷'),
    ('loricated_ringmail', 'Loricated Ringmail', '단단한 고리 갑옷'),
    ('conquest_chainmail', 'Conquest Chainmail', '점령자의 사슬 갑옷'),
    ('elegant_ringmail', 'Elegant Ringmail', '고상한 고리 갑옷'),
    ('saints_hauberk', 'Saint''s Hauberk', '성인의 쇠사슬 갑옷'),
    ('saintly_chainmail', 'Saintly Chainmail', '성인의 사슬 갑옷'),
    ('grand_ringmail', 'Grand Ringmail', '우수한 고리 갑옷'),
    ('paladins_hauberk', 'Paladin''s Hauberk', '성기사의 호버크'),
    ('sacred_chainmail', 'Sacred Chainmail', '성스러운 사슬 갑옷')
) AS v(name, display_en, display_kor)
JOIN item_types t ON t.name = 'body_armours_str_int';


INSERT INTO item_bases (name, display_en, display_kor, item_type_id)
SELECT v.name, v.display_en, v.display_kor, t.id
FROM (
    VALUES
    ('padded_vest', 'Padded Vest', '패드 조끼'),
    ('oiled_vest', 'Oiled Vest', '기름칠된 조끼'),
    ('padded_jacket', 'Padded Jacket', '패드 재킷'),
    ('oiled_coat', 'Oiled Coat', '기름칠된 외투'),
    ('scarlet_raiment', 'Scarlet Raiment', '다홍색 의복'),
    ('waxed_garb', 'Waxed Garb', '밀랍재질 의복'),
    ('bone_armour', 'Bone Armour', '뼈 갑옷'),
    ('quilted_jacket', 'Quilted Jacket', '누비 재킷'),
    ('sleek_coat', 'Sleek Coat', '매끈한 외투'),
    ('crimson_raiment', 'Crimson Raiment', '진홍색 의복'),
    ('lacquered_garb', 'Lacquered Garb', '옻칠한 의복'),
    ('crypt_armour', 'Crypt Armour', '지하실 갑옷'),
    ('sentinel_jacket', 'Sentinel Jacket', '파수꾼 재킷'),
    ('varnished_coat', 'Varnished Coat', '광택 나는 외투'),
    ('blood_raiment', 'Blood Raiment', '피의 의복'),
    ('sadist_garb', 'Sadist Garb', '가학자 의복'),
    ('carnal_armour', 'Carnal Armour', '육욕의 갑옷'),
    ('sanguine_raiment', 'Sanguine Raiment', '핏빛 의상'),
    ('torturer_garb', 'Torturer Garb', '고문자의 의복'),
    ('necrotic_armour', 'Necrotic Armour', '괴사의 방어구')
) AS v(name, display_en, display_kor)
JOIN item_types t ON t.name = 'body_armours_dex_int';


INSERT INTO item_bases (name, display_en, display_kor, item_type_id)
SELECT v.name, v.display_en, v.display_kor, t.id
FROM (
    VALUES
    ('sacrificial_garb', 'Sacrificial Garb', '희생의 의복'),
    ('golden_mantle', 'Golden Mantle', '황금 외투'),
    ('grasping_mail', 'Grasping Mail', '탐욕스러운 사슬옷')
) AS v(name, display_en, display_kor)
JOIN item_types t ON t.name = 'body_armours_str_dex_int';