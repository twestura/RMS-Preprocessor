//! Functions for creating the probability labels.
//!
//! Starting from the top-left and proceeding clockwise, label
//! the 20 positions from 0 to 19.
//! Then position k has opponent possibilities in `(k + 6) mod 20` (inclusive)
//! through `(k + 14) mod 20` (inclusive).
//!
//! Boxes are divided as follows, in map tiles. These values are converted to
//! percentages in `1..=99` in order to use as land positions.
//! Both the min and max arguments of `rnd` are inclusive.
//! rnd(18,29)
//! 30 to 89 | 30..=41, 42..=53, 54..=65, 66..= 77, 78..=89

use std::f64::consts::{PI, TAU};

use crate::utils::{self, Pointf64, Pointu32};

/// The number of player TC slots.
const NUM_SLOTS: usize = 20;

/// The count of the 4 sides of the map: top, right, bottom, left.
const NUM_SIDES: usize = 4;

/// The number of slots per map side.
const SLOTS_PER_SIZE: usize = NUM_SLOTS / NUM_SIDES;

/// Number of tiles from the edge of the map to the first allowable TC position.
/// This value is inclusive.
/// A value of `18` means the TC is allowed to spawn on tiles `18` and `102`.
const SIDE_DIST: usize = 20;

/// Number of tiles from the edge of the map to the final allowable TC position.
/// This value is exclusive.
/// A value of `30` means the TC is allowed to span on tiles `29` and `90`, but
/// not in between.
const MID_DIST: usize = 30;

/// The number of positions where P2 has a nonzero probability of spawning,
/// given that P1's slot is chosen.
const NUM_P2_POSITIONS: usize = 7;

/// The first slot away from P1's slot 0 where P2 may begin spawning.
/// That is, if P1 is in slot 0, then P2 has a nonzero probability of
/// spawning in slots 7, 8, 9, 10, 11, 12, and 13.
/// All other possibilities for P2 spawns are offset by this same distance from
/// P1's slot.
const P2_POS_OFFSET: usize = 7;

/// The number of directions used on Arena.
const ARENA_NUM_DIRECTIONS: usize = 25;

/// Radius of the circle of arena forests.
const ARENA_FOREST_RADIUS: f64 = 34.0;

/// Base size of Arena forests.
const ARENA_FOREST_BASE_SIZE: usize = 3;

/// The number of forest lands used for Arena.
const ARENA_NUM_FORESTS: usize = 128;

/// The numberf selections for land locations on Shoals.
const NUM_SHOALS_LANDS: usize = 100;

/// Circle radius for Shoals.
const SHOALS_RADIUS: f64 = 34.0;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
/// A player slot, must be in `0..NUM_SLOTS`.
struct Slot(usize);

/// Returns whether `s` satisfies the slot invariant.
fn check_slot(s: Slot) -> bool {
    s.0 < NUM_SLOTS
}

/// Returns the percent chance that a player in `p1_slot` has an opponent
/// in `p2_slot`. Requires `p1_slot` and `p2_slot` in `0..=19`.
fn opponent_probability(p1_slot: Slot, p2_slot: Slot) -> u32 {
    debug_assert!(check_slot(p1_slot) && check_slot(p2_slot));
    let min_slot = std::cmp::min(p1_slot.0, p2_slot.0);
    let max_slot = std::cmp::max(p1_slot.0, p2_slot.0);
    match (max_slot - min_slot) % NUM_SLOTS {
        7 | 13 => 10,
        8 | 9 | 10 | 11 | 12 => 16,
        _ => 0,
    }
}

/// Returns the `land_position` instruction for generating a player land at
/// the `slot`'s position.
fn slot_to_position(slot: Slot) -> String {
    let index = slot.0 % SLOTS_PER_SIZE;
    // Ensures the slot is counted clockwise.
    let index = if slot.0 >= 2 * SLOTS_PER_SIZE {
        SLOTS_PER_SIZE - 1 - index
    } else {
        index
    };
    let (low, high) = match index {
        0 => (30, 41),
        1 => (42, 53),
        2 => (54, 65),
        3 => (66, 77),
        4 => (78, 89),
        _ => panic!("Match was not exhaustive."),
    };
    // Converts from map tile coordinates to percentages of map side length.
    let (low, high) = (
        (low as f64 / 119.0 * 100.0).round(),
        (high as f64 / 119.0 * 100.0).round(),
    );
    let side = (SIDE_DIST as f64 / 119.0 * 100.0).round() as usize;
    let mid = (MID_DIST as f64 / 119.0 * 100.0).round() as usize;
    let end_side = 100 - side;
    let end_mid = 100 - mid;
    match slot.0 / SLOTS_PER_SIZE {
        0 => format!("land_position rnd({low},{high}) rnd({side},{mid})"),
        1 => format!("land_position rnd({end_mid},{end_side}) rnd({low},{high})"),
        2 => format!("land_position rnd({low},{high}) rnd({end_mid},{end_side})"),
        3 => format!("land_position rnd({side},{mid}) rnd({low},{high})"),
        _ => panic!("Match was not exhaustive."),
    }
}

/// Returns a list containing the random generation of labels. Each element of
/// the list is a line.
pub fn define_labels() -> Vec<String> {
    let mut p1_labels: Vec<String> = (0..NUM_SLOTS)
        .map(|k| format!("percent_chance 5 #define P1_SLOT_{k}"))
        .collect();
    let mut p2_labels = vec![
        "percent_chance 10 #define P2_POS_0".to_string(),
        "percent_chance 16 #define P2_POS_1".to_string(),
        "percent_chance 16 #define P2_POS_2".to_string(),
        "percent_chance 16 #define P2_POS_3".to_string(),
        "percent_chance 16 #define P2_POS_4".to_string(),
        "percent_chance 16 #define P2_POS_5".to_string(),
        "percent_chance 10 #define P2_POS_6".to_string(),
    ];

    let mut lines = vec!["start_random".to_string()];
    lines.append(&mut p1_labels);
    lines.push("end_random".to_string());
    lines.push("start_random".to_string());
    lines.append(&mut p2_labels);
    lines.push("end_random".to_string());
    lines
}

/// Returns `if` or `elseif` for the slot's conditional branch.
fn prefix_p1_slot(slot: Slot) -> String {
    debug_assert!(check_slot(slot));
    (if slot.0 == 0 { "if" } else { "elseif" }).to_string()
}

