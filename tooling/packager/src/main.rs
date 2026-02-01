//! Cartridge Packager Tool
//!
//! Compiles executables and seals them as verified artifacts with hash/signature.

use cartridge_common::{Artifact, ArtifactHeader, ArtifactType, Capability, Tanka};
use clap::{Parser, Subcommand};
use std::fs;
use std::path::PathBuf;

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
            let tanka_data = tanka.map(|path| load_tanka(&path));
            package_artifact(&input, &output, ArtifactType::Cartridge, caps, tanka_data);
        }
    }
}

fn package_kernel(input: &PathBuf, output: &PathBuf) {
    println!("Packaging kernel: {:?} -> {:?}", input, output);

    let payload = fs::read(input).expect("Failed to read kernel binary");

    // Kernel gets no capabilities (it enforces them, doesn't request them)
    let header = ArtifactHeader::new(ArtifactType::Kernel, 0, 0, &payload);

    let artifact = Artifact { header, payload };

    let artifact_bytes = artifact.to_bytes();
    fs::write(output, artifact_bytes).expect("Failed to write artifact");

    println!("✓ Kernel packaged successfully");
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

fn load_tanka(path: &PathBuf) -> Tanka {
    // TODO: Load Tanka from JSON file
    Tanka::default()
}
