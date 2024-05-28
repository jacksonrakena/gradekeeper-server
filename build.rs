use std::process::Command;

fn run(cmd: &mut Command) -> String {
   cmd.output().map_or("".to_string(), |d| String::from_utf8_lossy(&d.stdout).parse().unwrap())
}
fn main() {
    println!("cargo:rustc-env=GK_SERVER_COMMIT_MSG={}", run(Command::new("git").arg("show").arg("-s").arg("--format=%s")));
    println!("cargo:rustc-env=GK_SERVER_COMMIT_HASH={}", run(Command::new("git").arg("rev-parse").arg("--short").arg("HEAD")));
    println!("cargo:rustc-env=GK_SERVER_BRANCH={}", run(Command::new("git").arg("rev-parse").arg("--abbrev-ref").arg("HEAD")));
    println!("cargo:rustc-env=GK_SERVER_COMMITTER={}", run(Command::new("git").arg("show").arg("-s").arg("--format=%an")));
}