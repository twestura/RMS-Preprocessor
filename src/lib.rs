//! Library of helper functions for the map preprocessor.
//! Originally made for T90 Titans League Season 2 and expanded for other events.

use std::{
    collections::{HashMap, VecDeque},
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
};

use actorgen::{circle, rotations, trig};

mod actorgen;
mod circlegen;
mod landgen;
mod probgen;
mod utils;

/// Relative path to the directory of files that can be included.
// const INCLUDE_MAPS: &str = "include_maps";

// Process every `#include_drs` command from `lines`.
// If the file is found, replaces the `include_drs` with the lines of the
// included file. Files must be in the `INCLUDE_MAPS` directory.
// fn include_files(lines: Vec<String>) -> std::io::Result<Vec<String>> {
//     let mut output = vec![];
//     for line in lines {
//         if !line.contains("#include_drs") {
//             output.push(line);
//             continue;
//         }
//         let i = line.find(' ').unwrap();
//         let name = &line[i + 1..];
//         let mut include_file = File::open(format!("{INCLUDE_MAPS}/{name}"));
//         // Skips the file if it cannot be found.
//         if include_file.is_err() {
//             include_file = File::open(format!("../{INCLUDE_MAPS}/{name}"));
//             if include_file.is_err() {
//                 continue;
//             }
//         }
//         let include_file = include_file.unwrap();
//         let include_reader = BufReader::new(include_file);
//         output.extend(include_reader.lines().map(|l| l.unwrap()));
//     }
//     Ok(output)
// }

/// Takes ownership of `lines`` and returns two vectors splitting `lines` into two components.
/// The first is the text contained within the lines `#HEADER_START` and `#HEADER_END`,
/// if present. The second is all lines after `#HEADER_END`.
/// If the header comments are not present, the first vector is empty and the
/// second vector is `lines`, unmodified.
fn collect_header_comment(lines: Vec<String>) -> (Vec<String>, Vec<String>) {
    if lines.is_empty() || lines[0].trim().to_uppercase() != "#HEADER_START" {
        return (vec![], lines);
    }
    let n = lines.len();
    let mut i = 1;
    let mut header = vec![];
    while i != n {
        let line = &lines[i];
        if line.trim().to_uppercase() == "#HEADER_END" {
            let mut end = vec![];
            for j in i + 1..n {
                end.push(lines[j].clone());
            }
            return (header, end);
        }
        header.push(line.clone());
        i += 1;
    }
    panic!("Header comment never ends.")
}

/// Strips comments from line `s`, where `i` is the index of the first
/// occurrence of the open comment delimiter `"/*"` in `s`.
/// Requires that no close comment delimiter `"*/"` occurs prior to the first
/// open comment delimiter.
///
/// `depth` is the nested comment depth at the start of `s`.
///
/// Returns `(t, d)`, where `t` is the line with comments stripped and `d` is
/// the comment depth after stripping comments.
fn strip_start_comment(s: &str, depth: u32, i: usize) -> (String, u32) {
    let front = if depth > 0 { "" } else { &s[..i] };
    let tail = &s[i + 2..];
    let (rec_str, rec_depth) = strip_line_comments(tail, depth + 1);
    (format!("{}{}", front, rec_str), rec_depth)
}

/// Strips comments from line `s`, where `i` is the index of the first
/// occurrence of the close comment delimiter `"*/"` in `s`.
/// Requires that no open comment delimiter `"/*"` occurs prior to the first
/// close comment delimiter.
///
/// `depth` is the nested comment depth at the start of `s`.
///
/// /// Returns `(t, d)`, where `t` is the line with comments stripped and `d`
/// is the comment depth after stripping comments.
fn strip_end_comment(s: &str, depth: u32, j: usize) -> (String, u32) {
    let tail = &s[j + 2..];
    if depth > 0 {
        strip_line_comments(tail, depth - 1)
    } else {
        let front = &s[..j + 2];
        let (rec_str, rec_depth) = strip_line_comments(tail, 0);
        (format!("{}{}", front, rec_str), rec_depth)
    }
}

