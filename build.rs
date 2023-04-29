extern crate windres;

use windres::Build;

fn main() {
    Build::new().compile("assets/resources.rc").unwrap();
}