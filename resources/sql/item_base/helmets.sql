INSERT INTO item_bases (name, display_en, display_kor, item_type_id)
SELECT v.name, v.display_en, v.display_kor, t.id
FROM (
    VALUES
    ('iron_hat', 'Iron Hat', '철제 투구'),
    ('cone_helmet', 'Cone Helmet', '원뿔 투구'),
    ('barbute_helmet', 'Barbute Helmet', '철갑 투구'),
    ('close_helmet', 'Close Helmet', '폐쇄형 투구'),
    ('gladiator_helmet', 'Gladiator Helmet', '검투사 투구'),
    ('reaver_helmet', 'Reaver Helmet', '강탈자 투구'),
    ('siege_helmet', 'Siege Helmet', '공성 투구'),
    ('samnite_helmet', 'Samnite Helmet', '삼니움 투구'),
    ('ezomyte_burgonet', 'Ezomyte Burgonet', '에조미어 기병 투구'),
    ('royal_burgonet', 'Royal Burgonet', '왕실 기병 투구'),
    ('eternal_burgonet', 'Eternal Burgonet', '영원의 기병 투구'),
    ('generals_helmet', 'General''s Helmet', '장군의 투구'),
    ('conquerors_helmet', 'Conqueror''s Helmet', '정복자의 투구'),
    ('giantslayer_helmet', 'Giantslayer Helmet', '거인 처형자 투구')
) AS v(name, display_en, display_kor)
JOIN item_types t ON t.name = 'helmets_str';

INSERT INTO item_bases (name, display_en, display_kor, item_type_id)
SELECT v.name, v.display_en, v.display_kor, t.id
FROM (
    VALUES
    ('leather_cap', 'Leather Cap', '가죽 모자'),
    ('tricorne', 'Tricorne', '삼각모'),
    ('leather_hood', 'Leather Hood', '가죽 두건'),
    ('wolf_pelt', 'Wolf Pelt', '늑대 가죽모'),
    ('hunter_hood', 'Hunter Hood', '사냥꾼 두건'),
    ('noble_tricorne', 'Noble Tricorne', '귀족 삼각모'),
    ('ursine_pelt', 'Ursine Pelt', '곰 가죽모'),
    ('silken_hood', 'Silken Hood', '실크 두건'),
    ('sinner_tricorne', 'Sinner Tricorne', '죄인 삼각모'),
    ('lion_pelt', 'Lion Pelt', '사자 가죽모'),
    ('dire_pelt', 'Dire Pelt', '지독한 가죽모'),
    ('grizzly_pelt', 'Grizzly Pelt', '반백의 가죽모'),
    ('majestic_pelt', 'Majestic Pelt', '압도적인 가죽모')
) AS v(name, display_en, display_kor)
JOIN item_types t ON t.name = 'helmets_dex';

INSERT INTO item_bases (name, display_en, display_kor, item_type_id)
SELECT v.name, v.display_en, v.display_kor, t.id
FROM (
    VALUES
    ('vine_circlet', 'Vine Circlet', '덩굴 관'),
    ('iron_circlet', 'Iron Circlet', '철제 관'),
    ('torture_cage', 'Torture Cage', '고문용 창살'),
    ('tribal_circlet', 'Tribal Circlet', '부족 관'),
    ('bone_circlet', 'Bone Circlet', '뼈의 관'),
    ('lunaris_circlet', 'Lunaris Circlet', '달의 관'),
    ('steel_circlet', 'Steel Circlet', '강철 관'),
    ('necromancer_circlet', 'Necromancer Circlet', '네크로맨서 관'),
    ('solaris_circlet', 'Solaris Circlet', '태양의 관'),
    ('mind_cage', 'Mind Cage', '정신의 창살'),
    ('hubris_circlet', 'Hubris Circlet', '자만의 관'),
    ('moonlit_circlet', 'Moonlit Circlet', '달빛 관'),
    ('sunfire_circlet', 'Sunfire Circlet', '태양불 관'),
    ('lichs_circlet', 'Lich''s Circlet', '리치의 관')
) AS v(name, display_en, display_kor)
JOIN item_types t ON t.name = 'helmets_int';

