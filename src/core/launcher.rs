use std::env::consts::{OS};
use std::{env};
use std::fs::File;
use std::io::{Read};
use std::path::{Path, PathBuf};
use std::process::Command;
use serde_json::Value;
use crate::core::check_rule;

pub fn launch(
    name: &str,
    dir: &Path,
    java: &Path) -> Result<(), String> {
    // 御坂美琴生日快乐！

    // TODO: 我们是不是要支持下模组加载器？
    // TODO: 哼哼哼啊啊啊啊我做不到啊啊啊啊啊啊啊
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
    let replace_jvm_argument = |arg: String| -> String {
        let arg = arg;

        // 必要参数（迫真
        // 非必要参数（迫真
        let arg = arg.replace("${launcher_name}", "Akiraka");
        let arg = arg.replace("${launcher_version}", "internal");
        return arg;
    };

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
        // ${game_assets}
        // --assetIndex
        // ${assets_index_name}
        // --uuid
        // ${auth_uuid}
        // --accessToken
        // ${auth_access_token}
        // ${auth_session}
        // --userType
        // ${user_type}
        // --versionType
        // ${version_type}
        // --userProperties
        // ${user_properties}

        // 必要参数（迫真
        let arg = arg.replace("${game_directory}", to_absolute(dir).to_str().unwrap());
        let arg = arg.replace("${assets_root}", to_absolute(dir.clone().join("assets").as_path()).to_str().unwrap());
        let arg = arg.replace("${game_assets}", to_absolute(dir.clone().join("assets").as_path()).to_str().unwrap());
        let arg = arg.replace("${assets_index_name}", json["assets"].as_str().unwrap());
        let arg = arg.replace("${user_properties}", "{}");
        // TODO: 账号
        let arg = arg.replace("${auth_uuid}", "00000000-0000-0000-0000-000000000000");
        let arg = arg.replace("${auth_player_name}", "Dev");
        let arg = arg.replace("${auth_access_token}", "null");
        let arg = arg.replace("${auth_session}", "null");
        let arg = arg.replace("${clientid}", "null");
        let arg = arg.replace("${auth_xuid}", "null");
        let arg = arg.replace("${user_type}", "msa");
        // 非必要参数（迫真
        let arg = arg.replace("${version_type}", "akiraka");
        let arg = arg.replace("${version_name}", "vanilla");
        return arg;
    };

    // 启动参数
    let mut arguments: Vec<String> = Vec::new();

    // JVM参数
    arguments.push(format!("-Djava.library.path={}", to_absolute(versions_dir.clone().join("natives").as_path()).to_str().unwrap()));
    // 游戏提供的参数
    // 检查是否需要跳过
    fn should_skip(arg: String) -> bool {
        let mut res = false;
        res |= arg == String::from("-cp");
        res |= arg == String::from("${classpath}");
        if arg.find("=").is_some() {
            res |= arg.starts_with("-Djava.library.path");
            res |= arg.starts_with("-Dos.name");
            res |= arg.starts_with("-Dos.version");
        }
        return res;
    }
    // 添加参数
    if json.get("arguments").is_some() {
        for i in json["arguments"]["jvm"].as_array().unwrap() {
            if i.is_string() {
                if should_skip(String::from(i.as_str().unwrap())) {
                    continue
                }
                arguments.push(replace_jvm_argument(String::from(i.as_str().unwrap())));
            } else {
                if i.get("rules").is_some() {
                    if !check_rule(i["rules"].as_array().unwrap()) {
                        continue
                    }
                    if i["value"].is_string() {
                        if should_skip(String::from(i["value"].as_str().unwrap())) {
                            continue
                        }
                        arguments.push(replace_jvm_argument(String::from(i["value"].as_str().unwrap())));
                    } else if i["value"].is_array() {
                        for j in i["value"].as_array().unwrap() {
                            if should_skip(String::from(j.as_str().unwrap())) {
                                continue
                            }
                            arguments.push(replace_jvm_argument(String::from(j.as_str().unwrap())));
                        }
                    }
                }
            }
        }
    }
    // Classpath参数
    #[cfg(target_os = "windows")]
        let path_separator = ";";
    #[cfg(not(target_os = "windows"))]
        let path_separator = ":";
    let temp = versions_dir.join(format!("{}.jar", name));
    let temp = to_absolute(temp.as_path());
    let mut classpath = String::from(temp.as_path().to_str().unwrap());

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
            let arg = replace_game_argument(String::from(temp));
            arguments.push(arg);
        }
    } else if json.get("minecraftArguments").is_some() {
        // TODO: 扁平化前的参数
        for i in json["minecraftArguments"].as_str().unwrap().split(" ") {
            arguments.push(replace_game_argument(String::from(i)));
        }
    }

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