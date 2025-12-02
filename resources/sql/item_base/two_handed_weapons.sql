INSERT INTO item_bases (name, display_en, display_kor, item_type_id)
SELECT v.name, v.display_en, v.display_kor, t.id
FROM (
    VALUES
    ('crude_bow', 'Crude Bow', '조잡한 활'),
    ('short_bow', 'Short Bow', '단궁'),
    ('long_bow', 'Long Bow', '장궁'),
    ('composite_bow', 'Composite Bow', '합성궁'),
    ('recurve_bow', 'Recurve Bow', '반곡궁'),
    ('bone_bow', 'Bone Bow', '뼈 활'),
    ('royal_bow', 'Royal Bow', '왕실 활'),
    ('death_bow', 'Death Bow', '죽음의 활'),
    ('reflex_bow', 'Reflex Bow', '반동궁'),
    ('grove_bow', 'Grove Bow', '수목원 활'),
    ('decurve_bow', 'Decurve Bow', '비곡궁'),
    ('compound_bow', 'Compound Bow', '복합궁'),
    ('sniper_bow', 'Sniper Bow', '저격수 활'),
    ('ivory_bow', 'Ivory Bow', '상아색 활'),
    ('highborn_bow', 'Highborn Bow', '명문가의 활'),
    ('decimation_bow', 'Decimation Bow', '대량 학살의 활'),
    ('steelwood_bow', 'Steelwood Bow', '강철목 활'),
    ('thicket_bow', 'Thicket Bow', '잡목림 활'),
    ('citadel_bow', 'Citadel Bow', '성채 활'),
    ('ranger_bow', 'Ranger Bow', '레인저 활'),
    ('assassin_bow', 'Assassin Bow', '암살자 활'),
    ('spine_bow', 'Spine Bow', '척추 활'),
    ('imperial_bow', 'Imperial Bow', '제국 활'),
    ('harbinger_bow', 'Harbinger Bow', '선구자 활'),
    ('maraketh_bow', 'Maraketh Bow', '마라케스 활'),
    ('hedron_bow', 'Hedron Bow', '다면체 활'),
    ('foundry_bow', 'Foundry Bow', '주물 활'),
    ('solarine_bow', 'Solarine Bow', '태양의 활'),
    ('ethereal_bow', 'Ethereal Bow', '천상의 활')
) AS v(name, display_en, display_kor)
JOIN item_types t ON t.name = 'bows';

INSERT INTO item_bases (name, display_en, display_kor, item_type_id)
SELECT v.name, v.display_en, v.display_kor, t.id
FROM (
    VALUES
    ('gnarled_branch', 'Gnarled Branch', '울퉁불퉁한 나뭇가지'),
    ('primitive_staff', 'Primitive Staff', '원시의 지팡이'),
    ('long_staff', 'Long Staff', '긴 지팡이'),
    ('royal_staff', 'Royal Staff', '왕실 지팡이'),
    ('crescent_staff', 'Crescent Staff', '초승달의 지팡이'),
    ('woodful_staff', 'Woodful Staff', '목재 지팡이'),
    ('quarterstaff', 'Quarterstaff', '육척봉'),
    ('highborn_staff', 'Highborn Staff', '상류층의 지팡이'),
    ('moon_staff', 'Moon Staff', '달의 지팡이'),
    ('primordial_staff', 'Primordial Staff', '태고의 지팡이'),
    ('lathi', 'Lathi', '쇠테 곤봉'),
    ('imperial_staff', 'Imperial Staff', '제국 지팡이'),
    ('eclipse_staff', 'Eclipse Staff', '월식 지팡이'),
    ('transformer_staff', 'Transformer Staff', '변신 지팡이'),
    ('reciprocation_staff', 'Reciprocation Staff', '보복 지팡이'),
    ('battery_staff', 'Battery Staff', '충전 지팡이')
) AS v(name, display_en, display_kor)
JOIN item_types t ON t.name = 'staves';