/// Strips comments from line `s`, with `depth` being the depth of nested
/// comments at the start of parsing the line.
///
/// Requires that `s` does not contains a newline character.
///
/// If comments are unbalanced, that is, if the `depth` is `0` and the end
/// comment `*/` substring is encountered, then that substring is included in
/// the resulting line, and the depth remains `0` for further parsing after that
/// substring.
///
/// Returns `(t, d)`, where `t` is the line with comments stripped and `d` is
/// the comment depth after stripping comments.
fn strip_line_comments(s: &str, depth: u32) -> (String, u32) {
    debug_assert!(!s.contains("\n"));
    match (s.find("/*"), s.find("*/")) {
        (Some(i), Some(j)) => {
            debug_assert_ne!(i, j);
            if i < j {
                strip_start_comment(s, depth, i)
            } else {
                strip_end_comment(s, depth, j)
            }
        }
        (Some(i), None) => strip_start_comment(s, depth, i),
        (None, Some(j)) => strip_end_comment(s, depth, j),
        (None, None) => ((if depth > 0 { "" } else { s }).to_string(), depth),
    }
}

/// Takes ownership of `lines` and returns an equivalent vector with all
/// comments removed.
fn strip_comments(lines: Vec<String>) -> Vec<String> {
    let mut comment_depth = 0; // The depth of nested comments.
    let mut output = Vec::with_capacity(lines.len());
    for line in lines {
        let (stripped, d) = strip_line_comments(&line, comment_depth);
        output.push(stripped);
        comment_depth = d;
    }
    output
}

/// Returns a string with the same contents as `s`, but with leading and
/// trailing whitespace removed and with each substring of inner whitespace
/// replaced with a single space. If `s` is all whitespace, the empty string
/// is returned.
fn condense_line_whitespace(s: &str) -> String {
    s.trim()
        .split_whitespace()
        .enumerate()
        .map(|(i, w)| format!("{}{}", if i > 0 { " " } else { "" }, w))
        .collect()
}

/// Removes excess whitespace from each string of `lines`, and removes all blank
/// lines. Condenses whitespace withing each line.
fn condense_whitespace(lines: Vec<String>) -> Vec<String> {
    lines
        .iter()
        .map(|line| condense_line_whitespace(&line))
        .filter(|line| !line.is_empty())
        .collect()
}

/// Parses the unique label, degree constant name, and number of points
/// from a circle trigonometry macro.
fn parse_circle(line: &str) -> (&str, &str, usize) {
    let left = line.find('(').unwrap() + 1;
    let right = line.find(')').unwrap();
    let inner = &line[left..right];
    let components: Vec<&str> = inner.split(",").map(|s| s.trim()).collect();
    (
        components[0],
        components[1],
        components[2].parse::<usize>().unwrap(),
    )
}

/// Parses the unique label and number of points from a rotation macro.
fn parse_rotations(line: &str) -> (&str, usize) {
    let left = line.find('(').unwrap() + 1;
    let right = line.find(')').unwrap();
    let inner = &line[left..right];
    let components: Vec<&str> = inner.split(",").map(|s| s.trim()).collect();
    (components[0], components[1].parse::<usize>().unwrap())
}

/// Parses the degree constant from a trig macro.
fn parse_trig(line: &str) -> &str {
    let left = line.find('(').unwrap() + 1;
    let right = line.find(')').unwrap();
    &line[left..right].trim()
}

/// Parses the initial degree constant and the number of points from a macro.
fn parse_init_angle_trig(line: &str) -> (&str, usize, f64) {
    let left = line.find('(').unwrap() + 1;
    let right = line.find(')').unwrap();
    let inner = &line[left..right];
    let components: Vec<&str> = inner.split(",").map(|s| s.trim()).collect();
    (
        components[0],
        components[1].parse::<usize>().unwrap(),
        components[2].parse::<f64>().unwrap(),
    )
}

