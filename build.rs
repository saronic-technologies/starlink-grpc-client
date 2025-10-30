use std::path::PathBuf;
use prost_wkt_build::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Only regenerate when the user explicitly enables the feature:
    if std::env::var("CARGO_FEATURE_BUILD_PROTOS").is_ok() {
        // 1) Fetch the path to the vendored protoc binary
        let protoc_path = protoc_bin_vendored::protoc_bin_path()
            .map_err(|e| format!("could not find vendored protoc: {}", e))?;
        // 2) Export it so prost-build/tonic-build pick it up
        std::env::set_var("PROTOC", protoc_path);
        println!("cargo:rerun-if-changed=build.rs");
        println!("cargo:rerun-if-changed=protos/starlink_protos");
        println!("cargo:rerun-if-changed=proto_bindings");

        // Set up paths for descriptor file
        let out = PathBuf::from("proto_bindings");
        let descriptor_file = out.join("descriptors.bin");

        // Configure prost with serde, schemars and extern paths for well-known types
        let mut prost_config = prost_build::Config::new();
        prost_config
            .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)]")
            .extern_path(".google.protobuf.Timestamp", "::prost_wkt_types::Timestamp")
            .file_descriptor_set_path(&descriptor_file);

        tonic_build::configure()
            // don't generate any server stubs
            .build_server(false)
            // ask tonic to emit a single `mod.rs` in OUT_DIR
            .out_dir("proto_bindings")
            .include_file("mod.rs")
            .compile_protos_with_config(
                prost_config,
                // point at the one .proto that pulls in everything
                &["protos/starlink_protos/spacex/api/device/device.proto"],
                // tell it where to look for imports
                &["protos/starlink_protos"],
            )?;

        // Read and process the descriptor file to add serde impls for well-known types
        let descriptor_bytes = std::fs::read(&descriptor_file)?;
        let descriptor = FileDescriptorSet::decode(&descriptor_bytes[..])?;
        prost_wkt_build::add_serde(out, descriptor);
    }
    Ok(())
}
