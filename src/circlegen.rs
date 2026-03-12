//! Functions for generating TC locations around a circle.

use std::f64::consts::{PI, TAU};

/// Number of tiles of an unscaled map.
const NUM_TILES: u32 = 100;

/// Center of the map, x coordinate.
const CENTER_X: f64 = (NUM_TILES / 2) as f64;

/// Center of the map, y coordinate.
const CENTER_Y: f64 = CENTER_X; // Map is a square, so the centers are equal.

/// The map width percentage by which cliff avoidance lands are placed around
/// the centers of player lands.
const CLIFF_LAND_OFFSET: u32 = 4;

/// The coordinates of a map tile.
type Point = (u32, u32);

/// A list of points on the map.
type PointList = Vec<Point>;

/// Same as the standard `y.atan2(x)`, but the output is given in the region
/// `[0..TAU)`.
fn arctan(y: f64, x: f64) -> f64 {
    let a = y.atan2(x);
    if a >= 0.0 {
        a
    } else {
        a + TAU
    }
}

/// Convenience function for getting types to work with points
fn arctan_center(&(x, y): &Point) -> f64 {
    arctan(y as f64 - CENTER_Y, x as f64 - CENTER_X)
}

/// Sorts points counterclockwise by arctangents in the range `[0, TAU)`.
fn sort_points(points: &mut PointList) {
    points.sort_by(|a, b| arctan_center(a).partial_cmp(&arctan_center(b)).unwrap());
}

/// Sets the probabilities to add up to 100. `left` and `right` are the nonzero
/// probability endpoints, inclusive. Requires `left <= right`.
fn renormalize_probabilities(probs: &mut Vec<u32>, left: usize, right: usize) {
    debug_assert!(left <= right, "left {left} must be <= right {right}.");
    let total = probs.iter().fold(0, |a, b| a + b);
    if total < 100 {
        probs[(left + right) / 2] += 100 - total;
    } else if total > 100 {
        // Adjusts the side indices to bring the total down to 100.
        let mut i = 0;
        while *probs.get(i).unwrap() == 0 {
            i += 1;
        }
        let mut j = probs.len() - 1;
        while *probs.get(j).unwrap() == 0 {
            j -= 1;
        }
        let mut total = total;
        while total != 100 {
            debug_assert!(total > 100);
            let x = *probs.get(i).unwrap();
            let y = *probs.get(j).unwrap();
            if x >= y {
                probs[i] = x - 1;
                if x == 1 {
                    i += 1;
                }
            } else {
                probs[j] = y - 1;
                if y == 1 {
                    j -= 1;
                }
            }
            total -= 1;
        }
    }
}

/// Returns a vector of probabilities for the range `left..=right`.
/// Requires `left <= right`. Assigns roughly a Gaussian distribution to this
/// range massaged slightly to be a vector of integer percents that sums to
/// `100`.
pub fn probabilities(left: usize, right: usize) -> Vec<u32> {
    debug_assert!(left <= right, "left {left} must be <= right {right}.");
    let mut probs = Vec::with_capacity(100);
    let mu = (left + right) as f64 / 2.0;
    let sigma = left as f64 / 2.0;

    for i in 0..=99 {
        if i < left || i > right {
            probs.push(0.0);
            continue;
        }
        let to_square = (i as f64 - mu) / sigma;
        let p = 1.0 / sigma * TAU.sqrt() * f64::exp(-0.5 * to_square * to_square);
        probs.push(p);
    }
    let total = probs.iter().fold(0.0, |p, q| p + q);
    let mut probs: Vec<u32> = probs
        .iter()
        .map(|p| (p / total * 100.0).round() as u32)
        .collect();
    renormalize_probabilities(&mut probs, left, right);
    probs
}

/// Returns the tiny map points within Euclidean distance `<= 0.5` of the
/// boundary of the circle at the center of the map of size given by `radius`.
fn get_nearby_points(radius: f64) -> PointList {
    let mut points = vec![];
    for x in 0..NUM_TILES {
        for y in 0..NUM_TILES {
            let (dx, dy) = (x as f64 - CENTER_X, y as f64 - CENTER_Y);
            let dist = (dx * dx + dy * dy).sqrt();
            if (dist - radius).abs() <= 0.25 {
                points.push((x, y));
            }
        }
    }
    sort_points(&mut points);
    points
}

