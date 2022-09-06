use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../proto/consumer.proto"),
    )?;

    Ok(())
}
