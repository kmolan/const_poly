use std::process::Command;

fn main() {
    run_cargo_fmt();
}

//TODO: only do this for this crate's contents?
fn run_cargo_fmt() {
    let status = Command::new("cargo")
        .args(&["fmt"])
        .status()
        .expect("Failed to run cargo fmt");

    if !status.success() {
        panic!("cargo fmt failed with status: {:?}", status);
    }
}
