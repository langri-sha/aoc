use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    let source_file = Path::new("share/input.txt");
    let dest_file = Path::new(&out_dir).join("share/input.txt");

    println!("cargo:rerun-if-changed={}", source_file.to_str().unwrap());
    fs::create_dir_all(dest_file.parent().unwrap()).expect("Failed to create directory");
    fs::copy(source_file, dest_file).expect("Failed to copy file");
}
