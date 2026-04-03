//! Utility functions for writing code for actor areas.
//!
//! Throughout this module the TC squares are named with the following convention
//! - A is the left-center
//! - B is the top-center
//! - C is the bottom-center
//! - D is the right-center
//! By using `find_closest` with `min_distance_to_players` set to `1`, we
//! obtain a set R of two more squares:
//! - above and to the right of D
//! - below and to the right of D
//!
//! The boxes are then actor areas defining square boxes from the center
//! of the TC. `box0` is the 4 centermost tiles, `box1` is those tiles
//! and their adjacent neighbors, and so on.

/// Returns the lines used to define the placeholder constants.
///
/// Defines the following constants:
/// - `PHOFF 649`
/// - `PHON 1291`
/// - `TERRAIN_BLOCKER 1613`
/// - `SHEP0`, ..., `SHEP5` as `590` or `592`
///
/// Individual constants for `HERDABLE_A` and `STRAGGLER` must be defined in
/// each map script.
pub fn make_constants() -> Vec<String> {
    let mut lines = vec![
        String::from("#const PHOFF 1902"),
        String::from("#const PHON 1543"),
        String::from("#const TERRAIN_BLOCKER 1613"),
        String::from("#const REVEAL3_TEMP 1874"),
        String::from("#const TRIBUTE_INEFFICIENCY 46"),
    ];
    // 590 is VILLAGER_SHEPHERD_F; 592 is VILLAGER_SHEPHPERD_M
    for i in 0..6 {
        lines.push(String::from("start_random"));
        lines.push(format!("percent_chance 50 #const SHEP{i} 590"));
        lines.push(format!("percent_chance 50 #const SHEP{i} 592"));
        lines.push(String::from("end_random"));
    }
    // 123 is VILLAGER_WOOD_M; 218 is VILLAGER_WOOD_F
    for i in 0..3 {
        lines.push(String::from("start_random"));
        lines.push(format!("percent_chance 50 #const LUMBERJACK{i} 123"));
        lines.push(format!("percent_chance 50 #const LUMBERJACK{i} 218"));
        lines.push(String::from("end_random"));
    }
    lines
}

/// Returns the lines for 9-Villager start constants.
pub fn make_constants_9() -> Vec<String> {
    let mut lines = vec![
        String::from("#const PHOFF 1902"),
        String::from("#const PHON 1543"),
        String::from("#const TERRAIN_BLOCKER 1613"),
        String::from("#const REVEAL3_TEMP 1874"),
    ];
    // 590 is VILLAGER_SHEPHERD_F; 592 is VILLAGER_SHEPHPERD_M
    for i in 0..6 {
        lines.push(String::from("start_random"));
        lines.push(format!("percent_chance 50 #const SHEP{i} 590"));
        lines.push(format!("percent_chance 50 #const SHEP{i} 592"));
        lines.push(String::from("end_random"));
    }
    // 123 is VILLAGER_WOOD_M; 218 is VILLAGER_WOOD_F
    for i in 0..3 {
        lines.push(String::from("start_random"));
        lines.push(format!("percent_chance 50 #const LUMBERJACK{i} 123"));
        lines.push(format!("percent_chance 50 #const LUMBERJACK{i} 218"));
        lines.push(String::from("end_random"));
    }
    lines
}

/// Constants with fishers instead of shepherds.
pub fn make_constants_9_roe_rage() -> Vec<String> {
    let mut lines = vec![
        String::from("#const PHOFF 1902"),
        String::from("#const PHON 1543"),
        String::from("#const TERRAIN_BLOCKER 1613"),
        String::from("#const REVEAL3_TEMP 1874"),
    ];
    // 56 is VILLAGER_FISHER_M; 57 is VILLAGER_FISHER_F
    for i in 0..6 {
        lines.push(String::from("start_random"));
        lines.push(format!("percent_chance 50 #const SHEP{i} 56"));
        lines.push(format!("percent_chance 50 #const SHEP{i} 57"));
        lines.push(String::from("end_random"));
    }
    // 123 is VILLAGER_WOOD_M; 218 is VILLAGER_WOOD_F
    for i in 0..3 {
        lines.push(String::from("start_random"));
        lines.push(format!("percent_chance 50 #const LUMBERJACK{i} 123"));
        lines.push(format!("percent_chance 50 #const LUMBERJACK{i} 218"));
        lines.push(String::from("end_random"));
    }
    lines
}

/// Returns a vector of lines of map code defining the constants used
/// for placeholders and terrain blockers in small maps.
pub fn make_small_constants() -> Vec<String> {
    vec![
        String::from("#const PHON 1543"),
        String::from("#const PHOFF 1902"),
        String::from("#const TERRAIN_BLOCKER 1613"),
    ]
}

/// Returns the lines for clearing the placeholder attributes in `<PLAYER_SETUP>`.
/// Also sets the Gaia HP for `HERDABLE_A` to `0`.
pub fn set_placeholder_attributes() -> Vec<String> {
    vec![
        // String::from("effect_amount GAIA_SET_ATTRIBUTE HERDABLE_A ATTR_HITPOINTS 0"),
        String::from("effect_amount MOD_RESOURCE AMOUNT_STARTING_WOOD ATTR_ADD -30"),
        // String::from("effect_amount MOD_RESOURCE AMOUNT_STARTING_FOOD ATTR_ADD -100"),
        String::from("effect_amount MOD_RESOURCE AMOUNT_STARTING_FOOD ATTR_ADD -90"),
        String::from("effect_percent MOD_RESOURCE TRIBUTE_INEFFICIENCY ATTR_SET 50"),
    ]
}

/// Returns the lines for setting attributes for a 9 Villager start.
pub fn set_placeholder_attributes_9() -> Vec<String> {
    vec![
        String::from("effect_amount GAIA_SET_ATTRIBUTE HERDABLE_A ATTR_HITPOINTS 0"),
        String::from("effect_amount SET_ATTRIBUTE REVEAL3_TEMP ATTR_DEAD_ID -1"),
        String::from("effect_amount SET_ATTRIBUTE REVEAL3_TEMP ATTR_HITPOINTS 0"),
        String::from("effect_amount SET_ATTRIBUTE REVEAL3_TEMP ATTR_LINE_OF_SIGHT 18"),
        String::from("effect_amount MOD_RESOURCE AMOUNT_STARTING_WOOD ATTR_ADD -30"),
        String::from("effect_amount MOD_RESOURCE AMOUNT_STARTING_FOOD ATTR_ADD -90"),
    ]
}

/// Returns a vector of lines of map code setting the placeholder object
/// attributes for small maps.
pub fn set_placeholder_attributes_small() -> Vec<String> {
    vec![
    ]
}

/// Sets placeholder attributes for Four Seasons.
/// Sets the Gaia HP for all biome `BIOME_HERDABLE_A` to `0`.
pub fn set_placeholder_attributes_four_seasons() -> Vec<String> {
    vec![
        String::from("effect_amount GAIA_SET_ATTRIBUTE GRASS_HERDABLE_A ATTR_HITPOINTS 0"),
        String::from("effect_amount GAIA_SET_ATTRIBUTE SNOW_HERDABLE_A ATTR_HITPOINTS 0"),
        String::from("effect_amount GAIA_SET_ATTRIBUTE DIRT_HERDABLE_A ATTR_HITPOINTS 0"),
        String::from("effect_amount GAIA_SET_ATTRIBUTE JUNGLE_HERDABLE_A ATTR_HITPOINTS 0"),
        String::from("effect_amount MOD_RESOURCE AMOUNT_STARTING_WOOD ATTR_ADD -30"),
        String::from("effect_amount MOD_RESOURCE AMOUNT_STARTING_FOOD ATTR_ADD -100"),
        String::from("effect_percent MOD_RESOURCE TRIBUTE_INEFFICIENCY ATTR_SET 50"),
    ]
}

/// Same as `set_zewall_placeholder_attributes` but uses `SET_ATTRIBUTE`
/// instead of `GAIA_SET_ATTRIBUTE` for the initial dying herdable.
// Note the Goose still lives, so this function isn't useful.
// pub fn set_zewall_placeholder_attributes() -> Vec<String> {
//     vec![
//         String::from("effect_amount SET_ATTRIBUTE PHOFF ATTR_DEAD_ID -1"),
//         String::from("effect_amount SET_ATTRIBUTE PHOFF ATTR_TERRAIN_ID 0"),
//         String::from("effect_amount SET_ATTRIBUTE PHOFF ATTR_LINE_OF_SIGHT 0"),
//         String::from("effect_amount SET_ATTRIBUTE PHOFF ATTR_HITPOINTS 0"),
//         String::from("effect_amount SET_ATTRIBUTE PHOFF ATTR_STORAGE_VALUE 0"),
//         String::from("effect_amount SET_ATTRIBUTE PHON ATTR_DEAD_ID -1"),
//         String::from("effect_amount SET_ATTRIBUTE PHON ATTR_TERRAIN_ID 0"),
//         String::from("effect_amount SET_ATTRIBUTE PHON ATTR_LINE_OF_SIGHT 0"),
//         String::from("effect_amount SET_ATTRIBUTE PHON ATTR_HITPOINTS 0"),
//         String::from("effect_amount SET_ATTRIBUTE PHON ATTR_STORAGE_VALUE 0"),
//         String::from("effect_amount SET_ATTRIBUTE HERDABLE_A ATTR_HITPOINTS 0"),
//         String::from("effect_amount SET_ATTRIBUTE REVEAL3_TEMP ATTR_DEAD_ID -1"),
//         String::from("effect_amount SET_ATTRIBUTE REVEAL3_TEMP ATTR_HITPOINTS 0"),
//         String::from("effect_amount SET_ATTRIBUTE REVEAL3_TEMP ATTR_LINE_OF_SIGHT 18"),
//         String::from("effect_amount MOD_RESOURCE AMOUNT_STARTING_WOOD ATTR_ADD -30"),
//         String::from("effect_amount MOD_RESOURCE AMOUNT_STARTING_FOOD ATTR_ADD -100"),
//         String::from("effect_percent MOD_RESOURCE TRIBUTE_INEFFICIENCY ATTR_SET 50"),
//     ]
// }

