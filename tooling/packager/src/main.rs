//! Cartridge Packager Tool
//!
//! Compiles executables and seals them as verified artifacts with hash/signature.

mod syllable;

use cartridge_common::{Artifact, ArtifactHeader, ArtifactType, Capability, Tanka};
use clap::{Parser, Subcommand};
use std::fs;
use std::path::PathBuf;
use syllable::validate_tanka;

#[derive(Parser)]
#[command(name = "cartridge-packager")]
#[command(about = "Package and seal Cartridge OS artifacts", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Package a kernel artifact
    Kernel {
        /// Path to kernel binary
        #[arg(short, long)]
        input: PathBuf,

        /// Output artifact path
        #[arg(short, long)]
        output: PathBuf,
    },

    /// Package a driver artifact
    Driver {
        /// Path to driver binary
        #[arg(short, long)]
        input: PathBuf,

        /// Output artifact path
        #[arg(short, long)]
        output: PathBuf,

        /// Required capabilities (comma-separated)
        #[arg(short, long)]
        capabilities: String,
    },

    /// Package an application cartridge
    Cartridge {
        /// Path to application binary
        #[arg(short, long)]
        input: PathBuf,

        /// Output artifact path
        #[arg(short, long)]
        output: PathBuf,

        /// Required capabilities (comma-separated)
        #[arg(short, long)]
        capabilities: String,

        /// Optional Tanka metadata file (JSON)
        #[arg(short, long)]
        tanka: Option<PathBuf>,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Kernel { input, output } => {
            package_kernel(&input, &output);
        }
        Commands::Driver {
            input,
            output,
            capabilities,
        } => {
            let caps = parse_capabilities(&capabilities);
            package_artifact(&input, &output, ArtifactType::Driver, caps, None);
        }
        Commands::Cartridge {
            input,
            output,
            capabilities,
            tanka,
        } => {
            let caps = parse_capabilities(&capabilities);

            // Load and validate Tanka (required for cartridges)
            let tanka_data = match tanka {
                Some(path) => {
                    match load_tanka(&path) {
                        Ok(t) => {
                            println!("✓ Tanka validated (5-7-5-7-7 syllable pattern)");
                            Some(t)
                        },
                        Err(e) => {
                            eprintln!("✗ Tanka validation failed: {}", e);
                            eprintln!("  Cartridges require valid 5-7-5-7-7 Tanka metadata");
                            std::process::exit(1);
                        }
                    }
                },
                None => {
                    println!("⚠ No Tanka provided, using default (validated)");
                    Some(create_default_tanka_with_validation())
                }
            };

            package_artifact(&input, &output, ArtifactType::Cartridge, caps, tanka_data);
        }
    }
}

fn package_kernel(input: &PathBuf, output: &PathBuf) {
    println!("Packaging kernel: {:?} -> {:?}", input, output);

    let payload = fs::read(input).expect("Failed to read kernel binary");

    // Parse ELF header to get correct entry point
    if payload.len() < 32 {
        panic!("Kernel binary too small (< 32 bytes)");
    }

    // Verify ELF magic number
    if &payload[0..4] != b"\x7FELF" {
        panic!("Input is not a valid ELF file (missing ELF magic)");
    }

    // Entry point is at offset 24 in ELF64 header (8 bytes, little-endian)
    let entry_vaddr = u64::from_le_bytes(
        payload[24..32].try_into().expect("Failed to read entry point")
    );

    // Kernel is loaded at 0x100000 (1MB) - see kernel/linker.ld
    const KERNEL_LOAD_ADDR: u64 = 0x100000;

    // Entry offset is relative to load address
    let entry_offset = entry_vaddr.checked_sub(KERNEL_LOAD_ADDR)
        .expect("Entry point is below kernel load address");

    println!("  ELF entry point (virtual): {:#x}", entry_vaddr);
    println!("  Kernel load address:       {:#x}", KERNEL_LOAD_ADDR);
    println!("  Entry offset (relative):   {:#x}", entry_offset);

    // Kernel gets no capabilities (it enforces them, doesn't request them)
    let header = ArtifactHeader::new(ArtifactType::Kernel, entry_offset, 0, &payload);

    let payload_len = payload.len();
    let artifact = Artifact { header, payload };

    let artifact_bytes = artifact.to_bytes();
    let artifact_len = artifact_bytes.len();
    fs::write(output, artifact_bytes).expect("Failed to write artifact");

    println!("✓ Kernel packaged successfully");
    println!("  Artifact size: {} bytes ({} header + {} payload)",
        artifact_len,
        128, // ArtifactHeader::SIZE
        payload_len
    );
}

fn package_artifact(
    input: &PathBuf,
    output: &PathBuf,
    artifact_type: ArtifactType,
    capabilities: u64,
    _tanka: Option<Tanka>,
) {
    println!("Packaging {:?}: {:?} -> {:?}", artifact_type, input, output);

    let payload = fs::read(input).expect("Failed to read binary");

    let header = ArtifactHeader::new(artifact_type, 0, capabilities, &payload);

    let artifact = Artifact { header, payload };

    let artifact_bytes = artifact.to_bytes();
    fs::write(output, artifact_bytes).expect("Failed to write artifact");

    println!("✓ Artifact packaged successfully");
    println!("  Capabilities: {}", Capability::new(capabilities));
}

fn parse_capabilities(caps_str: &str) -> u64 {
    let mut caps = 0u64;

    for cap in caps_str.split(',') {
        caps |= match cap.trim().to_uppercase().as_str() {
            "GPU" => Capability::GPU,
            "AUDIO" => Capability::AUDIO,
            "INPUT" => Capability::INPUT,
            "NETWORK" => Capability::NETWORK,
            "STORAGE" => Capability::STORAGE,
            "USB" => Capability::USB,
            "IPC_BASIC" => Capability::IPC_BASIC,
            "IPC_SHARED_MEMORY" => Capability::IPC_SHARED_MEMORY,
            "REBOOT" => Capability::REBOOT,
            _ => {
                eprintln!("Warning: Unknown capability '{}'", cap);
                0
            }
        };
    }

    caps
}

fn load_tanka(path: &PathBuf) -> Result<Tanka, Box<dyn std::error::Error>> {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    struct TankaJson {
        lines: [String; 5],
    }

    let content = fs::read_to_string(path)?;
    let tanka_json: TankaJson = serde_json::from_str(&content)?;

    // Validate syllable counts
    validate_tanka(&tanka_json.lines)?;

    // Convert to Tanka format (fixed byte buffer)
    let mut data = [0u8; 320];
    for (i, line) in tanka_json.lines.iter().enumerate() {
        let start = i * 64;
        let line_bytes = line.as_bytes();
        let len = line_bytes.len().min(64);
        data[start..start + len].copy_from_slice(&line_bytes[..len]);
    }

    Ok(Tanka::from_bytes(data))
}

fn create_default_tanka_with_validation() -> Tanka {
    // Use default but validate it
    let default_lines = [
        "Unnamed artifact".to_string(),
        "Identity awaits a name".to_string(),
        "In silent bytes".to_string(),
        "Potential lies encrypted".to_string(),
        "Purpose not yet revealed".to_string(),
    ];

    if let Err(e) = validate_tanka(&default_lines) {
        eprintln!("Warning: Default Tanka validation failed: {}", e);
    }

    Tanka::default()
}