INSERT INTO item_bases (name, display_en, display_kor, item_type_id)
SELECT v.name, v.display_en, v.display_kor, t.id
FROM (
    VALUES
    ('corroded_blade', 'Corroded Blade', '부식된 칼날'),
    ('longsword', 'Longsword', '장검'),
    ('bastard_sword', 'Bastard Sword', '바스타드'),
    ('two_handed_sword', 'Two-Handed Sword', '양손검'),
    ('etched_greatsword', 'Etched Greatsword', '조각된 대검'),
    ('ornate_sword', 'Ornate Sword', '화려한 검'),
    ('spectral_sword', 'Spectral Sword', '영체 검'),
    ('curved_blade', 'Curved Blade', '곡도'),
    ('butcher_sword', 'Butcher Sword', '도살자 검'),
    ('footman_sword', 'Footman Sword', '보병 검'),
    ('highland_blade', 'Highland Blade', '고지대 검'),
    ('engraved_greatsword', 'Engraved Greatsword', '아로새긴 대검'),
    ('tiger_sword', 'Tiger Sword', '호랑이 검'),
    ('wraith_sword', 'Wraith Sword', '악령 검'),
    ('lithe_blade', 'Lithe Blade', '유연한 검'),
    ('headmans_sword', 'Headman''s Sword', '참수자의 검'),
    ('reaver_sword', 'Reaver Sword', '강탈자 검'),
    ('ezomyte_blade', 'Ezomyte Blade', '에조미어 검'),
    ('vaal_greatsword', 'Vaal Greatsword', '바알 대검'),
    ('lion_sword', 'Lion Sword', '사자 검'),
    ('infernal_sword', 'Infernal Sword', '지옥불 검'),
    ('exquisite_blade', 'Exquisite Blade', '정교한 검'),
    ('rebuking_blade', 'Rebuking Blade', '질책의 칼'),
    ('blasting_blade', 'Blasting Blade', '폭발성 칼'),
    ('banishing_blade', 'Banishing Blade', '추방의 칼')
) AS v(name, display_en, display_kor)
JOIN item_types t ON t.name = 'two_hand_swords';

INSERT INTO item_bases (name, display_en, display_kor, item_type_id)
SELECT v.name, v.display_en, v.display_kor, t.id
FROM (
    VALUES
    ('stone_axe', 'Stone Axe', '돌도끼'),
    ('jade_chopper', 'Jade Chopper', '비취 절단기'),
    ('woodsplitter', 'Woodsplitter', '벌목스플리터'),
    ('poleaxe', 'Poleaxe', '자루 도끼'),
    ('double_axe', 'Double Axe', '양날 도끼'),
    ('gilded_axe', 'Gilded Axe', '도금된 도끼'),
    ('shadow_axe', 'Shadow Axe', '그림자 도끼'),
    ('dagger_axe', 'Dagger Axe', '단검 도끼'),
    ('jasper_chopper', 'Jasper Chopper', '벽옥 절단기'),
    ('timber_axe', 'Timber Axe', '목재용 도끼'),
    ('headsman_axe', 'Headsman Axe', '참수자 도끼'),
    ('labrys', 'Labrys', '라브리스'),
    ('noble_axe', 'Noble Axe', '귀족 도끼'),
    ('abyssal_axe', 'Abyssal Axe', '심연의 도끼'),
    ('talon_axe', 'Talon Axe', '발톱 도끼'),
    ('karui_chopper', 'Karui Chopper', '카루이 절단기'),
    ('sundering_axe', 'Sundering Axe', '작렬하는 도끼'),
    ('ezomyte_axe', 'Ezomyte Axe', '에조미어 도끼'),
    ('vaal_axe', 'Vaal Axe', '바알 도끼'),
    ('despot_axe', 'Despot Axe', '절대군주 도끼'),
    ('void_axe', 'Void Axe', '공허의 도끼'),
    ('fleshripper', 'Fleshripper', '살점 거열기'),
    ('prime_cleaver', 'Prime Cleaver', '최종형 가로날도끼'),
    ('honed_cleaver', 'Honed Cleaver', '연마한 가로날도끼'),
    ('apex_cleaver', 'Apex Cleaver', '정점의 가로날도끼')
) AS v(name, display_en, display_kor)
JOIN item_types t ON t.name = 'two_hand_axes';