/// Returns a String of lines for setting up the placeholdres
/// `tc_a`, `tc_b`, `tc_c`, `tc_d`, `tc_r0`, `rc_r1`.
///
/// The map requires that `PHON` is setup with the invisible object as the
/// on-grid placeholder.
///
/// May be used for multiple TCs.
pub fn tc_center() -> Vec<String> {
    String::from(
        "create_object PHON {
set_place_for_every_player
set_gaia_object_only
max_distance_to_players 0
actor_area tc_d
actor_area_radius 0
}
create_object PHON {
number_of_objects 2
set_place_for_every_player
set_gaia_object_only
find_closest
min_distance_to_players 1
max_distance_to_players 1
actor_area tc_r0
actor_area_radius 0
}
create_object PHON {
set_place_for_every_player
set_gaia_object_only
find_closest
min_distance_to_players 1
max_distance_to_players 1
avoid_actor_area tc_r0
actor_area tc_c
actor_area_radius 0
}
create_object PHON {
set_place_for_every_player
set_gaia_object_only
find_closest
min_distance_to_players 1
max_distance_to_players 1
avoid_actor_area tc_r0
avoid_actor_area tc_c
actor_area tc_b
actor_area_radius 0
}
create_object PHON {
number_of_objects 2
set_place_for_every_player
set_gaia_object_only
find_closest
min_distance_to_players 1
max_distance_to_players 1
actor_area tc_r1
actor_area_radius 1
}
create_object PHON {
set_place_for_every_player
set_gaia_object_only
max_distance_to_players 1
avoid_actor_area tc_r1
actor_area tc_a
actor_area_radius 0
}",
    )
    .split("\n")
    .map(String::from)
    .collect::<Vec<String>>()
}

/// Returns a String of lines for setting up the placeholdres
/// `tc_a`, `tc_b`, `tc_c`, `tc_d`, `tc_r0`, `rc_r1`.
///
/// The map requires that `PHON` is setup with the invisible object as the
/// on-grid placeholder.
///
/// May be used for multiple TCs.
/// Used for player lands with explicit land ids 1, 2, 3, 4, 5, 6, 7, and 8.
pub fn tc_center2() -> Vec<String> {
    // TODO
    let strings: Vec<String> = [1, 2, 3, 4, 5, 6, 7, 8]
        .map(|land_id| {
            format!(
                "create_object PHON {{
place_on_specific_land_id {land_id}
set_gaia_object_only
max_distance_to_players 0
actor_area tc_d
actor_area_radius 0
}}
create_object PHON {{
number_of_objects 2
place_on_specific_land_id {land_id}
set_gaia_object_only
find_closest
min_distance_to_players 1
max_distance_to_players 1
actor_area tc_r0
actor_area_radius 0
}}
create_object PHON {{
place_on_specific_land_id {land_id}
set_gaia_object_only
find_closest
min_distance_to_players 1
max_distance_to_players 1
avoid_actor_area tc_r0
actor_area tc_c
actor_area_radius 0
}}
create_object PHON {{
place_on_specific_land_id {land_id}
set_gaia_object_only
find_closest
min_distance_to_players 1
max_distance_to_players 1
avoid_actor_area tc_r0
avoid_actor_area tc_c
actor_area tc_b
actor_area_radius 0
}}
create_object PHON {{
number_of_objects 2
place_on_specific_land_id {land_id}
set_gaia_object_only
find_closest
min_distance_to_players 1
max_distance_to_players 1
actor_area tc_r1
actor_area_radius 1
}}
create_object PHON {{
place_on_specific_land_id {land_id}
set_gaia_object_only
max_distance_to_players 1
avoid_actor_area tc_r1
actor_area tc_a
actor_area_radius 0
}}",
            )
        })
        .to_vec();
    strings
        .join("\n")
        .split("\n")
        .map(String::from)
        .collect::<Vec<String>>()
}

/// Returns a vector of lines for placing actor area boxes around player TCs.
pub fn tc_boxes() -> Vec<String> {
    let mut lines = vec![];
    for tile in ["a", "b", "c", "d"] {
        lines.push(String::from("create_object PHON {"));
        lines.push(String::from("set_place_for_every_player"));
        lines.push(String::from("set_gaia_object_only"));
        lines.push(format!("actor_area_to_place_in tc_{tile}"));
        lines.push(String::from("actor_area box0"));
        lines.push(String::from("actor_area_radius 0"));
        lines.push(String::from("}"));
    }
    for i in 1..64 {
        lines.push(String::from("create_object PHON {"));
        lines.push(String::from("number_of_objects 4"));
        lines.push(String::from("set_place_for_every_player"));
        lines.push(String::from("set_gaia_object_only"));
        lines.push(String::from("actor_area_to_place_in box0"));
        lines.push(format!("actor_area box{i}"));
        lines.push(format!("actor_area_radius {i}"));
        lines.push(String::from("}"));
    }
    lines
}

/// Returns a vector of lines for placing actor area boxes around player TCs.
/// Used for player lands that have individual land IDs 1, 2, 3, 4, 5, 6, 7, and 8.
pub fn tc_boxes2() -> Vec<String> {
    let mut lines = vec![];
    for land_id in [1, 2, 3, 4, 5, 6, 7, 8] {
        for tile in ["a", "b", "c", "d"] {
            lines.push(String::from("create_object PHON {"));
            lines.push(format!("place_on_specific_land_id {land_id}"));
            lines.push(String::from("set_gaia_object_only"));
            lines.push(format!("actor_area_to_place_in tc_{tile}"));
            lines.push(String::from("actor_area box0"));
            lines.push(String::from("actor_area_radius 0"));
            lines.push(String::from("}"));
        }
        for i in 1..64 {
            lines.push(String::from("create_object PHON {"));
            lines.push(String::from("number_of_objects 4"));
            lines.push(format!("place_on_specific_land_id {land_id}"));
            lines.push(String::from("set_gaia_object_only"));
            lines.push(String::from("actor_area_to_place_in box0"));
            lines.push(format!("actor_area box{i}"));
            lines.push(format!("actor_area_radius {i}"));
            lines.push(String::from("}"));
        }
    }
    lines
}

/// Returns a vector of lines for generating boxes for multiple TCs.
pub fn tc_multiboxes() -> Vec<String> {
    let mut lines = vec![];
    for tile in ["a", "b", "c", "d"] {
        lines.push(String::from("create_object PHON {"));
        lines.push(String::from("set_place_for_every_player"));
        lines.push(String::from("set_gaia_object_only"));
        lines.push(format!("actor_area_to_place_in tc_{tile}"));
        lines.push(String::from("avoid_actor_area box0"));
        lines.push(String::from("actor_area box0"));
        lines.push(String::from("actor_area_radius 0"));
        lines.push(String::from("}"));
    }
    for i in 1..64 {
        lines.push(String::from("create_object PHON {"));
        lines.push(String::from("number_of_objects 8"));
        lines.push(String::from("set_place_for_every_player"));
        lines.push(String::from("set_gaia_object_only"));
        lines.push(String::from("actor_area_to_place_in box0"));
        lines.push(format!("actor_area box{i}"));
        lines.push(format!("actor_area_radius {i}"));
        lines.push(String::from("}"));
    }
    lines
}

/// Places 9 Villagers under the TC.
///
/// Requries that the constants `SHEP0`, ..., `SHEP5` are set (randomly) as
/// male or female Shepherds.
pub fn vils_9_tc() -> Vec<String> {
    let mut lines = vec![];
    lines.append(
        &mut String::from(
            " create_object PHON {
set_place_for_every_player
set_gaia_object_only
number_of_objects 5
min_distance_to_players 1
max_distance_to_players 1
find_closest
actor_area near_positioner
actor_area_radius 0
}
create_object PHOFF {
set_place_for_every_player
min_distance_to_players 1
max_distance_to_players 1
find_closest
second_object HERDABLE_A
actor_area herd0
actor_area_radius 0
avoid_actor_area near_positioner
}
create_object PHON {
set_place_for_every_player
set_gaia_object_only
actor_area_to_place_in herd0
actor_area herd1
actor_area_radius 1
}
create_object PHON {
set_place_for_every_player
set_gaia_object_only
max_distance_to_players 0
actor_area tc_d1
actor_area_radius 1
}
create_object PHON {
set_place_for_every_player
set_gaia_object_only
number_of_objects 5
max_distance_to_players 2
temp_min_distance_group_placement 1
find_closest
avoid_actor_area tc_d1
actor_area far_positioner
actor_area_radius 1
}
create_object PHON {
set_place_for_every_player
set_gaia_object_only
max_distance_to_players 2
find_closest
avoid_actor_area tc_d1
avoid_actor_area far_positioner
actor_area blocking_seventh_villager
actor_area_radius 0
}",
        )
        .split("\n")
        .map(String::from)
        .collect::<Vec<String>>(),
    );
    for i in 0..6 {
        lines.push(String::from("create_object PHOFF {"));
        lines.push(String::from("set_place_for_every_player"));
        lines.push(String::from("actor_area_to_place_in herd1"));
        lines.push(String::from("avoid_actor_area tc_d"));
        lines.push(String::from("avoid_actor_area villager0"));
        lines.push(String::from("avoid_actor_area herd0"));
        lines.push(String::from("avoid_actor_area blocking_seventh_villager"));
        lines.push(String::from("actor_area villager0"));
        lines.push(String::from("actor_area_radius 0"));
        lines.push(format!("second_object SHEP{i}"));
        lines.push(String::from("}"));
    }
    lines
}

/// The 9 Villagers for Roe Rage.
pub fn vils_9_tc_roe_rage() -> Vec<String> {
    let mut lines = vec![];
    lines.append(
        &mut String::from(
            " create_object PHON {
set_place_for_every_player
set_gaia_object_only
number_of_objects 5
min_distance_to_players 1
max_distance_to_players 1
find_closest
actor_area near_positioner
actor_area_radius 0
}
create_object PHOFF {
set_place_for_every_player
set_gaia_object_only
min_distance_to_players 1
max_distance_to_players 1
find_closest
second_object MARLIN1
actor_area herd0
actor_area_radius 0
avoid_actor_area near_positioner
}
create_object PHON {
set_place_for_every_player
set_gaia_object_only
actor_area_to_place_in herd0
actor_area herd1
actor_area_radius 1
}
create_object PHON {
set_place_for_every_player
set_gaia_object_only
max_distance_to_players 0
actor_area tc_d1
actor_area_radius 1
}
create_object PHON {
set_place_for_every_player
set_gaia_object_only
number_of_objects 5
max_distance_to_players 2
temp_min_distance_group_placement 1
find_closest
avoid_actor_area tc_d1
actor_area far_positioner
actor_area_radius 1
}
create_object PHON {
set_place_for_every_player
set_gaia_object_only
max_distance_to_players 2
find_closest
avoid_actor_area tc_d1
avoid_actor_area far_positioner
actor_area blocking_seventh_villager
actor_area_radius 0
}",
        )
        .split("\n")
        .map(String::from)
        .collect::<Vec<String>>(),
    );
    for i in 0..6 {
        lines.push(String::from("create_object PHOFF {"));
        lines.push(String::from("set_place_for_every_player"));
        lines.push(String::from("actor_area_to_place_in herd1"));
        lines.push(String::from("avoid_actor_area tc_d"));
        lines.push(String::from("avoid_actor_area villager0"));
        lines.push(String::from("avoid_actor_area herd0"));
        lines.push(String::from("avoid_actor_area blocking_seventh_villager"));
        lines.push(String::from("actor_area villager0"));
        lines.push(String::from("actor_area_radius 0"));
        lines.push(format!("second_object SHEP{i}"));
        lines.push(String::from("}"));
    }
    lines
}

