fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure().compile(&["proto/txn.proto", "proto/universe.proto"], &["proto"])?;
    Ok(())
}
