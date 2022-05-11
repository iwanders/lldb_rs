extern crate bindgen;
extern crate cc;

use std::env;
use std::path::PathBuf;

fn main() {
    let lldb_lib = "lldb-13";
    let llvm_dir = "/usr/lib/llvm-13";

    println!("cargo:rustc-link-lib={lldb_lib}");
    println!("cargo:rustc-link-search=/usr/lib/x86_64-linux-gnu");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/lldb_api.h");


    // Actually lives in:
    // /usr/lib/llvm-13/include/lldb/API/LLDB.h
    let bindings = bindgen::Builder::default()
        .header("src/lldb_api.h")
        // Why do we need to specify the std libs manually?
        .clang_arg("-std=c++14")
        .clang_arg("-I/usr/include/c++/7/")
        .clang_arg("-I/usr/include/x86_64-linux-gnu/c++/7/")
        // Location where the lldb api lives.
        .clang_arg(format!("-I{llvm_dir}/include/"))
        .clang_arg("-xc++")
        .opaque_type("lldb::SB*")
        .opaque_type("lldb::SBDebugger")
        // Specify we're only interested in SB* things.
        .allowlist_type("lldb::SB.*")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
