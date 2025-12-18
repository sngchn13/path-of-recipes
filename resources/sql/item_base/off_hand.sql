INSERT INTO item_bases (name, display_en, display_kor, item_type_id)
SELECT v.name, v.display_en, v.display_kor, t.id
FROM (
    VALUES
    ('ornate_quiver', 'Ornate Quiver', '화려한 화살통'),
    ('serrated_arrow_quiver', 'Serrated Arrow Quiver', '톱니 화살통'),
    ('fire_arrow_quiver', 'Fire Arrow Quiver', '불화살통'),
    ('sharktooth_arrow_quiver', 'Sharktooth Arrow Quiver', '상어이빨 화살통'),
    ('feathered_arrow_quiver', 'Feathered Arrow Quiver', '깃 화살통'),
    ('penetrating_arrow_quiver', 'Penetrating Arrow Quiver', '관통 화살통'),
    ('blunt_arrow_quiver', 'Blunt Arrow Quiver', '뭉툭한 화살통'),
    ('two_point_arrow_quiver', 'Two-Point Arrow Quiver', '쌍촉 화살통'),
    ('spike_point_arrow_quiver', 'Spike-Point Arrow Quiver', '가시촉 화살통'),
    ('blazing_arrow_quiver', 'Blazing Arrow Quiver', '맹렬한 화살통'),
    ('broadhead_arrow_quiver', 'Broadhead Arrow Quiver', '넓은 촉 화살통'),
    ('vile_arrow_quiver', 'Vile Arrow Quiver', '끔찍한 화살통'),
    ('heavy_arrow_quiver', 'Heavy Arrow Quiver', '무거운 화살통'),
    ('primal_arrow_quiver', 'Primal Arrow Quiver', '원시 화살통'),
    ('artillery_quiver', 'Artillery Quiver', '포대 화살통')
) AS v(name, display_en, display_kor)
JOIN item_types t ON t.name = 'quivers';

INSERT INTO item_bases (name, display_en, display_kor, item_type_id)
SELECT v.name, v.display_en, v.display_kor, t.id
FROM (
    VALUES
    ('splintered_tower_shield', 'Splintered Tower Shield', '쪼개진 거대 방패'),
    ('corroded_tower_shield', 'Corroded Tower Shield', '부식된 거대 방패'),
    ('rawhide_tower_shield', 'Rawhide Tower Shield', '생가죽 거대 방패'),
    ('cedar_tower_shield', 'Cedar Tower Shield', '삼나무 거대 방패'),
    ('copper_tower_shield', 'Copper Tower Shield', '구리 거대 방패'),
    ('reinforced_tower_shield', 'Reinforced Tower Shield', '강화 거대 방패'),
    ('painted_tower_shield', 'Painted Tower Shield', '도색된 거대 방패'),
    ('buckskin_tower_shield', 'Buckskin Tower Shield', '사슴가죽 거대 방패'),
    ('mahogany_tower_shield', 'Mahogany Tower Shield', '마호가니 거대 방패'),
    ('bronze_tower_shield', 'Bronze Tower Shield', '청동 거대 방패'),
    ('girded_tower_shield', 'Girded Tower Shield', '둘러싼 거대 방패'),
    ('crested_tower_shield', 'Crested Tower Shield', '문장 거대 방패'),
    ('shagreen_tower_shield', 'Shagreen Tower Shield', '거친 생가죽 거대 방패'),
    ('ebony_tower_shield', 'Ebony Tower Shield', '칠흑의 거대 방패'),
    ('ezomyte_tower_shield', 'Ezomyte Tower Shield', '에조미어 거대 방패'),
    ('colossal_tower_shield', 'Colossal Tower Shield', '초대형 거대 방패'),
    ('pinnacle_tower_shield', 'Pinnacle Tower Shield', '최고의 거대 방패'),
    ('exothermic_tower_shield', 'Exothermic Tower Shield', '발열성 거대 방패'),
    ('magmatic_tower_shield', 'Magmatic Tower Shield', '마그마 거대 방패'),
    ('heat_attuned_tower_shield', 'Heat-attuned Tower Shield', '조율된 열기 거대 방패')
) AS v(name, display_en, display_kor)
JOIN item_types t ON t.name = 'shields_str';


