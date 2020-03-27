use std::env;
use std::fs;
use std::process::{Command, Stdio};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    for entry in fs::read_dir("samples").expect("samples/ dir exists") {
        let entry = entry.expect("valid entry");
        let path = entry.path();
        let path = path.to_str().expect("valid Unicode");
        println!("cargo:rerun-if-changed={}", path);
        // Compile to native and WASI
        let out_dir = env::var("OUT_DIR").expect("OUT_DIR is defined");
        let mut cmd = Command::new("rustc");
        cmd.args(&["-O", "--out-dir", &out_dir, &path])
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit());
        match cmd.status() {
            Ok(_) => {}
            Err(e) => panic!("rustc (native) failed with error: {}", e),
        }
        let mut cmd = Command::new("rustc");
        cmd.args(&[
            "--target",
            "wasm32-wasi",
            "-O",
            "--out-dir",
            &out_dir,
            &path,
        ])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());
        match cmd.status() {
            Ok(_) => {}
            Err(e) => panic!("rustc (wasi) failed with error: {}", e),
        }
    }
}
