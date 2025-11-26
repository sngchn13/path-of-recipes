INSERT INTO item_bases (name, display_en, display_kor, item_type_id)
SELECT v.name, v.display_en, v.display_kor, t.id
FROM (
    VALUES
    ('paua_amulet', 'Paua Amulet', '파우아 목걸이'),
    ('coral_amulet', 'Coral Amulet', '산호 목걸이'),
    ('amber_amulet', 'Amber Amulet', '호박 목걸이'),
    ('jade_amulet', 'Jade Amulet', '비취 목걸이'),
    ('lapis_amulet', 'Lapis Amulet', '청금석 목걸이'),
    ('gold_amulet', 'Gold Amulet', '황금 목걸이'),
    ('onyx_amulet', 'Onyx Amulet', '오닉스 목걸이'),
    ('turquoise_amulet', 'Turquoise Amulet', '터키석 목걸이'),
    ('agate_amulet', 'Agate Amulet', '마노 목걸이'),
    ('citrine_amulet', 'Citrine Amulet', '황수정 목걸이'),
    ('ruby_amulet', 'Ruby Amulet', '루비 목걸이'),
    ('unset_amulet', 'Unset Amulet', '무보석 목걸이'),
    ('astrolabe_amulet', 'Astrolabe Amulet', '관측기 목걸이'),
    ('simplex_amulet', 'Simplex Amulet', '평범한 목걸이'),
    ('focused_amulet', 'Focused Amulet', '집중된 목걸이'),
    ('jet_amulet', 'Jet Amulet', '흑옥 목걸이'),
    ('blue_pearl_amulet', 'Blue Pearl Amulet', '파란 진주 목걸이'),
    ('marble_amulet', 'Marble Amulet', '대리석 목걸이'),
    ('seaglass_amulet', 'Seaglass Amulet', '바다유리 목걸이'),

    ('black_maw_talisman', 'Black Maw Talisman', '검은 턱 부적'),
    ('bonespire_talisman', 'Bonespire Talisman', '뼈 첨탑 부적'),
    ('ashscale_talisman', 'Ashscale Talisman', '재비늘 부적'),
    ('lone_antler_talisman', 'Lone Antler Talisman', '외가지뿔 부적'),
    ('deep_one_talisman', 'Deep One Talisman', '심해의 존재 부적'),
    ('breakrib_talisman', 'Breakrib Talisman', '골절늑골 부적'),
    ('deadhand_talisman', 'Deadhand Talisman', '죽은 손 부적'),
    ('undying_flesh_talisman', 'Undying Flesh Talisman', '산 송장 살점 부적'),
    ('rot_head_talisman', 'Rot Head Talisman', '썩은 머리 부적'),
    ('mandible_talisman', 'Mandible Talisman', '아래턱 부적'),
    ('chrysalis_talisman', 'Chrysalis Talisman', '번데기 부적'),
    ('writhing_talisman', 'Writhing Talisman', '꿈틀거리는 부적'),

    ('hexclaw_talisman', 'Hexclaw Talisman', '사술발톱 부적'),
    ('primal_skull_talisman', 'Primal Skull Talisman', '원시 해골 부적'),
    ('wereclaw_talisman', 'Wereclaw Talisman', '변신자발톱 부적'),
    ('splitnewt_talisman', 'Splitnewt Talisman', '분할도롱뇽 부적'),
    ('clutching_talisman', 'Clutching Talisman', '움켜잡는 부적'),
    ('avian_twins_talisman', 'Avian Twins Talisman', '쌍둥이 새 부적'),
    ('fangjaw_talisman', 'Fangjaw Talisman', '송곳니턱 부적'),
    ('horned_talisman', 'Horned Talisman', '뿔 달린 부적'),

    ('spinefuse_talisman', 'Spinefuse Talisman', '척추융합 부적'),
    ('three_rat_talisman', 'Three Rat Talisman', '세 마리 쥐 부적'),
    ('monkey_twins_talisman', 'Monkey Twins Talisman', '쌍둥이 원숭이 부적'),
    ('longtooth_talisman', 'Longtooth Talisman', '긴 이빨 부적'),
    ('rotfeather_talisman', 'Rotfeather Talisman', '썩은 깃털 부적'),
    ('monkey_paw_talisman', 'Monkey Paw Talisman', '원숭이 발 부적'),
    ('three_hands_talisman', 'Three Hands Talisman', '세 개의 손 부적'),
    ('greatwolf_talisman', 'Greatwolf Talisman', '위대한 늑대 부적')
) AS v(name, display_en, display_kor)
JOIN item_types t ON t.name = 'amulets';

