

INSERT INTO item_bases (name, display_en, display_kor, item_type_id)
SELECT v.name, v.display_en, v.display_kor, t.id
FROM (
    VALUES
    ('nailed_fist', 'Nailed Fist', '못 박은 너클' ),
    ('sharktooth_claw', 'Sharktooth Claw' ,'상어이빨 클로'),
    ('awl', 'Awl', '송곳'),
    ('cats_paw', 'Cat''paw', '고양이 발'),
    ('blinder', 'Blinder', '실명기'),
    ('timeworn_claw', 'Timeworn Claw', '오래된 클로'),
    ('sparkling_claw', 'Sparkling Claw', '섬광 클로'),
    ('fright_claw', 'Fright Claw', '섬뜩한 클로'),
    ('double_claw', 'Double Claw', '이중 클로'),
    ('thresher_claw', 'Thresher Claw', '도리깨 클로'),
    ('gouger', 'Gouger', '후비개'),
    ('tigers_claw', 'Tiger''s Paw', '호랑이 발'),
    ('gut_ripper', 'Gut Ripper', '내장 거열기'),
    ('prehistoric_claw', 'Prehistoric Claw', '선사시대 클로'),
    ('noble_claw', 'Noble Claw', '귀족 클로'),
    ('eagle_claw', 'Eagle Claw', '독수리 클로'),
    ('twin_claw', 'Twin Claw', '쌍둥이 클로'),
    ('great_white_claw', 'Great White Claw', '거대 하얀 클로'),
    ('throat_stabber', 'Throat Stabber', '목구멍 송곳'),
    ('hellions_paw', 'Hellion''s Paw', '헬리온의 발'),
    ('eye_gouger', 'Eye Gouger', '눈 후비개'),
    ('vaal_claw', 'Vaal Claw', '바알 클로'),
    ('imperial_claw', 'Imperial Claw', '제국 클로'),
    ('terror_claw', 'Terror Claw', '공포의 클로'),
    ('gemini_claw', 'Gemini Claw', '쌍생아 클로'),
    ('shadow_fangs', 'Shadow Fangs', '그림자 송곳니'),
    ('malign_fangs', 'Malign Fangs', '해악의 송곳니'),
    ('void_fangs', 'Void Fangs', '공허의 송곳니')
) AS v(name, display_en, display_kor)
JOIN item_types t ON t.name = 'claws';




INSERT INTO item_bases (name, display_en, display_kor, item_type_id)
SELECT v.name, v.display_en, v.display_kor, t.id
FROM (
    VALUES
    ('glass_shank', 'Glass Shank', '유리 단도'),
    ('skinning_knife', 'Skinning Knife', '무두질용 단도'),
    ('stiletto', 'Stiletto', '스틸레토'),
    ('prong_dagger', 'Prong Dagger', '갈래 단검'),
    ('flaying_knife', 'Flaying Knife', '박피 단도'),
    ('poignard', 'Poignard', '포이너드'),
    ('trisula', 'Trisula', '트리술라'),
    ('gutting_knife', 'Gutting Knife', '내장 제거용 단도'),
    ('ambusher', 'Ambusher', '매복자'),
    ('sai', 'Sai', '사이'),
    ('hollowpoint_dagger', 'Hollowpoint Dagger', '중공 단검'),
    ('pressurised_dagger', 'Pressurised Dagger', '가압 단검'),
    ('pneumatic_dagger', 'Pneumatic Dagger', '공압 단검'),
    ('ethereal_blade', 'Ethereal Blade', '천상의 칼날')
) AS v(name, display_en, display_kor)
JOIN item_types t ON t.name = 'daggers';