/// Returns a list of strings representing the land_position instruction for
/// P1's TC.
pub fn p1_position() -> Vec<String> {
    let labels: Vec<String> = (0..NUM_SLOTS)
        .map(|k| Slot(k))
        .map(|slot| {
            format!(
                "{} P1_SLOT_{}\n{}",
                prefix_p1_slot(slot),
                slot.0,
                slot_to_position(slot)
            )
        })
        .collect();
    let label_string = labels.join("\n");
    let output = format!("{label_string}\nendif");
    output.split('\n').map(String::from).collect()
}

/// Returns the position generation code for player 2.
pub fn p2_position() -> Vec<String> {
    let mut lines = vec![];
    for i in 0..NUM_SLOTS {
        let s = Slot(i);
        lines.push(format!("{} P1_SLOT_{i}", prefix_p1_slot(s)));
        let mut ifword = "if";

        for j in 0..NUM_P2_POSITIONS {
            let t = Slot((i + P2_POS_OFFSET + j) % NUM_SLOTS);
            let prob = opponent_probability(s, t);
            debug_assert!(
                prob != 0,
                "({i}, {j}) - Slots ({s:?}, {t:?}), has probability 0."
            );
            lines.push(format!("{ifword} P2_POS_{j}\n{}", slot_to_position(t)));
            ifword = "elseif";
        }
        lines.push("endif".to_string());
    }
    lines.push("endif".to_string());
    lines
}

/// Returns the code for Ze Snake for generating elevated rocks along the
/// outside of the map.
pub fn rock_border() -> Vec<String> {
    let mut lines = vec![];
    let base_size = 1;
    for x in 0..=100 {
        for (y, zone) in [(0, 1), (99, 2)] {
            lines.push(String::from("create_land {"));
            lines.push(format!("land_position {x} {y}"));
            lines.push(String::from("number_of_tiles rnd(15,25)"));
            lines.push(format!("base_size {base_size}"));
            lines.push(format!("base_elevation rnd(4,6)"));
            lines.push(String::from("terrain_type DLC_ROCK"));
            lines.push(format!("zone {zone}"));
            lines.push(String::from("}"));
        }
    }
    lines
}

/// Converts the point `(x, y)`, where `x` and `y` are in `[-1.0, 1.0]`, to a point
/// on the circle with the given `radius` and `center`, scaled within
/// the boundaries from `0` to `100`, inclusive for `x` and `0` through `99` for `y`.
/// Returns the resulting point.
///
/// Requires that the conversion produces a
/// result where both coordinates are nonnegative `u32` values.
///
/// Typically `x` is the result of a cosine, and `y` is the result of a sine.
fn convert(&(x, y): &Pointf64, radius: f64, center: &Pointf64) -> Pointu32 {
    use utils::round;
    (round(x * radius + center.0), round(y * radius + center.1))
}

/// Returns a list of centers of lands to use for placing forests on Arena.
/// `num_lands` is the number of lands to use in the circles, must be strictly positive.
/// `radius` is the radius of the circle, must be strictly positive.
/// `center` is the center of the circle, both coordinates must be strictly positive.
/// The `radius` and `center` must result in all points having nonnegative coordinates.
fn arena_centers(num_lands: usize, radius: f64, center: &Pointf64) -> Vec<Pointu32> {
    debug_assert!(num_lands > 0 && radius > 0.0 && center.0 > 0.0 && center.1 > 0.0);
    use utils::{cos, sin};
    let increment = TAU / num_lands as f64;
    (0..num_lands)
        .map(|i| i as f64 * increment)
        .map(|theta| (cos(theta), sin(theta)))
        .map(|p| convert(&p, radius, center))
        .collect()
}

/// Converts the point `(x, y)` to a string representing a `create_land`
/// instruction.
fn land_string((x, y): &Pointu32) -> String {
    let mut components = vec![String::from("create_land {")];
    components.push(format!("land_position {x} {y}"));
    components.push(String::from("terrain_type OUTSIDE_FOREST"));
    components.push(String::from("base_size 4"));
    components.push(String::from("number_of_tiles 128"));
    components.push(String::from("clumping_factor 40"));
    components.push(String::from("}"));
    components.join(" ")
}

/// Returns the `OUTSIDE_FOREST` lands that form a circle for a
/// 2v2 game of Arena on a Medium (4 player) map size.
pub fn arena_circles_2v2() -> Vec<String> {
    // TODO avoid lands near player land centers?
    arena_centers(64, 40.0, &(50.0, 50.0))
        .iter()
        .map(land_string)
        .collect()
}

/// Returns a random block with a 1 percent chance to define
/// a label `directioni`, where `i` is in `0..=99`.
pub fn direction_labels() -> Vec<String> {
    let mut lines = Vec::with_capacity(102);
    lines.push(String::from("start_random"));
    for i in 0..=99 {
        lines.push(format!("percent_chance 1 #define DIRECTION{i}"));
    }
    lines.push(String::from("end_random"));
    lines
}

