use std::env;
use std::env::consts::OS;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::Path;
use std::time::Duration;
use serde_json::Value;
use crate::core::VersionSource;

fn check_rule(rules: &Vec<Value>) -> bool {
    let mut allow = false;
    for rule in rules {
        // TODO: Multiplatform and more rules
        // https://doc.rust-lang.org/std/env/consts/constant.OS.html
        println!("{}", serde_json::to_string(rule).unwrap());
        println!("{}", env::consts::ARCH);
        let allow_ = rule["action"].as_str().unwrap() == "allow";
        if rule.get("os").is_some() {
            let os = OS;
            let os_ = rule["os"]["name"].as_str().unwrap();

            if os == "windows" && os_ == "windows" {
                allow |= allow_;
            } else if os == "linux" && os_ == "linux" {
                allow |= allow_;
            } else if os == "macos" && os_ == "osx" {
                allow |= allow_;
            }
        } else {
            allow |= allow_;
        }
    }
    return allow;
}

pub fn install(
    source: &VersionSource,
    dir: String,
    time_out: usize,
    pool_size: usize) -> Result<(), std::io::Error> {
    // Http Client
    let client = reqwest::blocking::ClientBuilder::new().timeout(Duration::from_millis(time_out as u64)).build().expect("Could not create Http Client!");

    // Directories
    create_dir_all(dir.clone()).expect("Could not create directory!");
    let binding = Path::new(String::from(&dir).as_str()).join("versions").join(&source.version_id);
    let versions_path = binding.as_path();
    let binding = Path::new(String::from(&dir).as_str()).join("libraries");
    let library_path = binding.as_path();
    let binding = versions_path.clone().join(String::from(&source.version_id) + ".json");
    let json_path = binding.as_path();

    // Version Json
    create_dir_all(versions_path).expect("Could not create json file!");
    let mut json_file = File::create(json_path).expect("Could not create json file!");
    // TODO: Download sources
    let json = client.get(&source.version_url).send().expect("Errors while downloading Json!").text().expect("Errors while reading Json!");
    json_file.write_all(json.as_ref()).expect("Could not write to Json!");

    // Json parsing
    let json: Value = serde_json::from_str(json.as_str()).unwrap();

    // Libraries
    for library in json["libraries"].as_array().unwrap() {
        // Contains rules?
        let mut allow = true;
        if library.get("rules").is_some() {
            allow = check_rule(library.get("rules").unwrap().as_array().unwrap());
        }

        if !allow {
            continue;
        }

        // TODO: Download sources and Multithreading
        // Download
        // Artifact
        if library["downloads"].get("artifact").is_some() {
            let file_path = library_path.clone().join(library["downloads"]["artifact"]["path"].as_str().unwrap());
            create_dir_all(file_path.clone().parent().unwrap()).expect("Could not create library path!");
            let mut file = File::create(library_path.clone().join(library["downloads"]["artifact"]["path"].as_str().unwrap())).unwrap();
            file.write_all(&*client.get(library["downloads"]["artifact"]["url"].as_str().unwrap()).send().unwrap().bytes().unwrap()).expect("Could not write library file!");
        }
        // Classifiers
        if library["downloads"].get("classifiers").is_some() {
            // TODO: Classifiers
        }
    }

    Ok(())
}