INSERT INTO item_bases (name, display_en, display_kor, item_type_id)
SELECT v.name, v.display_en, v.display_kor, t.id
FROM (
    VALUES
    ('driftwood_wand', 'Driftwood Wand', '표류목 마법봉'),
    ('goats_horn', 'Goat''s Horn', '염소의 뿔'),
    ('carved_wand', 'Carved Wand', '조각된 마법봉'),
    ('quartz_wand', 'Quartz Wand', '석영 마법봉'),
    ('spiraled_wand', 'Spiraled Wand', '나선형 마법봉'),
    ('sage_wand', 'Sage Wand', '현자 마법봉'),
    ('pagan_wand', 'Pagan Wand', '이교도 마법봉'),
    ('fauns_horn', 'Faun''s Horn', '판의 뿔'),
    ('engraved_wand', 'Engraved Wand', '아로새긴 마법봉'),
    ('crystal_wand', 'Crystal Wand', '수정 마법봉'),
    ('coiled_wand', 'Coiled Wand', '뙤리 마법봉'),
    ('omen_wand', 'Omen Wand', '전조의 마법봉'),
    ('heathen_wand', 'Heathen Wand', '이방인 마법봉'),
    ('demons_horn', 'Demon''s Horn', '악마의 뿔'),
    ('imbued_wand', 'Imbued Wand', '부여된 마법봉'),
    ('opal_wand', 'Opal Wand', '오팔 마법봉'),
    ('tornado_wand', 'Tornado Wand', '회오리 마법봉'),
    ('prophecy_wand', 'Prophecy Wand', '예언 마법봉'),
    ('profane_wand', 'Profane Wand', '부정한 마법봉'),
    ('assembler_wand', 'Assembler Wand', '조립 마법봉'),
    ('congregator_wand', 'Congregator Wand', '군집 마법봉'),
    ('accumulator_wand', 'Accumulator Wand', '축적 마법봉'),
    ('somatic_wand', 'Somatic Wand', '신체의 마법봉'),
    ('blasting_wand', 'Blasting Wand', '폭발하는 마법봉'),
    ('kinetic_wand', 'Kinetic Wand', '역학 마법봉'),
    ('calling_wand', 'Calling Wand', '부름의 마법봉'),
    ('convening_wand', 'Convening Wand', '회합의 마법봉'),
    ('convoking_wand', 'Convoking Wand', '소집의 마법봉')
) AS v(name, display_en, display_kor)
JOIN item_types t ON t.name = 'wands';

INSERT INTO item_bases (name, display_en, display_kor, item_type_id)
SELECT v.name, v.display_en, v.display_kor, t.id
FROM (
    VALUES
    ('charans_sword', 'Charan''s Sword', '차란의 검'),
    ('golden_blade', 'Golden Blade', '황금 검'),
    ('rusted_sword', 'Rusted Sword', '녹슨 검'),
    ('copper_sword', 'Copper Sword', '구리 검'),
    ('sabre', 'Sabre', '사브르'),
    ('broad_sword', 'Broad Sword', '넓은날 검'),
    ('war_sword', 'War Sword', '전쟁 검'),
    ('ancient_sword', 'Ancient Sword', '고대의 검'),
    ('elegant_sword', 'Elegant Sword', '고상한 검'),
    ('dusk_blade', 'Dusk Blade', '황혼의 검'),
    ('hook_sword', 'Hook Sword', '갈고리 검'),
    ('variscite_blade', 'Variscite Blade', '바리사이트 검'),
    ('cutlass', 'Cutlass', '커틀라스'),
    ('baselard', 'Baselard', '베이스라드'),
    ('battle_sword', 'Battle Sword', '전투용 검'),
    ('elder_sword', 'Elder Sword', '장로의 검'),
    ('graceful_sword', 'Graceful Sword', '우아한 검'),
    ('twilight_blade', 'Twilight Blade', '여명의 검'),
    ('grappler', 'Grappler', '격투자'),
    ('gemstone_sword', 'Gemstone Sword', '원석 검'),
    ('corsair_sword', 'Corsair Sword', '해적의 검'),
    ('gladius', 'Gladius', '글라디우스'),
    ('legion_sword', 'Legion Sword', '군단 검'),
    ('vaal_blade', 'Vaal Blade', '바알 칼날'),
    ('eternal_sword', 'Eternal Sword', '영원의 검'),
    ('midnight_blade', 'Midnight Blade', '심야의 칼날'),
    ('tiger_hook', 'Tiger Hook', '호랑이 갈고리'),
    ('fickle_spiritblade', 'Fickle Spiritblade', '변화무쌍한 정령의 칼'),
    ('capricious_spiritblade', 'Capricious Spiritblade', '변덕스러운 정령의 칼'),
    ('anarchic_spiritblade', 'Anarchic Spiritblade', '무질서한 정령의 칼')
) AS v(name, display_en, display_kor)
JOIN item_types t ON t.name = 'one_hand_swords';

