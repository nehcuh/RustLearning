fn main() {
    let out_dir = "src/pb";
    if !std::path::Path::new(out_dir).exists() {
        std::fs::create_dir_all(out_dir).expect(&format!("Failed to create dir: {}", out_dir));
    }

    prost_build::Config::new()
        .bytes(&["."])
        .type_attribute(".", "#[derive(PartialOrd)]")
        .out_dir(out_dir)
        .compile_protos(&["abi.proto"], &["."])
        .expect("Failed to build proto file");
}
