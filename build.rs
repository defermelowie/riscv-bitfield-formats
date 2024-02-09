use std::process::Command;

const OPCODES: &str = "res/opcodes";

/// Build script:
///   - Create inst.rs & copy to src/encoding.rs
///
fn main() {
    // Only rerun if opcode submodule changes
    println!("cargo:rerun-if-changed={OPCODES}");
    // Create inst.rs
    Command::new("python3")
        .current_dir(OPCODES)
        .arg("parse.py")
        .arg("-rust")
        .arg("rv*")
        .status().unwrap();
    // Copy result to src/encoding.rs
    Command::new("cp")
        .arg(format!("{OPCODES}/inst.rs"))
        .arg("src/encoding.rs")
        .status().unwrap();
}