/// For Fortress to keep the bases near the edges of the map in a "square" while
/// still avoiding the extremes of the corners.
fn get_square_points() -> PointList {
    let mut points = vec![];
    for i in 25..=75 {
        points.push((i, 20));
        points.push((20, i));
        points.push((i, 80));
        points.push((80, i));
    }
    sort_points(&mut points);
    points
}

/// Returns the points for potential player lands on Migration.
fn get_square_points_migration() -> PointList {
    let mut points = vec![];
    for i in 10..=90 {
        points.push((i, 10));
        points.push((10, i));
        points.push((i, 90));
        points.push((90, i));
    }
    sort_points(&mut points);
    points
}

/// Returns a vector of 100 evenly spaced points from `points`.
fn select_100_points(points: &PointList) -> PointList {
    (0..=99)
        .map(|i| {
            let j = (i as f64 * points.len() as f64 / 100.0).round() as usize;
            let &p = points.get(j).unwrap();
            p
        })
        .collect()
}

// /// Prints the points with distance `1` of `radius`.
// pub fn print_points(radius: f64) {
//     let points = get_nearby_points(radius);
//     for &(x, y) in &points {
//         let (a, b) = (x as f64 - CENTER_X, y as f64 - CENTER_Y);
//         let t = arctan(b, a);
//         println!("arctan of point ({x}, {y}) is {t}");
//     }
//     println!("Number of points: {}", points.len());
// }

// /// Prints the points with distance `1` of `radius`, limited to 100 points.
// pub fn print_selection(radius: f64) {
//     let points = get_nearby_points(radius);
//     let points = select_100_points(&points);
//     for &(x, y) in &points {
//         let t = arctan_center(&(x, y));
//         println!("arctan of point ({x}, {y}) is {t}");
//     }
//     println!("Number of points: {}", points.len());
// }

/// Returns a list of strings that can be joined to create the start random
/// block for picking P1's position.
pub fn list_p1_random_selection() -> Vec<String> {
    let mut lines = vec!["start_random".to_string()];
    for i in 0..=99 {
        lines.push(format!("percent_chance 1 #define P1_POINT_{i}"));
    }
    lines.push("end_random".to_string());
    lines
}

/// Returns `(i, j)` indices in `points` where `i` is the maximum offset between
/// any two points of at least `angle` degrees and `j` is the minimum offset
/// between any two points of at most `angle` degrees.
/// The points in `points[i..=j]` are usable p2 positions for p1 at point `0`.
fn get_point_offsets(points: &PointList, angle: u32) -> (usize, usize) {
    let angle = angle as f64;
    let end_angle = 360.0 - angle;
    let mut left = None;
    let mut right = None;
    let first = points.first().unwrap();
    let theta0 = arctan_center(first);
    for (i, &p) in points.iter().enumerate() {
        let theta = (arctan_center(&p) - theta0) * 180.0 / PI;
        if theta >= angle && left.is_none() {
            left = Some(i);
        }
        if theta > end_angle && right.is_none() {
            right = Some(i - 1);
            break;
        }
    }
    (left.unwrap(), right.unwrap())
}

/// Returns a list of strings that can be joined to create the start random
/// block for picking P2's position relative to P1.
/// `radius` is the radius used for generating the circle.
/// `angle` is the minimum angle between the two points. Something between 90
/// and 135 is prob good. The angle is measured in degrees.
pub fn list_p2_random_selection(radius: f64, angle: u32) -> Vec<String> {
    assert!(90 <= angle && angle <= 135, "{angle} is not in 90..=135.");
    let points = get_nearby_points(radius);
    let points = select_100_points(&points);
    let (left, right) = get_point_offsets(&points, angle);
    let probabilities = probabilities(left, right);
    let mut lines = vec!["start_random".to_string()];
    for (i, &prob) in probabilities.iter().enumerate() {
        if prob > 0 {
            lines.push(format!("percent_chance {prob} #define P2_OFFSET_{i}"));
        }
    }
    lines.push("end_random".to_string());
    lines
}