INSERT INTO item_bases (name, display_en, display_kor, item_type_id)
SELECT v.name, v.display_en, v.display_kor, t.id
FROM (
    VALUES
    ('battered_helm', 'Battered Helm', '닳은 헬름'),
    ('sallet', 'Sallet', '샐릿'),
    ('visored_sallet', 'Visored Sallet', '차양 단 샐릿'),
    ('gilded_sallet', 'Gilded Sallet', '도금 샐릿'),
    ('secutor_helm', 'Secutor Helm', '호위병 샐릿'),
    ('fencer_helm', 'Fencer Helm', '검사 샐릿'),
    ('lacquered_helmet', 'Lacquered Helmet', '옻칠한 투구'),
    ('fluted_bascinet', 'Fluted Bascinet', '홈이 긴 배서닛'),
    ('pig_faced_bascinet', 'Pig-Faced Bascinet', '돼지 머리 배서닛'),
    ('nightmare_bascinet', 'Nightmare Bascinet', '악몽의 배서닛'),
    ('knight_helm', 'Knight Helm', '기사 투구'),
    ('conquest_helmet', 'Conquest Helmet', '정복 투구'),
    ('haunted_bascinet', 'Haunted Bascinet', '으스스한 배서닛'),
    ('sorrow_mask', 'Sorrow Mask', '슬픔 가면'),
    ('atonement_mask', 'Atonement Mask', '속죄 가면'),
    ('penitent_mask', 'Penitent Mask', '참회 가면')
) AS v(name, display_en, display_kor)
JOIN item_types t ON t.name = 'helmets_str_dex';

INSERT INTO item_bases (name, display_en, display_kor, item_type_id)
SELECT v.name, v.display_en, v.display_kor, t.id
FROM (
    VALUES
    ('rusted_coif', 'Rusted Coif', '녹슨 코이프'),
    ('soldier_helmet', 'Soldier Helmet', '병사 투구'),
    ('great_helmet', 'Great Helmet', '대형 투구'),
    ('crusader_helmet', 'Crusader Helmet', '십자군 투구'),
    ('aventail_helmet', 'Aventail Helmet', '쇠사슬 투구'),
    ('zealot_helmet', 'Zealot Helmet', '맹신자 투구'),
    ('great_crown', 'Great Crown', '대형 왕관'),
    ('magistrate_crown', 'Magistrate Crown', '재판관 왕관'),
    ('prophet_crown', 'Prophet Crown', '예언자 왕관'),
    ('praetor_crown', 'Praetor Crown', '집정관 왕관'),
    ('faithful_helmet', 'Faithful Helmet', '충실한 투구'),
    ('paladin_crown', 'Paladin Crown', '성기사 왕관'),
    ('divine_crown', 'Divine Crown', '신성한 왕관'),
    ('imp_crown', 'Imp Crown', '임프 왕관'),
    ('demon_crown', 'Demon Crown', '악마 왕관'),
    ('archdemon_crown', 'Archdemon Crown', '대악마 왕관'),
    ('bone_helmet', 'Bone Helmet', '뼈 투구')
) AS v(name, display_en, display_kor)
JOIN item_types t ON t.name = 'helmets_str_int';

INSERT INTO item_bases (name, display_en, display_kor, item_type_id)
SELECT v.name, v.display_en, v.display_kor, t.id
FROM (
    VALUES
    ('scare_mask', 'Scare Mask', '허수아비 가면'),
    ('plague_mask', 'Plague Mask', '역병 가면'),
    ('iron_mask', 'Iron Mask', '철 가면'),
    ('festival_mask', 'Festival Mask', '축제용 가면'),
    ('golden_mask', 'Golden Mask', '황금 가면'),
    ('raven_mask', 'Raven Mask', '큰까마귀 가면'),
    ('callous_mask', 'Callous Mask', '냉담한 가면'),
    ('regicide_mask', 'Regicide Mask', '왕 시해자 가면'),
    ('harlequin_mask', 'Harlequin Mask', '할리퀸 가면'),
    ('vaal_mask', 'Vaal Mask', '바알 가면'),
    ('deicide_mask', 'Deicide Mask', '신살자 가면'),
    ('jester_mask', 'Jester Mask', '어릿광대 가면'),
    ('ancient_mask', 'Ancient Mask', '고대 가면'),
    ('torturers_mask', 'Torturer''s Mask', '고문자의 가면'),
    ('gale_crown', 'Gale Crown', '강풍 왕관'),
    ('winter_crown', 'Winter Crown', '겨울 왕관'),
    ('blizzard_crown', 'Blizzard Crown', '눈보라 왕관')
) AS v(name, display_en, display_kor)
JOIN item_types t ON t.name = 'helmets_dex_int';