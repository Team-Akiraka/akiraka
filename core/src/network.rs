use reqwest;
use serde_json::Value;
use crate::core::VersionSource;

const URL_VERSION_MANIFEST: &str = "https://piston-meta.mojang.com/mc/game/version_manifest.json";

pub fn get_version_sources(snapshot: bool, release: bool, old_beta: bool, old_alpha: bool) -> Result<Vec<VersionSource>, std::io::Error> {
    let mut result = Vec::new();
    let get: Value = serde_json::from_str(&*reqwest::blocking::get(URL_VERSION_MANIFEST).unwrap().text().unwrap()).unwrap();
    for i in get["versions"].as_array().unwrap() {
        let version_type = i["type"].as_str().unwrap();
        if version_type == "snapshot" && snapshot ||
            version_type == "release" && release ||
            version_type == "old_beta" && old_beta ||
            version_type == "old_alpha" && old_alpha {
            result.push(VersionSource {
                version_id: i["id"].as_str().unwrap().parse().unwrap(),
                version_url: i["url"].as_str().unwrap().parse().unwrap(),
                version_type: i["type"].as_str().unwrap().parse().unwrap(),
                release_time: i["releaseTime"].as_str().unwrap().parse().unwrap(),
            });
        }
    }

    Ok(result)
}