/// Returns a vector of lines resulting from expanding macros in `line`.
/// If `line` has no macros, then the vector contians a single element
/// equivalent to the input `line`.
fn expand_line(line: &str) -> Vec<String> {
    let upper = &line.to_uppercase()[..];
    if let Some(i) = line.find('(') {
        if line.starts_with("#CIRCLE") && !line.starts_with("#CIRCLE_") {
            let (unique_label, degree_constant, num_points) = parse_circle(line);
            return vec![circle(unique_label, degree_constant, num_points)];
        }
        if line.starts_with("#ROTATIONS") {
            let (unique_label, num_points) = parse_rotations(line);
            return vec![rotations(unique_label, num_points)];
        }
        if line.starts_with("#TRIG") {
            let degree_constant = parse_trig(line);
            return vec![trig(degree_constant)];
        }
        if line.starts_with("#INIT_ANGLE_TRIG") {
            let (t, n, a) = parse_init_angle_trig(line);
            return vec![circlegen::init_angle_trig(t, n, a)];
        }
        // Ignores for loops.
        if line.starts_with("#FOR") || line.starts_with("#END_FOR") {
            return vec![line.to_string()];
        }
        let j = line.find(',');
        if j.is_none() {
            return vec![line.to_string()];
        }
        let j = j.unwrap();
        let k = line.find(')');
        if k.is_none() {
            return vec![line.to_string()];
        }
        let k = k.unwrap();
        let radius = (&line[i + 1..j]).parse::<f64>().unwrap();
        let angle = (&line[j + 1..k]).parse::<u32>().unwrap();
        match &upper[..i] {
            "#CIRCLE_LABELS" => circlegen::list_random_definitions(radius, angle),
            "#CIRCLE_POSITION_P1" => circlegen::list_p1_positions(radius),
            "#CIRCLE_POSITION_P2" => circlegen::list_p2_positions(radius, angle),
            "#SQUARE_LABELS" => circlegen::list_square_definitions(radius, angle),
            "#SQUARE_POSITION_P1" => circlegen::square_p1_positions(radius),
            "#SQUARE_POSITION_P2" => circlegen::square_p2_positions(radius, angle),
            "#MIGRA_LABELS" => circlegen::list_square_definitions_migra(radius, angle),
            "#MIGRA_POSITION_P1" => circlegen::square_p1_positions_migra(radius),
            "#MIGRA_POSITION_P2" => circlegen::square_p2_positions_migra(radius, angle),
            _ => vec![line.to_string()],
        }
    } else {
        match upper {
            "#POSITION_LABELS" => landgen::define_labels(),
            "#POSITION_P1" => landgen::p1_position(),
            "#POSITION_P2" => landgen::p2_position(),
            "#SQUARE_AVOID_CLIFFS" => circlegen::square_avoid_cliffs(),
            "#ROCKGEN" => landgen::rock_border(),
            "#MKCONSTS" => actorgen::make_constants(),
            "#MK9CONSTS" => actorgen::make_constants_9(),
            "#ROERAGEMK9CONSTS" => actorgen::make_constants_9_roe_rage(),
            "#SETPHATTR" => actorgen::set_placeholder_attributes(),
            "#SET9ATTR" => actorgen::set_placeholder_attributes_9(),
            "#SETPHATTR4SEASONS" => actorgen::set_placeholder_attributes_four_seasons(),
            "#TCCENTER" => actorgen::tc_center(),
            "#TCBOXES" => actorgen::tc_boxes(),
            "#TCCENTER2" => actorgen::tc_center2(),
            "#TCBOXES2" => actorgen::tc_boxes2(),
            "#TCMULTIBOXES" => actorgen::tc_multiboxes(),
            "#VISION" => actorgen::vision(),
            "#TC9VILS" => actorgen::vils_9_tc(),
            "#TC9VILSZEWALL" => actorgen::vils_9_tc_ze_wall(),
            "#TCMULTI9VILS" => actorgen::multi_vils_9_tc(),
            "#HOUSEGAP3" => actorgen::house_gap_3(),
            "#MULTIHOUSES" => actorgen::multi_houses(),
            "#HUTGAP3" => actorgen::hut_gap_3(),
            "#STRAGGLER9VILS" => actorgen::vils_9_straggler(),
            "#STRAGGLER9VILSSOCOTRA" => actorgen::vils_9_straggler_socotra(),
            "#MULTISTRAGGLER9VILS" => actorgen::multi_stragglers(),
            "#OBJECTS9VILS" => actorgen::objects_9_vils(),
            "#ROERAGE9VILS" => actorgen::roe_rage_9_vils(),
            "#OBJECTS9VILSZEWALL" => actorgen::objects_9_vils_ze_wall(),
            "#ARENACIRCLES2V2" => landgen::arena_circles_2v2(),
            "#DIRLABELS" => landgen::direction_labels(),
            "#SNAKELANDS" => landgen::snake_lands(),
            "#SNAKEBORDERS" => landgen::snake_borders(),
            "#ARENALANDS" => landgen::arena_lands(),
            "#FOURSEASONSLANDS" => landgen::four_seasons_lands(),
            "#FOURSEASONSLAKES" => landgen::four_seasons_lakes(),
            "#ARENA_CIRCLE_GAPS" => landgen::arena_circle_gaps(),
            "#ARENA_PLAYERS_GAPS" => landgen::arena_players_gaps(),
            "#BFLANDS" => landgen::bf_lands_2(100, 36.0),
            "#MKCONSTSSMALL" => actorgen::make_small_constants(),
            "#SETPHATTRSMALL" => actorgen::set_placeholder_attributes_small(),
            "#SHOALSLANDSELECT" => landgen::shoals_land_select(),
            "#SHOALSLANDS" => landgen::shoals_lands(),
            "#CORNERS" => actorgen::corners(),
            "#LAND_PROBS" => probgen::generate_probs_100("L"),
            "#STRANDED_2V2_LANDS" => landgen::stranded_2v2_lands(),
            "#MIDDLE_CIRCLE" => landgen::middle_circle(),
            "#VOK_TREES" => landgen::vok_trees(),
            "#ARENAOUTSIDECIRCLE" => actorgen::arena_outside_circle(),
            _ => vec![line.to_string()],
        }
    }
}