/// ZeWall version of `vils9tc` that does not make the herdable gaia.
pub fn vils_9_tc_ze_wall() -> Vec<String> {
    let mut lines = vec![];
    lines.append(
        &mut String::from(
            " create_object PHON {
set_place_for_every_player
set_gaia_object_only
number_of_objects 5
min_distance_to_players 1
max_distance_to_players 1
find_closest
actor_area near_positioner
actor_area_radius 0
}
create_object PHOFF {
set_place_for_every_player
min_distance_to_players 1
max_distance_to_players 1
find_closest
second_object HERDABLE_A
actor_area herd0
actor_area_radius 0
avoid_actor_area near_positioner
}
create_object PHON {
set_place_for_every_player
set_gaia_object_only
actor_area_to_place_in herd0
actor_area herd1
actor_area_radius 1
}
create_object PHON {
set_place_for_every_player
set_gaia_object_only
max_distance_to_players 0
actor_area tc_d1
actor_area_radius 1
}
create_object PHON {
set_place_for_every_player
set_gaia_object_only
number_of_objects 5
max_distance_to_players 2
temp_min_distance_group_placement 1
find_closest
avoid_actor_area tc_d1
actor_area far_positioner
actor_area_radius 1
}
create_object PHON {
set_place_for_every_player
set_gaia_object_only
max_distance_to_players 2
find_closest
avoid_actor_area tc_d1
avoid_actor_area far_positioner
actor_area blocking_seventh_villager
actor_area_radius 0
}",
        )
        .split("\n")
        .map(String::from)
        .collect::<Vec<String>>(),
    );
    for i in 0..6 {
        lines.push(String::from("create_object PHOFF {"));
        lines.push(String::from("set_place_for_every_player"));
        lines.push(String::from("actor_area_to_place_in herd1"));
        lines.push(String::from("avoid_actor_area tc_d"));
        lines.push(String::from("avoid_actor_area villager0"));
        lines.push(String::from("avoid_actor_area herd0"));
        lines.push(String::from("avoid_actor_area blocking_seventh_villager"));
        lines.push(String::from("actor_area villager0"));
        lines.push(String::from("actor_area_radius 0"));
        lines.push(format!("second_object SHEP{i}"));
        lines.push(String::from("}"));
    }
    lines
}

/// Returns the lines for 9 Villagers with a 2-TC start.
pub fn multi_vils_9_tc() -> Vec<String> {
    let mut lines = vec![
        String::from("create_object PHON {"),
        String::from("set_place_for_every_player"),
        String::from("set_gaia_object_only"),
        String::from("number_of_objects 5"),
        String::from("min_distance_to_players 1"),
        String::from("max_distance_to_players 1"),
        String::from("find_closest"),
        String::from("actor_area near_positioner"),
        String::from("actor_area_radius 0"),
        String::from("}"),
        String::from("create_object PHOFF {"),
        String::from("set_place_for_every_player"),
        String::from("set_gaia_object_only"),
        String::from("min_distance_to_players 1"),
        String::from("max_distance_to_players 1"),
        String::from("find_closest"),
        String::from("second_object GRASS_HERDABLE_A"),
        String::from("avoid_actor_area snow-region"),
        String::from("avoid_actor_area dirt-region"),
        String::from("avoid_actor_area jungle-region"),
        String::from("actor_area herd0"),
        String::from("actor_area_radius 0"),
        String::from("avoid_actor_area near_positioner"),
        String::from("}"),
        String::from("create_object PHOFF {"),
        String::from("set_place_for_every_player"),
        String::from("set_gaia_object_only"),
        String::from("min_distance_to_players 1"),
        String::from("max_distance_to_players 1"),
        String::from("find_closest"),
        String::from("second_object SNOW_HERDABLE_A"),
        String::from("avoid_actor_area grass-region"),
        String::from("avoid_actor_area dirt-region"),
        String::from("avoid_actor_area jungle-region"),
        String::from("actor_area herd0"),
        String::from("actor_area_radius 0"),
        String::from("avoid_actor_area near_positioner"),
        String::from("}"),
        String::from("create_object PHOFF {"),
        String::from("set_place_for_every_player"),
        String::from("set_gaia_object_only"),
        String::from("min_distance_to_players 1"),
        String::from("max_distance_to_players 1"),
        String::from("find_closest"),
        String::from("second_object DIRT_HERDABLE_A"),
        String::from("avoid_actor_area grass-region"),
        String::from("avoid_actor_area snow-region"),
        String::from("avoid_actor_area jungle-region"),
        String::from("actor_area herd0"),
        String::from("actor_area_radius 0"),
        String::from("avoid_actor_area near_positioner"),
        String::from("}"),
        String::from("create_object PHOFF {"),
        String::from("set_place_for_every_player"),
        String::from("set_gaia_object_only"),
        String::from("min_distance_to_players 1"),
        String::from("max_distance_to_players 1"),
        String::from("find_closest"),
        String::from("second_object JUNGLE_HERDABLE_A"),
        String::from("avoid_actor_area grass-region"),
        String::from("avoid_actor_area snow-region"),
        String::from("avoid_actor_area dirt-region"),
        String::from("actor_area herd0"),
        String::from("actor_area_radius 0"),
        String::from("avoid_actor_area near_positioner"),
        String::from("}"),
        String::from("create_object PHON {"),
        String::from("number_of_objects 2"),
        String::from("set_place_for_every_player"),
        String::from("set_gaia_object_only"),
        String::from("actor_area_to_place_in herd0"),
        String::from("actor_area herd1"),
        String::from("actor_area_radius 1"),
        String::from("}"),
        String::from("create_object PHON {"),
        String::from("set_place_for_every_player"),
        String::from("set_gaia_object_only"),
        String::from("max_distance_to_players 0"),
        String::from("actor_area tc_d1"),
        String::from("actor_area_radius 1"),
        String::from("}"),
        String::from("create_object PHON {"),
        String::from("set_place_for_every_player"),
        String::from("set_gaia_object_only"),
        String::from("number_of_objects 5"),
        String::from("max_distance_to_players 2"),
        String::from("temp_min_distance_group_placement 1"),
        String::from("find_closest"),
        String::from("avoid_actor_area tc_d1"),
        String::from("actor_area far_positioner"),
        String::from("actor_area_radius 1"),
        String::from("}"),
        String::from("create_object PHON {"),
        String::from("set_place_for_every_player"),
        String::from("set_gaia_object_only"),
        String::from("max_distance_to_players 2"),
        String::from("find_closest"),
        String::from("avoid_actor_area tc_d1"),
        String::from("avoid_actor_area far_positioner"),
        String::from("actor_area blocking_seventh_villager"),
        String::from("actor_area_radius 0"),
        String::from("}"),
    ];
    // Each Shepard is duplicated at each TC.
    // But in total 12 Villagers are placed, 6 at each TC.
    for i in 0..6 {
        lines.push(String::from("create_object PHOFF {"));
        // lines.push(String::from("generate_for_first_land_only"));
        lines.push(String::from("set_place_for_every_player"));
        lines.push(String::from("actor_area_to_place_in herd1"));
        lines.push(String::from("avoid_actor_area tc_d"));
        lines.push(String::from("avoid_actor_area villager0"));
        lines.push(String::from("avoid_actor_area herd0"));
        lines.push(String::from("avoid_actor_area blocking_seventh_villager"));
        lines.push(String::from("actor_area villager0"));
        lines.push(String::from("actor_area_radius 0"));
        lines.push(String::from("max_distance_to_players 4"));
        lines.push(format!("second_object SHEP{i}"));
        lines.push(String::from("}"));
    }
    // for i in 6..12 {
    //     lines.push(String::from("create_object PHOFF {"));
    //     lines.push(String::from("avoid_actor_area first_land_20"));
    //     lines.push(String::from("set_place_for_every_player"));
    //     lines.push(String::from("actor_area_to_place_in herd1"));
    //     lines.push(String::from("avoid_actor_area tc_d"));
    //     lines.push(String::from("avoid_actor_area villager0"));
    //     lines.push(String::from("avoid_actor_area herd0"));
    //     lines.push(String::from("avoid_actor_area blocking_seventh_villager"));
    //     lines.push(String::from("actor_area villager0"));
    //     lines.push(String::from("actor_area_radius 0"));
    //     lines.push(String::from("max_distance_to_players 4"));
    //     lines.push(format!("second_object SHEP{i}"));
    //     lines.push(String::from("}"));
    // }
    lines
}

/// Returns a list of actor areas for placing Houses.
///
/// The Houses spawn with a 3-tile gap between them and the TC.
/// A sequence of actor areas is needed in order to maintain the same gap
/// on both the left and right sides of the TC. The right corner (D) of the
/// House is the location where it is placed.
///
/// Actor areas `house0`, `house1`, ... `house9` are returned, giving boxes
/// around the Houses of equal distance. Again, these boxes are squares
/// around the houses, with the center `house0` including the 2x2 tiles
/// covering the House.
///
/// Places a terrain blocker 1 tile around the House.
pub fn house_gap_3() -> Vec<String> {
    let mut lines = String::from(
        "create_object PHON {
number_of_objects 2
set_gaia_object_only
set_place_for_every_player
find_closest
min_distance_to_players 5
max_distance_to_players 5
actor_area house_avoid_box5
actor_area_radius 5
}
create_object PHON {
number_of_objects 99
set_gaia_object_only
set_place_for_every_player
actor_area_to_place_in box5
avoid_actor_area house_avoid_box5
avoid_actor_area box4
actor_area house_placement
actor_area_radius 0
}
create_object PHON {
number_of_objects 2
set_gaia_object_only
set_place_for_every_player
find_closest
min_distance_to_players 5
max_distance_to_players 5
actor_area house_placement_box6
actor_area_radius 6
}
create_object PHON {
number_of_objects 99
set_place_for_every_player
set_gaia_object_only
actor_area_to_place_in house_placement_box6
avoid_actor_area box5
max_distance_to_players 7
actor_area house_placement
actor_area_radius 0
}",
    )
    .split("\n")
    .map(String::from)
    .collect::<Vec<String>>();
    lines.append(
        &mut String::from(
            "create_object HOUSE {
number_of_objects 2
temp_min_distance_group_placement 7
set_place_for_every_player
avoid_forest_zone 2
actor_area_to_place_in house_placement
actor_area house_right_0
actor_area_radius 0
}
create_object PHON {
number_of_objects 2
set_place_for_every_player
set_gaia_object_only
actor_area_to_place_in house_right_0
actor_area house_right_1
actor_area_radius 1
}
create_object PHON {
number_of_objects 18
set_place_for_every_player
set_gaia_object_only
actor_area_to_place_in house_right_1
actor_area house1_cover
actor_area_radius 0
}
create_object TERRAIN_BLOCKER {
number_of_objects 10
set_place_for_every_player
set_gaia_object_only
actor_area_to_place_in house1_cover
actor_area outside_house1
actor_area_radius 0
}
create_object PHON {
number_of_objects 8
set_place_for_every_player
set_gaia_object_only
actor_area_to_place_in house1_cover
avoid_actor_area outside_house1
actor_area house0
actor_area_radius 0
}
create_object PHON {
number_of_objects 8
set_place_for_every_player
set_gaia_object_only
actor_area_to_place_in house0
actor_area house1
actor_area_radius 1
}
create_object PHON {
number_of_objects 8
set_place_for_every_player
set_gaia_object_only
actor_area_to_place_in house0
actor_area house2
actor_area_radius 2
}
create_object PHON {
number_of_objects 8
set_place_for_every_player
set_gaia_object_only
actor_area_to_place_in house0
actor_area house3
actor_area_radius 3
}
create_object PHON {
number_of_objects 8
set_place_for_every_player
set_gaia_object_only
actor_area_to_place_in house0
actor_area house4
actor_area_radius 4
}
create_object TERRAIN_BLOCKER {
number_of_objects 14
set_place_for_every_player
set_gaia_object_only
actor_area_to_place_in house1
}",
        )
        .split("\n")
        .map(String::from)
        .collect::<Vec<String>>(),
    );
    for i in 2..10 {
        lines.push(String::from("create_object PHON {"));
        lines.push(String::from("number_of_objects 8"));
        lines.push(String::from("set_place_for_every_player"));
        lines.push(String::from("set_gaia_object_only"));
        lines.push(String::from("actor_area_to_place_in house0"));
        lines.push(format!("actor_area house{i}"));
        lines.push(format!("actor_area_radius {i}"));
        lines.push(String::from("}"));
    }
    lines
}

