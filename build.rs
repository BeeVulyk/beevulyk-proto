fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_prost_build::configure()
        .build_server(true)
        .build_client(true)
        .compile_protos(
            &[
                "proto/beekeeping/reference/v1/reference.proto",
                "proto/common/geo/v1/geo.proto",
                "proto/common/money/v1/money.proto",
                "proto/identity/users/v1/users.proto",
                "proto/identity/profiles/v1/profiles.proto",
                "proto/marketplace/reference/v1/reference.proto",
                "proto/marketplace/listings/v1/listings.proto",
            ],
            &["proto"],
        )?;

    // Re-run whenever any file under proto/ changes.
    println!("cargo:rerun-if-changed=proto");

    Ok(())
}
