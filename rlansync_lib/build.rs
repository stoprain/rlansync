use protobuf_codegen::Codegen;
use std::path::PathBuf;

fn main() {
    Codegen::new()
    .pure()
    .cargo_out_dir("generated_with_pure")
    .input("src/protos/example.proto")
    .include("src/protos")
    .run_from_script();

    let out_dir = PathBuf::from("./generated");

    let bridges = vec!["src/lib.rs"];
    for path in &bridges {
        println!("cargo:rerun-if-changed={}", path);
    }

    swift_bridge_build::parse_bridges(bridges)
        .write_all_concatenated(out_dir, env!("CARGO_PKG_NAME"));
}