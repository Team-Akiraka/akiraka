use std::path::Path;

pub fn launch(name: &str, dir: &Path) -> Result<(), std::io::Error> {
    // 路径
    let versions_dir = dir.clone().join("versions").join(name).as_path();
    if !versions_dir.exists() || !versions_dir.is_dir() {
        Err("Version is not available!")
    }
    let library_dir = dir.clone().join("libraries").as_path();
    Ok(())
}