INSERT INTO item_bases (name, display_en, display_kor, item_type_id)
SELECT v.name, v.display_en, v.display_kor, t.id
FROM (
    VALUES
    ('rusted_hatchet', 'Rusted Hatchet', '녹슨 손도끼'),
    ('jade_hatchet', 'Jade Hatchet', '비취 손도끼'),
    ('boarding_axe', 'Boarding Axe', '선상 도끼'),
    ('cleaver', 'Cleaver', '가로날도끼'),
    ('broad_axe', 'Broad Axe', '넓은날 도끼'),
    ('arming_axe', 'Arming Axe', '전투용 도끼'),
    ('decorative_axe', 'Decorative Axe', '장식된 도끼'),
    ('spectral_axe', 'Spectral Axe', '영체 도끼'),
    ('etched_hatchet', 'Etched Hatchet', '조각된 손도끼'),
    ('jasper_axe', 'Jasper Axe', '벽옥 손도끼'),
    ('tomahawk', 'Tomahawk', '토마호크'),
    ('wrist_chopper', 'Wrist Chopper', '팔목 절단기'),
    ('war_axe', 'War Axe', '전쟁용 도끼'),
    ('chest_splitter', 'Chest Splitter', '가르기용 스플리터'),
    ('ceremonial_axe', 'Ceremonial Axe', '의식용 도끼'),
    ('wraith_axe', 'Wraith Axe', '악령 도끼'),
    ('engraved_hatchet', 'Engraved Hatchet', '아로새긴 손도끼'),
    ('karui_axe', 'Karui Axe', '카루이 도끼'),
    ('siege_axe', 'Siege Axe', '공성용 도끼'),
    ('reaver_axe', 'Reaver Axe', '강탈자 도끼'),
    ('butcher_axe', 'Butcher Axe', '도살자 도끼'),
    ('vaal_hatchet', 'Vaal Hatchet', '바알 손도끼'),
    ('royal_axe', 'Royal Axe', '왕실 도끼'),
    ('infernal_axe', 'Infernal Axe', '지옥불 도끼'),
    ('runic_hatchet', 'Runic Hatchet', '룬 손도끼'),
    ('maltreatment_axe', 'Maltreatment Axe', '가학의 도끼'),
    ('disapprobation_axe', 'Disapprobation Axe', '책망의 도끼'),
    ('psychotic_axe', 'Psychotic Axe', '광기의 도끼')
) AS v(name, display_en, display_kor)
JOIN item_types t ON t.name = 'one_hand_axes';