/// Returns the lines for Houses for a 2-TC start.
pub fn multi_houses() -> Vec<String> {
    vec![
        String::from("create_object PHON {"),
        String::from("number_of_objects 2"),
        String::from("set_gaia_object_only"),
        String::from("set_place_for_every_player"),
        String::from("find_closest"),
        String::from("min_distance_to_players 5"),
        String::from("max_distance_to_players 5"),
        String::from("actor_area house_avoid_box5"),
        String::from("actor_area_radius 5"),
        String::from("}"),
        String::from("create_object PHON {"),
        String::from("number_of_objects 99"),
        String::from("set_gaia_object_only"),
        String::from("set_place_for_every_player"),
        String::from("actor_area_to_place_in box5"),
        String::from("avoid_actor_area house_avoid_box5"),
        String::from("avoid_actor_area box4"),
        String::from("actor_area house_placement"),
        String::from("actor_area_radius 0"),
        String::from("}"),
        String::from("create_object PHON {"),
        String::from("number_of_objects 2"),
        String::from("set_gaia_object_only"),
        String::from("set_place_for_every_player"),
        String::from("find_closest"),
        String::from("min_distance_to_players 5"),
        String::from("max_distance_to_players 5"),
        String::from("actor_area house_placement_box6"),
        String::from("actor_area_radius 6"),
        String::from("}"),
        String::from("create_object PHON {"),
        String::from("number_of_objects 99"),
        String::from("set_place_for_every_player"),
        String::from("set_gaia_object_only"),
        String::from("actor_area_to_place_in house_placement_box6"),
        String::from("avoid_actor_area box5"),
        String::from("max_distance_to_players 7"),
        String::from("actor_area house_placement"),
        String::from("actor_area_radius 0"),
        String::from("}"),
        String::from("create_object HOUSE {"),
        String::from("number_of_objects 2"),
        String::from("temp_min_distance_group_placement 7"),
        String::from("set_place_for_every_player"),
        String::from("avoid_forest_zone 2"),
        String::from("actor_area_to_place_in house_placement"),
        String::from("actor_area house_right_0"),
        String::from("actor_area_radius 0"),
        String::from("max_distance_to_players 10"),
        String::from("}"),
        String::from("create_object PHON {"),
        String::from("number_of_objects 2"),
        String::from("set_place_for_every_player"),
        String::from("set_gaia_object_only"),
        String::from("actor_area_to_place_in house_right_0"),
        String::from("actor_area house_right_1"),
        String::from("actor_area_radius 1"),
        String::from("max_distance_to_players 10"),
        String::from("}"),
        String::from("create_object PHON {"),
        String::from("number_of_objects 18"),
        String::from("set_place_for_every_player"),
        String::from("set_gaia_object_only"),
        String::from("actor_area_to_place_in house_right_1"),
        String::from("actor_area house1_cover"),
        String::from("actor_area_radius 0"),
        String::from("max_distance_to_players 10"),
        String::from("}"),
        String::from("create_object TERRAIN_BLOCKER {"),
        String::from("number_of_objects 10"),
        String::from("set_place_for_every_player"),
        String::from("set_gaia_object_only"),
        String::from("actor_area_to_place_in house1_cover"),
        String::from("actor_area outside_house1"),
        String::from("actor_area_radius 0"),
        String::from("max_distance_to_players 10"),
        String::from("}"),
        String::from("create_object PHON {"),
        String::from("number_of_objects 8"),
        String::from("set_place_for_every_player"),
        String::from("set_gaia_object_only"),
        String::from("actor_area_to_place_in house1_cover"),
        String::from("avoid_actor_area outside_house1"),
        String::from("actor_area house0"),
        String::from("actor_area_radius 0"),
        String::from("max_distance_to_players 10"),
        String::from("}"),
        String::from("create_object PHON {"),
        String::from("number_of_objects 8"),
        String::from("set_place_for_every_player"),
        String::from("set_gaia_object_only"),
        String::from("actor_area_to_place_in house0"),
        String::from("actor_area house1"),
        String::from("actor_area_radius 1"),
        String::from("max_distance_to_players 10"),
        String::from("}"),
        String::from("create_object PHON {"),
        String::from("number_of_objects 8"),
        String::from("set_place_for_every_player"),
        String::from("set_gaia_object_only"),
        String::from("actor_area_to_place_in house0"),
        String::from("actor_area house2"),
        String::from("actor_area_radius 2"),
        String::from("max_distance_to_players 10"),
        String::from("}"),
        String::from("create_object PHON {"),
        String::from("number_of_objects 8"),
        String::from("set_place_for_every_player"),
        String::from("set_gaia_object_only"),
        String::from("actor_area_to_place_in house0"),
        String::from("actor_area house3"),
        String::from("actor_area_radius 3"),
        String::from("max_distance_to_players 10"),
        String::from("}"),
        String::from("create_object PHON {"),
        String::from("number_of_objects 8"),
        String::from("set_place_for_every_player"),
        String::from("set_gaia_object_only"),
        String::from("actor_area_to_place_in house0"),
        String::from("actor_area house4"),
        String::from("actor_area_radius 4"),
        String::from("max_distance_to_players 10"),
        String::from("} "),
        String::from("create_object TERRAIN_BLOCKER {"),
        String::from("number_of_objects 14"),
        String::from("set_place_for_every_player"),
        String::from("set_gaia_object_only"),
        String::from("actor_area_to_place_in house1"),
        String::from("max_distance_to_players 10"),
        String::from("}"),
    ]
}

/// The same as `house_gap_3`, but uses Huts instead of Houses.
pub fn hut_gap_3() -> Vec<String> {
    house_gap_3()
        .iter()
        .map(|s| s.replace("HOUSE", "HUT"))
        .collect()
}

/// Places straggler trees, with one surrounded by Villagers.
/// Places 2 straggler trees 2 tiles from the TC.
/// Places 3 straggler trees 3 tiles from the TC.
/// Places 3 Lumberjacks around one of the 3-tile stragglers.
pub fn vils_9_straggler() -> Vec<String> {
    let mut lines = String::from(
        "create_object STRAGGLER {
set_place_for_every_player
set_gaia_object_only
avoid_forest_zone 2
actor_area_to_place_in box5
avoid_actor_area box4
avoid_actor_area house2
actor_area villager_tree0
actor_area_radius 0
}
create_object PHON {
set_place_for_every_player
set_gaia_object_only
actor_area_to_place_in villager_tree0
actor_area villager_tree1
actor_area_radius 1
}
create_object PHON {
set_place_for_every_player
set_gaia_object_only
actor_area_to_place_in villager_tree0
actor_area straggler2
actor_area_radius 2
}
create_object STRAGGLER {
number_of_objects 2
set_place_for_every_player
set_gaia_object_only
temp_min_distance_group_placement 2
avoid_forest_zone 2
actor_area_to_place_in box4
avoid_actor_area box3
avoid_actor_area house1
avoid_actor_area straggler2
actor_area straggler2
actor_area_radius 2
}
create_object STRAGGLER {
number_of_objects 2
set_place_for_every_player
set_gaia_object_only
temp_min_distance_group_placement 3
avoid_forest_zone 2
actor_area_to_place_in box5
avoid_actor_area box4
avoid_actor_area house1
avoid_actor_area straggler2
actor_area straggler2
actor_area_radius 2
}",
    )
    .split("\n")
    .map(String::from)
    .collect::<Vec<String>>();
    for i in 0..3 {
        lines.push(format!("create_object LUMBERJACK{i} {{"));
        lines.push(String::from("set_place_for_every_player"));
        lines.push(String::from("actor_area_to_place_in villager_tree1"));
        lines.push(String::from("actor_area villager0"));
        lines.push(String::from("actor_area_radius 0"));
        lines.push(String::from("}"));
    }
    lines
}

/// Places straggler trees, with one surrounded by Villagers.
/// Keeps the one surrounded by Villagers as close to the map
/// edge as possible.
/// Places 2 straggler trees 2 tiles from the TC.
/// Places 3 straggler trees 3 tiles from the TC.
/// Places 3 Lumberjacks around one of the 3-tile stragglers.
pub fn vils_9_straggler_socotra() -> Vec<String> {
    let mut lines = String::from(
        "create_object STRAGGLER {
set_place_for_every_player
set_gaia_object_only
find_closest_to_map_edge
avoid_forest_zone 2
actor_area_to_place_in box5
avoid_actor_area box4
avoid_actor_area house2
actor_area villager_tree0
actor_area_radius 0
}
create_object PHON {
set_place_for_every_player
set_gaia_object_only
actor_area_to_place_in villager_tree0
actor_area villager_tree1
actor_area_radius 1
}
create_object PHON {
set_place_for_every_player
set_gaia_object_only
actor_area_to_place_in villager_tree0
actor_area straggler2
actor_area_radius 2
}
create_object STRAGGLER {
number_of_objects 2
set_place_for_every_player
set_gaia_object_only
temp_min_distance_group_placement 2
avoid_forest_zone 2
actor_area_to_place_in box4
avoid_actor_area box3
avoid_actor_area house1
avoid_actor_area straggler2
actor_area straggler2
actor_area_radius 2
}
create_object STRAGGLER {
number_of_objects 2
set_place_for_every_player
set_gaia_object_only
temp_min_distance_group_placement 3
avoid_forest_zone 2
actor_area_to_place_in box5
avoid_actor_area box4
avoid_actor_area house1
avoid_actor_area straggler2
actor_area straggler2
actor_area_radius 2
}",
    )
    .split("\n")
    .map(String::from)
    .collect::<Vec<String>>();
    for i in 0..3 {
        lines.push(format!("create_object LUMBERJACK{i} {{"));
        lines.push(String::from("set_place_for_every_player"));
        lines.push(String::from("actor_area_to_place_in villager_tree1"));
        lines.push(String::from("actor_area villager0"));
        lines.push(String::from("actor_area_radius 0"));
        lines.push(String::from("}"));
    }
    lines
}

