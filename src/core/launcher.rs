use std::env::consts::OS;
use std::fmt::format;
use std::{env, fs};
use std::fs::File;
use std::io::{Error, ErrorKind, Read};
use std::path::{Path, PathBuf};
use std::process::Command;
use serde_json::Value;

pub fn launch(
    name: &str,
    dir: &Path,
    java: &Path) -> Result<(), String> {

    // 龟则检查函数
    fn check_rule(rules: &Vec<Value>) -> bool {
        let mut allow = false;
        for rule in rules {
            // TODO: 跨平台以及更多的规则
            // https://doc.rust-lang.org/std/env/consts/constant.OS.html
            // println!("{}", env::consts::ARCH);
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
                } else {
                    allow |= false;
                }
            } else {
                allow |= allow_;
            }
            // println!("{}", serde_json::to_string(rule).unwrap());
        }
        // println!("{}", allow);
        return allow;
    }

    // 文件夹检查函数
    fn check_dir(dir: &Path) -> bool {
        if !dir.exists() || !dir.is_dir() {
            return false;
        }
        return true;
    }

    // 转换为绝对路径
    fn to_absolute(path: &Path) -> PathBuf {
        let temp = env::current_dir().unwrap().join(path);
        return temp;
    }

    // 路径检查
    let temp = dir.clone().join("versions").join(name);
    let temp = to_absolute(temp.as_path());
    let versions_dir = temp.as_path();
    if !check_dir(versions_dir) {
        return Err(format!("Could not find version \"{}\"!", name));
    }
    let temp = dir.clone().join("libraries");
    let temp = to_absolute(temp.as_path());
    let library_dir = temp.as_path();
    if !check_dir(library_dir) {
        return Err("Could not find libraries!".parse().unwrap())
    }
    let temp = dir.clone().join("assets");
    let temp = to_absolute(temp.as_path());
    let asset_dir = temp.as_path();
    if !check_dir(asset_dir) {
        return Err("Could not find libraries!".parse().unwrap())
    }

    // JSON文件的读取
    let mut json = File::open(versions_dir.clone().join(format!("{}.json", name))).expect("Could not open JSON!");
    let temp: &mut String = &mut String::new();
    json.read_to_string(temp).expect("Could not read JSON!");
    let json: Value = serde_json::from_str(temp.as_str()).expect("JSON format error!");

    // 启动参数
    #[cfg(target_os = "windows")]
        let path_separator = ";";
    #[cfg(not(target_os = "windows"))]
        let path_separator = ":";
    let temp = versions_dir.join(format!("{}.jar", name));
    let temp = to_absolute(temp.as_path());
    let mut classpath = String::from(temp.as_path().to_str().unwrap());
    println!("{}{}", classpath, path_separator);

    for i in json["libraries"].as_array().unwrap() {
        // 检查规则
        let allow = if i.get("rules").is_some() {
            check_rule(i["rules"].as_array().unwrap())
        } else {
            true
        };
        if !allow {
            continue;
        }

        // TODO: Path by name
        if i.get("downloads").is_some() && i["downloads"].get("artifact").is_some() {
            classpath += path_separator;
            classpath += library_dir.clone().join(i["downloads"]["artifact"]["path"].as_str().unwrap()).to_str().unwrap();
        }
        if i.get("downloads").is_some() && i["downloads"].get("classifiers").is_some() {
            classpath += path_separator;
            classpath += library_dir.clone().join(i["downloads"]["classifiers"]["path"].as_str().unwrap()).to_str().unwrap();
        }
    }
    println!("{}", classpath);

    let proc = Command::new(java)
        .arg("-version")
        .spawn()
        .expect("Could not execute Minecraft!");
    let stdout = proc.wait_with_output().unwrap().stdout;
    println!("{}", String::from_utf8(stdout).unwrap());
    Ok(())
}