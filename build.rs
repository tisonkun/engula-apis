fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure().compile(
        &[
            "engula/v1/engula.proto",
            "engula/v1alpha/txn.proto",
            "engula/v1alpha/universe.proto",
        ],
        &["."],
    )?;
    Ok(())
}
