fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .compile_protos(
            &["proto/identity/users/v1/users.proto"],
            &["proto"],
        )?;

    // Re-run whenever any file under proto/ changes.
    println!("cargo:rerun-if-changed=proto");

    Ok(())
}