/// Pushes strings for the `create_land` commands for a single player to `lines`,
/// where the player's main base is at angle `theta`.
/// `player` is the player whose land is appened.
/// Requires `1 <= player <= 4` and `0.0 <= theta < TAU`.
fn push_arena_player_lands(player: usize, theta: f64, lines: &mut Vec<String>) {
    debug_assert!(1 <= player && player <= 4 && 0.0 <= theta && theta < TAU);
    use utils::{cos, round, sin};
    let radius = 34.0;
    let center = 50.0;
    let (x, y) = (
        round(cos(theta) * radius + center),
        round(sin(theta) * radius + center),
    );

    // Player land.
    lines.push(String::from("create_land {"));
    lines.push(format!("land_position {x} {y}"));
    lines.push(String::from("base_size 12"));
    lines.push(String::from("number_of_tiles 0"));
    lines.push(String::from("terrain_type PLAYER_TERRAIN"));
    lines.push(format!("assign_to_player {player}"));
    lines.push(String::from("}"));

    // Avoid forest wibbles.
    let turns = [-TAU / 45.0, TAU / 45.0];
    for t in turns {
        for i in -10..=5 {
            let (x, y) = (
                round(cos(theta + t) * (radius + i as f64) + center),
                round(sin(theta + t) * (radius + i as f64) + center),
            );
            lines.push(String::from("create_land {"));
            lines.push(format!("land_position {x} {y}"));
            lines.push(String::from("base_size 3"));
            lines.push(String::from("number_of_tiles 0"));
            lines.push(String::from("terrain_type PLAYER_TERRAIN"));
            lines.push(String::from("}"));
        }
    }

    // Front spacing lands.
    // let (x1, y1) = (
    //     round(cos(theta - turn) * (radius - 6.0) + center),
    //     round(sin(theta - turn) * (radius - 6.0) + center),
    // );
    // lines.push(String::from("create_land {"));
    // lines.push(format!("land_position {x1} {y1}"));
    // lines.push(String::from("base_size 7"));
    // lines.push(String::from("number_of_tiles 0"));
    // lines.push(String::from("terrain_type PLAYER_TERRAIN"));
    // lines.push(String::from("}"));
    // let (x2, y2) = (
    //     round(cos(theta + turn) * (radius - 6.0) + center),
    //     round(sin(theta + turn) * (radius - 6.0) + center),
    // );
    // lines.push(String::from("create_land {"));
    // lines.push(format!("land_position {x2} {y2}"));
    // lines.push(String::from("base_size 7"));
    // lines.push(String::from("number_of_tiles 0"));
    // lines.push(String::from("terrain_type PLAYER_TERRAIN"));
    // lines.push(String::from("}"));
    // let (x10, y10) = (
    //     round(cos(theta - turn) * (radius - 9.0) + center),
    //     round(sin(theta - turn) * (radius - 9.0) + center),
    // );
    // lines.push(String::from("create_land {"));
    // lines.push(format!("land_position {x10} {y10}"));
    // lines.push(String::from("base_size 9"));
    // lines.push(String::from("number_of_tiles 0"));
    // lines.push(String::from("terrain_type MIDDLE_TERRAIN"));
    // lines.push(String::from("}"));
    // let (x11, y11) = (
    //     round(cos(theta + turn) * (radius - 9.0) + center),
    //     round(sin(theta + turn) * (radius - 9.0) + center),
    // );
    // lines.push(String::from("create_land {"));
    // lines.push(format!("land_position {x11} {y11}"));
    // lines.push(String::from("base_size 9"));
    // lines.push(String::from("number_of_tiles 0"));
    // lines.push(String::from("terrain_type MIDDLE_TERRAIN"));
    // lines.push(String::from("}"));

    // Back growth lands.
    // let (x3, y3) = (
    //     round(cos(theta) * (radius + 4.0) + center),
    //     round(sin(theta) * (radius + 4.0) + center),
    // );
    // lines.push(String::from("create_land {"));
    // lines.push(format!("land_position {x3} {y3}"));
    // lines.push(String::from("base_size 12"));
    // lines.push(String::from("clumping_factor 15"));
    // lines.push(String::from("number_of_tiles 512"));
    // lines.push(String::from("terrain_type PLAYER_TERRAIN"));
    // lines.push(String::from("}"));
    // let (x4, y4) = (
    //     round(cos(theta - turn) * (radius + 6.0) + center),
    //     round(sin(theta - turn) * (radius + 6.0) + center),
    // );
    // lines.push(String::from("create_land {"));
    // lines.push(format!("land_position {x4} {y4}"));
    // lines.push(String::from("base_size 6"));
    // lines.push(String::from("clumping_factor 15"));
    // lines.push(String::from("number_of_tiles 128"));
    // lines.push(String::from("terrain_type PLAYER_TERRAIN"));
    // lines.push(String::from("}"));
    // let (x5, y5) = (
    //     round(cos(theta + turn) * (radius + 6.0) + center),
    //     round(sin(theta + turn) * (radius + 6.0) + center),
    // );
    // lines.push(String::from("create_land {"));
    // lines.push(format!("land_position {x5} {y5}"));
    // lines.push(String::from("base_size 6"));
    // lines.push(String::from("clumping_factor 15"));
    // lines.push(String::from("number_of_tiles 128"));
    // lines.push(String::from("terrain_type PLAYER_TERRAIN"));
    // lines.push(String::from("}"));
}

/// Returns four `create_land` commands for combining player lands on Arena.
pub fn arena_lands() -> Vec<String> {
    let increment = TAU / 100.0;
    let quarter = TAU / 4.0;
    let mut lines = vec![];
    for i in 0..=99 {
        lines.push(format!("elseif DIRECTION{i}"));
        for p in 1..=4 {
            let theta = (i as f64 * increment + (p - 1) as f64 * quarter) % TAU;
            push_arena_player_lands(p, theta, &mut lines);
        }
    }
    lines[0] = String::from("if DIRECTION0");
    lines.push(String::from("endif"));
    lines
}

/// Returns `create_land` commands for generating the shallow terrains
/// down the middle for Ze Snake.
pub fn snake_lands() -> Vec<String> {
    use utils::{cos, sin};
    let scale = 2.0;
    let n = 100;
    let mut lines = vec![];
    for direction in 0..100 {
        lines.push(format!("elseif DIRECTION{direction}"));
        let mut points = vec![];
        for t in 0..n {
            let x = t as f64;
            points.push((x, scale * sin(x)));
            if t != 0 {
                points.push((-x, scale * sin(-x)));
            }
        }
        let theta = direction as f64 * PI / 50.0;
        let (cost, sint) = (cos(theta), sin(theta));
        let points: Vec<(i32, i32)> = points
            .iter()
            .map(|(x, y)| (x * cost - y * sint, x * sint - y * cost))
            .map(|(x, y)| (x + 50.0, y + 50.0))
            .map(|(x, y)| (x.round() as i32, y.round() as i32))
            .filter(|(x, y)| *x >= 0 && *x <= 100 && *y >= 0 && *y < 100)
            .collect();
        for (x, y) in points {
            lines.push(String::from("create_land {"));
            lines.push(String::from("terrain_type SHALLOW"));
            lines.push(String::from("number_of_tiles 0"));
            lines.push(String::from("base_size 1"));
            lines.push(format!("land_position {x} {y}"));
            lines.push(String::from("}"));
        }
    }
    lines[0] = String::from("if DIRECTION0");
    lines.push(String::from("endif"));
    // for t in 0..100 {
    //     lines.push(String::from("create_land {"));
    //     lines.push(String::from("terrain_type SHALLOW"));
    //     lines.push(String::from("number_of_tiles 0"));
    //     lines.push(String::from("base_size 2"));
    //     lines.push(format!("land_position 50 {t}"));
    //     lines.push(String::from("}"));
    // }
    lines
}