/// Returns a list of the random blocks defining the labels for p1 and p2
/// positions.
pub fn list_random_definitions(radius: f64, angle: u32) -> Vec<String> {
    let mut lines = list_p1_random_selection();
    lines.append(&mut list_p2_random_selection(radius, angle));
    lines
}

/// Returns the if statement to place in a `create_land` command for choosing
/// p1's land position based on the chosen label. `radius` is the radius of the
/// circle used.
pub fn list_p1_positions(radius: f64) -> Vec<String> {
    let points = get_nearby_points(radius);
    let points = select_100_points(&points);
    let mut lines = vec![];
    let mut delim = "if";
    for (i, &(x, y)) in points.iter().enumerate() {
        lines.push(format!("{delim} P1_POINT_{i}"));
        lines.push(format!("land_position {x} {y}"));
        delim = "elseif";
    }
    lines.push("endif".to_string());
    lines
}

/// Returns the if statement to place in a `create_land` command for choosing
/// p2's land position based on the chosen labels. `radius` is the radius of the
/// circle used. `angle` is the degrees the angle at the center of the circle
/// makes between the TCs.
pub fn list_p2_positions(radius: f64, angle: u32) -> Vec<String> {
    let points = get_nearby_points(radius);
    let points = select_100_points(&points);
    let (left, right) = get_point_offsets(&points, angle);
    let mut delim_outer = "if";
    let mut lines = vec![];
    for i in 0..points.len() {
        lines.push(format!("{delim_outer} P1_POINT_{i}"));
        let mut delim_inner = "if";
        for j in left..=right {
            lines.push(format!("{delim_inner} P2_OFFSET_{j}"));
            let slot = (i + j) % 100;
            let (x, y) = points.get(slot).unwrap();
            lines.push(format!("land_position {x} {y}"));
            delim_inner = "elseif";
        }
        lines.push("endif".to_string());
        delim_outer = "elseif";
    }
    lines.push("endif".to_string());
    lines
}

/// Returns the initial label generation for square positioning.
/// The `_radius` is unused, but kept to match the call signature of the
/// analogous circle functions (yes, this should be refactored).
pub fn list_square_definitions(_radius: f64, angle: u32) -> Vec<String> {
    let mut lines = list_p1_random_selection();
    let points = get_square_points();
    let points = select_100_points(&points);
    let (left, right) = get_point_offsets(&points, angle);
    let probabilities = probabilities(left, right);
    lines.push("start_random".to_string());
    for (i, &prob) in probabilities.iter().enumerate() {
        if prob > 0 {
            lines.push(format!("percent_chance {prob} #define P2_OFFSET_{i}"));
        }
    }
    lines.push("end_random".to_string());
    lines
}

/// Returns the statement to place in a `create_land` command for p1's square.
/// The `_radius` is unused, but kept to match the call signature of the
/// analogous circle functions (yes, this should be refactored).
pub fn square_p1_positions(_radius: f64) -> Vec<String> {
    let points = get_square_points();
    let points = select_100_points(&points);
    let mut lines = vec![];
    let mut delim = "if";
    for (i, &(x, y)) in points.iter().enumerate() {
        lines.push(format!("{delim} P1_POINT_{i}"));
        lines.push(format!("land_position {x} {y}"));
        delim = "elseif";
    }
    lines.push("endif".to_string());
    lines
}

