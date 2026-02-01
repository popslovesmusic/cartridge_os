//! Cartridge Switcher
//!
//! Discovers, verifies, and loads driver modules and application cartridges.
//! Acts as the userspace component that orchestrates the boot process.

#![no_std]
#![no_main]

use cartridge_common::{Artifact, ArtifactType, Capability};

/// Switcher entry point (called by kernel)
#[no_mangle]
pub extern "C" fn _start() -> ! {
    switcher_log("Cartridge Switcher started");

    // Step 1: Discover available cartridges
    let cartridges = discover_cartridges();

    // Step 2: Select cartridge to boot (for now, just first one)
    if let Some(cartridge_path) = cartridges.first() {
        boot_cartridge(cartridge_path);
    } else {
        switcher_log("No cartridges found - entering idle mode");
    }

    loop {
        // Wait for user input or system events
    }
}

/// Discover cartridges from storage partition
fn discover_cartridges() -> Vec<&'static str> {
    // TODO: Scan cartridge partition for .cart files
    // TODO: Parse manifest to get cartridge metadata
    Vec::new()
}

/// Boot a specific cartridge
fn boot_cartridge(path: &str) -> ! {
    switcher_log(&alloc::format!("Loading cartridge: {}", path));

    // Step 1: Load cartridge artifact
    let cartridge_data = load_cartridge(path);

    // Step 2: Verify artifact integrity
    let artifact = match Artifact::from_bytes(&cartridge_data) {
        Ok(art) => art,
        Err(_) => {
            switcher_log("Cartridge verification failed");
            return retry_or_panic();
        }
    };

    if artifact.header.artifact_type != ArtifactType::Cartridge as u8 {
        switcher_log("Invalid artifact type");
        return retry_or_panic();
    }

    // Step 3: Check required capabilities
    let required_caps = Capability::new(artifact.header.capabilities);
    switcher_log(&alloc::format!("Required capabilities: {}", required_caps));

    // Step 4: Load required drivers
    load_required_drivers(&required_caps);

    // Step 5: Request kernel to allocate isolated memory and map cartridge
    // TODO: Syscall to kernel to allocate cartridge region

    // Step 6: Transfer control to cartridge entry point
    jump_to_cartridge(&artifact);

    loop {}
}

fn load_cartridge(path: &str) -> Vec<u8> {
    // TODO: Read cartridge file from storage
    Vec::new()
}

fn load_required_drivers(caps: &Capability) {
    // TODO: For each capability, load corresponding driver
    // TODO: Verify driver artifacts
    // TODO: Start driver processes
}

fn jump_to_cartridge(artifact: &Artifact) -> ! {
    switcher_log("Transferring control to cartridge...");

    // TODO: Syscall to kernel to execute cartridge at entry point

    loop {}
}

fn retry_or_panic() -> ! {
    switcher_log("Fatal error - halting");
    loop {}
}

fn switcher_log(msg: &str) {
    // TODO: Output via kernel logging syscall
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

extern crate alloc;