/// Returns a line to create a a land with `number_of_tiles` set to `0`
/// at position `x` `y` with the given `base_size`.
/// Requires `x <= 100` and `y <= 100`.
pub fn snake_land_line(base_size: usize, x: usize, y: usize) -> String {
    debug_assert!(x <= 100 && y <= 100);
    vec![
        String::from("create_land {"),
        String::from("terrain_type DLC_MANGROVESHALLOW"),
        String::from("number_of_tiles 0"),
        format!("base_size {base_size}"),
        format!("land_position {x} {y}"),
        String::from("}"),
    ]
    .join("\n")
}

/// Returns a vector with the rock border lands for ZeSnake.
pub fn snake_borders() -> Vec<String> {
    let mut lines = vec![];
    for x in 0..=100 {
        lines.push(snake_land_line(1, x, 0));
        lines.push(snake_land_line(2, x, 100));
    }
    for y in 0..=100 {
        lines.push(snake_land_line(1, 0, y));
        lines.push(snake_land_line(2, 100, y));
    }
    lines
}

/// Returns a vector with the middle separating lands for Four Seasons.
pub fn four_seasons_lands() -> Vec<String> {
    let mut lines = vec![];
    // Middle forest.
    for x in 0..=100 {
        lines.push(format!("create_land {{ land_position {x} 50 base_size 1 number_of_tiles 0 terrain_type DLC_ROCK }}"))
    }
    for y in 0..=100 {
        lines.push(format!("create_land {{ land_position 50 {y} base_size 1 number_of_tiles 0 terrain_type DLC_ROCK }}"))
    }
    lines
}

/// Appends the lines for the lake in the corner of the given region.
/// Requires `region` is one of `GRASS`, `SNOW`, `DIRT`, or `JUNGLE`.
fn append_lake(region: &str, lines: &mut Vec<String>) {
    let base_size = 5;
    let border = 86;
    let fuzz = 15;
    lines.push(format!("if WEST_{region}"));
    lines.push(format!("create_land {{ land_position 0 0 base_size {base_size} border_fuzziness {fuzz} right_border {border} bottom_border {border} land_percent 100 terrain_type {region}_WATER_PLACEHOLDER }}"));
    lines.push(format!("elseif NORTH_{region}"));
    lines.push(format!("create_land {{ land_position 100 0 base_size {base_size} border_fuzziness {fuzz} left_border {border} bottom_border {border} land_percent 100 terrain_type {region}_WATER_PLACEHOLDER }}"));
    lines.push(format!("elseif SOUTH_{region}"));
    lines.push(format!("create_land {{ land_position 0 99 base_size {base_size} border_fuzziness {fuzz} right_border {border} top_border {border} land_percent 100 terrain_type {region}_WATER_PLACEHOLDER }}"));
    lines.push(format!("elseif EAST_{region}"));
    lines.push(format!("create_land {{ land_position 100 99 base_size {base_size} border_fuzziness {fuzz} left_border {border} top_border {border} land_percent 100 terrain_type {region}_WATER_PLACEHOLDER }}"));
    lines.push(String::from("endif"));
}

/// Returns a vector with the if statements for creating the corner lakes.
pub fn four_seasons_lakes() -> Vec<String> {
    let mut lines = vec![];
    let regions = ["GRASS", "SNOW", "DIRT", "JUNGLE"];
    for region in regions {
        lines.push(format!("if {region}_LAKE"));
        append_lake(region, &mut lines);
        lines.push(String::from("endif"));
    }
    lines
}

/// Returns a string `land_position rnd({x - 1},{x + 1}) rnd({y - 1},{y + 1})`,
/// but clamping both numbers to `1..98`.
fn rnd_pos(x: i32, y: i32) -> String {
    fn clamp(x: i32) -> i32 {
        std::cmp::max(1, std::cmp::min(x, 98))
    }
    format! {"land_position rnd({},{}) rnd({},{})", clamp(x - 1), clamp(x + 1), clamp(y - 1), clamp(y + 1)}
}

// /// Returns a list of the positions of 8 points centered around `(x, y)`.
// /// The resulting list is sorted by distance to the center `(50, 50)`,
// /// with the closest point being at the front of the list.
// fn pond_positions(x: u32, y: u32) -> Vec<(i32, i32)> {
//     let distance = 13;
//     let mut pond_points = Vec::with_capacity(8);
//     for r in -1..=1 {
//         for c in -1..=1 {
//             if r == 0 && c == 0 {
//                 continue;
//             }
//             let d = if (r + c + 2) % 2 == 0 {
//                 distance - 2
//             } else {
//                 distance
//             };
//             let pondx = x as i32 + d * c;
//             let pondy = y as i32 + d * r;
//             pond_points.push((pondx, pondy));
//         }
//     }

//     // Sorts by distance to map center.
//     pond_points.sort_by(|(x0, y0), (x1, y1)| {
//         let dx0 = x0 - 50;
//         let dy0 = y0 - 50;
//         let dx1 = x1 - 50;
//         let dy1 = y1 - 50;
//         let d0 = dx0 * dx0 + dy0 * dy0;
//         let d1 = dx1 * dx1 + dy1 * dy1;
//         d0.partial_cmp(&d1).unwrap()
//     });
//     pond_points
// }

// /// TODO specify
// /// Requires `player`` in `1..=4` and `dir` in `0..=99`.
// fn bf_land_player(player: usize, dir: usize, lines: &mut Vec<String>) {
//     debug_assert!(1 <= player && player <= 4);
//     debug_assert!(dir <= 99);
//     use utils::{cos, round, sin};
//     let rad = 23.5;
//     let theta = dir as f64 / 100.0 * TAU;
//     let theta = ((player - 1) as f64 * (PI / 2.0) + theta) % TAU;
//     let x = round(50.0 + rad * (cos(theta) - sin(theta)));
//     let y = round(50.0 + rad * (sin(theta) + cos(theta)));
//     lines.push(String::from("create_land {"));
//     lines.push(String::from("terrain_type LAYER_A"));
//     lines.push(String::from("land_percent 10"));
//     lines.push(String::from("base_size 5"));
//     let team = if player <= 2 { 1 } else { 2 };
//     lines.push(format!("assign_to AT_TEAM {team} 0 0"));
//     lines.push(rnd_pos(x as i32, y as i32));
//     lines.push(String::from("}"));