/// Inserts preprocessor commands into `lines`.
/// Commands include `#POSITION_LABELS`, `#POSITION_P1`, and `#POSITION_P2`.
fn insert_macros(lines: Vec<String>) -> Vec<String> {
    lines.iter().flat_map(|line| expand_line(&line)).collect()
}

/// Returns a copy of `lines` with all for loop macros expanded.
fn insert_for_loops(lines: Vec<String>) -> Vec<String> {
    // TODO this code is ugly and should be refactored.
    let mut depth = 0;
    let mut output = vec![];
    let mut for_lines = vec![];
    let mut replace_token = None;
    let mut replace_start = None;
    let mut replace_end = None;
    for line in lines.iter() {
        if depth == 0 {
            if line.starts_with("#FOR") {
                depth += 1;
                let first_par_index = line.find('(').unwrap();
                let first_comma_index = line.find(',').unwrap();
                let last_comma_index = line.rfind(',').unwrap();
                let last_par_index = line.find(')').unwrap();
                let token = &line[(first_par_index + 1)..first_comma_index];
                replace_token = Some(token.trim());
                let middle_number = &line[(first_comma_index + 1)..last_comma_index].trim();
                replace_start = Some(middle_number.parse::<usize>().unwrap());
                let end_number = &line[(last_comma_index + 1)..last_par_index].trim();
                replace_end = Some(end_number.parse::<usize>().unwrap());
                debug_assert!(for_lines.is_empty());
            } else {
                assert!(!line.starts_with("#END_FOR"));
                output.push(line.to_string());
            }
        } else {
            if line.starts_with("#FOR") {
                depth += 1;
                for_lines.push(line.to_string());
            } else if line.starts_with("#END_FOR") {
                depth -= 1;
                if depth == 0 {
                    // Replaces the tokens for each iteration.
                    let template = for_lines.join("\n");
                    for_lines.clear();

                    let mut components = vec![];
                    for i in replace_start.unwrap()..replace_end.unwrap() {
                        components.push(template.replace(replace_token.unwrap(), &format!("{i}")));
                    }

                    // Recurse on each replaced component.
                    let recursed_components = components
                        .iter()
                        .map(|s| {
                            let separated_references = s.split("\n");
                            let separated_strings = separated_references
                                .map(|t| t.to_string())
                                .collect::<Vec<String>>();
                            insert_for_loops(separated_strings)
                        })
                        .collect::<Vec<Vec<String>>>();

                    // Insert the components into the output.
                    for mut component in recursed_components {
                        output.append(&mut component);
                    }

                    // After expansion no more for loop active, so clear components.
                    replace_token = None;
                    replace_start = None;
                    replace_end = None;
                } else {
                    for_lines.push(line.to_string());
                }
            } else {
                for_lines.push(line.to_string());
            }
        }
    }
    assert_eq!(depth, 0);
    output
}

