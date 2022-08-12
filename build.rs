use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
};

extern crate tonic_build;

fn main() {
    let reflection_descriptor = PathBuf::from(env::var("OUT_DIR").unwrap())
        .join("mayastor_reflection.bin");
    tonic_build::configure()
        .file_descriptor_set_path(&reflection_descriptor)
        .build_server(true)
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile(&["protobuf/mayastor.proto"], &["protobuf"])
        .unwrap_or_else(|e| {
            panic!("mayastor protobuf compilation failed: {}", e)
        });

    tonic_build::configure()
        .file_descriptor_set_path(&reflection_descriptor)
        .build_server(true)
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile(
            &[
                "protobuf/v1/bdev.proto",
                "protobuf/v1/json.proto",
                "protobuf/v1/pool.proto",
                "protobuf/v1/replica.proto",
                "protobuf/v1/host.proto",
                "protobuf/v1/nexus.proto",
                "protobuf/v1/registration.proto",
            ],
            &["protobuf/v1"],
        )
        .unwrap_or_else(|e| {
            panic!("mayastor v1 protobuf compilation failed: {}", e)
        });
}
