//! Build script for Bucket module
//!
//! This script compiles Protocol Buffer definitions to Rust code using tonic-build.
//! The generated code is placed in `src/generated/` directory.
//!
//! ## Proto Structure
//!
//! The proto files are organized following DDD patterns:
//! - `proto/domain/entity/` - Domain entities
//! - `proto/domain/repository/` - Repository service definitions
//! - `proto/domain/usecase/` - Use case commands and queries (CQRS)
//!
//! ## Usage
//!
//! Proto compilation is disabled by default. To enable:
//! Set BUCKET_COMPILE_PROTOS=1 environment variable

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Proto compilation is DISABLED by default because:
    // 1. Proto files use buf/validate which requires buf CLI setup
    // 2. The module uses Rust-native domain entities instead
    //
    // To enable proto compilation:
    // 1. Install buf CLI and run `buf mod update` in proto/ directory
    // 2. Set BUCKET_COMPILE_PROTOS=1 environment variable
    if std::env::var("BUCKET_COMPILE_PROTOS").is_ok() {
        compile_protos()?;
    } else {
        println!("cargo:warning=Proto compilation disabled. Using Rust domain entities.");
    }

    Ok(())
}

fn compile_protos() -> Result<(), Box<dyn std::error::Error>> {
    use std::path::PathBuf;

    // Only regenerate if proto files changed
    println!("cargo:rerun-if-changed=proto/");

    // Get the output directory
    let out_dir = PathBuf::from("src/generated");

    // Ensure output directory exists
    std::fs::create_dir_all(&out_dir)?;

    // Collect all proto files from domain layer
    let proto_files = collect_proto_files("proto/domain")?;

    if proto_files.is_empty() {
        println!("cargo:warning=No proto files found in proto/domain/");
        return Ok(());
    }

    println!("cargo:warning=Compiling {} proto files...", proto_files.len());

    // Configure tonic-build for better code generation
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .build_transport(true)
        .out_dir(&out_dir)
        .extern_path(".google.protobuf", "::prost_types")
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .field_attribute(".", "#[serde(default)]")
        .compile_protos(&proto_files, &["."])?;

    println!("cargo:warning=Proto compilation completed successfully");

    Ok(())
}

/// Recursively collect all .proto files from a directory
fn collect_proto_files(dir: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    use std::path::PathBuf;

    let mut proto_files = Vec::new();
    let path = PathBuf::from(dir);

    if !path.exists() {
        println!("cargo:warning=Proto directory does not exist: {}", dir);
        return Ok(proto_files);
    }

    collect_proto_files_recursive(&path, &mut proto_files)?;

    Ok(proto_files)
}

fn collect_proto_files_recursive(
    dir: &std::path::Path,
    files: &mut Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    if dir.is_dir() {
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                collect_proto_files_recursive(&path, files)?;
            } else if let Some(ext) = path.extension() {
                if ext == "proto" {
                    if let Some(path_str) = path.to_str() {
                        files.push(path_str.to_string());
                        println!("cargo:rerun-if-changed={}", path_str);
                    }
                }
            }
        }
    }

    Ok(())
}