INSERT INTO item_bases (name, display_en, display_kor, item_type_id)
SELECT v.name, v.display_en, v.display_kor, t.id
FROM (
    VALUES
    ('goathide_buckler', 'Goathide Buckler', '염소 가죽 버클러'),
    ('pine_buckler', 'Pine Buckler', '소나무 버클러'),
    ('painted_buckler', 'Painted Buckler', '도색된 버클러'),
    ('hammered_buckler', 'Hammered Buckler', '단련한 버클러'),
    ('war_buckler', 'War Buckler', '전쟁용 버클러'),
    ('gilded_buckler', 'Gilded Buckler', '도금 버클러'),
    ('oak_buckler', 'Oak Buckler', '참나무 버클러'),
    ('enameled_buckler', 'Enameled Buckler', '유약 도색 버클러'),
    ('corrugated_buckler', 'Corrugated Buckler', '주름진 버클러'),
    ('battle_buckler', 'Battle Buckler', '전투용 버클러'),
    ('golden_buckler', 'Golden Buckler', '금색 버클러'),
    ('ironwood_buckler', 'Ironwood Buckler', '철목 버클러'),
    ('lacquered_buckler', 'Lacquered Buckler', '옻칠한 버클러'),
    ('vaal_buckler', 'Vaal Buckler', '바알 버클러'),
    ('crusader_buckler', 'Crusader Buckler', '십자군 버클러'),
    ('imperial_buckler', 'Imperial Buckler', '제국 버클러'),
    ('endothermic_buckler', 'Endothermic Buckler', '흡열성 버클러'),
    ('polar_buckler', 'Polar Buckler', '극지의 버클러'),
    ('cold_attuned_buckler', 'Cold-attuned Buckler', '조율된 냉기 버클러')
) AS v(name, display_en, display_kor)
JOIN item_types t ON t.name = 'shields_dex';


INSERT INTO item_bases (name, display_en, display_kor, item_type_id)
SELECT v.name, v.display_en, v.display_kor, t.id
FROM (
    VALUES
    ('twig_spirit_shield', 'Twig Spirit Shield', '잔가지 영혼 방패'),
    ('yew_spirit_shield', 'Yew Spirit Shield', '주목 영혼 방패'),
    ('bone_spirit_shield', 'Bone Spirit Shield', '뼈 영혼 방패'),
    ('tarnished_spirit_shield', 'Tarnished Spirit Shield', '변색된 영혼 방패'),
    ('jingling_spirit_shield', 'Jingling Spirit Shield', '방울 영혼 방패'),
    ('brass_spirit_shield', 'Brass Spirit Shield', '황동 영혼 방패'),
    ('walnut_spirit_shield', 'Walnut Spirit Shield', '호두나무 영혼 방패'),
    ('ivory_spirit_shield', 'Ivory Spirit Shield', '상아 영혼 방패'),
    ('ancient_spirit_shield', 'Ancient Spirit Shield', '고대의 영혼 방패'),
    ('chiming_spirit_shield', 'Chiming Spirit Shield', '차임벨 영혼 방패'),
    ('thorium_spirit_shield', 'Thorium Spirit Shield', '토륨 영혼 방패'),
    ('lacewood_spirit_shield', 'Lacewood Spirit Shield', '쥐방울나무 영혼 방패'),
    ('fossilised_spirit_shield', 'Fossilised Spirit Shield', '화석 영혼 방패'),
    ('vaal_spirit_shield', 'Vaal Spirit Shield', '바알 영혼 방패'),
    ('harmonic_spirit_shield', 'Harmonic Spirit Shield', '화음 영혼 방패'),
    ('titanium_spirit_shield', 'Titanium Spirit Shield', '티타늄 영혼 방패'),
    ('exhausting_spirit_shield', 'Exhausting Spirit Shield', '기진맥진한 영혼 방패'),
    ('subsuming_spirit_shield', 'Subsuming Spirit Shield', '아우르는 영혼 방패'),
    ('transfer_attuned_spirit_shield', 'Transfer-attuned Spirit Shield', '조율된 전이 영혼 방패')
) AS v(name, display_en, display_kor)
JOIN item_types t ON t.name = 'shields_int';