/// Returns the lines for straggler for a 2-TC start.
pub fn multi_stragglers() -> Vec<String> {
    let mut lines = vec![
        String::from("create_object GRASS_STRAGGLER {"),
        String::from("set_place_for_every_player"),
        String::from("set_gaia_object_only"),
        String::from("avoid_forest_zone 2"),
        String::from("actor_area_to_place_in box5"),
        String::from("avoid_actor_area box4"),
        String::from("avoid_actor_area house2"),
        String::from("actor_area villager_tree0"),
        String::from("actor_area_radius 0"),
        String::from("max_distance_to_players 10"),
        String::from("avoid_actor_area snow-region"),
        String::from("avoid_actor_area dirt-region"),
        String::from("avoid_actor_area jungle-region"),
        String::from("}"),
        String::from("create_object SNOW_STRAGGLER {"),
        String::from("set_place_for_every_player"),
        String::from("set_gaia_object_only"),
        String::from("avoid_forest_zone 2"),
        String::from("actor_area_to_place_in box5"),
        String::from("avoid_actor_area box4"),
        String::from("avoid_actor_area house2"),
        String::from("actor_area villager_tree0"),
        String::from("actor_area_radius 0"),
        String::from("max_distance_to_players 10"),
        String::from("avoid_actor_area grass-region"),
        String::from("avoid_actor_area dirt-region"),
        String::from("avoid_actor_area jungle-region"),
        String::from("}"),
        String::from("create_object DIRT_STRAGGLER {"),
        String::from("set_place_for_every_player"),
        String::from("set_gaia_object_only"),
        String::from("avoid_forest_zone 2"),
        String::from("actor_area_to_place_in box5"),
        String::from("avoid_actor_area box4"),
        String::from("avoid_actor_area house2"),
        String::from("actor_area villager_tree0"),
        String::from("actor_area_radius 0"),
        String::from("max_distance_to_players 10"),
        String::from("avoid_actor_area grass-region"),
        String::from("avoid_actor_area snow-region"),
        String::from("avoid_actor_area jungle-region"),
        String::from("}"),
        String::from("create_object JUNGLE_STRAGGLER {"),
        String::from("set_place_for_every_player"),
        String::from("set_gaia_object_only"),
        String::from("avoid_forest_zone 2"),
        String::from("actor_area_to_place_in box5"),
        String::from("avoid_actor_area box4"),
        String::from("avoid_actor_area house2"),
        String::from("actor_area villager_tree0"),
        String::from("actor_area_radius 0"),
        String::from("max_distance_to_players 10"),
        String::from("avoid_actor_area grass-region"),
        String::from("avoid_actor_area snow-region"),
        String::from("avoid_actor_area dirt-region"),
        String::from("}"),
        String::from("create_object PHON {"),
        String::from("set_place_for_every_player"),
        String::from("set_gaia_object_only"),
        String::from("actor_area_to_place_in villager_tree0"),
        String::from("actor_area villager_tree1"),
        String::from("actor_area_radius 1"),
        String::from("max_distance_to_players 10"),
        String::from("}"),
        String::from("create_object PHON {"),
        String::from("set_place_for_every_player"),
        String::from("set_gaia_object_only"),
        String::from("actor_area_to_place_in villager_tree0"),
        String::from("actor_area straggler2"),
        String::from("actor_area_radius 2"),
        String::from("max_distance_to_players 10"),
        String::from("}"),
        String::from("create_object GRASS_STRAGGLER {"),
        String::from("number_of_objects 2"),
        String::from("set_place_for_every_player"),
        String::from("set_gaia_object_only"),
        String::from("temp_min_distance_group_placement 2"),
        String::from("avoid_forest_zone 2"),
        String::from("actor_area_to_place_in box4"),
        String::from("avoid_actor_area box3"),
        String::from("avoid_actor_area house1"),
        String::from("avoid_actor_area straggler0"),
        String::from("actor_area straggler0"),
        String::from("actor_area_radius 0"),
        String::from("max_distance_to_players 10"),
        String::from("avoid_actor_area snow-region"),
        String::from("avoid_actor_area dirt-region"),
        String::from("avoid_actor_area jungle-region"),
        String::from("}"),
        String::from("create_object SNOW_STRAGGLER {"),
        String::from("number_of_objects 2"),
        String::from("set_place_for_every_player"),
        String::from("set_gaia_object_only"),
        String::from("temp_min_distance_group_placement 2"),
        String::from("avoid_forest_zone 2"),
        String::from("actor_area_to_place_in box4"),
        String::from("avoid_actor_area box3"),
        String::from("avoid_actor_area house1"),
        String::from("avoid_actor_area straggler0"),
        String::from("actor_area straggler0"),
        String::from("actor_area_radius 0"),
        String::from("max_distance_to_players 10"),
        String::from("avoid_actor_area grass-region"),
        String::from("avoid_actor_area dirt-region"),
        String::from("avoid_actor_area jungle-region"),
        String::from("}"),
        String::from("create_object DIRT_STRAGGLER {"),
        String::from("number_of_objects 2"),
        String::from("set_place_for_every_player"),
        String::from("set_gaia_object_only"),
        String::from("temp_min_distance_group_placement 2"),
        String::from("avoid_forest_zone 2"),
        String::from("actor_area_to_place_in box4"),
        String::from("avoid_actor_area box3"),
        String::from("avoid_actor_area house1"),
        String::from("avoid_actor_area straggler0"),
        String::from("actor_area straggler0"),
        String::from("actor_area_radius 0"),
        String::from("max_distance_to_players 10"),
        String::from("avoid_actor_area grass-region"),
        String::from("avoid_actor_area snow-region"),
        String::from("avoid_actor_area jungle-region"),
        String::from("}"),
        String::from("create_object JUNGLE_STRAGGLER {"),
        String::from("number_of_objects 2"),
        String::from("set_place_for_every_player"),
        String::from("set_gaia_object_only"),
        String::from("temp_min_distance_group_placement 2"),
        String::from("avoid_forest_zone 2"),
        String::from("actor_area_to_place_in box4"),
        String::from("avoid_actor_area box3"),
        String::from("avoid_actor_area house1"),
        String::from("avoid_actor_area straggler0"),
        String::from("actor_area straggler0"),
        String::from("actor_area_radius 0"),
        String::from("max_distance_to_players 10"),
        String::from("avoid_actor_area grass-region"),
        String::from("avoid_actor_area snow-region"),
        String::from("avoid_actor_area dirt-region"),
        String::from("}"),
        String::from("create_object PHON {"),
        String::from("number_of_objects 2"),
        String::from("set_place_for_every_player"),
        String::from("set_gaia_object_only"),
        String::from("actor_area_to_place_in straggler0"),
        String::from("actor_area straggler2"),
        String::from("actor_area_radius 2"),
        String::from("max_distance_to_players 10"),
        String::from("}"),
        String::from("create_object GRASS_STRAGGLER {"),
        String::from("number_of_objects 2"),
        String::from("set_place_for_every_player"),
        String::from("set_gaia_object_only"),
        String::from("temp_min_distance_group_placement 2"),
        String::from("avoid_forest_zone 2"),
        String::from("actor_area_to_place_in box5"),
        String::from("avoid_actor_area box4"),
        String::from("avoid_actor_area house1"),
        String::from("avoid_actor_area straggler0"),
        String::from("actor_area straggler0"),
        String::from("actor_area_radius 0"),
        String::from("max_distance_to_players 10"),
        String::from("avoid_actor_area snow-region"),
        String::from("avoid_actor_area dirt-region"),
        String::from("avoid_actor_area jungle-region"),
        String::from("}"),
        String::from("create_object SNOW_STRAGGLER {"),
        String::from("number_of_objects 2"),
        String::from("set_place_for_every_player"),
        String::from("set_gaia_object_only"),
        String::from("temp_min_distance_group_placement 2"),
        String::from("avoid_forest_zone 2"),
        String::from("actor_area_to_place_in box5"),
        String::from("avoid_actor_area box4"),
        String::from("avoid_actor_area house1"),
        String::from("avoid_actor_area straggler0"),
        String::from("actor_area straggler0"),
        String::from("actor_area_radius 0"),
        String::from("max_distance_to_players 10"),
        String::from("avoid_actor_area grass-region"),
        String::from("avoid_actor_area dirt-region"),
        String::from("avoid_actor_area jungle-region"),
        String::from("}"),
        String::from("create_object DIRT_STRAGGLER {"),
        String::from("number_of_objects 2"),
        String::from("set_place_for_every_player"),
        String::from("set_gaia_object_only"),
        String::from("temp_min_distance_group_placement 2"),
        String::from("avoid_forest_zone 2"),
        String::from("actor_area_to_place_in box5"),
        String::from("avoid_actor_area box4"),
        String::from("avoid_actor_area house1"),
        String::from("avoid_actor_area straggler0"),
        String::from("actor_area straggler0"),
        String::from("actor_area_radius 0"),
        String::from("max_distance_to_players 10"),
        String::from("avoid_actor_area grass-region"),
        String::from("avoid_actor_area snow-region"),
        String::from("avoid_actor_area jungle-region"),
        String::from("}"),
        String::from("create_object JUNGLE_STRAGGLER {"),
        String::from("number_of_objects 2"),
        String::from("set_place_for_every_player"),
        String::from("set_gaia_object_only"),
        String::from("temp_min_distance_group_placement 2"),
        String::from("avoid_forest_zone 2"),
        String::from("actor_area_to_place_in box5"),
        String::from("avoid_actor_area box4"),
        String::from("avoid_actor_area house1"),
        String::from("avoid_actor_area straggler0"),
        String::from("actor_area straggler0"),
        String::from("actor_area_radius 0"),
        String::from("max_distance_to_players 10"),
        String::from("avoid_actor_area grass-region"),
        String::from("avoid_actor_area snow-region"),
        String::from("avoid_actor_area dirt-region"),
        String::from("}"),
        String::from("create_object PHON {"),
        String::from("number_of_objects 4"),
        String::from("set_place_for_every_player"),
        String::from("set_gaia_object_only"),
        String::from("actor_area_to_place_in straggler0"),
        String::from("actor_area straggler2"),
        String::from("actor_area_radius 2"),
        String::from("max_distance_to_players 10"),
        String::from("}"),
        String::from("create_object PHON {"),
        String::from("set_place_for_every_player"),
        String::from("set_gaia_object_only"),
        String::from("actor_area_to_place_in villager_tree0"),
        String::from("actor_area straggler2"),
        String::from("actor_area_radius 2"),
        String::from("max_distance_to_players 10"),
        String::from("}"),
    ];
    for i in 0..3 {
        lines.push(format!("create_object LUMBERJACK{i} {{"));
        // lines.push(String::from("generate_for_first_land_only"));
        lines.push(String::from("max_distance_to_players 10"));
        lines.push(String::from("set_place_for_every_player"));
        lines.push(String::from("actor_area_to_place_in villager_tree1"));
        lines.push(String::from("actor_area villager0"));
        lines.push(String::from("actor_area_radius 0"));
        lines.push(String::from("}"));
    }
    // for i in 3..6 {
    //     lines.push(format!("create_object LUMBERJACK{i} {{"));
    //     lines.push(String::from("avoid_actor_area first_land_20"));
    //     lines.push(String::from("max_distance_to_players 10"));
    //     lines.push(String::from("set_place_for_every_player"));
    //     lines.push(String::from("actor_area_to_place_in villager_tree1"));
    //     lines.push(String::from("actor_area villager0"));
    //     lines.push(String::from("actor_area_radius 0"));
    //     lines.push(String::from("}"));
    // }
    lines
}

