fn main() -> miette::Result<()> {
    let lldb_lib = "lldb-13";
    let llvm_dir = "/usr/lib/llvm-13";

    let lldb_lib_dir = format!("{llvm_dir}/lib/");
    let lldb_include_dir = format!("{llvm_dir}/include/");

    // System header libraries
    let s1 = std::path::PathBuf::from("/usr/include/c++/7/");
    let s2 = std::path::PathBuf::from("/usr/include/x86_64-linux-gnu/c++/7/");

    // Tell rustc to link against lldb
    println!("cargo:rustc-link-lib={lldb_lib}");
    println!("cargo:rustc-link-search={lldb_lib_dir}");

    // When to rerun the whole generation.
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/lldb_api.h");
    println!("cargo:rerun-if-changed=src/api.rs");

    // This is super fragile, running into order of linking arguments.
    // found https://github.com/rust-lang/cc-rs/issues/672#issue-1194329134
    // rescan doesn't work, but this argument gets tacked on at the end... so we can totally
    // misuse that.
    println!("cargo:rustc-link-arg=-l{lldb_lib}");

    let include_p = std::path::PathBuf::from(&lldb_include_dir);

    let path = std::path::PathBuf::from("src"); // include path
    let mut b = autocxx_build::Builder::new("src/api.rs", &[&path, &include_p, &s1, &s2])
        .extra_clang_args(&[&format!("-L{lldb_lib_dir}"), &format!("-l{lldb_lib}")])
        .build()?;

    b.flag_if_supported("-std=c++14")
        .includes(&[&path, &include_p, &s1, &s2])
        .compile("autocxx-lldb.so");
    Ok(())
}