INSERT INTO item_bases (name, display_en, display_kor, item_type_id)
SELECT v.name, v.display_en, v.display_kor, t.id
FROM (
    VALUES
    ('driftwood_club', 'Driftwood Club', '표류목 곤봉'),
    ('tribal_club', 'Tribal Club', '부족 곤봉'),
    ('spiked_club', 'Spiked Club', '가시 박힌 곤봉'),
    ('stone_hammer', 'Stone Hammer', '쇄석 망치'),
    ('war_hammer', 'War Hammer', '전쟁 망치'),
    ('bladed_mace', 'Bladed Mace', '톱날 철퇴'),
    ('ceremonial_mace', 'Ceremonial Mace', '의식용 철퇴'),
    ('dream_mace', 'Dream Mace', '몽환의 철퇴'),
    ('wyrm_mace', 'Wyrm Mace', '아룡 철퇴'),
    ('petrified_club', 'Petrified Club', '석화된 곤봉'),
    ('barbed_club', 'Barbed Club', '가시 돋친 곤봉'),
    ('rock_breaker', 'Rock Breaker', '바위 분쇄기'),
    ('battle_hammer', 'Battle Hammer', '전투용 망치'),
    ('flanged_mace', 'Flanged Mace', '테두리 철퇴'),
    ('ornate_mace', 'Ornate Mace', '화려한 철퇴'),
    ('phantom_mace', 'Phantom Mace', '혼령 철퇴'),
    ('dragon_mace', 'Dragon Mace', '용 철퇴'),
    ('ancestral_club', 'Ancestral Club', '선대의 곤봉'),
    ('tenderizer', 'Tenderizer', '고기 망치'),
    ('gavel', 'Gavel', '의사봉'),
    ('legion_hammer', 'Legion Hammer', '군단 망치'),
    ('pernach', 'Pernach', '페르나치'),
    ('auric_mace', 'Auric Mace', '금빛 철퇴'),
    ('nightmare_mace', 'Nightmare Mace', '악몽의 철퇴'),
    ('behemoth_mace', 'Behemoth Mace', '거대한 철퇴'),
    ('flare_mace', 'Flare Mace', '조명탄 철퇴'),
    ('crack_mace', 'Crack Mace', '균열 철퇴'),
    ('boom_mace', 'Boom Mace', '우뢰 철퇴')
) AS v(name, display_en, display_kor)
JOIN item_types t ON t.name = 'one_hand_maces';

INSERT INTO item_bases (name, display_en, display_kor, item_type_id)
SELECT v.name, v.display_en, v.display_kor, t.id
FROM (
    VALUES
    ('driftwood_sceptre', 'Driftwood Sceptre', '표류목 셉터'),
    ('darkwood_sceptre', 'Darkwood Sceptre', '흑단목 셉터'),
    ('bronze_sceptre', 'Bronze Sceptre', '청동 셉터'),
    ('quartz_sceptre', 'Quartz Sceptre', '석영 셉터'),
    ('iron_sceptre', 'Iron Sceptre', '철제 셉터'),
    ('ochre_sceptre', 'Ochre Sceptre', '오커 셉터'),
    ('ritual_sceptre', 'Ritual Sceptre', '의식용 셉터'),
    ('shadow_sceptre', 'Shadow Sceptre', '그림자 셉터'),
    ('horned_sceptre', 'Horned Sceptre', '뿔 달린 셉터'),
    ('grinning_fetish', 'Grinning Fetish', '웃는 조각상 홀'),
    ('sekhem', 'Sekhem', '세크헴'),
    ('crystal_sceptre', 'Crystal Sceptre', '수정 셉터'),
    ('lead_sceptre', 'Lead Sceptre', '납 셉터'),
    ('blood_sceptre', 'Blood Sceptre', '피의 셉터'),
    ('royal_sceptre', 'Royal Sceptre', '왕실의 셉터'),
    ('abyssal_sceptre', 'Abyssal Sceptre', '심연의 셉터'),
    ('stag_sceptre', 'Stag Sceptre', '수사슴 셉터'),
    ('karui_sceptre', 'Karui Sceptre', '카루이 셉터'),
    ('tyrants_sekhem', 'Tyrant''s Sekhem', '폭군의 세크헴'),
    ('opal_sceptre', 'Opal Sceptre', '오팔 셉터'),
    ('platinum_sceptre', 'Platinum Sceptre', '백금 셉터'),
    ('vaal_sceptre', 'Vaal Sceptre', '바알 셉터'),
    ('carnal_sceptre', 'Carnal Sceptre', '육욕의 셉터'),
    ('void_sceptre', 'Void Sceptre', '공허의 셉터'),
    ('sambar_sceptre', 'Sambar Sceptre', '삼바 셉터'),
    ('oscillating_sceptre', 'Oscillating Sceptre', '진동 셉터'),
    ('stabilising_sceptre', 'Stabilising Sceptre', '안정화의 셉터'),
    ('alternating_sceptre', 'Alternating Sceptre', '갈마드는 셉터')
) AS v(name, display_en, display_kor)
JOIN item_types t ON t.name = 'sceptres';

