//! Preprocessor for Age of Empires II random map scripts.
//!
//! Provides three options:
//! - Copy all scripts in all directories.
//! - Copy all scripts in a specific directory.
//! - Copy a single script.
//!
//! The main usage of this program is for watchexec to run it automatically
//! whenever a script file is modified. It can also be invoked manually for
//! mass preprocessing of all script files, regardless of whether they have
//! been modified.
//!
//! To aid in the responsiveness of running with watchexec, this program should
//! be compiled in release mode, and the release binary should be run.

use std::{
    env,
    path::{Path, PathBuf},
};

/// Filename of the toml file for configuring the RMS preprocessor settings.
const RMS_CONFIG_FILENAME: &str = "rmsconfig.toml";

/// Returns the path to the `rmsconfig.toml` file in the directory of the given
/// script path, or `None` if no such file exists.
fn toml_path(script_path: &str) -> Result<Option<PathBuf>, Box<dyn std::error::Error>> {
    let parent = match Path::new(script_path).parent() {
        Some(parent) => parent,
        None => return Ok(None),
    };
    if !parent.is_dir() {
        return Ok(None);
    }
    Ok(Some(parent.join(RMS_CONFIG_FILENAME)))
}

/// Parses the `rmsconfig.toml` file at the given path and returns the value of
/// the `test_mod_directory` key. Returns an error if the file cannot be read or
/// if the key is not found.
fn parse_toml(config_path: &PathBuf) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let config_file = std::fs::read_to_string(config_path)?;
    for line in config_file.lines() {
        let line = line.trim();
        let Some(i) = line.find('=') else {
            continue;
        };
        let key = line[..i].trim();
        let value = line[i + 1..].trim().trim_matches('"');
        if key != "test_mod_directory" {
            continue;
        }
        return Ok(PathBuf::from(value));
    }
    Err("cannot find test_mod_directory in rmsconfig.toml".into())
}

/// Returns the path to the directory where the output of preprocessing
/// `script_path` should be written.
/// Returns `None` if no target path can be determined (either a rmsconfig.toml
/// file does not exist or does not specify a target path).
fn target_path(script_path: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let config_path = match toml_path(script_path)? {
        Some(path) => path,
        None => return Err("cannot load rmsconfig.toml".into()),
    };
    parse_toml(&config_path)
}

/// Runs the proprocessor on a single script file and write the result.
/// `script_path` is the path to the script file to preprocess.
/// `target_dir` is the directory where the preprocessed file will be written.
/// If a file with the same name as the name of `script_path` already exists
/// in `target_dir`, it will be overwritten.
/// Returns an error if the preprocessor fails or if there is an I/O error.
fn preprocess(script_path: &str, target_dir: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let script_path = PathBuf::from(script_path);
    println!("script_path: {script_path:?}");
    let map_name = match script_path.file_name() {
        Some(name) => name,
        None => return Err("cannot get file name".into()),
    };
    let target_path = target_dir.join(map_name);
    println!("target_path: {target_path:?}");
    let map_contents = std::fs::read_to_string(script_path)?;
    println!("map_contents: {map_contents:?}");
    // TODO call the preprocessor here on the map contents
    // std::fs::write(target_path, map_contents)?;
    Ok(())
}

/// Runs the preprocessor on a single script file and writes the result.
/// `script_path` is the path to the script file to preprocess.
///
/// Searches the parent directory of `script_path` for the toml file containing
/// the target directory path. If no such file is found or the file does not
/// contain a valid target directory path, returns an error.
///
/// Further returns an error if the preprocessor fails or if there is
/// an I/O error.
fn preprocess_script(script_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    preprocess(script_path, &target_path(script_path)?)
}

/// Runs the preprocessor on all script files in `dir_path` and writes the
/// results, overwriting any existing files.
///
/// Searches `dir_path` for `rmsconfig.toml` and uses the output directory
/// specified therein. If no such file is found or the file does not contain
/// a valid target directory path, returns an error.
///
/// Further returns an error if the preprocessor fails or if there is
/// an I/O error.
fn preprocess_dir(dir_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let toml_path = PathBuf::from(dir_path).join(RMS_CONFIG_FILENAME);
    let target_dir = parse_toml(&toml_path)?;
    // Process all .rms files in the `dir_path` directory.
    for entry in std::fs::read_dir(dir_path)? {
        let path = entry?.path();
        if path.is_file() && path.extension().map_or(false, |e| e == "rms") {
            let script_path = path.to_str().unwrap();
            preprocess(script_path, &target_dir)?;
        }
    }
    Ok(())
}

/// Run the preprocessor on all subdirectories of `scripts_dir_path`.
/// Returns an error if any subdirectory fails or if there is an I/O error.
fn preprocess_all_scripts(scripts_dir_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    for entry in std::fs::read_dir(scripts_dir_path)? {
        let path = entry?.path();
        if path.is_dir() {
            let dir_path = path.to_str().unwrap();
            preprocess_dir(dir_path)?;
        }
    }
    Ok(())
}

/// Runs the preprocessor, returning an error if it does not succeed.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let program_type = args.get(1).unwrap();
    let path = args.get(2).unwrap();
    match &program_type[..] {
        "all" => preprocess_all_scripts(path),
        "dir" => preprocess_dir(path),
        "script" => preprocess_script(path),
        _ => Err("Invalid program type".into()),
    }
}
