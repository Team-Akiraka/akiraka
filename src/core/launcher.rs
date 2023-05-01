use std::env::consts::OS;
use std::{env};
use std::fs::File;
use std::io::{Read};
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

    // 替换游戏参数的函数
    let replace_game_argument = |arg: String| -> String {
        let arg = arg;
        // TODO: 替换游戏参数
        // --username
        // ${auth_player_name}
        // --version
        // ${version_name}
        // --gameDir
        // ${game_directory}
        // --assetsDir
        // ${assets_root}
        // --assetIndex
        // ${assets_index_name}
        // --uuid
        // ${auth_uuid}
        // --accessToken
        // ${auth_access_token}
        // --userType
        // ${user_type}
        // --versionType
        // ${version_type}

        // 必要参数（迫真
        let arg = arg.replace("${game_directory}", to_absolute(dir).to_str().unwrap());
        let arg = arg.replace("${assets_root}", to_absolute(dir.clone().join("assets").as_path()).to_str().unwrap());
        let arg = arg.replace("${assets_index_name}", json["assets"].as_str().unwrap());
        // 非必要参数（迫真
        let arg = arg.replace("${version_type}", "akiraka");
        let arg = arg.replace("${version_name}", "vanilla");
        return arg;
    };

    // 启动参数
    let mut arguments: Vec<String> = Vec::new();

    // JVM参数
    // Classpath参数
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
            let natives = &i["natives"];
            // 系统
            let os = if OS == "macos" {
                "osx"
            } else {
                OS
            };
            if natives.get(os).is_some() {
                let natives = &i["downloads"]["classifiers"][&i["natives"][os].as_str().unwrap()];
                if natives.is_null() {
                    continue
                }
                classpath += path_separator;
                classpath += library_dir.clone().join(natives["path"].as_str().unwrap()).to_str().unwrap();
            }
        }
    }
    arguments.push(String::from("-cp"));
    arguments.push(classpath.clone());
    println!("{}", classpath);

    // 主类
    arguments.push(String::from(json["mainClass"].as_str().unwrap()));

    // 游戏参数
    if json.get("arguments").is_some() {
        // 扁平化后的参数
        let args = json["arguments"]["game"].as_array().unwrap();
        for i in args {
            if i.as_str().is_none() {
                continue
            }
            let temp = i.as_str().unwrap();
            println!("{}", temp);
            let arg = replace_game_argument(String::from(temp));
            arguments.push(arg);
        }
    } else {
        // TODO: 扁平化前的参数
    }

    // println!("{}", to_absolute(java).to_str().unwrap());
    for i in &arguments {
        println!("{}", i);
    }

    let proc = Command::new(java)
        .args(arguments)
        .current_dir(dir)
        .spawn()
        .expect("Could not execute Minecraft!");
    let stdout = proc.wait_with_output().unwrap().stdout;
    println!("{}", String::from_utf8(stdout).unwrap());
    Ok(())
}