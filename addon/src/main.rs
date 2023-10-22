use std::{
    env,
    fs::{self, create_dir_all},
    path::{Path, PathBuf},
    process::Command,
};

pub fn main() {
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let status = Command::new(cargo)
        .current_dir(project_root())
        .args(&["build", "--release"])
        .status()
        .expect("build failed");
    assert!(status.success());

    let library_file = project_root()
        .join("target")
        .join("release")
        .join("libnbody.dylib");
    let blender_location = env::var("BLENDER_PATH").expect("you didn't set blender path");
    let blender_location = Path::new(&blender_location);
    if !blender_location.exists() {
        create_dir_all(blender_location).expect("create failed");
    }
    let installed_location = blender_location.join("nbody.so");

    fs::copy(&library_file, &installed_location).expect("copy failed");
}

fn project_root() -> PathBuf {
    Path::new(&env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(1)
        .unwrap()
        .to_path_buf()
}
