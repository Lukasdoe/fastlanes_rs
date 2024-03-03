use std::path::{Path, PathBuf};
use std::process::Command;
use std::{env, fs};

const FASTLANES_REPO: &str = "https://github.com/Lukasdoe/FastLanes";
const FASTLANES_BRANCH: &str = "rust-pkg";
const FASTLANES_PATH: &str = "thirdparty/FastLanes";
const FASTLANES_HEADER_PATH1: &str = "fls_generated/include/fls_gen/rsum/rsum.hpp";
const FASTLANES_HEADER_PATH2: &str = "fls_generated/include/fls_gen/untranspose/untranspose.hpp";

fn main() {
    if !Path::new(FASTLANES_PATH).exists() {
        fs::create_dir_all(FASTLANES_PATH).unwrap();
        let mut cmd = Command::new("git");
        cmd.arg("clone")
            .args(["--branch", FASTLANES_BRANCH])
            .arg(FASTLANES_REPO)
            .arg(FASTLANES_PATH);
        if !cmd.status().unwrap().success() {
            fs::remove_dir(FASTLANES_PATH).unwrap();
            panic!("Failed to clone fastlanes!");
        }
    }

    let mut cmake_config = cmake::Config::new(FASTLANES_PATH);
    cmake_config.define("CMAKE_C_COMPILER", "/usr/bin/clang");
    cmake_config.define("CMAKE_CXX_COMPILER", "/usr/bin/clang++");
    cmake_config.build_target("rust-bundle");
    let lib_path = cmake_config.build();

    let lib_dir = lib_path.join("build").to_str().unwrap().to_string();
    println!("{}", lib_dir);

    // // add lib to rustc search path
    println!("cargo:rustc-link-search={}", lib_dir);
    // // link with fastlanes lib
    println!("cargo:rustc-link-lib=static=rust-bundle");

    let bindings = bindgen::Builder::default()
        .header(
            Path::new(FASTLANES_PATH)
                .join(FASTLANES_HEADER_PATH1)
                .to_str()
                .unwrap(),
        )
        .header(
            Path::new(FASTLANES_PATH)
                .join(FASTLANES_HEADER_PATH2)
                .to_str()
                .unwrap(),
        )
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate fastlanes bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
