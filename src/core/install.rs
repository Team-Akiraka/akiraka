use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::Path;
use serde_json::Value;
use crate::core::VersionSource;

// TODO Rule checking
fn check_rule(rule: &str) -> bool {
    return false;
}

pub fn install(
    source: &VersionSource,
    dir: String,
    timeout: usize,
    pool_size: usize) -> Result<(), std::io::Error> {

    // Directories
    create_dir_all(dir.clone()).expect("Could not create directory!");
    let binding = Path::new(String::from(&dir).as_str()).join("versions").join(&source.version_id);
    let versions_path = binding.as_path();
    let binding = versions_path.clone().join(String::from(&source.version_id) + ".json");
    let json_path = binding.as_path();

    // Version Json
    println!("{}", json_path.to_str().unwrap());
    create_dir_all(versions_path).expect("Could not create json file!");
    let mut json_file = File::create(json_path).expect("Could not create json file!");
    // TODO: Download sources
    let json = reqwest::blocking::get(&source.version_url).expect("Errors while downloading Json!").text().expect("Errors while reading Json!");
    json_file.write_all(json.as_ref()).expect("Could not write to Json!");

    // Json parsing
    let json: Value = serde_json::from_str(json.as_str()).unwrap();
    // Libraries
    for library in json["libraries"].as_array().unwrap() {
        // println!("{}", library["name"].as_str().unwrap());
        // println!("{}", serde_json::to_string(library).unwrap());
        // Contains rules
        if library.get("rules").is_some() {
            println!("{}", serde_json::to_string(library.get("rules").unwrap()).unwrap())
        } else {
            println!("no rules");
        }
    }

    Ok(())
}