INSERT INTO item_bases (name, display_en, display_kor, item_type_id)
SELECT v.name, v.display_en, v.display_kor, t.id
FROM (
    VALUES
    ('golden_hoop', 'Golden Hoop', '황금 고리'),
    ('iron_ring', 'Iron Ring', '철제 반지'),
    ('coral_ring', 'Coral Ring', '산호 반지'),
    ('paua_ring', 'Paua Ring', '파우아 반지'),
    ('gold_ring', 'Gold Ring', '황금 반지'),
    ('topaz_ring', 'Topaz Ring', '토파즈 반지'),
    ('sapphire_ring', 'Sapphire Ring', '사파이어 반지'),
    ('ruby_ring', 'Ruby Ring', '루비 반지'),
    ('prismatic_ring', 'Prismatic Ring', '분광 반지'),
    ('moonstone_ring', 'Moonstone Ring', '월장석 반지'),
    ('amethyst_ring', 'Amethyst Ring', '자수정 반지'),
    ('diamond_ring', 'Diamond Ring', '다이아몬드 반지'),
    ('two_stone_ring_fire_lightning', 'Two-Stone Ring', '쌍보석 반지'),
    ('two_stone_ring_cold_lightning', 'Two-Stone Ring', '쌍보석 반지'),
    ('two_stone_ring_fire_cold', 'Two-Stone Ring', '쌍보석 반지'),
    ('unset_ring', 'Unset Ring', '무보석 반지'),
    ('cryonic_ring', 'Cryonic Ring', '냉동 보존 반지'),
    ('enthalpic_ring', 'Enthalpic Ring', '엔탈피 반지'),
    ('synaptic_ring', 'Synaptic Ring', '시냅스 반지'),
    ('organic_ring', 'Organic Ring', '유기적 반지'),
    ('fugitive_ring', 'Fugitive Ring', '도망자 반지'),
    ('formless_ring', 'Formless Ring', '형상 없는 반지'),

    ('cogwork_ring', 'Cogwork Ring', '톱니장치 반지'),
    ('geodesic_ring', 'Geodesic Ring', '정동석 반지'),
    ('composite_ring', 'Composite Ring', '합성 반지'),
    ('manifold_ring', 'Manifold Ring', '다면의 반지'),
    ('ratcheting_ring', 'Ratcheting Ring', '미늘톱니 반지'),
    ('helical_ring', 'Helical Ring', '나선형 반지'),

    ('dusk_ring', 'Dusk Ring', '황혼 반지'),
    ('penumbra_ring', 'Penumbra Ring', '음영 반지'),
    ('gloam_ring', 'Gloam Ring', '석양 반지'),
    ('tenebrous_ring', 'Tenebrous Ring', '어둑한 반지'),
    ('shadowed_ring_cold_fire', 'Shadowed Ring', '그늘진 반지'),
    ('shadowed_ring_fire_lightning', 'Shadowed Ring', '그늘진 반지'),
    ('shadowed_ring_lightning_cold', 'Shadowed Ring', '그늘진 반지'),
    ('nameless_ring', 'Nameless Ring', '이름 없는 반지'),

    ('jet_ring', 'Jet Ring', '흑옥 반지'),
    ('steel_ring', 'Steel Ring', '강철 반지'),
    ('opal_ring', 'Opal Ring', '오팔 반지'),
    ('vermillion_ring', 'Vermillion Ring', '주색 반지'),
    ('cerulean_ring', 'Cerulean Ring', '진청색 반지'),
    ('iolite_ring', 'Iolite Ring', '근청석 반지'),

    ('ring', 'Ring', '반지'),
    ('breach_ring', 'Breach Ring', '균열 반지')
) AS v(name, display_en, display_kor)
JOIN item_types t ON t.name = 'rings';

INSERT INTO item_bases (name, display_en, display_kor, item_type_id)
SELECT v.name, v.display_en, v.display_kor, t.id
FROM (
    VALUES
    ('rustic_sash', 'Rustic Sash', '소박한 띠'),
    ('chain_belt', 'Chain Belt', '사슬 허리띠'),
    ('leather_belt', 'Leather Belt', '가죽 허리띠'),
    ('heavy_belt', 'Heavy Belt', '무거운 허리띠'),
    ('cloth_belt', 'Cloth Belt', '헝겊 허리띠'),
    ('studded_belt', 'Studded Belt', '징 박힌 허리띠'),

    ('micro_distillery_belt', 'Micro-Distillery Belt', '섬세한 주조자 허리띠'),
    ('mechalarm_belt', 'Mechalarm Belt', '기계 경보 허리띠'),
    ('mechanical_belt', 'Mechanical Belt', '기계식 허리띠'),
    ('vanguard_belt', 'Vanguard Belt', '선봉대 허리띠'),
    ('crystal_belt', 'Crystal Belt', '수정 허리띠'),
    ('golden_obi', 'Golden Obi', '황금 오비'),

    ('stygian_vise', 'Stygian Vise', '명계의 조임쇠')
) AS v(name, display_en, display_kor)
JOIN item_types t ON t.name = 'belts';

INSERT INTO item_bases (name, display_en, display_kor, item_type_id)
SELECT v.name, v.display_en, v.display_kor, t.id
FROM (
    VALUES
   ('bone_ring', 'Bone Ring', '뼈 반지')
) AS v(name, display_en, display_kor)
JOIN item_types t ON t.name = 'bone_rings';
