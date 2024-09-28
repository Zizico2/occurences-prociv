use protox::prost::Message;
use std::{env, fs, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=migrations");
    let file_descriptors = protox::compile(["occurrence/v1/occurrences_service.proto"], ["./protos"]).unwrap();

    let file_descriptor_path = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR not set"))
        .join("file_descriptor_set.bin");
    fs::write(&file_descriptor_path, file_descriptors.encode_to_vec()).unwrap();

    tonic_build::configure()
        .build_server(true)
        .build_client(false)
        .file_descriptor_set_path(&file_descriptor_path)
        .skip_protoc_run()
        .compile_protos(&["occurrence/v1/occurrences_service.proto"], &["./protos"])
        .unwrap();
}