#[derive(Debug, PartialEq, Eq)]
/// Represents a list of lines to be repeated.
struct RepeatLines {
    /// The number of times to repeat the lines.
    count: usize,
    /// The lines to repeat. Lines must have comments removed and have minimal
    /// whitespace.
    lines: Vec<String>,
}

impl RepeatLines {
    /// Returns a new `RepeatLines` struct that is initially empty without any
    /// lines.
    fn new(count: usize) -> Self {
        return RepeatLines {
            count,
            lines: vec![],
        };
    }

    /// Adds `line` to the end of this list, taking ownership of it.
    fn push_line(&mut self, line: String) {
        self.lines.push(line.to_string());
    }

    /// Returns the contents of this list, repeated and joined by new lines.
    /// The text does not contain a leading newline.
    fn get_text(&self) -> String {
        if self.count == 0 {
            return String::new();
        }
        let joined = self.lines.join("\n");
        let repeated = format!("\n{joined}").repeat(self.count - 1);
        format!("{joined}{repeated}")
    }
}

/// Returns the repeat count, parsed from `repeat_line`.
/// Requires that `repeat_line` has exactly one set of parentheses enclosing a
/// `usize` literal, such as `"REPEAT(5)"`.
fn parse_repeat_count(repeat_line: &str) -> usize {
    let i = repeat_line.find('(').unwrap();
    let j = repeat_line.rfind(')').unwrap();
    let s = &repeat_line[i + 1..j];
    s.parse::<usize>().unwrap()
}

/// Returns a copy of `lines` with all repeat blocks included the indicated
/// number of times.
fn repeat_lines(lines: Vec<String>) -> Vec<String> {
    // Stack of lines to repeat.
    // The element with the highest index is the top of the stack.
    // Each element is a `RepeatLines` struct. Lines are added to
    // this list as they are encountered. A new vector is pushed to the stack
    // when a repeat line is opened. And when a vector is popped, all of those
    // lines are added repeatedly to the previous vector.
    let mut repeats: Vec<RepeatLines> = vec![];
    let mut output: Vec<String> = vec![];
    for line in lines {
        if line.to_uppercase().starts_with("#REPEAT(") {
            repeats.push(RepeatLines::new(parse_repeat_count(&line)));
        } else if line.eq_ignore_ascii_case("#END_REPEAT") {
            let last = repeats.pop().expect("Unexpected end repeat.");
            for text in last.get_text().split("\n") {
                match repeats.last_mut() {
                    Some(prev) => prev.push_line(text.to_string()),
                    None => output.push(text.to_string()),
                }
            }
        } else {
            match repeats.last_mut() {
                Some(repeat_list) => repeat_list.push_line(line),
                None => output.push(line),
            }
        }
    }
    assert!(
        repeats.is_empty(),
        "Repeats is nonempty. Not all lines are written to output."
    );
    output
}