/// Returns the statement to place in a `create_land` command for p2's square.
/// The `_radius` is unused, but kept to match the call signature of the
/// analogous circle functions (yes, this should be refactored).
pub fn square_p2_positions(_radius: f64, angle: u32) -> Vec<String> {
    let points = get_square_points();
    let points = select_100_points(&points);
    let (left, right) = get_point_offsets(&points, angle);
    let mut delim_outer = "if";
    let mut lines = vec![];
    for i in 0..points.len() {
        lines.push(format!("{delim_outer} P1_POINT_{i}"));
        let mut delim_inner = "if";
        for j in left..=right {
            lines.push(format!("{delim_inner} P2_OFFSET_{j}"));
            let slot = (i + j) % 100;
            let (x, y) = points.get(slot).unwrap();
            lines.push(format!("land_position {x} {y}"));
            delim_inner = "elseif";
        }
        lines.push("endif".to_string());
        delim_outer = "elseif";
    }
    lines.push("endif".to_string());
    lines
}

/// Returns the initial label generation for square positioning.
/// The `_radius` is unused, but kept to match the call signature of the
/// analogous circle functions (yes, this should be refactored).
pub fn list_square_definitions_migra(_radius: f64, angle: u32) -> Vec<String> {
    let mut lines = list_p1_random_selection();
    let points = get_square_points_migration();
    let points = select_100_points(&points);
    let (left, right) = get_point_offsets(&points, angle);
    let probabilities = probabilities(left, right);
    lines.push("start_random".to_string());
    for (i, &prob) in probabilities.iter().enumerate() {
        if prob > 0 {
            lines.push(format!("percent_chance {prob} #define P2_OFFSET_{i}"));
        }
    }
    lines.push("end_random".to_string());
    lines
}

/// Returns the statement to place in a `create_land` command for p1's square.
/// The `_radius` is unused, but kept to match the call signature of the
/// analogous circle functions (yes, this should be refactored).
pub fn square_p1_positions_migra(_radius: f64) -> Vec<String> {
    let points = get_square_points_migration();
    let points = select_100_points(&points);
    let mut lines = vec![];
    let mut delim = "if";
    for (i, &(x, y)) in points.iter().enumerate() {
        lines.push(format!("{delim} P1_POINT_{i}"));
        lines.push(format!("land_position {x} {y}"));
        delim = "elseif";
    }
    lines.push("endif".to_string());
    lines
}

/// Returns the statement to place in a `create_land` command for p2's square.
/// The `_radius` is unused, but kept to match the call signature of the
/// analogous circle functions (yes, this should be refactored).
pub fn square_p2_positions_migra(_radius: f64, angle: u32) -> Vec<String> {
    let points = get_square_points_migration();
    let points = select_100_points(&points);
    let (left, right) = get_point_offsets(&points, angle);
    let mut delim_outer = "if";
    let mut lines = vec![];
    for i in 0..points.len() {
        lines.push(format!("{delim_outer} P1_POINT_{i}"));
        let mut delim_inner = "if";
        for j in left..=right {
            lines.push(format!("{delim_inner} P2_OFFSET_{j}"));
            let slot = (i + j) % 100;
            let (x, y) = points.get(slot).unwrap();
            lines.push(format!("land_position {x} {y}"));
            delim_inner = "elseif";
        }
        lines.push("endif".to_string());
        delim_outer = "elseif";
    }
    lines.push("endif".to_string());
    lines
}

/// Pushes 4 `create_land` commands to `lines` surrounding point `(x, y)` for
/// the indicated player. `player` must be `1` or `2` to indicate the player.
fn push_cliff_lands(lines: &mut Vec<String>, player: u32, x: u32, y: u32) {
    let player_number = match player {
        1 => "1",
        2 => "2",
        _ => panic!("{}", format!("{player} must be 1 or 2")),
    };
    let centers = [
        (x - CLIFF_LAND_OFFSET, y - CLIFF_LAND_OFFSET),
        (x - CLIFF_LAND_OFFSET, y + CLIFF_LAND_OFFSET),
        (x + CLIFF_LAND_OFFSET, y - CLIFF_LAND_OFFSET),
        (x + CLIFF_LAND_OFFSET, y + CLIFF_LAND_OFFSET),
    ];
    for (x0, y0) in centers {
        lines.push("create_land {".to_string());
        lines.push(format!("terrain_type PLAYER_PLACEHOLDER_{player_number}"));
        lines.push("base_size 0".to_string());
        lines.push("number_of_tiles 0".to_string());
        lines.push(format!("zone {player_number}"));
        lines.push(format!("land position {x0} {y0}"));
        lines.push("}".to_string());
    }
}

