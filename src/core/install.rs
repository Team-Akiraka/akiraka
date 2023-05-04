use std::env::consts::OS;
use std::fs::{create_dir_all, File};
use std::io::{Read, Write};
use std::path::{Path};
use std::time::Duration;
use reqwest::blocking::Client;
use serde_json::Value;
use zip::ZipArchive;
use crate::core::{check_rule, VersionSource};

const ASSETS_URL: &str = "https://resources.download.minecraft.net/";

pub fn install(
    source: &VersionSource,
    dir: String,
    time_out: usize,
    // TODO: 多线程
    pool_size: usize) -> Result<(), std::io::Error> {
    // 御坂美琴生日快乐！

    // 下崽函数
    fn download(url: &str, mut file: &File, client: &Client) {
        file.write_all(&client.get(url).send().unwrap().bytes().unwrap()).expect("Could not write library file!");
        file.flush().expect("Could not flush file!");
    }

    // 下崽并解鸭函数
    fn download_and_extract(url: &str, mut file: File, extract_path: &Path, client: Client) {
        create_dir_all(extract_path).expect("Could not crate natives directory!");
        let buf = &client.get(url).send().unwrap().bytes().unwrap();
        file.write_all(buf).expect("Could not write library file!");
        file.flush().expect("Could not flush file!");

        let contents = buf.to_vec();
        let mut zip_archive = ZipArchive::new(std::io::Cursor::new(&contents[..])).expect("Could not open Native library as Zip file!");
        for i in 0..zip_archive.len() {
            let mut buf = zip_archive.by_index(i).expect("Could not extract Native library!");
            if buf.is_dir() {
                create_dir_all(extract_path.clone().join(buf.name())).expect("Could not crate directory!");
            } else if buf.is_file() {
                let mut file = File::create(extract_path.clone().join(buf.name())).expect("Could not create extracted file!");
                let mut extracted = Vec::new();
                buf.read_to_end(&mut extracted).expect("Could not read zip file!");
                file.write_all(&extracted[..]).expect("Could not write to extracted file!");
                file.flush().expect("Could not flush extracted file!");
            }
        }
    }

    // Http客户端
    let client = reqwest::blocking::ClientBuilder::new().timeout(Duration::from_millis(time_out as u64)).build().expect("Could not create a Http Client!");

    // 目录
    create_dir_all(dir.clone()).expect("Could not create directory!");
    let binding = Path::new(String::from(&dir).as_str()).join("assets");
    let assets_path = binding.as_path();
    let binding = Path::new(String::from(&dir).as_str()).join("libraries");
    let library_path = binding.as_path();
    let binding = Path::new(String::from(&dir).as_str()).join("versions").join(&source.version_id);
    let versions_path = binding.as_path();
    let binding = versions_path.clone().join(String::from(&source.version_id) + ".json");
    let json_path = binding.as_path();
    let binding = versions_path.clone().join("natives");
    let natives_path = binding.as_path();

    // 版本Json
    create_dir_all(versions_path).expect("Could not create json file!");
    let mut json_file = File::create(json_path).expect("Could not create json file!");
    // TODO: 下载源
    let json = client.get(&source.version_url).send().expect("Errors while downloading Json!").text().expect("Errors while reading Json!");
    json_file.write_all(json.as_ref()).expect("Could not write to Json!");

    // 解析Json
    let json: Value = serde_json::from_str(json.as_str()).unwrap();

    // 线程池
    let pool = rayon::ThreadPoolBuilder::new().num_threads(pool_size).build().expect("Could not create a Thread pool!");
    pool.install(|| println!("Thread pool created!"));

    // 下载
    // 游戏主文件
    let binding = versions_path.clone().join(String::from(&source.version_id) + ".jar");
    let url = &json["downloads"]["client"]["url"].as_str().unwrap();
    let file = File::create(binding.as_path()).expect("Could not create Main file!");
    let url = String::from(*url);
    let file = file.try_clone().unwrap();
    let c = client.clone();
    pool.spawn(move || {
        println!("Downloading Main file: {}", url);
        download(url.as_str(), &file, &c);
    });

    // 依赖库
    for library in json["libraries"].as_array().unwrap() {
        // 检查是否包含规则
        let mut allow = true;
        if library.get("rules").is_some() {
            allow = check_rule(library.get("rules").unwrap().as_array().unwrap());
        }

        if !allow {
            continue;
        }

        // TODO: 下载源的支持
        // 下载
        // Artifact
        if library["downloads"].get("artifact").is_some() {
            let file_path = library_path.clone().join(library["downloads"]["artifact"]["path"].as_str().unwrap());
            create_dir_all(file_path.clone().parent().unwrap()).expect("Could not create library path!");
            let url = library["downloads"]["artifact"]["url"].as_str().unwrap();
            let file = File::create(library_path.clone().join(library["downloads"]["artifact"]["path"].as_str().unwrap())).unwrap();
            let url = String::from(url);
            let file = file.try_clone().unwrap();
            let c = client.clone();
            pool.spawn(move || {
                println!("Downloading Library: {}", url);
                download(url.as_str(), &file, &c);
            });
        }
        // TODO: 多线程下载以及下载源的支持
        // Classifiers
        if library["downloads"].get("classifiers").is_some() && library.get("natives").is_some() {
            let natives = &library["natives"];
            // 系统
            let os = if OS == "macos" {
                "osx"
            } else {
                OS
            };
            if natives.get(os).is_some() {
                create_dir_all(natives_path).expect("Could not create natives directory!");
                let natives = &library["downloads"]["classifiers"][&library["natives"][os].as_str().unwrap()];
                if natives.is_null() {
                    continue
                }
                let url = natives["url"].as_str().unwrap();
                create_dir_all(library_path.clone().join(natives["path"].as_str().unwrap()).as_path().parent().unwrap()).expect("Could not create native directory!");
                let file = File::create(library_path.clone().join(natives["path"].as_str().unwrap()).as_path()).expect("Could not create native library!");
                println!("Downloading Natives: {}", url);
                download_and_extract(url, file.try_clone().unwrap(), natives_path, client.clone());
            }
        }
    }

    // 资源索引
    let asset_index = &json["assetIndex"];
    let binding = assets_path.clone().join("indexes").join(String::from(asset_index["id"].as_str().unwrap()) + ".json");
    create_dir_all(binding.parent().unwrap()).expect("Could not create directory!");
    let mut file = File::create(binding.as_path()).expect("Could not create Asset index file!");
    let url = asset_index["url"].as_str().unwrap();
    println!("Downloading Asset Index: {}", url);
    let asset_index = client.get(url).send().expect("Errors while downloading Json!").text().expect("Errors while reading Json!");
    file.write_all(asset_index.as_ref()).expect("Could not write to Asset index!");
    let asset_index: Value = serde_json::from_str(&asset_index).unwrap();

    // TODO: 下载源的支持
    // 资源
    let mut keys: Vec<&String> = asset_index["objects"].as_object().unwrap().keys().collect();
    keys.sort_by(|a, b| {
        let temp_a = asset_index["objects"][a]["size"].as_u64().unwrap();
        let temp_b = asset_index["objects"][b]["size"].as_u64().unwrap();
        temp_b.cmp(&temp_a)
    });
    for i in keys {
        let hash = asset_index["objects"][i]["hash"].as_str().unwrap();
        let hash_short = &hash[0..2];
        let temp = assets_path.clone().join(format!("objects/{}", hash_short));
        let path = temp.as_path();
        let temp = format!("{}{}/{}", ASSETS_URL, hash_short, hash);
        let url = temp.as_str();
        create_dir_all(path.clone()).expect("Could not create parent directories for Asset!");
        let file = File::create(path.join(hash)).expect("Could not create Asset file!");
        let url = String::from(url);
        let file = file.try_clone().unwrap();
        let c = client.clone();
        pool.spawn(move || {
            println!("Downloading Asset: {}", url);
            download(&url, &file, &c);
        });
    }

    pool.join(|| (), || ());

    Ok(())
}