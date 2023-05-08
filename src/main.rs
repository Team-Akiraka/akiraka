use std::fs::{create_dir_all, File, read_dir};
use std::io::Write;
use std::net::{SocketAddrV4, UdpSocket};
use std::path::Path;
use std::time::SystemTime;
use crate::core::install::install;
use crate::core::launcher::launch;
use crate::core::network::get_version_sources;
use crate::core::{Asset, check_java, util};

mod core;


static mut DIR: String = String::new();
const TEMP_DIR: &str = ".akiraka/.temp";

fn main() {
    // 御坂美琴生日快乐！
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
                                let path = args[2].to_owned() + "/.minecraft";
                                let path = Path::new(path.as_str());
                                create_dir_all(path).expect("Could not create directories!");
                                let mut launcher_profiles = File::create(path.clone().join("launcher_profiles.json")).expect("Could not create Launcher Profiles!");
                                launcher_profiles.write(String::from("{}").as_bytes()).expect("Could not write to Launcher Profiles!");
                                launcher_profiles.flush().expect("Could not close Launcher Profiles!");
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
                    unknown_arguments();
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
            } else if head == "local" {
                unsafe {
                    for i in read_dir(Path::new(&DIR).join("versions")).unwrap() {
                        let dir = i.unwrap();
                        let json = dir.path().clone().join(format!("{}.json", dir.file_name().to_str().unwrap()));
                        if json.exists() && json.is_file() {
                            println!("Found Version: {}", dir.file_name().to_str().unwrap());
                        }
                    }
                }
            } else if head == "java" {
                let res = check_java(args[1], TEMP_DIR);
                match res {
                    Ok(..) => {
                        let temp = res.unwrap().clone();
                        println!("Java Home: {}\nJava Version: {}\nJava Class Version: {}", temp["java_home"].as_str().unwrap(), temp["java_class_version"].as_f64().unwrap(), temp["java_class_version"].as_f64().unwrap() + 44.0)
                    },
                    Err(..) => println!("Error while checking Java: {}", res.err().unwrap())
                }
            } else if head == "run" {
                if args.len() == 3 {
                    unsafe {
                        let x = launch(args[1], Path::new(&DIR), Path::new(args[2]));
                        match x {
                            Ok(..) => {},
                            Err(..) => println!("{}", format!("Error while launching game: {}", x.err().unwrap()))
                        }
                    }
                } else {
                    unknown_arguments();
                }
            } else if head == "bridge" {
                let socket = UdpSocket::bind("127.0.0.1:25565").unwrap();
                socket.connect(SocketAddrV4::new("38.6.226.131".parse().unwrap(), 40001)).unwrap();
                socket.send("hello, akiraka!".as_bytes()).unwrap();
                let mut buf = [0u8; 1500];
                let amt = socket.recv(&mut buf).unwrap();
                let buf = &mut buf[..amt];
                println!("{}", std::str::from_utf8(buf).unwrap());
            } else if head == "help" {
                let data = Asset::get("help.txt").unwrap();
                let data = std::str::from_utf8(&*data.data).unwrap();
                println!("{}", data);
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
    // Minecraft购买链接：https://www.xbox.com/zh-cn/games/store/minecraft-java-bedrock-edition-for-pc/9nxp44l49shj
    // 购买链接更新：https://www.xbox.com/zh-CN/games/store/minecraft-java-bedrock-edition-for-pc/9NXP44L49SHJ/0010
    unsafe { DIR = String::from(Path::new(util::current_dir().as_str()).join(".minecraft").as_path().to_str().unwrap()); }
    println!("Akiraka Command Tool [Version 0.1.0-dev.20230508]\n(c) Arrokoth233. All rights reserved\n");
    loop {
        print!(">>");
        std::io::stdout().flush().expect("Flush Error!");

        let mut  input = String::new();
        std::io::stdin().read_line(&mut input).expect("Read line Error!");
        let input = input.trim();

        handle_command(input);
    }
}