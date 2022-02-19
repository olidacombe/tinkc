fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure().build_server(false).compile(
        &["proto/workflow.proto", "proto/hardware.proto"],
        &["proto"],
    )?;
    Ok(())
}
