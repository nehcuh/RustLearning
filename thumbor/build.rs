use std::{fs, path::Path, process::Command};

fn main() {
    // 检查是否安装了 protoc
    if !is_protoc_installed() {
        println!("protoc not found. Attempting to install using hombrew...");
        install_protobuf();
    }

    //  检查输出目录是否存在，不存在则创建输出目录
    let out_dir = Path::new("src").join("pb");
    if !out_dir.exists() {
        fs::create_dir_all(&out_dir).expect("Failed to create src/pb directory");
        println!("cargo::warning=Created directory: {:?}", out_dir);
    } else {
        println!("cargo::warning=Directory alread exists: {:?}", out_dir);
    }

    prost_build::Config::new()
        .out_dir(&out_dir)
        .compile_protos(&["abi.proto"], &["."])
        .unwrap()
}

fn is_protoc_installed() -> bool {
    Command::new("protoc").arg("--version").output().is_ok()
}

fn install_protobuf() {
    let output = Command::new("brew")
        .args(&["install", "protobuf"])
        .output()
        .expect("Failed to execute brew command");

    if !output.status.success() {
        panic!(
            "Failed to install protobuf: {}",
            String::from_utf8_lossy(&output.stderr)
        )
    }
}
