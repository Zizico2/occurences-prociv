use async_tempfile::TempDir;
use protox::prost::Message;
use std::{env, fs, path::PathBuf};

const PROTO_FILES: [&str; 1] = ["occurrence/v1/occurrences_service.proto"];

//INFO: change this to update version
// const BUF_SCHEMA: &str = "buf.build/zizico2/prociv-reverse-proxy:228c470f729042b69cf6b9360e2bad4b";
const BUF_SCHEMA_TAR_GZ: &str =
    "https://buf.build/zizico2/prociv-reverse-proxy/archive/228c470f729042b69cf6b9360e2bad4b.zip";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let bytes = reqwest::get(BUF_SCHEMA_TAR_GZ).await?.bytes().await?;

    let temp_dir = TempDir::new().await?;
    let temp_dir_path = temp_dir.dir_path();
    let temp_dir_path_str = temp_dir.dir_path().to_str().unwrap();

    zip_extract::extract(std::io::Cursor::new(bytes), &temp_dir_path, true)?;

    let proto_includes: [&str; 1] = [temp_dir_path_str];

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
