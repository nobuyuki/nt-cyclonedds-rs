fn gen_git_hash() {
    use std::process::Command;

    let output = Command::new("git")
        .args(&["describe", "--abbrev=4", "--dirty", "--always", "--tag"])
        .output()
        .unwrap();
    let git_hash = String::from_utf8(output.stdout).unwrap();
    println!("cargo:rustc-env=GIT_HASH={}", git_hash);
}

fn main() {
    gen_git_hash();
}
