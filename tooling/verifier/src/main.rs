//! Cartridge Verifier Tool
//!
//! Verifies artifact integrity and displays metadata.

use cartridge_common::{Artifact, ArtifactType, Capability};
use clap::Parser;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "cartridge-verifier")]
#[command(about = "Verify Cartridge OS artifact integrity", long_about = None)]
struct Cli {
    /// Path to artifact file
    artifact: PathBuf,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() {
    let cli = Cli::parse();

    println!("Verifying artifact: {:?}", cli.artifact);

    let data = fs::read(&cli.artifact).expect("Failed to read artifact file");

    match Artifact::from_bytes(&data) {
        Ok(artifact) => {
            println!("✓ Artifact verified successfully\n");
            display_artifact_info(&artifact, cli.verbose);
        }
        Err(err) => {
            eprintln!("✗ Verification failed: {:?}", err);
            std::process::exit(1);
        }
    }
}

fn display_artifact_info(artifact: &Artifact, verbose: bool) {
    let header = &artifact.header;

    println!("Artifact Information:");
    println!("---------------------");

    let artifact_type = match header.artifact_type {
        0 => "Kernel",
        1 => "Driver",
        2 => "Cartridge",
        _ => "Unknown",
    };
    println!("Type:         {}", artifact_type);
    println!("Version:      {}", header.version);
    println!("Entry Offset: 0x{:x}", header.entry_offset);
    println!("Payload Size: {} bytes", header.payload_size);

    let caps = Capability::new(header.capabilities);
    println!("Capabilities: {}", caps);

    if verbose {
        println!("\nPayload Hash:");
        print!("  ");
        for byte in &header.payload_hash {
            print!("{:02x}", byte);
        }
        println!("\n");
    }
}
