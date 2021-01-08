use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

// Get current git commit for HEAD, suffixed with a + if there are uncommitted
// changes
fn get_git_commit() -> String {
    let commit = Command::new("git")
        .arg("rev-parse")
        .arg("--verify")
        .arg("HEAD")
        .output()
        .expect("Could not get git commit");
    let commit_string = String::from_utf8_lossy(&commit.stdout);
    let commit_str = commit_string.lines().next().unwrap_or("?");

    let status = Command::new("git")
        .arg("status")
        .arg("--porcelain")
        .output()
        .expect("Could not get git status")
        .stdout;

    // If there are uncommitted changes
    if status.len() > 0 {
        format!("{}+", commit_str)
    } else {
        commit_str.to_string()
    }
}

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("version.rs");
    fs::write(
        &dest_path,
        format!(r#"#[allow(dead_code)]
const VERSION: &str = "git-{}";
"#,
            get_git_commit())
    ).unwrap();
}