//     // Accumulate pond positions for a given center.
//     let pond_points = pond_positions(x, y);
//     for (i, (pondx, pondy)) in pond_points.iter().enumerate() {
//         lines.push(format!("if POND_{player}_{i}"));
//         lines.push(String::from("create_land {"));
//         lines.push(String::from("terrain_type WATER"));
//         lines.push(format!("if FISH_{player}_3"));
//         lines.push(String::from("number_of_tiles 75"));
//         lines.push(format!("elseif FISH_{player}_4"));
//         lines.push(String::from("number_of_tiles 90"));
//         lines.push(String::from("endif"));
//         lines.push(String::from("clumping_factor 4"));
//         lines.push(String::from("base_size 2"));
//         lines.push(rnd_pos(*pondx as i32, *pondy as i32));
//         lines.push(format!("land_id 1{player}"));
//         lines.push(String::from("}"));
//         lines.push(String::from("endif"));
//     }
// }

// /// Returns a vector of strings containing the lines for adding player
// /// and pond lands on 2v2 medium-sized Black Forest.
// pub fn bf_lands() -> Vec<String> {
//     let mut lines = vec![];
//     for dir in 0..100 {
//         // Convert direction to a theta value
//         let start = if dir == 0 { "if" } else { "elseif" };
//         lines.push(format!("{start} DIRECTION{dir}"));
//         // Generate 4 player lands, roughly on a circle.
//         for player in 1..=4 {
//             bf_land_player(player, dir, &mut lines);
//         }
//     }
//     lines.push(String::from("endif"));
//     lines
// }

/// Returns the lines for placing forests on Arena in direction `0 <= d < ARENA_NUM_DIRECTIONS`.
pub fn arena_forest_lines_direction(d: usize) -> Vec<String> {
    debug_assert!(d <= ARENA_NUM_DIRECTIONS);
    use utils::{cos, round, sin};
    let increment = TAU / ARENA_NUM_FORESTS as f64;
    let space = 7.0;
    let mut lines = vec![];
    let quartertau = TAU / 4.0;
    let player_angles: Vec<f64> = (0..=3)
        .map(|i| (d as f64 + i as f64 * quartertau) % TAU)
        .collect();
    let tolerance = TAU / 32.0 + 0.1;
    for i in 0..ARENA_NUM_FORESTS {
        let theta = i as f64 * increment;
        // Avoid placing forests near the player lands.
        // Account for wraparound.
        if player_angles.iter().any(|phi| {
            let abs = (theta - phi).abs();
            abs < tolerance || abs > TAU - tolerance
        }) {
            continue;
        }
        let x0 = round(50.0 + ARENA_FOREST_RADIUS * cos(theta));
        let y0 = round(50.0 + ARENA_FOREST_RADIUS * sin(theta));
        lines.push(format!("create_land {{ land_position {x0} {y0} base_size {ARENA_FOREST_BASE_SIZE} number_of_tiles 60 terrain_type OUTSIDE_FOREST }}"));
        let x1 = round(50.0 + (ARENA_FOREST_RADIUS + space) * cos(theta));
        let y1 = round(50.0 + (ARENA_FOREST_RADIUS + space) * sin(theta));
        lines.push(format!("create_land {{ land_position {x1} {y1} base_size {ARENA_FOREST_BASE_SIZE} number_of_tiles 60 terrain_type OUTSIDE_FOREST }}"));
    }
    lines
}

/// Returns the if statement for placing the forest lands on arena.
pub fn arena_circle_gaps() -> Vec<String> {
    use utils::{cos, round, sin};
    let quartertau = TAU / 4.0;
    let mut lines = vec![];
    for d in 0..=24 {
        let start = if d == 0 { "if" } else { "elseif" };
        let player_angles = (0..=3).map(|i| (d as f64 + i as f64 * quartertau) % TAU);
        lines.push(format!("{start} DIRECTION{d}"));
        for (i, theta) in player_angles.enumerate() {
            let x = round(50.0 + ARENA_FOREST_RADIUS * cos(theta));
            let y = round(50.0 + ARENA_FOREST_RADIUS * sin(theta));
            let team = if i <= 1 { 1 } else { 2 };
            lines.push(format!("create_land {{ land_position {x} {y} base_size 14 land_percent 6 terrain_type PLAYER_TERRAIN assign_to AT_TEAM {team} 0 0 clumping_factor 30 top_border 3 right_border 3 bottom_border 3 left_border 3 other_zone_avoidance_distance 30 }}"))
        }
        lines.append(&mut arena_forest_lines_direction(d));
    }
    lines.push(String::from("endif"));
    lines
}

/// Returns the if statement for placing player lands inside of the game.
/// Between these statements must be the middle land.
pub fn arena_players_gaps() -> Vec<String> {
    let mut lines = vec![];
    for d in 0..=24 {
        let start = if d == 0 { "if" } else { "elseif" };
        lines.push(format!("{start} DIRECTION{d}"));
        lines.append(&mut arena_forest_lines_direction(d));
    }
    lines.push(String::from("endif"));
    lines
}

/// An instance represents the `(x, y)` coordinate of a land position.
type LandPoint = (i32, i32);

/// Returns equally spaces angles for 4 players lands with the
/// first land at angle `theta` and the remaining angles being ordered
/// counterclockwise, starting from `theta`.
pub fn land_centers(theta: f64) -> [f64; 4] {
    let quarterturn = TAU / 4.0;
    [
        theta % TAU,
        (theta + quarterturn) % TAU,
        (theta + 2.0 * quarterturn) % TAU,
        (theta + 3.0 * quarterturn) % TAU,
    ]
}

/// TODO
pub fn bf_circle_land_coordinates(num_directions: u32, radius: f64) -> Vec<[LandPoint; 4]> {
    debug_assert!(num_directions > 0 && num_directions <= 100);
    debug_assert!(100 % num_directions == 0);
    debug_assert!(radius > 0.0);
    use utils::{cos, round, sin};
    let increment = TAU / num_directions as f64;
    let mut cooridinates = vec![];
    for d in 0..num_directions {
        let theta = d as f64 * increment;
        let angles = land_centers(theta);
        let centers: Vec<(i32, i32)> = angles
            .iter()
            .map(|&theta| {
                (
                    round(50.0 + radius * cos(theta)) as i32,
                    round(50.0 + radius * sin(theta)) as i32,
                )
            })
            .collect();
        cooridinates.push([centers[0], centers[1], centers[2], centers[3]]);
    }
    cooridinates
}

/// Returns a list of 8 ponds surrounding `(x, y)`.
/// The returning ponds are not in any specified order.
pub fn bf_pond_centers((x, y): LandPoint, pond_distance: f64) -> Vec<LandPoint> {
    debug_assert!(pond_distance > 0.0);
    use utils::{cos, round, sin};
    let increment = TAU / 8.0;
    let mut points = vec![];
    for d in 0..=7 {
        let theta = d as f64 * increment;
        let pond_x = round(x as f64 + pond_distance * cos(theta));
        let pond_y = round(y as f64 + pond_distance * sin(theta));
        points.push((pond_x as i32, pond_y as i32));
    }
    vec![
        points[0], points[1], points[2], points[3], points[4], points[5], points[6], points[7],
    ]
}

