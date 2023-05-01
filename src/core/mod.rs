pub mod install;
pub mod launcher;
pub mod network;
pub mod util;

pub struct VersionSource {
    pub version_id: String,
    pub version_url: String,
    pub version_type: String,
    pub release_time: String
}