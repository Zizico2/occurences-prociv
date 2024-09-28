use protox::prost::Message;
use std::{env, fs, path::PathBuf, process::Command};

const PROTO_DOWNLOAD: &str = "___protos";
const PROTO_INCLUDES: [&str; 1] = [PROTO_DOWNLOAD];
const PROTO_FILES: [&str; 1] = ["occurrence/v1/occurrences_service.proto"];

const BUF_SCHEMA: &str = "buf.build/zizico2/prociv-reverse-proxy:228c470f729042b69cf6b9360e2bad4b";

fn main() -> anyhow::Result<()> {
    Command::new("buf")
        .args(["export", BUF_SCHEMA])
        .args(["--output", PROTO_DOWNLOAD])
        .output()?;

    let file_descriptors = protox::compile(&PROTO_FILES, &PROTO_INCLUDES)?;

    let file_descriptor_path =
        PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("file_descriptor_set.bin");
    fs::write(&file_descriptor_path, file_descriptors.encode_to_vec())?;

    tonic_build::configure()
        .build_server(true)
        .build_client(false)
        .file_descriptor_set_path(&file_descriptor_path)
        .skip_protoc_run()
        .compile_protos(&PROTO_FILES, &PROTO_INCLUDES)?;

    Ok(())
}
