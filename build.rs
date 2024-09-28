use protox::prost::Message;
use tempfile::tempdir;
use std::{env, fs, path::PathBuf, process::Command};


const PROTO_FILES: [&str; 1] = ["occurrence/v1/occurrences_service.proto"];

//INFO: change this to update version
const BUF_SCHEMA: &str = "buf.build/zizico2/prociv-reverse-proxy:228c470f729042b69cf6b9360e2bad4b";

fn main() -> anyhow::Result<()> {
    let mut proto_download = tempdir()?.into_path();
    proto_download.push("___protos");
    let proto_download = proto_download.to_str().unwrap();

    let proto_includes: [&str; 1] = [proto_download];

    Command::new("buf")
        .args(["export", BUF_SCHEMA])
        .args(["--output", proto_download])
        .output()?;

    let file_descriptors = protox::compile(&PROTO_FILES, &proto_includes)?;

    let file_descriptor_path =
        PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("file_descriptor_set.bin");
    fs::write(&file_descriptor_path, file_descriptors.encode_to_vec())?;

    tonic_build::configure()
        .build_server(true)
        .build_client(false)
        .file_descriptor_set_path(&file_descriptor_path)
        .skip_protoc_run()
        .compile_protos(&PROTO_FILES, &proto_includes)?;

    Ok(())
}
