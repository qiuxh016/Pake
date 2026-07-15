fn main() {
    println!("cargo:rerun-if-changed=.pake/pake.json");
    println!("cargo:rerun-if-changed=.pake/tauri.conf.json");
    tauri_build::build();
    built::write_built_file().expect("Failed to write built.rs");
}
