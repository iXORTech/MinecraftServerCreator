use std::process::Command;
fn main() {
    // Get Git
    let git_short_hash = String::from_utf8(
        Command::new("git").args(&["rev-parse", "--short", "HEAD"])
            .output().unwrap().stdout
    ).unwrap().to_uppercase();
    println!("cargo:rustc-env=GIT_SHORT_HASH={}", git_short_hash);
}
