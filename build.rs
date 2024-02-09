use std::process::Command;

const OPCODES: &str = "res/opcodes";

/// Build script:
///   - Create src/encoding.rs
///
fn main() {
    // Only rerun if opcode submodule changes
    println!("cargo:rerun-if-changed={OPCODES}/*");
    // Install python dependencies
    Command::new("pip3")
        .current_dir(OPCODES)
        .arg("install")
        .arg("-r")
        .arg("requirements.txt")
        .status().expect("Failed to install python dependencies");
    // Create inst.rs
    Command::new("python3")
        .current_dir(OPCODES)
        .arg("parse.py")
        .arg("-rust")
        .arg("rv*")
        .status().expect("Failed to create");
    // Copy result to src/encoding.rs
    Command::new("cp")
        .arg(format!("{OPCODES}/inst.rs"))
        .arg("src/encoding.rs")
        .status().expect("Failed to copy");
}
