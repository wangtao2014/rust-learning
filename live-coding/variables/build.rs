use std::{env, path::Path, fs};

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("hello.rs");
    println!("dest_path: {}", dest_path.display());

    fs::write(&dest_path, "pub fn message() -> &'static str {
        \"Hello, World!\" 
    }
    "
    ).unwrap();

    println!("cargo:rerun-if-changed=build.rs");
}