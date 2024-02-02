use clap::{Command};

// -- include generated git_commit.rs file
include!(concat!(env!("OUT_DIR"), "/git_commit.rs"));

fn main() {
    let app = Command::new("Version")
        .version(GIT_COMMIT) // Use the git commit hash as the version
        .about("Testing outputting the git-commit via a build.rs script.");

    let _matches = app.get_matches();

    println!("Hello, world!");
}
