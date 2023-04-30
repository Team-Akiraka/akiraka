extern crate windres;

use windres::Build;

fn main() {
    // https://learn.microsoft.com/zh-cn/windows/apps/design/style/iconography/app-icon-construction
    Build::new().compile("assets/resources.rc").unwrap();
}