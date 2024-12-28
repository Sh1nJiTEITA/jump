use std::env;
use std::fs;
use std::path::PathBuf;

fn set_script_permission(script_path: &PathBuf) {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&script_path).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&script_path, perms).unwrap();
    }
}

fn copy_to_install_path(script_path: &PathBuf) {
    let cargo_bin_dir: PathBuf = env::var_os("CARGO_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("~/.cargo"))
        .join("bin");
    let destination = cargo_bin_dir.join("jmp");
    fs::copy(&script_path, &destination).expect("Failed to copy script to Cargo bin directory");
    set_script_permission(&destination);
    println!(
        "Script copied to {} and made executable.",
        destination.display()
    );
}

fn copy_to_build_path(script_path: &PathBuf) {
    let out_dir = env::var_os("OUT_DIR").expect("OUT_DIR environment variable is not set");
    let out_dir = PathBuf::from(out_dir);
    let executable_dir = out_dir
        .ancestors()
        .nth(3)
        .expect("Failed to determine executable directory");
    let destination = executable_dir.join("jmp");
    fs::copy(&script_path, &destination).expect("Failed to copy script to build directory");
    set_script_permission(&destination);
}

fn main() {
    let script_path = PathBuf::from("scripts/jmp");
    if !script_path.exists() {
        panic!("Script file {} does not exist.", script_path.display());
    }

    copy_to_install_path(&script_path);
    copy_to_build_path(&script_path);
}
