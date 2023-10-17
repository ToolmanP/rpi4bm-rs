use std::{env, process::Command};
fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let start_out = format!("{}/start.o", out_dir);
    Command::new("aarch64-unknown-linux-gnu-gcc").args(&[
        "-c",
        "src/asm/start.S",
        "-o",
        start_out.as_str(),
    ]).status().unwrap();
    println!("cargo:rustc-link-arg=-nostartfiles");
    println!("cargo:rustc-link-arg={}", start_out);
    println!("cargo:rustc-link-arg=-Tsrc/ld/linker.ld");
}
