use std::fs::{create_dir_all};
use std::io::Write;
use std::path::Path;
use std::time::SystemTime;
use rust_embed::RustEmbed;
use crate::core::install::install;
use crate::core::network::get_version_sources;
use crate::core::util;

mod core;

static mut DIR: String = String::new();

#[derive(RustEmbed)]
#[folder = "assets"]
struct Asset;

fn main() {
    // let test = Asset::get("icon.txt").unwrap();
    // println!("{:?}", std::str::from_utf8(test.data.as_ref()).unwrap());
    unsafe { DIR = String::from(Path::new(util::current_dir().as_str()).join(".minecraft").as_path().to_str().unwrap()); }
    println!("Akiraka Command Tool [Version 0.1.0-dev.20230420]\n(c) Arrokoth233. All rights reserved\n");
    loop {
        print!(">>");
        std::io::stdout().flush().expect("Flush Error!");

        let mut  input = String::new();
        std::io::stdin().read_line(&mut input).expect("Read line Error!");
        let input = input.trim();

        handle_command(input);
    }
}

fn handle_command(command: &str) {
    if command == "" {
        // 检查命令是否为空
        return;
    } else if command == "exit" {
        // 退出程序
        std::process::exit(0);
    } else {
        // 真正有用的命令（迫真
        // 解析命令
        let args: Vec<&str> = command.split(" ").collect();
        let head = args[0];

        // 游戏目录
        if head == "dir" {
            if args.len() >= 2 {
                if args[1] == "new" {
                    // 创建
                    create_dir_all(args[2]).expect("Could not create directories!");
                    if args[2].replace("\\", "/").contains("/") {
                        if &(args[2].replace("\\", "/"))[args[2].replace("\\", "/").rfind("/").unwrap()..] != "/.minecraft" {
                            create_dir_all(String::from(args[2]) + "/.minecraft").expect("Could not create directories!");
                        }
                    }
                } else if args[1] == "change" {
                    // 更改
                    unsafe {
                        let path = Path::new(args[2]);
                        if path.exists() {
                            DIR = String::from(path.to_str().unwrap());
                            println!("Game Directory changed: {}", DIR);
                        }
                    }
                } else {
                    unknown_arguments();
                }
            } else if args.len() == 1 {
                unsafe { println!("Current Game directory: {}", DIR); }
            } else {
                unknown_arguments();
            }
        } else if head == "index" {
            let mut s = false;
            let mut r = true;
            let mut ob = false;
            let mut oa = false;

            for i in args {
                if i == "+s" {
                    s = true;
                } else if i == "+r" {
                    r = true;
                } else if i == "+ob" {
                    ob = true;
                } else if i == "+oa" {
                    oa = true;
                } else if i == "-s" {
                    s = false;
                } else if i == "-r" {
                    r = false;
                } else if i == "-ob" {
                    ob = false;
                } else if i == "-oa" {
                    oa = false;
                }
            }
            println!("Downloading version manifest...");
            let sources = get_version_sources(true, true, true, true).unwrap();
            println!("Available versions:");
            for i in 0..sources.len() {
                let version = sources.get(i).unwrap();
                if version.version_type == "snapshot" && s ||
                    version.version_type == "release" && r ||
                    version.version_type == "old_beta" && ob ||
                    version.version_type == "old_alpha" && oa {
                    println!("Index {}: {}", i, sources.get(i).unwrap().version_id);
                }
            }
        } else if head == "install" {
            if args.len() != 4 {
                println!("Unknown arguments!");
                return;
            }
            let start_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis();

            let index = args[1].parse::<usize>().unwrap();
            let time_out = args[2].parse::<usize>().unwrap();
            let pool_size = args[3].parse::<usize>().unwrap();

            let sources = get_version_sources(true, true, true, true).unwrap();
            let source = sources.get(index).unwrap();

            println!("Minecraft is installing..\nIndex: {}\nId: {}\nTime out: {}\nPool size: {}",
                     index,
                     source.version_id,
                     time_out,
                     pool_size);

            unsafe { install(source, String::from(&DIR), time_out, pool_size).expect("Error while installing Minecraft!"); }

            let end_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis();
            println!("Install process end! Time used: {}ms.", (end_time - start_time));
        } else {
            unknown_command(command);
        }
    }
    println!();
}

fn unknown_command(command: &str) {
    println!("\"{}\" is not a registered command.", command);
}

fn unknown_arguments() {
    println!("Unknown arguments.");
}