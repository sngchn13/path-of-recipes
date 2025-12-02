INSERT INTO item_bases (name, display_en, display_kor, item_type_id)
SELECT v.name, v.display_en, v.display_kor, t.id
FROM (
    VALUES
    ('iron_greaves', 'Iron Greaves', '철제 각반'),
    ('steel_greaves', 'Steel Greaves', '강철 각반'),
    ('plated_greaves', 'Plated Greaves', '판금 각반'),
    ('reinforced_greaves', 'Reinforced Greaves', '강화 각반'),
    ('antique_greaves', 'Antique Greaves', '앤티크 각반'),
    ('ancient_greaves', 'Ancient Greaves', '고대 각반'),
    ('goliath_greaves', 'Goliath Greaves', '거인 각반'),
    ('vaal_greaves', 'Vaal Greaves', '바알 각반'),
    ('titan_greaves', 'Titan Greaves', '거신 각반'),
    ('precursor_greaves', 'Precursor Greaves', '선도자 각반'),
    ('leviathan_greaves', 'Leviathan Greaves', '레비아탄 각반'),
    ('basemetal_treads', 'Basemetal Treads', '일반 금속 발 보호대'),
    ('darksteel_treads', 'Darksteel Treads', '검은 강철 발 보호대'),
    ('brimstone_treads', 'Brimstone Treads', '유황 발 보호대'),
    ('kaoms_greaves', 'Kaom''s Greaves', '카옴의 각반')
) AS v(name, display_en, display_kor)
JOIN item_types t ON t.name = 'boots(str)';

INSERT INTO item_bases (name, display_en, display_kor, item_type_id)
SELECT v.name, v.display_en, v.display_kor, t.id
FROM (
    VALUES
    ('rawhide_boots', 'Rawhide Boots', '생가죽 장화'),
(   'goathide_boots', 'Goathide Boots', '염소가죽 장화'),
    ('deerskin_boots', 'Deerskin Boots', '사슴가죽 장화'),
    ('nubuck_boots', 'Nubuck Boots', '소가죽 장화'),
    ('eelskin_boots', 'Eelskin Boots', '장어가죽 장화'),
    ('sharkskin_boots', 'Sharkskin Boots', '상어가죽 장화'),
    ('shagreen_boots', 'Shagreen Boots', '거친 생가죽 장화'),
    ('stealth_boots', 'Stealth Boots', '은신 장화'),
    ('slink_boots', 'Slink Boots', '송아지 가죽 장화'),
    ('harpyskin_boots', 'Harpyskin Boots', '하피가죽 장화'),
    ('velour_boots', 'Velour Boots', '벨루어 장화'),
    ('cloudwhisper_boots', 'Cloudwhisper Boots', '구름속삭임 장화'),
    ('windbreak_boots', 'Windbreak Boots', '바람가름 장화'),
    ('stormrider_boots', 'Stormrider Boots', '태풍의 기수 장화')
) AS v(name, display_en, display_kor)
JOIN item_types t ON t.name = 'boots(dex)';

INSERT INTO item_bases (name, display_en, display_kor, item_type_id)
SELECT v.name, v.display_en, v.display_kor, t.id
FROM (
    VALUES
    ('wool_shoes', 'Wool Shoes', '양모 신발'),
    ('velvet_slippers', 'Velvet Slippers', '벨벳 슬리퍼'),
    ('silk_slippers', 'Silk Slippers', '실크 슬리퍼'),
    ('scholar_boots', 'Scholar Boots', '학자 장화'),
    ('satin_slippers', 'Satin Slippers', '새틴 슬리퍼'),
    ('samite_slippers', 'Samite Slippers', '금실 슬리퍼'),
    ('conjurer_boots', 'Conjurer Boots', '요술사 장화'),
    ('arcanist_slippers', 'Arcanist Slippers', '신비학자의 슬리퍼'),
    ('sorcerer_boots', 'Sorcerer Boots', '악령술사 장화'),
    ('sage_slippers', 'Sage Slippers', '현자 슬리퍼'),
    ('warlock_boots', 'Warlock Boots', '마법사 장화'),
    ('duskwalk_slippers', 'Duskwalk Slippers', '황혼걸음 슬리퍼'),
    ('nightwind_slippers', 'Nightwind Slippers', '밤바람 슬리퍼'),
    ('dreamquest_slippers', 'Dreamquest Slippers', '꿈의 정벌 슬리퍼')
) AS v(name, display_en, display_kor)
JOIN item_types t ON t.name = 'boots(int)';

