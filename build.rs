fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure().compile(
        &[
            "proto/txn.proto",
            "proto/databases.proto",
            "proto/collections.proto",
        ],
        &["proto"],
    )?;
    Ok(())
}
