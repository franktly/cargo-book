use std::os::raw::{c_uint, c_ulong};
include!(concat!(env!("OUT_DIR"), "/hello.rs"));

extern "C" {
    fn hello();}

extern "C" {
    fn crc32(crc: c_ulong, buf: *const u8, len: c_uint) -> c_ulong;
}

fn main() {
    println!("Rust Building Scripts....");
    println!("{}", message());

    println!("Rust AND C Building Scripts....");
    unsafe {
        hello();
    }

    println!("Rust System Library Link...");
    let s = "hello";
    unsafe {
        println!(
            "crc32 result is {:#x} ",
            crc32(0, s.as_ptr(), s.len() as c_uint)
        );
    }
}