INSERT INTO item_bases (name, display_en, display_kor, item_type_id)
SELECT v.name, v.display_en, v.display_kor, t.id
FROM (
    VALUES
    ('leatherscale_boots', 'Leatherscale Boots', '가죽 비늘 장화'),
    ('ironscale_boots', 'Ironscale Boots', '철제 비늘 장화'),
    ('bronzescale_boots', 'Bronzescale Boots', '청동 비늘 장화'),
    ('steelscale_boots', 'Steelscale Boots', '강철 비늘 장화'),
    ('serpentscale_boots', 'Serpentscale Boots', '뱀 비늘 장화'),
    ('wyrmscale_boots', 'Wyrmscale Boots', '아룡 비늘 장화'),
    ('hydrascale_boots', 'Hydrascale Boots', '히드라 비늘 장화'),
    ('dragonscale_boots', 'Dragonscale Boots', '용 비늘 장화'),
    ('chimerascale_boots', 'Chimerascale Boots', '키메라비늘 장화'),
    ('wyvernscale_boots', 'Wyvernscale Boots', '비룡비늘 장화'),
    ('two_toned_boots_fire_cold', 'Two-Toned Boots', '두 가지 색 장화')
) AS v(name, display_en, display_kor)
JOIN item_types t ON t.name = 'boots(str_dex)';

INSERT INTO item_bases (name, display_en, display_kor, item_type_id)
SELECT v.name, v.display_en, v.display_kor, t.id
FROM (
    VALUES
    ('chain_boots', 'Chain Boots', '사슬 장화'),
    ('ringmail_boots', 'Ringmail Boots', '고리 장화'),
    ('mesh_boots', 'Mesh Boots', '망사 장화'),
    ('riveted_boots', 'Riveted Boots', '고정못 장화'),
    ('zealot_boots', 'Zealot Boots', '맹신자 장화'),
    ('soldier_boots', 'Soldier Boots', '병사 장화'),
    ('legion_boots', 'Legion Boots', '군단 장화'),
    ('crusader_boots', 'Crusader Boots', '십자군 장화'),
    ('martyr_boots', 'Martyr Boots', '순교자 장화'),
    ('paladin_boots', 'Paladin Boots', '성기사 장화'),
    ('two_toned_boots_fire_lightning', 'Two-Toned Boots', '두 가지 색 장화')
) AS v(name, display_en, display_kor)
JOIN item_types t ON t.name = 'boots(str_int)';

INSERT INTO item_bases (name, display_en, display_kor, item_type_id)
SELECT v.name, v.display_en, v.display_kor, t.id
FROM (
    VALUES
    ('wrapped_boots', 'Wrapped Boots', '스트랩형 장화'),
    ('strapped_boots', 'Strapped Boots', '끈 장화'),
    ('clasped_boots', 'Clasped Boots', '버클 장화'),
    ('shackled_boots', 'Shackled Boots', '섀클 장화'),
    ('trapper_boots', 'Trapper Boots', '덫사냥꾼 장화'),
    ('ambush_boots', 'Ambush Boots', '매복 장화'),
    ('carnal_boots', 'Carnal Boots', '육욕의 장화'),
    ('assassins_boots', 'Assassin''s Boots', '암살자의 장화'),
    ('murder_boots', 'Murder Boots', '살인자의 장화'),
    ('infiltrator_boots', 'Infiltrator Boots', '잠입자 장화'),
    ('phantom_boots', 'Phantom Boots', '혼령 장화'),
    ('two_toned_boots_cold_lightning', 'Two-Toned Boots', '두 가지 색 장화'),
    ('fugitive_boots', 'Fugitive Boots', '도망자 장화')
) AS v(name, display_en, display_kor)
JOIN item_types t ON t.name = 'boots(dex_int)';