/// Returns a vector of strings for placing `REVEAL3_TEMP`s
/// inside of the `box0` near the TC.
pub fn vision() -> Vec<String> {
    vec![
        String::from("create_object REVEAL3_TEMP {"),
        String::from("number_of_objects 4"),
        String::from("actor_area_to_place_in box0"),
        String::from("set_place_for_every_player"),
        String::from("max_distance_to_players 2"),
        String::from("}"),
    ]
}

/// Returns a vector of all strings needed for objects generation
/// for a 9-Villager start.
pub fn objects_9_vils() -> Vec<String> {
    let mut lines = vec![
        String::from("create_object TOWN_CENTER {"),
        String::from("set_place_for_every_player"),
        String::from("max_distance_to_players 0"),
        String::from("}"),
    ];
    lines.append(&mut tc_center());
    lines.append(&mut tc_boxes());
    lines.append(&mut vision());
    lines.append(&mut vils_9_tc());
    lines.append(&mut house_gap_3());
    lines.append(&mut vils_9_straggler());
    for i in 1..10 {
        lines.push(String::from("create_object PHON {"));
        lines.push(String::from("number_of_objects 9"));
        lines.push(String::from("set_place_for_every_player"));
        lines.push(String::from("set_gaia_object_only"));
        lines.push(String::from("actor_area_to_place_in villager0"));
        lines.push(format!("actor_area villager{i}"));
        lines.push(format!("actor_area_radius {i}"));
        lines.push(String::from("}"));
    }
    lines
}

/// Returns a vector of all strings needed for objects generation
/// for a 9-Villager start on roe rage.
pub fn roe_rage_9_vils() -> Vec<String> {
    let mut lines = vec![
        String::from("create_object TOWN_CENTER {"),
        String::from("set_place_for_every_player"),
        String::from("max_distance_to_players 0"),
        String::from("}"),
    ];
    lines.append(&mut tc_center());
    lines.append(&mut tc_boxes());
    lines.append(&mut vision());
    lines.append(&mut vils_9_tc_roe_rage());
    lines.append(&mut house_gap_3());
    lines.append(&mut vils_9_straggler());
    for i in 1..10 {
        lines.push(String::from("create_object PHON {"));
        lines.push(String::from("number_of_objects 9"));
        lines.push(String::from("set_place_for_every_player"));
        lines.push(String::from("set_gaia_object_only"));
        lines.push(String::from("actor_area_to_place_in villager0"));
        lines.push(format!("actor_area villager{i}"));
        lines.push(format!("actor_area_radius {i}"));
        lines.push(String::from("}"));
    }
    lines
}

/// Makes the 9-Villager start for ZeWall by using `place_on_specific_land_id`
/// for lands `1`, `2`, `3`, and `4`.
pub fn objects_9_vils_ze_wall() -> Vec<String> {
    let standard = objects_9_vils();
    let mut object: Vec<String> = vec![];
    let mut has_set_place_for_every_player = false;
    let mut lines = vec![];
    for line in standard {
        if line == "}" {
            if has_set_place_for_every_player {
                for i in 1..=4 {
                    for object_line in &object {
                        lines.push(object_line.clone());
                    }
                    lines.push(format!("place_on_specific_land_id {i}"));
                    lines.push(String::from("}"));
                }
                has_set_place_for_every_player = false;
                object.clear();
            } else {
                lines.append(&mut object);
                lines.push(String::from("}"));
            }
        } else if line == "set_place_for_every_player" {
            has_set_place_for_every_player = true;
        } else {
            object.push(line);
        }
    }
    debug_assert!(object.is_empty(), "{object:?}");
    lines
}

/// Creates actor areas in the corners of a tiny map.
pub fn corners() -> Vec<String> {
    let threshold = 61.0;
    let mut areas = vec![];
    for x in 0i32..120i32 {
        for y in 0i32..120i32 {
            let d = (((x - 60).pow(2) + (y - 60).pow(2)) as f64).sqrt();
            if d < threshold {
                continue;
            }
            let name = match (x < 60, y < 60) {
                (true, true) => "corner-left",
                (true, false) => "corner-bottom",
                (false, true) => "corner-top",
                (false, false) => "corner-right",
            };
            areas.push(format!("create_actor_area {x} {y} {name} 0"));
        }
    }
    areas
}

/// TODO specify
fn make_init_label(
    unique_label: &str,
    degree_constant: &str,
    trig_function: &str,
    iter: usize,
    increment: usize,
) -> String {
    let prefix = format!("#const {unique_label}{iter}_{trig_function}_INITIAL_DEG");
    if iter == 0 {
        if trig_function == "SIN" {
            format!("{prefix} {degree_constant}")
        } else if trig_function == "COS" {
            format!("{prefix} (90 - {degree_constant})")
        } else {
            panic!()
        }
    } else {
        if trig_function == "SIN" {
            let prev = iter - 1;
            format!("{prefix} ({increment} + {unique_label}{prev}_{trig_function}_INITIAL_DEG)")
        } else if trig_function == "COS" {
            format!("{prefix} (90 - {unique_label}{iter}_SIN_CLAMPED_DEG)")
        } else {
            panic!()
        }
    }
}

/// Generates the calculations for the cosine and sine functions for a given nubmer of points
/// on a circle.
///
/// `unique_label`: A unique identifier prefix for constants so they do not conflict with
/// other constants.
/// `degree_constant`: The name of the rms constant containing the input degrees.
/// `num_points`: The number of points around the outside of the circle.
/// Requires `0 < num_points <= 360`, and `num_points` divides `360`.
#[allow(dead_code)]
pub fn circle_readable(unique_label: &str, degree_constant: &str, num_points: usize) -> String {
    let mut lines: Vec<String> = vec![];
    let inc = 360 / num_points;
    for i in 0..num_points {
        for f in ["SIN", "COS"] {
            let prefix = format!("{unique_label}{i}_{f}_");

            let init_label = make_init_label(unique_label, degree_constant, f, i, inc);
            lines.push(format!("{init_label}"));

            let init_is_neg = format!("{prefix}INITIAL_IS_NEG");
            lines.push(format!("#const {init_is_neg} ({prefix}INITIAL_DEG / 2 / 2 / 2 / 2 / 2 / 2 / 2 / 2 / 2 / 2 + 1 / 2 * -1 + 1)"));

            let nonneg_deg = format!("{prefix}NONNEG_DEG");
            lines.push(format!(
                "#const {nonneg_deg} (360 * {init_is_neg} + {prefix}INITIAL_DEG)"
            ));

            let init_is_large = format!("{prefix}INITIAL_IS_LARGE");
            lines.push(format!(
                "#const {init_is_large} ({nonneg_deg} - 360 / 2 / 2 / 2 / 2 / 2 / 2 / 2 / 2 / 2 + 1 / 2)"
            ));

            let clamped_deg = format!("{prefix}CLAMPED_DEG");
            lines.push(format!(
                "#const {clamped_deg} (-360 * {init_is_large} + {nonneg_deg})"
            ));

            let is_upper_half = format!("{prefix}IS_UPPER_HALF");
            lines.push(format!(
                "#const {is_upper_half} ({clamped_deg} - 180 / 2 / 2 / 2 / 2 / 2 / 2 / 2 / 2 + 1 / 2)"
            ));

            let argument = format!("{prefix}ARG");
            lines.push(format!(
                "#const {argument} (-180 * {is_upper_half} + {clamped_deg}"
            ));

            let xcomp = format!("{prefix}XCOMP");
            lines.push(format!("#const {xcomp} (180 - {argument} * {argument})"));

            let numerator = format!("{prefix}NUMERATOR");
            lines.push(format!("#const {numerator} (4 * {xcomp} * PADDING)"));

            let denominator = format!("{prefix}DENOMINATOR");
            lines.push(format!("#const {denominator} (40500 - {xcomp})"));

            let fraction = format!("{prefix}FRACTION");
            lines.push(format!("#const {fraction} ({numerator} / {denominator})"));

            let output = format!("{prefix}PADDED");
            lines.push(format!(
                "#const {output} (-2 * {is_upper_half} + 1 * {fraction})"
            ));
        }
    }
    lines.join("\n")
}

