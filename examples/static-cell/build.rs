use std::{env, path::PathBuf};

fn main() {
    println!("cargo:rustc-link-arg=--nmagic");
    println!("cargo:rustc-link-arg=-Tlink.x");
    println!("cargo:rustc-link-arg=-Tdefmt.x");

    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let crate_name = env!("CARGO_PKG_NAME");

    let mut map_file_path = PathBuf::from(&manifest_dir);
    map_file_path.push(crate_name);
    println!("cargo:rustc-link-arg=-Map={}.map", map_file_path.display());

    let linker_search_path = PathBuf::from(&manifest_dir);
    println!("cargo:rustc-link-search={}", linker_search_path.display());
}
