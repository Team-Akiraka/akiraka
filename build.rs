#[cfg(target_os="windows")]
extern crate windres;

#[cfg(target_os="windows")]
use windres::Build;

fn main() {
    // https://learn.microsoft.com/zh-cn/windows/apps/design/style/iconography/app-icon-construction
    #[cfg(target_os="windows")]
    Build::new().compile("assets/resources.rc").unwrap();
}