/// Generates the calculations for the cosine and sine functions for a given nubmer of points
/// on a circle.
///
/// `unique_label`: A unique identifier prefix for constants so they do not conflict with
/// other constants.
/// `degree_constant`: The name of the rms constant containing the input degrees.
/// `num_points`: The number of points around the outside of the circle.
/// Requires `0 < num_points <= 360`, and `num_points` divides `360`.
#[allow(dead_code)]
pub fn circle_optimized(unique_label: &str, degree_constant: &str, num_points: usize) -> String {
    let mut lines: Vec<String> = vec![];
    let inc = 360 / num_points;
    for i in 0..num_points {
        let prefix = format!("{unique_label}{i}");
        // Sine
        lines.push(if i == 0 {
            format!("#const {prefix}INITIAL_DEG {degree_constant}")
        } else {
            let j = i - 1;
            format!("#const {prefix}INITIAL_DEG ({inc} + {unique_label}{j}INITIAL_DEG)")
        });
        lines.push(format!("#const {prefix}NONNEG_DEG ({prefix}INITIAL_DEG / 2 / 2 / 2 / 2 / 2 / 2 / 2 / 2 / 2 / 2 + 1 / 2 * -1 + 1 * 360 + {prefix}INITIAL_DEG)"));
        lines.push(format!("#const {prefix}CLAMPED_DEG ({prefix}NONNEG_DEG - 360 / 2 / 2 / 2 / 2 / 2 / 2 / 2 / 2 / 2 + 1 / 2 * -360 + {prefix}NONNEG_DEG)"));
        lines.push(format!("#const {prefix}IS_UPPER_HALF ({prefix}CLAMPED_DEG - 180 / 2 / 2 / 2 / 2 / 2 / 2 / 2 / 2 + 1 / 2)"));
        lines.push(format!(
            "#const {prefix}ARG (-180 * {prefix}IS_UPPER_HALF + {prefix}CLAMPED_DEG)"
        ));
        lines.push(format!(
            "#const {prefix}XCOMP (180 - {prefix}ARG * {prefix}ARG)"
        ));
        lines.push(format!(
            "#const {prefix}DENOMINATOR (40500 - {prefix}XCOMP)"
        ));
        lines.push(format!(
            "#const {prefix}FRACTION (4 * {prefix}XCOMP * PADDING / {prefix}DENOMINATOR)"
        ));
        lines.push(format!(
            "#const {prefix}_SIN_PADDED (-2 * {prefix}IS_UPPER_HALF + 1 * {prefix}FRACTION)"
        ));
        // Cosine
        lines.push(format!(
            "#const {prefix}COS_INITIAL_DEG (90 - {prefix}CLAMPED_DEG)"
        ));
        lines.push(format!("#const {prefix}COS_CLAMPED_DEG ({prefix}COS_INITIAL_DEG / 2 / 2 / 2 / 2 / 2 / 2 / 2 / 2 / 2 + 1 / 2 * -1 + 1 * 360 + {prefix}COS_INITIAL_DEG)"));
        lines.push(format!("#const {prefix}COS_IS_UPPER_HALF ({prefix}COS_CLAMPED_DEG - 180 / 2 / 2 / 2 / 2 / 2 / 2 / 2 / 2 + 1 / 2)"));
        lines.push(format!(
            "#const {prefix}COS_ARG (-180 * {prefix}COS_IS_UPPER_HALF + {prefix}COS_CLAMPED_DEG)"
        ));
        lines.push(format!(
            "#const {prefix}COS_XCOMP (180 - {prefix}COS_ARG * {prefix}COS_ARG)"
        ));
        lines.push(format!(
            "#const {prefix}COS_DENOMINATOR (40500 - {prefix}COS_XCOMP)"
        ));
        lines.push(format!(
            "#const {prefix}COS_FRACTION (4 * {prefix}COS_XCOMP * PADDING / {prefix}COS_DENOMINATOR)"
        ));
        lines.push(format!(
            "#const {prefix}_COS_PADDED (-2 * {prefix}COS_IS_UPPER_HALF + 1 * {prefix}COS_FRACTION)"
        ));
    }
    lines.join("\n")
}

/// Generates the calculations for the cosine and sine functions for a given number of points
/// on a circle.
///
/// `unique_label`: A unique identifier prefix for constants so they do not conflict with
/// other constants.
/// `degree_constant`: The name of the rms constant containing the input degrees.
/// `num_points`: The number of points around the outside of the circle.
/// Requires `0 < num_points <= 360`, and `num_points` divides `360`.
pub fn circle(unique_label: &str, degree_constant: &str, num_points: usize) -> String {
    let mut lines: Vec<String> = vec![];
    let inc = 360 / num_points;
    for i in 0..num_points {
        let prefix = format!("{unique_label}{i}");
        // Sine
        lines.push(if i == 0 {
            format!("#const {prefix}A {degree_constant}")
        } else {
            let j = i - 1;
            format!("#const {prefix}A ({inc} + {unique_label}{j}A)")
        });
        lines.push(format!("#const {prefix}B ({prefix}A / 2 / 2 / 2 / 2 / 2 / 2 / 2 / 2 / 2 / 2 + 1 / 2 * -1 + 1 * 360 + {prefix}A)"));
        lines.push(format!("#const {prefix}C ({prefix}B - 360 / 2 / 2 / 2 / 2 / 2 / 2 / 2 / 2 / 2 + 1 / 2 * -360 + {prefix}B)"));
        lines.push(format!(
            "#const {prefix}D ({prefix}C - 180 / 2 / 2 / 2 / 2 / 2 / 2 / 2 / 2 + 1 / 2)"
        ));
        lines.push(format!("#const {prefix}E (-180 * {prefix}D + {prefix}C)"));
        lines.push(format!("#const {prefix}F (180 - {prefix}E * {prefix}E)"));
        lines.push(format!("#const {prefix}G (40500 - {prefix}F)"));
        lines.push(format!(
            "#const {prefix}H (4 * {prefix}F * PADDING / {prefix}G)"
        ));
        lines.push(format!(
            "#const {prefix}_SIN_PADDED (-2 * {prefix}D + 1 * {prefix}H)"
        ));
        // Cosine
        lines.push(format!("#const {prefix}I (90 - {prefix}C)"));
        lines.push(format!("#const {prefix}J ({prefix}I / 2 / 2 / 2 / 2 / 2 / 2 / 2 / 2 / 2 + 1 / 2 * -1 + 1 * 360 + {prefix}I)"));
        lines.push(format!(
            "#const {prefix}K ({prefix}J - 180 / 2 / 2 / 2 / 2 / 2 / 2 / 2 / 2 + 1 / 2)"
        ));
        lines.push(format!("#const {prefix}L (-180 * {prefix}K + {prefix}J)"));
        lines.push(format!("#const {prefix}M (180 - {prefix}L * {prefix}L)"));
        lines.push(format!("#const {prefix}O (40500 - {prefix}M)"));
        lines.push(format!(
            "#const {prefix}P (4 * {prefix}M * PADDING / {prefix}O)"
        ));
        lines.push(format!(
            "#const {prefix}_COS_PADDED (-2 * {prefix}K + 1 * {prefix}P)"
        ));
    }
    lines.join("\n")
}

/// A unique identifier for the macro's variables.
/// Requires `4 <= num_points <= 360`.
#[allow(dead_code)]
pub fn rotations_april_2025(unique_label: &str, num_points: usize) -> String {
    // TODO cases where num points == 1, 2, and 3
    // TODO hmm this works... but it feels different somehow?
    debug_assert!(4 <= num_points && num_points <= 360 && 360 % num_points == 0);
    let s = unique_label; // Give the variable a single-letter name so lines are shorter.
    let mut lines = vec![
        format!("#const {s}T rnd(0,89)"), // Generate angle in [0, 89].
        // Compute the angle's cosine and sine approximations.
        format!("#const {s}TT ({s}T * {s}T)"),
        format!("#const {s}COSD (32400 + {s}TT)"),
        format!("#const {s}COS (-4 * {s}TT + 32400 * PAD / {s}COSD)"),
        format!("#const {s}TCOMP (180 - {s}T * {s}T)"),
        format!("#const {s}SIND (40500 - {s}TCOMP)"),
        format!("#const {s}SIN (4 * {s}TCOMP * PAD / {s}SIND)"),
        // Choose a quadrant and compute the x and y coordinates of the angle.
        String::from("start_random"),
        format!("percent_chance 25 #const {s}T0 {s}T #const {s}X0 {s}COS #const {s}Y0 {s}SIN"),
        format!("percent_chance 25 #const {s}T0 ({s}T + 90) #const {s}X0 (-1 * {s}SIN) #const {s}Y0 {s}COS"),
        format!("percent_chance 25 #const {s}T0 ({s}T + 180) #const {s}X0 (-1 * {s}COS) #const {s}Y0 (-1 * {s}SIN)"),
        format!("percent_chance 25 #const {s}T0 ({s}T + 270) #const {s}X0 {s}SIN #const {s}Y0 (-1 * {s}COS)"),
        String::from("end_random"),

        // Rotation matrix entries.
        format!("#const {s}N {num_points}"),
        format!("#const {s}RT (360 / {s}N)"),
        format!("#const {s}RTT ({s}RT * {s}RT)"),
        format!("#const {s}RCOSD (32400 + {s}RTT)"),
        format!("#const {s}RCOS (-4 * {s}RTT + 32400 * PAD / {s}RCOSD)"),
        format!("#const {s}RTCOMP (180 - {s}RT * {s}RT)"),
        format!("#const {s}RSIND (40500 - {s}RTCOMP)"),
        format!("#const {s}RSIN (4 * {s}RTCOMP * PAD / {s}RSIND)"),
    ];
    // Apply the rotation for the remaining coordinates.
    // The padding is in both the points and the rotation values, so divide by it
    // to avoid double counting it.
    for i in 1..num_points {
        let j = i - 1;
        let line = vec![
            format!("#const {s}X{i}T ({s}X{j} * {s}RCOS / PAD)"),
            format!("#const {s}X{i} (-1 * {s}Y{j} * {s}RSIN / PAD + {s}X{i}T)"),
            format!("#const {s}Y{i}T ({s}X{j} * {s}RSIN / PAD)"),
            format!("#const {s}Y{i} ({s}Y{j} * {s}RCOS / PAD + {s}Y{i}T)"),
        ];
        lines.push(line.join(" "));
    }
    lines.join("\n")
}

/// A unique identifier for the macro's variables.
/// Requires `4 <= num_points <= 360`.
pub fn rotations(unique_label: &str, num_points: usize) -> String {
    // TODO cases where num points == 1, 2, and 3
    // TODO hmm this works... but it feels different somehow?
    debug_assert!(4 <= num_points && num_points <= 360 && 360 % num_points == 0);
    let s = unique_label; // Give the variable a single-letter name so lines are shorter.
    let mut lines = vec![
        format!("#const {s}T rnd(0,89)"), // Generate angle in [0, 89].
        // Compute the angle's cosine and sine approximations.
        format!("#const {s}TT ({s}T * {s}T)"),
        format!("#const {s}COSD (32400 + {s}TT)"),
        format!("#const {s}COS (-4 * {s}TT + 32400 / {s}COSD)"),
        format!("#const {s}TCOMP (180 - {s}T * {s}T)"),
        format!("#const {s}SIND (40500 - {s}TCOMP)"),
        format!("#const {s}SIN (4 * {s}TCOMP / {s}SIND)"),
        // Choose a quadrant and compute the x and y coordinates of the angle.
        String::from("start_random"),
        format!("percent_chance 25 #const {s}T0 {s}T #const {s}X0 {s}COS #const {s}Y0 {s}SIN"),
        format!("percent_chance 25 #const {s}T0 ({s}T + 90) #const {s}X0 (-1 * {s}SIN) #const {s}Y0 {s}COS"),
        format!("percent_chance 25 #const {s}T0 ({s}T + 180) #const {s}X0 (-1 * {s}COS) #const {s}Y0 (-1 * {s}SIN)"),
        format!("percent_chance 25 #const {s}T0 ({s}T + 270) #const {s}X0 {s}SIN #const {s}Y0 (-1 * {s}COS)"),
        String::from("end_random"),

        // Rotation matrix entries.
        format!("#const {s}N {num_points}"),
        format!("#const {s}RT (360 / {s}N)"),
        format!("#const {s}RTT ({s}RT * {s}RT)"),
        format!("#const {s}RCOSD (32400 + {s}RTT)"),
        format!("#const {s}RCOS (-4 * {s}RTT + 32400 / {s}RCOSD)"),
        format!("#const {s}RTCOMP (180 - {s}RT * {s}RT)"),
        format!("#const {s}RSIND (40500 - {s}RTCOMP)"),
        format!("#const {s}RSIN (4 * {s}RTCOMP / {s}RSIND)"),
    ];
    // Apply the rotation for the remaining coordinates.
    // The padding is in both the points and the rotation values, so divide by it
    // to avoid double counting it.
    for i in 1..num_points {
        let j = i - 1;
        let line = vec![
            format!("#const {s}X{i}T ({s}X{j} * {s}RCOS)"),
            format!("#const {s}X{i} (-1 * {s}Y{j} * {s}RSIN + {s}X{i}T)"),
            format!("#const {s}Y{i}T ({s}X{j} * {s}RSIN)"),
            format!("#const {s}Y{i} ({s}Y{j} * {s}RCOS + {s}Y{i}T)"),
        ];
        lines.push(line.join(" "));
    }
    lines.join("\n")
}

