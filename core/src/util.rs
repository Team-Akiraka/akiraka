use std::env;

pub fn current_dir() -> String {
    return match env::current_dir() {
        Ok(path) => {
            path.to_str().unwrap().to_string()
        },
        Err(e) => {
            panic!("Failed to get current path: {}", e);
        }
    };
}