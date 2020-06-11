use std::process::Command;
use std::env;
// use std::path::Path;

#[cfg(feature = "adx_backend")]
static BUILD_SCRIPT_NAME: &'static str = "make_adx.sh";

#[cfg(feature = "asm_backend")]
static BUILD_SCRIPT_NAME: &'static str = "make_asm.sh";

#[cfg(all(not(feature = "adx_backend"), not(feature = "asm_backend")))]
static BUILD_SCRIPT_NAME: &'static str = "make.sh";

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("{}", format!("cargo:rerun-if-changed={}", BUILD_SCRIPT_NAME));

    let out_dir = env::var("OUT_DIR").unwrap();
    let arg = format!("./{}", BUILD_SCRIPT_NAME);
    Command::new("sh").args(&[arg]).status().unwrap();

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=eip2537");

    Command::new("touch").args(&["build.rs"]).status().unwrap();
    Command::new("touch").args(&[BUILD_SCRIPT_NAME]).status().unwrap();
}