fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=include/rust-titan.h");
    println!("cargo:rerun-if-changed=src/rust-titan.cc");
    println!("cargo:rustc-link-arg=-lcxxbridge1");

    // FIXME: Use linklib() from top level build.rs
    println!("cargo:rustc-link-arg=-lstdc++");
    println!("cargo:rerun-if-changed=src/rust-titan.cc");

    println!("cargo:rustc-link-lib=static=rust-titan");

    cxx_build::bridge("src/lib.rs")
        .file("src/rust-titan.cc")
        .flag("-DCXXASYNC_HAVE_COROUTINE_HEADER")
        .flag("-fcoroutines")
        .flag("-std=c++20")
        .flag_if_supported("-Wall")
        .include("include")
        .include("../../rocksdb/include")
        .include("../../crocksdb")
        .compile("rust-titan");
}
