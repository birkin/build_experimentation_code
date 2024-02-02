use std::process::Command;
use std::env;
use std::path::Path;
use std::fs::File;
use std::io::Write;

fn main() {
    // Get the current git commit hash
    let output = Command::new("git")
        .args(&["rev-parse", "HEAD"])
        .output()
        .expect("Failed to execute git command");

    let git_hash = String::from_utf8_lossy(&output.stdout).trim().to_string();

    // Write the git hash to a file that can be included in the main binary
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("git_commit.rs");
    let mut f = File::create(&dest_path).unwrap();

    // Write the git commit hash to the file
    write!(f, "pub const GIT_COMMIT: &str = \"{}\";", git_hash).unwrap();
}
