#[cfg(target_os="windows")]
extern crate winres;

#[cfg(target_os="windows")]
fn main() {
    // https://learn.microsoft.com/zh-cn/windows/apps/design/style/iconography/app-icon-construction
    let mut res = winres::WindowsResource::new();
    res.set_icon("icon/icon_64.ico");
    res.set_language(0x0000);
    res.compile().unwrap();
    // Build::new().compile("icon/resources.rc").unwrap();
}