/// Returns the index of the ally of the player at index `i`.
/// Requires `0 <= i <= 3`.
pub fn bf_ally(i: usize) -> usize {
    match i {
        0 => 1,
        1 => 1,
        2 => 3,
        3 => 2,
        _ => panic!(),
    }
}

/// Removes the pond closest to `(x, y)`.
/// Requires `ponds.len() == 8`.
pub fn remove_road_pond(ponds: &mut Vec<LandPoint>, (x, y): LandPoint) {
    debug_assert!(ponds.len() == 8);
    ponds.sort_by_key(|(pond_x, pond_y)| {
        let (dx, dy) = (pond_x - x, pond_y - y);
        -(dx * dx + dy * dy)
    });
    ponds.pop();
}

/// Returns the index of the enemy flank of the player at index `i`.
/// Requires `0 <= i <= 3`.
pub fn bf_flank(i: usize) -> usize {
    debug_assert!(i <= 3);
    3 - i
}

/// Sorts the ponds by distance to `(x, y)`.
/// Requires `ponds.len() == 7`.
pub fn sort_ponds(ponds: &mut Vec<LandPoint>, (x, y): LandPoint) {
    debug_assert!(ponds.len() == 7);
    ponds.sort_by_key(|(pond_x, pond_y)| {
        let (dx, dy) = (pond_x - x, pond_y - y);
        dx * dx + dy * dy
    })
}

/// Returns the land generation code for player lands and ponds.
/// Requires `1 <= num_directions <= 100` and `100` is divisible by `num_directions`.
/// Requires `radius > 0.0` and the `radius` produces points within `0.0`
/// and `100.0` for both lands and ponds.
pub fn bf_lands_2(num_directions: u32, radius: f64) -> Vec<String> {
    debug_assert!(num_directions > 0 && num_directions <= 100);
    debug_assert!(100 % num_directions == 0);
    debug_assert!(radius > 0.0);
    let mut lines = vec![];
    for (d, player_lands) in bf_circle_land_coordinates(num_directions, radius)
        .iter()
        .enumerate()
    {
        let start = if d == 0 { "if" } else { "elseif" };
        lines.push(format!("{start} DIRECTION{d}"));
        for i in 0..=3 {
            let (x, y) = player_lands[i];
            let team = if i <= 1 { 1 } else { 2 };
            let player = i + 1;
            let zone = i + 1;
            let pos = rnd_pos(x, y);
            let mut ponds: Vec<LandPoint> = bf_pond_centers((x, y), 14.0);
            let ally = bf_ally(i);
            remove_road_pond(&mut ponds, player_lands[ally]);
            let enemy = bf_flank(i);
            sort_ponds(&mut ponds, player_lands[enemy]);
            lines.push(format!(
                "create_land {{\n{pos}\nassign_to AT_TEAM {team} 0 0\nzone {zone}\nterrain_type BASE_TERRAIN\nnumber_of_tiles 3815\nbase_size 7\nother_zone_avoidance_distance 6\n}}"
            ));
            for (i, &(pond_x, pond_y)) in ponds.iter().enumerate() {
                lines.push(format!("if POND_{player}_{i}"));
                let pond_pos = rnd_pos(pond_x, pond_y);
                lines.push(format!(
                    "create_land {{\n{pond_pos}\nzone {zone}\nland_id 2{zone}\nterrain_type WATER\nbase_size 2\nnumber_of_tiles rnd(80,90)\n}}"
                ));
                lines.push(String::from("endif"));
            }
        }
    }
    lines.push(String::from("endif"));
    lines
}

/// Returns a random block for selecting among land generations on Shoals.
pub fn shoals_land_select() -> Vec<String> {
    let mut lines = vec![];
    lines.push(String::from("start_random"));
    for i in 0..NUM_SHOALS_LANDS {
        lines.push(format!("  percent_chance 1 #define L{i}"));
    }
    lines.push(String::from("end_random"));
    lines
}

/// Returns a list of 4 points to use for lands on Shoals, with the initial
/// point starting at the angle given by `angle`.
fn shoals_lands_points(angle: f64) -> Vec<(u32, u32)> {
    let mut points = vec![];
    for i in 0..4 {
        let theta = angle + i as f64 * std::f64::consts::TAU / 4.0;
        let (x, y) = (f64::cos(theta), f64::sin(theta));
        let (x, y) = (SHOALS_RADIUS * x, SHOALS_RADIUS * y);
        let (x, y) = (x + 50.0, y + 50.0);
        let (x, y) = (x.round() as u32, y.round() as u32);
        let (x, y) = (x.clamp(1, 99), y.clamp(1, 98));
        points.push((x, y));
    }
    points
}

/// Returns `rnd(x-1,x+1) rnd(y-1,y+1)`.
fn random_block(x: u32, y: u32) -> String {
    format!("rnd({},{}) rnd({},{})", x - 1, x + 1, y - 1, y + 1)
}

/// Returns the distance between the points.
fn dist(x0: u32, y0: u32, x1: u32, y1: u32) -> f64 {
    let dx = x0 as i32 - x1 as i32;
    let dy = y0 as i32 - y1 as i32;
    let dist_squared = ((dx * dx) + (dy * dy)) as f64;
    dist_squared.sqrt()
}