/// Creates a list of lands inside of player bases to force cliffs away from
/// player walls.
///
/// Place this macro in the map script after the `create_land` commands that
/// create the player lands.
///
/// Cliffs avoid the centers of player lands by 22 tiles. This command generates
/// the code for creating lands at positions 3 percent away from the center of
/// each player land. These "dummy lands" for the cliffs to stay further away
/// from the Fortress walls.
pub fn square_avoid_cliffs() -> Vec<String> {
    let mut lines = vec![];
    // Player 1 lands.
    let points = get_square_points();
    let points = select_100_points(&points);
    let mut delim = "if";
    for (i, &(x, y)) in points.iter().enumerate() {
        lines.push(format!("{delim} P1_POINT_{i}"));
        push_cliff_lands(&mut lines, 1, x, y);
        delim = "elseif";
    }
    lines.push("endif".to_string());

    // Player 2 lands.
    let angle = 130; // hard code the value used in Fortress, again this is ugly
    let (left, right) = get_point_offsets(&points, angle);
    let mut delim_outer = "if";
    for i in 0..points.len() {
        lines.push(format!("{delim_outer} P1_POINT_{i}"));
        let mut delim_inner = "if";
        for j in left..=right {
            lines.push(format!("{delim_inner} P2_OFFSET_{j}"));
            let slot = (i + j) % 100;
            let &(x, y) = points.get(slot).unwrap();
            push_cliff_lands(&mut lines, 2, x, y);
            delim_inner = "elseif";
        }
        lines.push("endif".to_string());
        delim_outer = "elseif";
    }
    lines.push("endif".to_string());

    lines
}

// create_land {
//     terrain_type PLAYER_PLACEHOLDER_1
//     base_size 1
//     number_of_tiles 0
//     zone 1
//     land_position 20 60
// }

/// TODO
#[allow(dead_code)]
pub fn init_angle_trig_april_2025(t: &str, n: usize, a: f64) -> String {
    // Compute the padded sine and cosine of `t`,
    // saving them in constants PCOS_{t} and TSIN_{t}.
    debug_assert!(n > 0);
    let r = 360.0 / n as f64;
    let xt = format!("PCOS_{t}");
    let yt = format!("PSIN_{t}");
    let mut lines = vec![
        format!("#const R_{t} ({t} / 360 * -360 + {t})"),
        format!("#const SGN_{t} (R_{t} / 2 / 2 / 2 / 2 / 2 / 2 / 2 / 2)"),
        format!("#const ARG_SUPP_{t} (180 * SGN_{t} - R_{t} * R_{t}"),
        format!("#const DENOM_{t} (40500 - ARG_SUPP_{t}"),
        format!("#const {yt} (SGN_{t} * 4 * ARG_SUPP_{t} * PAD / DENOM_{t}"),
        format!("#const CODEG_{t} (90 - R_{t})"),
        format!("#const COR_{t} (CODEG_{t} / 360 * -360 + CODEG_{t})"),
        format!("#const COSGN_{t} (COR_{t} / 2 / 2 / 2 / 2 / 2 / 2 / 2 / 2)"),
        format!("#const COARG_SUPP_{t} (180 * COSGN_{t} - COR_{t} * COR_{t}"),
        format!("#const CODENOM_{t} (40500 - COARG_SUPP_{t}"),
        format!("#const {xt} (COSGN_{t} * 4 * COARG_SUPP_{t} * PAD / CODENOM_{t}"),
    ];
    let epsilon = 0.01;
    for k in 0..n {
        let kr = k as f64 * r;
        let (y, x) = kr.to_radians().sin_cos();
        if y.abs() < epsilon {
            let x = (100000.0 * a * x).round() / 100000.0; // Round to 5 decimal places.
            lines.push(format!("#const PX_{t}{k} ({xt} * {x})"));
            lines.push(format!("#const PY_{t}{k} ({yt} * {x})"));
        } else if x.abs() < epsilon {
            let y = (100000.0 * y).round() / 100000.0;
            lines.push(format!("#const PX_{t}{k} (-1 * {yt} * {y})"));
            lines.push(format!("#const PY_{t}{k} ({xt} * {y})"));
        } else {
            let cot = (100000.0 * a * x / y).round() / 100000.0;
            let y = (100000.0 * y).round() / 100000.0;
            lines.push(format!("#const PX_{t}{k} ({xt} * {cot} - {yt} * {y})"));
            lines.push(format!("#const PY_{t}{k} ({yt} * {cot} + {xt} * {y})"));
        }
    }
    lines.join("\n")
}