INSERT INTO item_bases (name, display_en, display_kor, item_type_id)
SELECT v.name, v.display_en, v.display_kor, t.id
FROM (
    VALUES
    ('driftwood_maul', 'Driftwood Maul', '표류목 대형 망치'),
    ('tribal_maul', 'Tribal Maul', '부족 대형 망치'),
    ('mallet', 'Mallet', '나무 망치'),
    ('sledgehammer', 'Sledgehammer', '대형 양손 망치'),
    ('jagged_maul', 'Jagged Maul', '삐죽삐죽한 대형 망치'),
    ('brass_maul', 'Brass Maul', '황동 대형 망치'),
    ('fright_maul', 'Fright Maul', '섬뜩한 대형 망치'),
    ('morning_star', 'Morning Star', '모닝스타'),
    ('totemic_maul', 'Totemic Maul', '토템 대형 망치'),
    ('great_mallet', 'Great Mallet', '대형 나무 망치'),
    ('steelhead', 'Steelhead', '강철머리 망치'),
    ('spiny_maul', 'Spiny Maul', '가시투성이 대형 망치'),
    ('plated_maul', 'Plated Maul', '판금 대형 망치'),
    ('dread_maul', 'Dread Maul', '끔찍한 대형 망치'),
    ('solar_maul', 'Solar Maul', '태양의 대형 망치'),
    ('karui_maul', 'Karui Maul', '카루이 대형 망치'),
    ('colossus_mallet', 'Colossus Mallet', '초대형 나무 망치'),
    ('piledriver', 'Piledriver', '말뚝 망치'),
    ('meatgrinder', 'Meatgrinder', '고기 갈이개'),
    ('imperial_maul', 'Imperial Maul', '제국 대형 망치'),
    ('terror_maul', 'Terror Maul', '공포의 대형 망치'),
    ('coronal_maul', 'Coronal Maul', '광환의 대형 망치'),
    ('blunt_force_condenser', 'Blunt Force Condenser', '둔력 응축자'),
    ('crushing_force_magnifier', 'Crushing Force Magnifier', '압착력 확대자'),
    ('impact_force_propagator', 'Impact Force Propagator', '충격력 확산자')
) AS v(name, display_en, display_kor)
JOIN item_types t ON t.name = 'two_hand_maces';

INSERT INTO item_bases (name, display_en, display_kor, item_type_id)
SELECT v.name, v.display_en, v.display_kor, t.id
FROM (
    VALUES
    ('iron_staff', 'Iron Staff', '철제 지팡이'),
    ('coiled_staff', 'Coiled Staff', '코일형 지팡이'),
    ('vile_staff', 'Vile Staff', '끔찍한 지팡이'),
    ('military_staff', 'Military Staff', '군용 지팡이'),
    ('serpentine_staff', 'Serpentine Staff', '사문석 지팡이'),
    ('foul_staff', 'Foul Staff', '사악한 지팡이'),
    ('ezomyte_staff', 'Ezomyte Staff', '에조미어 지팡이'),
    ('maelstrom_staff', 'Maelström Staff', '마엘스트롬 지팡이'),
    ('judgement_staff', 'Judgement Staff', '심판 지팡이'),
    ('capacity_rod', 'Capacity Rod', '가능성의 막대'),
    ('potentiality_rod', 'Potentiality Rod', '잠재력의 막대'),
    ('eventuality_rod', 'Eventuality Rod', '불예측의 막대')
) AS v(name, display_en, display_kor)
JOIN item_types t ON t.name = 'warstaves';

INSERT INTO item_bases (name, display_en, display_kor, item_type_id)
SELECT v.name, v.display_en, v.display_kor, t.id
FROM (
    VALUES
    ('fishing_rod', 'Fishing Rod', '낚싯대')
) AS v(name, display_en, display_kor)
JOIN item_types t ON t.name = 'fishing_rods';