/// Returns the strings for creating a group of four lands for Shoals,
/// with the initial land at the angle from the x-axis given by `angle`.
fn shoals_lands_at_angle(angle: f64) -> Vec<String> {
    let points = shoals_lands_points(angle);
    let mut lines = vec![];
    let threshold = 45.0;
    let corners = vec![
        (0, 0, "LEFT"),
        (0, 100, "BOTTOM"),
        (100, 0, "TOP"),
        (100, 100, "RIGHT"),
    ];
    for (x, y, label) in corners {
        if dist(x, y, points[0].0, points[0].1) < threshold
            || dist(x, y, points[2].0, points[2].1) < threshold
        {
            lines.push(format!("#define CORNER_{label}"));
        }
    }

    lines.push(String::from("create_land {"));
    lines.push(String::from("terrain_type SPAWN_TERRAIN"));
    lines.push(String::from("number_of_tiles 984"));
    lines.push(String::from("base_size 13"));
    lines.push(String::from("clumping_factor 15"));
    lines.push(String::from("zone 20"));
    lines.push(String::from("assign_to_player 1"));
    lines.push(format!(
        "land_position {}",
        random_block(points[0].0, points[0].1)
    ));
    lines.push(String::from("}"));

    lines.push(String::from("create_land {"));
    lines.push(String::from("terrain_type BETWEEN_BORDER"));
    lines.push(String::from("number_of_tiles 984"));
    lines.push(String::from("base_size 13"));
    lines.push(String::from("clumping_factor 15"));
    lines.push(String::from("assign_to_player 1"));
    lines.push(String::from("land_id 500"));
    lines.push(String::from("zone 30"));
    lines.push(format!(
        "land_position {}",
        random_block(points[1].0, points[1].1)
    ));
    lines.push(String::from("}"));

    lines.push(String::from("create_land {"));
    lines.push(String::from("terrain_type SPAWN_TERRAIN"));
    lines.push(String::from("number_of_tiles 984"));
    lines.push(String::from("base_size 13"));
    lines.push(String::from("clumping_factor 15"));
    lines.push(String::from("assign_to_player 2"));
    lines.push(String::from("zone 20"));
    lines.push(format!(
        "land_position {}",
        random_block(points[2].0, points[2].1)
    ));
    lines.push(String::from("}"));

    lines.push(String::from("create_land {"));
    lines.push(String::from("terrain_type BETWEEN_BORDER"));
    lines.push(String::from("number_of_tiles 984"));
    lines.push(String::from("base_size 13"));
    lines.push(String::from("clumping_factor 15"));
    lines.push(String::from("assign_to_player 2"));
    lines.push(String::from("land_id 500"));
    lines.push(String::from("zone 30"));
    lines.push(format!(
        "land_position {}",
        random_block(points[3].0, points[3].1)
    ));
    lines.push(String::from("}"));
    lines
}

/// Returns a vector of strings generating the lands for shoals.
/// The "center" land of shallows is written directly in the map script
/// and is not included in this function.
pub fn shoals_lands() -> Vec<String> {
    let mut lines = vec![];
    let mut delim = "if";
    let fraction = std::f64::consts::TAU / NUM_SHOALS_LANDS as f64;
    for i in 0..NUM_SHOALS_LANDS {
        lines.push(format!("{delim} L{i}"));
        delim = "elseif";
        lines.extend(shoals_lands_at_angle(i as f64 * fraction));
    }
    lines.push(String::from("endif"));
    lines
}