/// Takes the cosine and sine of a single angle in degrees, stored
/// in the constant named `degree_constant`. The degrees may be
/// any integer and are not restricted to any particular range.
/// Returns a string that performs the calculation, with every step
/// prefixed by `{degree_constant}`.
/// The two "return" constants created by the command are:
///
/// - `COS_{degree_constant}`
/// - `SIN_{degree_constant}`
///
/// `degree_constant` is the name of the variable in which the input
/// degrees are stored.
/// Requires `PAD` is defined in the map script as a power of 10.
#[allow(dead_code)]
pub fn trig_old(degree_constant: &str) -> String {
    let d = degree_constant;
    let lines = vec![
        format!("#const {d}_ROUNDED ({d} / 360 * -360 + {d})"),
        format!("#const {d}_ROUNDED_IS_NEGATIVE ({d}_ROUNDED / 2 / 2 / 2 / 2 / 2 / 2 / 2 / 2 + 1 / 2 - 1 * -1)"),
        format!("#const {d}_CLAMPED_DEG ({d}_ROUNDED_IS_NEGATIVE * 360 + {d}_ROUNDED)"),
        format!("#const {d}_IS_UPPER_HALF ({d}_CLAMPED_DEG - 180 / 2 / 2 / 2 / 2 / 2 / 2 / 2 / 2 + 1 / 2)"),
        format!("#const {d}_SIN_ARG (-180 * {d}_IS_UPPER_HALF + {d}_CLAMPED_DEG)"),
        format!("#const {d}_XCOMP (180 - {d}_SIN_ARG * {d}_SIN_ARG)"),
        format!("#const {d}_NUMERATOR (4 * {d}_XCOMP * PAD)"),
        format!("#const {d}_DENOMINATOR (40500 - {d}_XCOMP)"),
        format!("#const {d}_FRACTION ({d}_NUMERATOR / {d}_DENOMINATOR)"),
        format!("#const SIN_{d} (-2 * {d}_IS_UPPER_HALF + 1 * {d}_FRACTION)"),

        format!("#const {d}_COS_INITIAL_DEG (90 - {d}_CLAMPED_DEG)"),
        format!("#const {d}_COS_INITIAL_IS_NEG ({d}_COS_INITIAL_DEG / 2 / 2 / 2 / 2 / 2 / 2 / 2 / 2 / 2 + 1 / 2 * -1 + 1)"),
        format!("#const {d}_COS_CLAMPED_DEG (360 * {d}_COS_INITIAL_IS_NEG + {d}_COS_INITIAL_DEG)"),
        format!("#const {d}_COS_IS_UPPER_HALF ({d}_COS_CLAMPED_DEG - 180 / 2 / 2 / 2 / 2 / 2 / 2 / 2 / 2 + 1 / 2)"),
        format!("#const {d}_COS_ARG (-180 * {d}_COS_IS_UPPER_HALF + {d}_COS_CLAMPED_DEG)"),
        format!("#const {d}_COS_XCOMP (180 - {d}_COS_ARG * {d}_COS_ARG)"),
        format!("#const {d}_COS_NUMERATOR (4 * {d}_COS_XCOMP * PAD)"),
        format!("#const {d}_COS_DENOMINATOR (40500 - {d}_COS_XCOMP)"),
        format!("#const {d}_COS_FRACTION ({d}_COS_NUMERATOR / {d}_COS_DENOMINATOR)"),
        format!("#const COS_{d} (-2 * {d}_COS_IS_UPPER_HALF + 1 * {d}_COS_FRACTION)"),
    ];
    lines.join("\n")
}

/// Defines `COS_{degree_constant}` and `SIN_{degree_constant}`.
#[allow(dead_code)]
pub fn trig_april_2025(degree_constant: &str) -> String {
    let d = degree_constant;
    let lines = vec![
        format!("#const R{d} ({d} / 360 * -360 + {d})"),
        format!("#const SGN{d} (R{d} / 2 / 2 / 2 / 2 / 2 / 2 / 2 / 2)"),
        format!("#const ARG_SUPP{d} (180 * SGN{d} - R{d} * R{d})"),
        format!("#const DENOM{d} (40500 - ARG_SUPP{d})"),
        format!("#const SIN_{d} (SGN{d} * 4 * ARG_SUPP{d} * PAD / DENOM{d})"),
        format!("#const CDEG{d} (90 - {d})"),
        format!("#const CR{d} (CDEG{d} / 360 * -360 + CDEG{d})"),
        format!("#const CSGN{d} (CR{d} / 2 / 2 / 2 / 2 / 2 / 2 / 2 / 2)"),
        format!("#const CARG_SUPP{d} (180 * CSGN{d} - CR{d} * CR{d})"),
        format!("#const CDENOM{d} (40500 - CARG_SUPP{d})"),
        format!("#const COS_{d} (CSGN{d} * 4 * CARG_SUPP{d} * PAD / CDENOM{d})"),
    ];
    lines.join("\n")
}

/// Defines `COS_{degree_constant}` and `SIN_{degree_constant}`.
pub fn trig(degree_constant: &str) -> String {
    let d = degree_constant;
    let lines = vec![
        format!("#const R{d} ({d} + 360000 % 360 * -1 + 180)"),
        format!("#const S{d} (R{d} + 1 / 2 % -inf + 1 / 2 % -inf + 1 / 2 % -inf + 1 / 2 % -inf + 1 / 2 % -inf + 1 / 2 % -inf + 1 / 2 % -inf + 1 / 2 % -inf * 2 - 1)"),
        format!("#const ARG_SUPP{d} (180 * S{d} - R{d} * R{d})"),
        format!("#const DENOM{d} (40500 - ARG_SUPP{d})"),
        format!("#const SIN_{d} (S{d} * 4 * ARG_SUPP{d} / DENOM{d})"),
        format!("#const CDEG{d} (90 - {d})"),
        format!("#const CR{d} (CDEG{d} + 360000 % 360 * -1 + 180)"),
        format!("#const CS{d} (CR{d} + 1 / 2 % -inf + 1 / 2 % -inf + 1 / 2 % -inf + 1 / 2 % -inf + 1 / 2 % -inf + 1 / 2 % -inf + 1 / 2 % -inf + 1 / 2 % -inf * 2 - 1)"),
        format!("#const CARG_SUPP{d} (180 * CS{d} - CR{d} * CR{d})"),
        format!("#const CDENOM{d} (40500 - CARG_SUPP{d})"),
        format!("#const COS_{d} (CS{d} * 4 * CARG_SUPP{d} / CDENOM{d})"),
    ];
    lines.join("\n")
}

/// Outputs labels `Xi` and `Yi` for `num_points` points, 0-indexed.
/// `unique_label`: A unique identifier for the macro's variables.
/// Requires `5 <= num_points <= 360`.
#[allow(dead_code)]
pub fn trig_sums(unique_label: &str, num_points: usize) -> String {
    // TODO test
    // TODO num_points of 2, 3, and 4
    debug_assert!(5 <= num_points && num_points <= 360 && 360 % num_points == 0);
    let s = unique_label; // Give the variable a single-letter name so lines are shorter.
    let mut lines = vec![
        format!("#const {s}T rnd(0,359)"), // Generate angle in [0, 89].
        // Sin and Cos of the angle.
        format!("#const R{s} ({s} / 360 * -360 + {s})"),
        format!("#const SGN{s} (R{s} / 2 / 2 / 2 / 2 / 2 / 2 / 2 / 2)"),
        format!("#const ARG_SUPP{s} (180 * SGN{s} - R{s} * R{s})"),
        format!("#const DENOM{s} (40500 - ARG_SUPP{s})"),
        format!("#const {s}Y0 (SGN{s} * 4 * ARG_SUPP{s} * PAD / DENOM{s})"),
        format!("#const CDEG{s} (90 - {s})"),
        format!("#const CR{s} (CDEG{s} / 360 * -360 + CDEG{s})"),
        format!("#const CSGN{s} (CR{s} / 2 / 2 / 2 / 2 / 2 / 2 / 2 / 2)"),
        format!("#const CARG_SUPP{s} (180 * CSGN{s} - CR{s} * CR{s})"),
        format!("#const CDENOM{s} (40500 - CARG_SUPP{s})"),
        format!("#const {s}X0 (CSGN{s} * 4 * CARG_SUPP{s} * PAD / CDENOM{s})"),
        // Rotation angles.
        format!("#const {s}N {num_points}"),
        format!("#const {s}RT (360 / {s}N)"),
        format!("#const {s}RTT ({s}RT * {s}RT)"),
        format!("#const {s}RCOSD (32400 + {s}RTT)"),
        format!("#const {s}XR (-4 * {s}RTT + 32400 * PAD / {s}RCOSD)"),
        format!("#const {s}RTSUPP (180 - {s}RT * {s}RT)"),
        format!("#const {s}RSIND (40500 - {s}RTSUPP)"),
        format!("#const {s}YR (4 * {s}RTSUPP * PAD / {s}RSIND)"),
    ];
    // Apply the rotation for the remaining coordinates.
    // The padding is in both the points and the rotation values, so divide by it
    // to avoid double counting it.
    for i in 1..num_points {
        let j = i - 1;
        let line = vec![
            format!("#const {s}X{i} ({s}X{j} * {s}XR / {s}YR - {s}Y{j} / PAD)"),
            format!("#const {s}Y{i} ({s}X{j} * {s}YR / {s}XR + {s}Y{j} / PAD)"),
        ];
        lines.push(line.join(" "));
    }
    lines.join("\n")
}

/// Returns lines for generating a circle of actor areas on the outside of Arena.
pub fn arena_outside_circle() -> Vec<String> {
    let radius = 45.0;
    let rr = radius * radius;
    let rout = 60.0;
    let rout2 = rout * rout;
    let mid = 168.0 / 2.0 + 0.5;
    let mut lines = vec![];
    for x in 0..168 {
        let dx = x as f64 - mid;
        for y in 0..168 {
            let dy = y as f64 - mid;
            let dist2 = dx * dx + dy * dy;
            if dist2 > rr && dist2 < rout2 {
                lines.push(format!("create_actor_area {x} {y} outside_circle 0"));
            }
        }
    }
    lines
}
