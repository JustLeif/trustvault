fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manifest_dir = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?);
    let workspace_root = manifest_dir
        .parent()
        .and_then(|path| path.parent())
        .expect("host crate should be under apps/host");
    let proto_dir = workspace_root.join("proto");
    let proto_file = proto_dir.join("trustvault.proto");

    println!("cargo:rerun-if-changed={}", proto_file.display());

    tonic_prost_build::configure().compile_protos(&[proto_file], &[proto_dir])?;

    Ok(())
}