/// TODO lands for stranded
pub fn stranded_2v2_lands() -> Vec<String> {
    let player_start_radius = 41.0;
    let inner_radii = [25.0, 28.0, 31.0, 34.0, 37.0];
    let forest_radius = 20.0;
    let gold_radius = forest_radius + 2.0;
    let relic_radius = 33.0;
    let hill_radius = 10.0;
    let tau = std::f64::consts::TAU;
    let quarter = tau / 4.0;
    let twelfth = tau / 12.0;
    let angles: Vec<Vec<f64>> = (0..100)
        .map(|i| {
            let theta = i as f64 / std::f64::consts::TAU;
            vec![
                theta,
                (theta + quarter) % tau,
                (theta + 2.0 * quarter) % tau,
                (theta + 3.0 * quarter) % tau,
            ]
        })
        .collect();
    let land_groups: Vec<Vec<String>> = angles
        .into_iter()
        .map(|angles| {
            let mut group: Vec<String> = Vec::with_capacity(4); // TODO higher capacity
            let relic_theta0 = (angles[0] + 3.0 * quarter / 2.0) % tau;
            let relic_theta1 = (angles[0] - quarter / 2.0) % tau;
            for (i, theta) in angles.into_iter().enumerate() {
                let id = i + 1;
                let team = if i < 2 { 1 } else { 2 };
                let (cos, sin) = (theta.cos(), theta.sin());
                let x = (player_start_radius * cos).round() as i32 + 50;
                let (x0, x1) = (x - 1, x + 1);
                let y = (player_start_radius * sin).round() as i32 + 50;
                let (y0, y1) = (y - 1, y + 1);
                let create_land = format!("L {{ land_position rnd({x0},{x1}) rnd({y0},{y1}) terrain_type DESERT assign_to AT_TEAM {team} 0 0 land_id {id} number_of_tiles 0 base_size 0 }}");
                group.push(create_land);

                // Lands for per-player middle forests.
                let x2 = (forest_radius * cos).round() as i32 + 50;
                let y2 = (forest_radius * sin).round() as i32 + 50;
                let create_forest = format!("L {{ land_position {x2} {y2} terrain_type FOREST number_of_tiles 35 base_size 1 clumping_factor 25 }}");
                group.push(create_forest);

                // Lands to prevent forests blocking the path to the middle.
                for inner_radius in inner_radii {
                    let x3 = (inner_radius * cos).round() as i32 + 50;
                    let y3 = (inner_radius * sin).round() as i32 + 50;
                    let inner_land = format!("L {{ land_position {x3} {y3} terrain_type DESERT number_of_tiles 0 base_size 0 assign_to_player 1 land_id 10 }}");
                    group.push(inner_land);
                }

                // Gold Lands.
                let x4 = (gold_radius * (theta + twelfth).cos()).round() as i32 + 50;
                let y4 = (gold_radius * (theta + twelfth).sin()).round() as i32 + 50;
                let gold_land_0 = format!("L {{ land_position {x4} {y4} terrain_type DESERT base_size 0 number_of_tiles 0 land_id 100 }}");
                group.push(gold_land_0);
                let x5 = (gold_radius * (theta + 2.0 * twelfth).cos()).round() as i32 + 50;
                let y5 = (gold_radius * (theta + 2.0 * twelfth).sin()).round() as i32 + 50;
                let gold_land_1 = format!("L {{ land_position {x5} {y5} terrain_type DESERT base_size 0 number_of_tiles 0 land_id 100 }}");
                group.push(gold_land_1);
            }
            // Relic lands.
            for theta in [relic_theta0, relic_theta1] {
                let x = (relic_radius * theta.cos()).round() as i32 + 50;
                let y = (relic_radius * theta.sin()).round() as i32 + 50;
                let (x0, x1) = (x - 1, x + 1);
                let (y0, y1) = (y - 1, y + 1);
                let relic_land = format!("L {{ land_position rnd({x0},{x1}) rnd({y0},{y1}) terrain_type DESERT number_of_tiles 0 base_size 0 land_id 20 }}");
                group.push(relic_land);
            }
            // TODO hills
            // Hills for the 3 hill case.
            group.push("if HILLS_3".to_string());
            group.push("if HILL_FLIP_0".to_string());
            let t0 = relic_theta0;
            let t1 = (t0 + tau / 3.0) % tau;
            let t2 = (t0 + 2.0 * tau / 3.0) % tau;
            for t in [t0, t1, t2] {
                let x = (hill_radius * t.cos()).round() as i32 + 50;
                let y = (hill_radius * t.sin()).round() as i32 + 50;
                let (x0, x1) = (x - 2, x + 2);
                let (y0, y1) = (y - 2, y + 2);
                let hill_land = format!("L {{ land_position rnd({x0},{x1}) rnd({y0},{y1}) base_size rnd(3,5) number_of_tiles rnd(175,200) terrain_type RAINFOREST_GRASS base_elevation 5 }}");
                group.push(hill_land);
            }
            group.push("elseif HILL_FLIP_1".to_string());
            let t0 = relic_theta1;
            let t1 = (t0 + tau / 3.0) % tau;
            let t2 = (t0 + 2.0 * tau / 3.0) % tau;
            for t in [t0, t1, t2] {
                let x = (hill_radius * t.cos()).round() as i32 + 50;
                let y = (hill_radius * t.sin()).round() as i32 + 50;
                let (x0, x1) = (x - 2, x + 2);
                let (y0, y1) = (y - 2, y + 2);
                let hill_land = format!("L {{ land_position rnd({x0},{x1}) rnd({y0},{y1}) base_size rnd(3,5) number_of_tiles rnd(175,200) terrain_type RAINFOREST_GRASS base_elevation 5 }}");
                group.push(hill_land);
            }
            group.push("endif".to_string());
            group.push("endif".to_string());
            group
        })
        .collect();
    let mut lines = Vec::with_capacity(201);
    let mut delim = "if";
    for (i, group) in land_groups.into_iter().enumerate() {
        lines.push(format!("{delim} L{i}"));
        lines.push(group.join("\n"));
        delim = "elseif";
    }
    lines.push("endif".to_string());
    lines.push("if HILLS_2".to_string());
    let mut delim2 = "if";
    for i in 0..10 {
        lines.push(format!("{delim2} HILL_ROTATION_{i}"));
        let start = i as f64 * tau / 20.0;
        for t in [start, (start + tau / 2.0) % tau] {
            let x = (hill_radius * t.cos()).round() as i32 + 50;
            let y = (hill_radius * t.sin()).round() as i32 + 50;
            let (x0, x1) = (x - 2, x + 2);
            let (y0, y1) = (y - 2, y + 2);
            let hill_land = format!("L {{ land_position rnd({x0},{x1}) rnd({y0},{y1}) base_size rnd(4,5) number_of_tiles rnd(200,225) terrain_type RAINFOREST_GRASS base_elevation 5 }}");
            lines.push(hill_land);
        }
        delim2 = "elseif";
    }
    lines.push("endif".to_string());
    lines.push("elseif HILLS_4".to_string());
    let mut delim4 = "if";
    for i in 0..10 {
        lines.push(format!("{delim4} HILL_ROTATION_{i}"));
        let start = i as f64 * tau / 20.0;
        for t in [
            start,
            (start + tau / 4.0) % tau,
            (start + tau / 2.0) % tau,
            (start + 3.0 * tau / 4.0) % tau,
        ] {
            let x = (hill_radius * t.cos()).round() as i32 + 50;
            let y = (hill_radius * t.sin()).round() as i32 + 50;
            let (x0, x1) = (x - 2, x + 2);
            let (y0, y1) = (y - 2, y + 2);
            let hill_land = format!("L {{ land_position rnd({x0},{x1}) rnd({y0},{y1}) base_size rnd(3,4) number_of_tiles rnd(150,175) terrain_type RAINFOREST_GRASS base_elevation 5 }}");
            lines.push(hill_land);
        }
        delim4 = "elseif";
    }
    lines.push("endif".to_string());
    lines.push("endif".to_string());
    lines
}

/// Generates a circle of create actor area commands at the middle of a medium sized map.
pub fn middle_circle() -> Vec<String> {
    let radius = 50.0;
    let r2 = radius * radius;
    let mut lines = vec![];
    for i in 0..168 {
        let x = 168.0 / 2.0 - i as f64;
        for j in 0..168 {
            let y = 168.0 / 2.0 - j as f64;
            if ((x * x + y * y) as f64) < r2 {
                lines.push(format!("create_actor_area {i} {j} map_middle 0"));
            }
        }
    }
    lines
}

/// Returns lands delimiting the corner forests on Valley of Kinds.
pub fn vok_trees() -> Vec<String> {
    let mut lines = vec![];
    let radius = 60.0;
    let center = 50.0;
    for d in 0..360 {
        let angle = d as f64 * std::f64::consts::PI / 180.0;
        let x = (radius * angle.cos() + center).round() as i32;
        let y = (radius * angle.sin() + center).round() as i32;
        if x >= 0 && x <= 99 && y >= 0 && y <= 99 {
            lines.push(format!("create_land {{ land_position {x} {y} terrain_type PALM_DESERT base_size 0 number_of_tiles 15 base_elevation 6 }}"))
        }
    }
    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests the opponent probability calculations.
    #[test]
    fn test_probs() {
        assert_eq!(0, opponent_probability(Slot(0), Slot(0)));
        assert_eq!(0, opponent_probability(Slot(0), Slot(5)));
        assert_eq!(0, opponent_probability(Slot(0), Slot(6)));
        assert_eq!(10, opponent_probability(Slot(0), Slot(7)));
        assert_eq!(16, opponent_probability(Slot(0), Slot(8)));
        assert_eq!(16, opponent_probability(Slot(0), Slot(9)));
        assert_eq!(16, opponent_probability(Slot(0), Slot(10)));
        assert_eq!(16, opponent_probability(Slot(0), Slot(11)));
        assert_eq!(16, opponent_probability(Slot(0), Slot(12)));
        assert_eq!(10, opponent_probability(Slot(0), Slot(13)));
        assert_eq!(0, opponent_probability(Slot(0), Slot(14)));
        assert_eq!(0, opponent_probability(Slot(0), Slot(15)));
        assert_eq!(0, opponent_probability(Slot(0), Slot(19)));

        assert_eq!(16, opponent_probability(Slot(11), Slot(0)));
    }
}