/// Divides `m` into `n` numbers of equal probability. The first `n % m` numbers
/// have value `n / m + 1`, the remaining have value `n / m`.
pub fn probs(n: u32, m: u32) -> Vec<u32> {
    let q = n / m;
    let r = n % m;
    let mut results = Vec::with_capacity(m as usize);
    for _ in 0..r {
        results.push(q + 1);
    }
    for _ in r..m {
        results.push(q);
    }
    results
}

/// Returns the next label name immediately succeeding `label`.
///
/// Labels are used for control flow statements in random map scripts. This
/// function provides the ability to generate unique label names. These names
/// are not designed to be human-readable, but they do still follow the
/// convention of using only capital letters and a leading underscore. Passing
/// `&None` as an argument returns a default label. Starting with that label,
/// the results of subsequent calls to this function may continue to be passed
/// to generate unique labels.
///
/// Note it is also possible that the map script itself may define a label with
/// such a name, so a user of this function should ensure they do not begin
/// their own label names with underscores.
pub fn next_label(label: &Option<&str>) -> String {
    if let Some(label) = label {
        let n = label.len();
        let last = label.as_bytes()[n - 1];
        match last {
            b'Z' => format!("{label}A"),
            c => format!("{}{}", &label[..n - 1], (c + 1) as char),
        }
    } else {
        "_A".to_string()
    }
}

/// Returns the random block for the script's preamble for a `rnd(min,max)`
/// instructions. Requires `min < max`. `label` is the prefix of the label
/// defined by the precent random block. The labels have the form `label_k`,
/// where `k` is an integer making each label unique.
pub fn prob_definitions(label: &str, min: u32, max: u32) -> String {
    assert!(min < max, "Requires {min} < {max}.");
    let length = max + 1 - min;
    let percents = probs(100, length);
    let mut lines = Vec::with_capacity(length as usize + 2);
    lines.push("start_random".to_string());
    for (k, &p) in percents.iter().enumerate() {
        lines.push(format!("percent_chance {p} #define {label}_{k}"))
    }
    lines.push("end_random".to_string());
    lines.join("\n")
}

/// Returns the code for the if statement
pub fn prob_conditional(label: &str, instruction: &str, min: u32, max: u32) -> String {
    let mut lines = Vec::with_capacity((max + 3 - min) as usize);
    let mut delim = "if";
    for (k, v) in (min..=max).enumerate() {
        lines.push(format!("{delim} {label}_{k}\n{instruction} {v}"));
        delim = "elseif";
    }
    lines.push("endif".to_string());
    lines.join("\n")
}

/// Returns `(instruction, min, max)`.
pub fn extract_random_line(line: &str) -> (&str, u32, u32) {
    let h = line.find(' ').unwrap();
    let i = line.find('(').unwrap();
    let j = line.find(',').unwrap();
    let k = line.find(')').unwrap();

    let instruction = &line[..h];
    let min = (&line[i + 1..j]).parse::<u32>().unwrap();
    let max = (&line[j + 1..k]).parse::<u32>().unwrap();
    (instruction, min, max)
}

/// Applies the `#SET_PLACE_FOR_EVERY_PLAYER` macro, copying the object and
/// assigning it to individual lands for each player. This allows player lands
/// to have a `land_id` and still "use" the `set_place_for_every_player`
/// instruction.
fn assign_objects(lines: Vec<String>) -> Vec<String> {
    // The loop has two states: `object` is empty or `object` is nonempty.
    // A line is added to object upon reaching the first line of `create_object`
    // command, and all lines from the command are collected until the command's
    // closing curly brace. The `every_player` flag is set if the corresponding
    // macro is encountered while parsing the object. When the object is parsed,
    // it is pushed to the output, and the `every_player` flag is reset and the
    // `object` queue is emptied.
    let mut output = vec![];
    let mut object = VecDeque::new();
    let mut every_player = false;
    let mut num_players = 2;
    for line in lines {
        if object.is_empty() {
            assert!(
                !line.eq_ignore_ascii_case("#SET_PLACE_FOR_EVERY_PLAYER"),
                "Macro encountered outside of create_object command."
            );
            if line.starts_with("create_object") {
                object.push_back(line);
            } else {
                output.push(line);
            }
            continue;
        }
        match &line[..] {
            "}" => {
                if every_player {
                    // Pushes the object for each player, adding the land id.
                    for land_id in 1..=num_players {
                        for s in &object {
                            output.push(s.clone());
                        }
                        output.push(format!("place_on_specific_land_id {land_id}"));
                        output.push("}".to_string());
                    }
                    object.clear();
                } else {
                    // Pushes the object once, as is.
                    while !object.is_empty() {
                        output.push(object.pop_front().unwrap());
                    }
                    output.push("}".to_string());
                }
                every_player = false
            }
            "#SET_PLACE_FOR_EVERY_PLAYER" => {
                every_player = true;
                num_players = 2;
            }
            "#PLACE8" => {
                every_player = true;
                num_players = 8;
            }
            _ => object.push_back(line),
        }
    }
    assert!(object.is_empty(), "Object not closed, missing `}}`.");
    output
}