INSERT INTO item_bases (name, display_en, display_kor, item_type_id)
SELECT v.name, v.display_en, v.display_kor, t.id
FROM (
    VALUES
    ('carving_knife', 'Carving Knife', '조각용 단도'),
    ('boot_knife', 'Boot Knife', '주머니 단도'),
    ('copper_kris', 'Copper Kris', '구리 크리스'),
    ('skean', 'Skean', '양날 단검'),
    ('imp_dagger', 'Imp Dagger', '임프 단검'),
    ('butcher_knife', 'Butcher Knife', '도살자 단도'),
    ('boot_blade', 'Boot Blade', '주머니칼'),
    ('golden_kris', 'Golden Kris', '황금 크리스'),
    ('royal_skean', 'Royal Skean', '왕실 양날 단검'),
    ('fiend_dagger', 'Fiend Dagger', '마귀 단검'),
    ('slaughter_knife', 'Slaughter Knife', '살육 단도'),
    ('ezomyte_dagger', 'Ezomyte Dagger', '에조미어 단검'),
    ('platinum_kris', 'Platinum Kris', '백금 크리스'),
    ('imperial_skean', 'Imperial Skean', '제국 양날 단검'),
    ('demon_dagger', 'Demon Dagger', '악마 단검'),
    ('flickerflame_blade', 'Flickerflame Blade', '일렁이는 화염 칼'),
    ('flashfire_blade', 'Flashfire Blade', '번쩍이는 불꽃 칼'),
    ('infernal_blade', 'Infernal Blade', '지옥불 칼')
) AS v(name, display_en, display_kor)
JOIN item_types t ON t.name = 'rune_daggers';

INSERT INTO item_bases (name, display_en, display_kor, item_type_id)
SELECT v.name, v.display_en, v.display_kor, t.id
FROM (
    VALUES
    ('rusted_spike', 'Rusted Spike', '녹슨 스파이크'),
    ('whalebone_rapier', 'Whalebone Rapier', '고래뼈 레이피어'),
    ('battered_foil', 'Battered Foil', '닳은 펜싱 검'),
    ('basket_rapier', 'Basket Rapier', '손보호구 레이피어'),
    ('jagged_foil', 'Jagged Foil', '삐죽삐죽한 펜싱 검'),
    ('antique_rapier', 'Antique Rapier', '앤티크 레이피어'),
    ('elegant_foil', 'Elegant Foil', '고상한 펜싱 검'),
    ('thorn_rapier', 'Thorn Rapier', '가시 레이피어'),
    ('smallsword', 'Smallsword', '스몰 소드'),
    ('wyrmbone_rapier', 'Wyrmbone Rapier', '아룡뼈 레이피어'),
    ('burnished_foil', 'Burnished Foil', '윤이 나는 펜싱 검'),
    ('estoc', 'Estoc', '에스톡'),
    ('serrated_foil', 'Serrated Foil', '톱니 펜싱 검'),
    ('primeval_rapier', 'Primeval Rapier', '원시 레이피어'),
    ('fancy_foil', 'Fancy Foil', '장식용 펜싱 검'),
    ('apex_rapier', 'Apex Rapier', '정점 레이피어'),
    ('courtesan_sword', 'Courtesan Sword', '코르티잔 검'),
    ('dragonbone_rapier', 'Dragonbone Rapier', '용뼈 레이피어'),
    ('tempered_foil', 'Tempered Foil', '단조한 펜싱 검'),
    ('pecoraro', 'Pecoraro', '페코라로'),
    ('spiraled_foil', 'Spiraled Foil', '나선형 펜싱 검'),
    ('vaal_rapier', 'Vaal Rapier', '바알 레이피어'),
    ('jewelled_foil', 'Jewelled Foil', '보석이 박힌 펜싱 검'),
    ('harpy_rapier', 'Harpy Rapier', '하피 레이피어'),
    ('dragoon_sword', 'Dragoon Sword', '용기병 검')
) AS v(name, display_en, display_kor)
JOIN item_types t ON t.name = 'thrusting_one_hand_swords';