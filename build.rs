fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        // don’t generate any server stubs
        .build_server(false)
        // ask tonic to emit a single `mod.rs` in OUT_DIR
        .include_file("mod.rs")
        .compile(
            // point at the one .proto that pulls in everything
            &["protos/starlink_protos/spacex/api/device/device.proto"],
            // tell it where to look for imports
            &["protos/starlink_protos"],
        )?;
    Ok(())
}
