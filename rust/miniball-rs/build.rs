fn main() {
    cxx_build::bridge("src/lib.rs")
        .file("src/miniball_rs.cpp")
        .include("src")
        .include("../../cpp/main")
        .std("c++17")
        .compile("miniball_rs");

    println!("cargo:rerun-if-changed=src/miniball_rs.cpp");
    println!("cargo:rerun-if-changed=src/miniball_rs.h");
}