/// Moves the `rnd` commands from every line after land generation to be a
/// random block at the start of the file, along with an if statement where the
/// rnd was located. Requires that every line after land generation as at most
/// one `rnd` command.
fn extract_rnd(lines: Vec<String>) -> Vec<String> {
    // If the script does not specify to extract the `rnd` instructions for
    // debugging, then don't extract them.
    if lines
        .iter()
        .all(|line| !line.eq_ignore_ascii_case("#EXTRACT_RND"))
    {
        return lines;
    }

    let mut preamble = Vec::new();
    let mut body = Vec::new();
    let mut label = next_label(&None);
    let mut finished_land = false;
    for line in lines {
        // Avoids copying the extract random flag to the final output.
        if line.eq_ignore_ascii_case("#EXTRACT_RND") {
            continue;
        }
        if !finished_land {
            finished_land = line.contains("ELEVATION_GENERATION");
            body.push(line);
            continue;
        }
        if !line.contains("rnd") {
            body.push(line);
            continue;
        }
        let (instruction, min, max) = extract_random_line(&line);
        preamble.push(prob_definitions(&label, min, max));
        body.push(prob_conditional(&label, instruction, min, max));
        label = next_label(&Some(&label));
    }
    let mut output = preamble;
    output.append(&mut body);
    output
}

/// Replaces actor areas in `lines` with names. Allows for string-named actor
/// areas instead of pure numbers.
fn substitute_actor_area_names(lines: Vec<String>) -> Vec<String> {
    let mut next_id = 20_000; // Start at a high number to avoid conflicts with DE maps.
    let mut actor_areas = HashMap::new();
    // Assigns a unique ID number to each named actor area.
    for line in &lines[..] {
        if !line.starts_with("actor_area ") && !line.starts_with("create_actor_area ") {
            continue;
        }
        let name = if line.starts_with("actor_area ") {
            let i = line.find(' ').unwrap();
            &line[i + 1..]
        } else {
            let j = line.rfind(' ').unwrap();
            let i = &line[..j].rfind(' ').unwrap();
            &line[i + 1..j]
        };
        if actor_areas.contains_key(name) {
            continue; // Don't re-insert the key if the actor area exists.
        }
        actor_areas.insert(name.to_string(), next_id);
        next_id += 1;
    }

    // Replaces the actor areas with their ID numbers.
    lines
        .iter()
        .map(|line| {
            let Some(i) = line.find(' ') else {
                return line.to_string();
            };
            let command = &line[..i];
            match command {
                "actor_area" | "avoid_actor_area" | "actor_area_to_place_in" => {
                    let name = &line[i + 1..];
                    if let Some(id) = actor_areas.get(name) {
                        format!("{command} {id}")
                    } else {
                        // The actor area is not defined. This exists in DE's official
                        // map scripts, so allow for the possibility instead of
                        // panicing.
                        line.to_string()
                    }
                }
                "create_actor_area" => {
                    let k = line.rfind(' ').unwrap();
                    let j = line[..k].rfind(' ').unwrap();
                    let name = &line[j + 1..k];
                    if let Some(id) = actor_areas.get(name) {
                        format!("{} {id}{}", &line[..j], &line[k..])
                    } else {
                        // The actor area is not defined. This exists in DE's official
                        // map scripts, so allow for the possibility instead of
                        // panicing.
                        line.to_string()
                    }
                }
                _ => return line.to_string(),
            }
        })
        .collect()
}

