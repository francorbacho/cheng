// https://stackoverflow.com/a/44407625
use std::process::Command;

fn main() {
    let git_hash = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .unwrap()
        .stdout;
    let git_hash = String::from_utf8(git_hash).unwrap();

    // https://stackoverflow.com/a/2659808
    let git_dirty = if Command::new("git")
        .args(["diff-index", "--quiet", "HEAD"])
        .status()
        .unwrap()
        .success()
    {
        "clean"
    } else {
        "dirty"
    };

    let date = Command::new("date").arg("-R").output().unwrap().stdout;
    let date = String::from_utf8(date).unwrap();

    println!("cargo:rustc-env=GIT_HASH={git_hash}");
    println!("cargo:rustc-env=GIT_DIRTY={git_dirty}");
    println!("cargo:rustc-env=DATE={date}");
}
