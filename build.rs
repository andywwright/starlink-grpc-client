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
        tonic_build::configure()
            // don’t generate any server stubs
            .build_server(false)
            // ask tonic to emit a single `mod.rs` in OUT_DIR
            .out_dir("proto_bindings")
            .include_file("mod.rs")
            .compile_protos(
                // point at the one .proto that pulls in everything
                &["protos/starlink_protos/spacex/api/device/device.proto"],
                // tell it where to look for imports
                &["protos/starlink_protos"],
            )?;
    }
    Ok(())
}