/// Writes every line of `lines` to `dest`, including a single new line between
/// each line but not at the end. Stops writing lines if a line contains
/// `"#BREAK"`.
fn write_until_break(lines: Vec<String>, dest: &mut BufWriter<File>) -> std::io::Result<()> {
    let mut delim = ""; // Initially empty delim prevents a trailing newline.
    for line in lines {
        if line.to_uppercase().contains("#BREAK") {
            break;
        }
        write!(dest, "{delim}{line}")?;
        delim = "\n"; // Use a newline as a leading delim after the first line.
    }
    Ok(())
}

/// Reads the map script in `src`, applies preprocessing steps, and writes the
/// output to `dest`.
pub fn process_script(
    src: &mut BufReader<File>,
    dest: &mut BufWriter<File>,
) -> std::io::Result<()> {
    // This doesn't feel very idomatic, at least without a pipe operator.
    let lines = src.lines().collect::<std::io::Result<Vec<String>>>()?;
    let (header, lines) = collect_header_comment(lines);
    // Including files feels more trouble than it's worth, just do the Boars
    // in the maps themselves.
    // let lines = strip_comments(lines);
    // let lines = condense_whitespace(lines);
    // let lines = include_files(lines)?;
    let lines = strip_comments(lines); // Strip again for included files.
    let lines = condense_whitespace(lines);
    let lines = insert_macros(lines);
    let lines = insert_for_loops(lines);
    let lines = repeat_lines(lines);
    let lines = assign_objects(lines);
    let lines = extract_rnd(lines);
    let lines = substitute_actor_area_names(lines);
    let mut total = vec![];
    for line in header {
        total.push(line.clone());
    }
    for line in lines {
        total.push(line.clone());
    }
    write_until_break(total, dest)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests a generic example of condensing a string with whitespace.
    #[test]
    fn test_condense_whitespace() {
        assert_eq!(condense_line_whitespace("   "), "");
    }

    /// Tests that condensing a string consisting of only whitespace is empty.
    #[test]
    fn test_condense_only_whitespace() {
        assert_eq!(condense_line_whitespace("   "), "");
    }

    /// Tests that stripping comments from the empty string is still empty.
    #[test]
    fn strip_comments_empty() {
        for d in 0..1000 {
            let (s, d_out) = strip_line_comments("", d);
            assert_eq!("", s);
            assert_eq!(d, d_out);
        }
    }

    /// Tests stripping text that does not contain any comment delimeters.
    #[test]
    fn strip_no_comments() {
        let (s, d) = strip_line_comments("Hello, World!", 0);
        assert_eq!("Hello, World!", s);
        assert_eq!(0, d);
    }

    /// Tests stripping a string that does not contain any comment characters
    /// when the depth is strictly positive.
    #[test]
    fn strip_during_comment() {
        let (s, d) = strip_line_comments("Hello, World!", 1);
        assert_eq!("", s);
        assert_eq!(1, d);
    }

    /// Tests stripping a line that starts but does not finish a comment.
    #[test]
    fn strip_comment_start() {
        let (s, d) = strip_line_comments("a /* b", 0);
        assert_eq!("a ", s);
        assert_eq!(1, d);
    }

    /// Tests that the nested comment counter is incremented when the depth is
    /// positive and the line contains a comment start substring.
    #[test]
    fn strip_comment_nested_start() {
        let (s, d) = strip_line_comments("a /* b", 1);
        assert_eq!("", s);
        assert_eq!(2, d);
    }

    /// Tests stripping a lien that starts and ends with the comment delimiters.
    #[test]
    fn strip_entire_line() {
        let (s, d) = strip_line_comments("/* this is a comment */", 0);
        assert_eq!("", s);
        assert_eq!(0, d);
    }
}
