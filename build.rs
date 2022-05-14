extern crate bindgen;
extern crate cc;

use std::env;
use std::path::PathBuf;

fn main() -> miette::Result<()> {
    let lldb_lib = "lldb-13";
    let llvm_dir = "/usr/lib/llvm-13";
    let p = std::path::PathBuf::from(format!("{llvm_dir}/include/"));
    let s1 = std::path::PathBuf::from("/usr/include/c++/7/");
    let s2 = std::path::PathBuf::from("/usr/include/x86_64-linux-gnu/c++/7/");

    let path = std::path::PathBuf::from("src"); // include path
    let mut b = autocxx_build::Builder::new("src/api.rs", &[&path, &p, &s1, &s2]).build()?;
    b.flag_if_supported("-std=c++14")
        .includes(&[&p, &path, &s1, &s2])
        .compile("autocxx-lldb"); // arbitrary library name, pick anything
    println!("cargo:rerun-if-changed=src/api.rs");
    // Add instructions to link to any C++ libraries you need.
    Ok(())
}
/*
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
        // Specify we're only interested in SB* things.
        .allowlist_type("lldb::SB.*")
        .allowlist_type("std::shared_ptr")
        .allowlist_type("std::unique_ptr")
        .allowlist_type("std::weak_ptr")
        .allowlist_type("simple_shared_ptr")
        .allowlist_type("simple_unique_ptr")
        .allowlist_type("simple_weak_ptr")
        // Convert size_t's into usize, it's the same for x86_64.
        .size_t_is_usize(true)
        // .disable_name_namespacing()
        .enable_cxx_namespaces()
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
*/