use cc;
use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    rust_build_test();
    // c_build_test();
    c_build_test_v2();
    link_sys_lib();
}

fn link_sys_lib() {
    pkg_config::Config::new().probe("zlib").unwrap();
    println!("cargo: rerun-if-changed=build.rs");
}

fn c_build_test_v2() {
    cc::Build::new().file("src/hello.c").compile("hello");
    println!("cargo: return-if-changed=src/hello.c");
}

fn c_build_test() {
    let out_dir = env::var("OUT_DIR").unwrap();

    Command::new("gcc")
        .args(&["src/hello.c", "-c", "-fPIC", "-o"])
        .arg(&format!("{}/hello.o", out_dir))
        .status()
        .unwrap();

    Command::new("ar")
        .args(&["crus", "libhello.a", "hello.o"])
        .current_dir(&Path::new(&out_dir))
        .status()
        .unwrap();

    println!("cargo: rustc-link-search=native={}", out_dir);
    println!("cargo: rustc-link-lib=static=hello");
    println!("cargo: rerun-if-changed=src/hello.c");
}

fn rust_build_test() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dst_path = Path::new(&out_dir).join("hello.rs");

    fs::write(
        &dst_path,
        "pub fn message() -> &'static str  {\"Hello, World from Rust \"}",
    )
    .unwrap();
    println!("cargo: rerun-if-changed=build.rs");
}