INSERT INTO item_bases (name, display_en, display_kor, item_type_id)
SELECT v.name, v.display_en, v.display_kor, t.id
FROM (
    VALUES
    ('rotted_round_shield', 'Rotted Round Shield', '썩은 원형 방패'),
    ('fir_round_shield', 'Fir Round Shield', '전나무 원형 방패'),
    ('studded_round_shield', 'Studded Round Shield', '징 박힌 원형 방패'),
    ('scarlet_round_shield', 'Scarlet Round Shield', '다홍색 원형 방패'),
    ('splendid_round_shield', 'Splendid Round Shield', '화려한 원형 방패'),
    ('maple_round_shield', 'Maple Round Shield', '단풍나무 원형 방패'),
    ('spiked_round_shield', 'Spiked Round Shield', '가시 박힌 원형 방패'),
    ('crimson_round_shield', 'Crimson Round Shield', '진홍색 원형 방패'),
    ('baroque_round_shield', 'Baroque Round Shield', '바로크 원형 방패'),
    ('teak_round_shield', 'Teak Round Shield', '티크목 원형 방패'),
    ('spiny_round_shield', 'Spiny Round Shield', '가시투성이 원형 방패'),
    ('cardinal_round_shield', 'Cardinal Round Shield', '선홍색 원형 방패'),
    ('elegant_round_shield', 'Elegant Round Shield', '고상한 원형 방패')
) AS v(name, display_en, display_kor)
JOIN item_types t ON t.name = 'shields_str_dex';


INSERT INTO item_bases (name, display_en, display_kor, item_type_id)
SELECT v.name, v.display_en, v.display_kor, t.id
FROM (
    VALUES
    ('plank_kite_shield', 'Plank Kite Shield', '널빤지 연모양 방패'),
    ('linden_kite_shield', 'Linden Kite Shield', '린덴목 연모양 방패'),
    ('reinforced_kite_shield', 'Reinforced Kite Shield', '강화 연모양 방패'),
    ('layered_kite_shield', 'Layered Kite Shield', '겹재질 연모양 방패'),
    ('ceremonial_kite_shield', 'Ceremonial Kite Shield', '제사장의 연모양 방패'),
    ('etched_kite_shield', 'Etched Kite Shield', '조각된 연모양 방패'),
    ('steel_kite_shield', 'Steel Kite Shield', '강철 연모양 방패'),
    ('laminated_kite_shield', 'Laminated Kite Shield', '합판 연모양 방패'),
    ('angelic_kite_shield', 'Angelic Kite Shield', '천사의 연모양 방패'),
    ('branded_kite_shield', 'Branded Kite Shield', '각인한 연모양 방패'),
    ('champion_kite_shield', 'Champion Kite Shield', '용사의 연모양 방패'),
    ('mosaic_kite_shield', 'Mosaic Kite Shield', '조각무늬 연모양 방패'),
    ('archon_kite_shield', 'Archon Kite Shield', '집정관의 연모양 방패')
) AS v(name, display_en, display_kor)
JOIN item_types t ON t.name = 'shields_str_int';


INSERT INTO item_bases (name, display_en, display_kor, item_type_id)
SELECT v.name, v.display_en, v.display_kor, t.id
FROM (
    VALUES
    ('spiked_bundle', 'Spiked Bundle', '가시 박힌 꾸러미'),
    ('driftwood_spiked_shield', 'Driftwood Spiked Shield', '유목 가시 방패'),
    ('alloyed_spiked_shield', 'Alloyed Spiked Shield', '합금 가시 방패'),
    ('burnished_spiked_shield', 'Burnished Spiked Shield', '윤이 나는 가시 방패'),
    ('ornate_spiked_shield', 'Ornate Spiked Shield', '화려한 가시 방패'),
    ('redwood_spiked_shield', 'Redwood Spiked Shield', '삼나무 가시 방패'),
    ('compound_spiked_shield', 'Compound Spiked Shield', '합성 가시 방패'),
    ('polished_spiked_shield', 'Polished Spiked Shield', '광택을 낸 가시 방패'),
    ('sovereign_spiked_shield', 'Sovereign Spiked Shield', '주권자 가시 방패'),
    ('alder_spiked_shield', 'Alder Spiked Shield', '오리나무 가시 방패'),
    ('ezomyte_spiked_shield', 'Ezomyte Spiked Shield', '에조미어 가시 방패'),
    ('mirrored_spiked_shield', 'Mirrored Spiked Shield', '거울 같은 가시 방패'),
    ('supreme_spiked_shield', 'Supreme Spiked Shield', '최상급 가시 방패')
) AS v(name, display_en, display_kor)
JOIN item_types t ON t.name = 'shields_dex_int';