/// TODO
pub fn init_angle_trig(t: &str, n: usize, a: f64) -> String {
    // Compute the padded sine and cosine of `t`,
    // saving them in constants PCOS_{t} and TSIN_{t}.
    debug_assert!(n > 0);
    let r = 360.0 / n as f64;
    let xt = format!("COS_{t}");
    let yt = format!("SIN_{t}");
    let mut lines = vec![
        format!("#const R_{t} ({t} + 360000 % 360 * -1 + 180)"),
        format!("#const SGN_{t} (R_{t} + 1 / 2 % -inf + 1 / 2 % -inf + 1 / 2 % -inf + 1 / 2 % -inf + 1 / 2 % -inf + 1 / 2 % -inf + 1 / 2 % -inf + 1 / 2 % -inf * 2 - 1)"),
        format!("#const ARG_SUPP_{t} (180 * SGN_{t} - R_{t} * R_{t}"),
        format!("#const DENOM_{t} (40500 - ARG_SUPP_{t}"),
        format!("#const {yt} (SGN_{t} * 4 * ARG_SUPP_{t} / DENOM_{t}"),
        format!("#const CODEG_{t} (90 - R_{t})"),
        format!("#const COR_{t} (CODEG_{t} + 360000 % 360 * -1 + 180)"),
        format!("#const COSGN_{t} (COR_{t} + 1 / 2 % -inf + 1 / 2 % -inf + 1 / 2 % -inf + 1 / 2 % -inf + 1 / 2 % -inf + 1 / 2 % -inf + 1 / 2 % -inf + 1 / 2 % -inf * 2 - 1)"),
        format!("#const COARG_SUPP_{t} (180 * COSGN_{t} - COR_{t} * COR_{t}"),
        format!("#const CODENOM_{t} (40500 - COARG_SUPP_{t}"),
        format!("#const {xt} (COSGN_{t} * 4 * COARG_SUPP_{t} / CODENOM_{t}"),
    ];
    let epsilon = 0.01;
    for k in 0..n {
        let kr = k as f64 * r;
        let (y, x) = kr.to_radians().sin_cos();
        if y.abs() < epsilon {
            let x = (100000.0 * a * x).round() / 100000.0; // Round to 5 decimal places.
            lines.push(format!("#const X_{t}{k} ({xt} * {x})"));
            lines.push(format!("#const Y_{t}{k} ({yt} * {x})"));
        } else if x.abs() < epsilon {
            let y = (100000.0 * y).round() / 100000.0;
            lines.push(format!("#const X_{t}{k} (-1 * {yt} * {y})"));
            lines.push(format!("#const Y_{t}{k} ({xt} * {y})"));
        } else {
            let cot = (100000.0 * a * x / y).round() / 100000.0;
            let y = (100000.0 * y).round() / 100000.0;
            lines.push(format!("#const X_{t}{k} ({xt} * {cot} - {yt} * {y})"));
            lines.push(format!("#const Y_{t}{k} ({yt} * {cot} + {xt} * {y})"));
        }
    }
    lines.join("\n")
}

// #const R (DEGREES / 360 * -360 + DEGREES)
// #const SGN (R / 2 / 2 / 2 / 2 / 2 / 2 / 2 / 2)
// #const ARG_SUPP (180 * SGN - R * R)
// #const DENOMINATOR (40500 - ARG_SUPP)
// #const SIN (SGN * 4 * ARG_SUPP * PADDING / DENOMINATOR)
