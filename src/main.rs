use clap::{Command};

fn main() {

    let app = Command::new("Version")
    .version("`foo`") // Sets the version string that is displayed with the --version flag
    .about("Testing outputting the git-commit via a build.rs script.");

    let _matches = app.get_matches();

    println!("Hello, world!");
}
